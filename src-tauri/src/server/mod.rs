use chrono::Utc;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Arc;
use tiny_http::{Header, Method, Request, Response, Server};

use crate::AppState;

fn compute_checksum(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("sha256:{}", hex::encode(&hasher.finalize()[..12]))
}

fn json_response(body: &str) -> Response<Cursor<Vec<u8>>> {
    let data = body.as_bytes().to_vec();
    Response::from_data(data)
        .with_header(Header::from_bytes("Content-Type", "application/json").unwrap())
        .with_header(Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap())
}

fn read_body(request: &mut Request) -> String {
    let mut body = String::new();
    let _ = request.as_reader().read_to_string(&mut body);
    body
}

pub fn start_server(state: Arc<AppState>, frontend_dir: PathBuf) {
    let port = state.server_port();
    let addr = format!("127.0.0.1:{}", port);

    let server = match Server::http(&addr) {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to start HTTP server: {}", e);
            return;
        }
    };

    log::info!("HTTP server running on http://{}", addr);

    for mut request in server.incoming_requests() {
        let url = request.url().to_string();
        let method = request.method().clone();

        // CORS preflight
        if method == Method::Options {
            let response = Response::empty(200)
                .with_header(Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap())
                .with_header(Header::from_bytes("Access-Control-Allow-Methods", "GET, POST, OPTIONS").unwrap())
                .with_header(Header::from_bytes("Access-Control-Allow-Headers", "Content-Type").unwrap());
            let _ = request.respond(response);
            continue;
        }

        let response = match (method, url.as_str()) {
            (Method::Get, "/api/health") => handle_health(&state),
            (Method::Get, "/api/backlog") => handle_load_backlog(&state),
            (Method::Post, "/api/backlog") => handle_save_backlog(&state, &mut request),
            (Method::Get, "/api/backups") => handle_list_backups(&state),
            (Method::Post, "/api/backups/restore") => handle_restore_backup(&state, &mut request),
            (Method::Get, "/api/archive") => handle_load_archive(&state),
            (Method::Post, "/api/archive") => handle_save_archive(&state, &mut request),
            (Method::Post, "/api/archive/restore") => handle_save_archive(&state, &mut request),
            (Method::Post, "/api/config") => handle_set_config(&state, &mut request),
            (Method::Get, path) => serve_static(&frontend_dir, path),
            _ => json_response(r#"{"error": "Not found"}"#),
        };

        let _ = request.respond(response);
    }
}

fn serve_static(frontend_dir: &PathBuf, path: &str) -> Response<Cursor<Vec<u8>>> {
    let file_path = if path == "/" {
        frontend_dir.join("index.html")
    } else {
        frontend_dir.join(path.trim_start_matches('/'))
    };

    match fs::read(&file_path) {
        Ok(content) => {
            let mime = if file_path.extension().map(|e| e == "html").unwrap_or(false) {
                "text/html; charset=utf-8"
            } else if file_path.extension().map(|e| e == "css").unwrap_or(false) {
                "text/css"
            } else if file_path.extension().map(|e| e == "js").unwrap_or(false) {
                "application/javascript"
            } else {
                "application/octet-stream"
            };
            Response::from_data(content)
                .with_header(Header::from_bytes("Content-Type", mime).unwrap())
        }
        Err(_) => {
            // Fallback to index.html for SPA routing
            if let Ok(content) = fs::read(frontend_dir.join("index.html")) {
                Response::from_data(content)
                    .with_header(Header::from_bytes("Content-Type", "text/html; charset=utf-8").unwrap())
            } else {
                Response::from_string("Not Found").with_status_code(404)
            }
        }
    }
}

fn handle_health(state: &Arc<AppState>) -> Response<Cursor<Vec<u8>>> {
    let data_dir = state.data_dir();

    let (master_path, master_size) = data_dir
        .as_ref()
        .and_then(|d| {
            let p = d.join("backlog.md");
            fs::metadata(&p).ok().map(|m| (p.to_string_lossy().to_string(), m.len()))
        })
        .unwrap_or_default();

    let backups_path = data_dir
        .as_ref()
        .map(|d| d.join("backups").to_string_lossy().to_string())
        .unwrap_or_default();

    let (archive_path, archive_exists, archive_size) = data_dir
        .as_ref()
        .map(|d| {
            let p = d.join("archive.md");
            let exists = p.exists();
            let size = fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
            (p.to_string_lossy().to_string(), exists, size)
        })
        .unwrap_or_default();

    let body = serde_json::json!({
        "ok": data_dir.is_some(),
        "masterPath": master_path,
        "masterSize": master_size,
        "backupsPath": backups_path,
        "archiveExists": archive_exists,
        "archivePath": archive_path,
        "archiveSize": archive_size
    });

    json_response(&body.to_string())
}

fn handle_load_backlog(state: &Arc<AppState>) -> Response<Cursor<Vec<u8>>> {
    let Some(path) = state.backlog_path() else {
        return json_response(r#"{"error": "Data directory not set"}"#);
    };

    if !path.exists() {
        let default = "# Backlog\n\n<!-- SECTION: ENTRIES -->\n\n<!-- SECTION: HISTORY -->\n\n| Timestamp | Item ID | Action | Details |\n|-----------|---------|--------|--------|\n\n<!-- SECTION: INTEGRITY -->\n\n<!-- saved: | checksum: | entries: 0 | history: 0 -->\n";
        if let Err(e) = fs::write(&path, default) {
            return json_response(&format!(r#"{{"error": "Failed to create: {}"}}"#, e));
        }
    }

    match fs::read_to_string(&path) {
        Ok(content) => {
            let checksum = compute_checksum(&content);
            let body = serde_json::json!({ "content": content, "checksum": checksum });
            json_response(&body.to_string())
        }
        Err(e) => json_response(&format!(r#"{{"error": "Failed to load: {}"}}"#, e)),
    }
}

fn handle_save_backlog(state: &Arc<AppState>, request: &mut Request) -> Response<Cursor<Vec<u8>>> {
    let body = read_body(request);
    let json: serde_json::Value = match serde_json::from_str(&body) {
        Ok(v) => v,
        Err(_) => return json_response(r#"{"error": "Invalid JSON"}"#),
    };

    let content = json["content"].as_str().unwrap_or("");

    let Some(path) = state.backlog_path() else {
        return json_response(r#"{"error": "Data directory not set"}"#);
    };

    let Some(backup_dir) = state.backup_dir() else {
        return json_response(r#"{"error": "Data directory not set"}"#);
    };

    if !backup_dir.exists() {
        let _ = fs::create_dir_all(&backup_dir);
    }

    if path.exists() {
        let now = Utc::now();
        let backup_name = format!("backlog_{}.md", now.format("%Y%m%d_%H%M%S"));
        let _ = fs::copy(&path, backup_dir.join(&backup_name));
    }

    match fs::write(&path, content) {
        Ok(()) => {
            let checksum = compute_checksum(content);
            let saved = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
            let body = serde_json::json!({ "ok": true, "checksum": checksum, "saved": saved });
            json_response(&body.to_string())
        }
        Err(e) => json_response(&format!(r#"{{"error": "Failed to save: {}"}}"#, e)),
    }
}

fn handle_list_backups(state: &Arc<AppState>) -> Response<Cursor<Vec<u8>>> {
    let Some(backup_dir) = state.backup_dir() else {
        return json_response(r#"{"backups": []}"#);
    };

    let mut backups = Vec::new();
    if let Ok(entries) = fs::read_dir(&backup_dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.starts_with("backlog_") && name.ends_with(".md") {
                    backups.push(name.to_string());
                }
            }
        }
    }
    backups.sort();
    backups.reverse();

    let body = serde_json::json!({ "backups": backups });
    json_response(&body.to_string())
}

fn handle_restore_backup(state: &Arc<AppState>, request: &mut Request) -> Response<Cursor<Vec<u8>>> {
    let body = read_body(request);
    let json: serde_json::Value = match serde_json::from_str(&body) {
        Ok(v) => v,
        Err(_) => return json_response(r#"{"ok": false, "error": "Invalid JSON"}"#),
    };

    let name = json["name"].as_str().unwrap_or("");

    let Some(backup_dir) = state.backup_dir() else {
        return json_response(r#"{"ok": false, "error": "No backup dir"}"#);
    };
    let Some(backlog_path) = state.backlog_path() else {
        return json_response(r#"{"ok": false, "error": "No data dir"}"#);
    };

    let backup_path = backup_dir.join(name);
    if !backup_path.exists() {
        return json_response(r#"{"ok": false, "error": "Backup not found"}"#);
    }

    match fs::read_to_string(&backup_path) {
        Ok(content) => {
            if let Err(e) = fs::write(&backlog_path, &content) {
                return json_response(&format!(r#"{{"ok": false, "error": "Restore failed: {}"}}"#, e));
            }
            let checksum = compute_checksum(&content);
            let body = serde_json::json!({"ok": true, "content": content, "checksum": checksum});
            json_response(&body.to_string())
        }
        Err(e) => json_response(&format!(r#"{{"ok": false, "error": "Read failed: {}"}}"#, e)),
    }
}

fn handle_load_archive(state: &Arc<AppState>) -> Response<Cursor<Vec<u8>>> {
    let Some(path) = state.archive_path() else {
        return json_response(r#"{"error": "Data directory not set"}"#);
    };

    if !path.exists() {
        return json_response(r#"{"content": "", "checksum": "", "exists": false}"#);
    }

    match fs::read_to_string(&path) {
        Ok(content) => {
            let checksum = compute_checksum(&content);
            let body = serde_json::json!({ "content": content, "checksum": checksum, "exists": true });
            json_response(&body.to_string())
        }
        Err(e) => json_response(&format!(r#"{{"error": "Failed to load: {}"}}"#, e)),
    }
}

fn handle_save_archive(state: &Arc<AppState>, request: &mut Request) -> Response<Cursor<Vec<u8>>> {
    let body = read_body(request);
    let json: serde_json::Value = match serde_json::from_str(&body) {
        Ok(v) => v,
        Err(_) => return json_response(r#"{"ok": false}"#),
    };

    let backlog = json["backlog"].as_str().unwrap_or("");
    let archive = json["archive"].as_str().unwrap_or("");

    let Some(backlog_path) = state.backlog_path() else {
        return json_response(r#"{"ok": false}"#);
    };
    let Some(archive_path) = state.archive_path() else {
        return json_response(r#"{"ok": false}"#);
    };

    if let Err(e) = fs::write(&backlog_path, backlog) {
        return json_response(&format!(r#"{{"ok": false, "error": "{}"}}"#, e));
    }
    if let Err(e) = fs::write(&archive_path, archive) {
        return json_response(&format!(r#"{{"ok": false, "error": "{}"}}"#, e));
    }

    json_response(r#"{"ok": true}"#)
}

fn handle_set_config(state: &Arc<AppState>, request: &mut Request) -> Response<Cursor<Vec<u8>>> {
    let body = read_body(request);
    let json: serde_json::Value = match serde_json::from_str(&body) {
        Ok(v) => v,
        Err(_) => return json_response(r#"{"ok": false}"#),
    };

    let data_dir = json["data_dir"].as_str().unwrap_or("");

    match state.set_data_dir(data_dir) {
        Ok(()) => json_response(r#"{"ok": true}"#),
        Err(e) => json_response(&format!(r#"{{"ok": false, "error": "{}"}}"#, e)),
    }
}
