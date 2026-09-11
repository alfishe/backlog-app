# Backlog

<!-- SECTION: ENTRIES -->

- [/] [P0] Ship v2 of personal backlog app *(due: 2026-09-07, tags: build ui, priority: P0, progress: 60)*
  > ## Goals
  > - Single-file build
  > - **Zero** runtime deps
  > 
  > See `bundle.js` for the pipeline.
  - [x] [P0] Markdown parser with integrity checksum *(due: 2026-08-06, tags: build, priority: P0, progress: 100)*
    > Uses SHA-256 over entries + history.
    - [x] [P0] Tokenize three sections *(priority: P0, progress: 100)*
    - [x] [P1] Checksum mismatch banner *(progress: 100)*
  - [/] [P0] Tree view with drag & drop *(due: 2026-08-29, tags: ui, priority: P0, progress: 55)*
    > ### Remaining
    > 1. Keyboard reordering
    > 2. Multi-select drag
    > 
    > > Note: only within the same priority group.
    - [ ] [P1] Expand / collapse animations *(tags: ui, progress: 25)*
    - [!] [P1] Drag handle for touch devices *(reason: Need a real iPad to test, tags: ui)*
  - [!] [P1] Cross-browser File System Access *(due: 2026-09-10, reason: Waiting for Safari 27 API, tags: research, progress: 40)*
    > Chrome only for now. Firefox & Safari fall back to *download/upload*.
    > 
    > | Browser | Status |
    > |---|---|
    > | Chrome | ok |
    > | Firefox | fallback |
    > | Safari | fallback |
  - [ ] [P2] Light / dark / system theme switcher *(due: 2026-08-27, tags: ui, priority: P2)*
    > Persist in `localStorage` under tweaks.
  - [>] [P3] Keyboard shortcut cheat-sheet *(tags: ui docs, priority: P3, progress: 10)*
  - [ ] [P2] Hover peek popover for rows *(due: 2026-08-26, tags: ui, priority: P2)*
    > Show after **1s** of no mouse movement.
    > - title, status, due
    > - tags, progress
    > - rich text body
- [/] [P1] Home renovation *(due: 2026-10-25, tags: home, progress: 30)*
  > Budget: **$18k**. Contractor: *Mike*.
  - [x] [P1] Get three quotes for kitchen *(due: 2026-07-27, progress: 100)*
  - [/] [P1] Order cabinets *(due: 2026-09-02, tags: home shopping, progress: 50)*
    > Model: `Sektion` white. Lead time 6 weeks.
  - [ ] [P2] Pick backsplash tile *(due: 2026-09-09, tags: home, priority: P2)*
  - [ ] [P2] Repaint hallway *(tags: home, priority: P2)*
  - [!] [P1] Replace bathroom fan *(reason: Electrician booked for next month, tags: home)*
  - [-] [P3] Install smart thermostat *(tags: home, priority: P3)*
    > Landlord said *no*.
  - [>] [P3] Garden shed *(due: 2026-12-24, tags: home outdoor, priority: P3)*
- [ ] [P1] Health & fitness *(tags: health)*
  - [/] [P1] Run 5k under 25 min *(due: 2026-10-10, tags: health running, progress: 65)*
    > Current PB: **26:40**.
    > 
    > Plan:
    > - Tue intervals
    > - Thu tempo
    > - Sun long run
  - [ ] [P2] Annual dental checkup *(due: 2026-08-23, tags: health, priority: P2)*
  - [x] [P2] Book eye exam *(due: 2026-08-16, tags: health, priority: P2, progress: 100)*
  - [ ] [P3] Try a climbing gym *(tags: health fun, priority: P3)*
  - [>] [P3] Meditation habit — 10 min daily *(tags: health, priority: P3, progress: 15)*
    > Use the *Waking Up* app.
- [/] [P0] Finances Q3 *(due: 2026-09-15, tags: finance, priority: P0, progress: 40)*
  > Deadline for tax estimate is **Sep 15**.
  - [ ] [P0] Pay estimated taxes *(due: 2026-09-15, tags: finance, priority: P0)*
  - [x] [P1] Rebalance index portfolio *(due: 2026-07-17, tags: finance, progress: 100)*
  - [ ] [P2] Review subscriptions and cancel unused *(tags: finance, priority: P2)*
    > Suspects: `Spotify family`, two VPNs, a gym I never visit.
  - [!] [P1] Refinance mortgage *(reason: Rates still above 6%, tags: finance)*
  - [ ] [P3] Open kids’ savings account *(tags: finance family, priority: P3)*
- [ ] [P2] Learning *(tags: learning, priority: P2)*
  - [/] [P2] Read “Designing Data-Intensive Applications” *(tags: learning books, priority: P2, progress: 58)*
    > Chapter **7/12**. Notes in Obsidian.
  - [ ] [P2] Rust course — ownership chapter *(due: 2026-09-25, tags: learning code, priority: P2)*
    > ```rust
    > let s = String::from("hi");
    > let t = s; // moved
    > ```
  - [ ] [P3] Learn 200 Spanish words *(tags: learning language, priority: P3, progress: 35)*
  - [-] [P3] Piano lessons *(tags: learning music, priority: P3)*
    > No time this year.
  - [x] [P2] Finish SQL window functions tutorial *(due: 2026-08-21, tags: learning code, priority: P2, progress: 100)*
- [ ] [P1] Trip to Japan *(due: 2026-11-03, tags: travel, progress: 20)*
  > Dates: **Nov 3–17**. Cities: Tokyo → Kyoto → Osaka.
  - [x] [P0] Book flights *(due: 2026-08-11, tags: travel, priority: P0, progress: 100)*
  - [/] [P1] Book hotels *(due: 2026-09-05, tags: travel, progress: 33)*
    > - [x] Tokyo
    > - [ ] Kyoto
    > - [ ] Osaka
  - [ ] [P1] Renew passport *(due: 2026-08-25, tags: travel admin)*
    > Expires in **5 months** — Japan requires 6.
  - [ ] [P2] JR Pass or individual tickets? *(tags: travel research, priority: P2)*
  - [ ] [P3] Learn 20 phrases *(tags: travel language, priority: P3)*
  - [>] [P3] Day trip to Nara *(tags: travel, priority: P3)*
- [ ] [P2] Side project: recipe scraper *(tags: code side, priority: P2, progress: 10)*
  > Scrape → normalise → export to *Paprika*.
  - [/] [P2] Write parser for schema.org Recipe *(tags: code, priority: P2, progress: 45)*
    > Handle `@graph` wrappers too.
  - [ ] [P2] CLI with `--out` flag *(tags: code, priority: P2)*
  - [ ] [P3] Publish to PyPI *(tags: code, priority: P3)*
  - [!] [P2] Handle paywalled sites *(reason: Legal question unresolved, tags: code, priority: P2)*
- [ ] [P3] Misc admin *(tags: admin, priority: P3)*
  - [ ] [P2] Renew car registration *(due: 2026-08-31, tags: admin, priority: P2)*
  - [ ] [P3] Sort photo library 2019–2021 *(tags: admin photos, priority: P3, progress: 5)*
  - [x] [P3] Cancel old phone plan *(due: 2026-06-27, tags: admin, priority: P3, progress: 100)*
  - [ ] [P3] Back up NAS to cold storage *(tags: admin tech, priority: P3)*
    > Use `restic` to B2. Verify with `restic check`.
  - [-] [P3] Buy a label printer *(tags: admin shopping, priority: P3)*
  - [ ] [P2] Write a will *(due: 2026-11-24, tags: admin family, priority: P2)*

<!-- SECTION: HISTORY -->

| Timestamp | Item ID | Action | Details |
|-----------|---------|--------|---------|
| 2026-08-26T09:00:00Z | i-0 | item_created | Ship v2 of personal backlog app |
| 2026-08-26T02:00:00Z | i-1 | item_created | Markdown parser with integrity checksum |
| 2026-08-25T19:00:00Z | i-2 | item_created | Tokenize three sections |
| 2026-08-25T12:00:00Z | i-3 | item_created | Checksum mismatch banner |
| 2026-08-25T05:00:00Z | i-4 | item_created | Tree view with drag & drop |
| 2026-08-24T22:00:00Z | i-5 | item_created | Expand / collapse animations |
| 2026-08-24T15:00:00Z | i-6 | item_created | Drag handle for touch devices |
| 2026-08-24T08:00:00Z | i-7 | item_created | Cross-browser File System Access |
| 2026-08-24T01:00:00Z | i-8 | item_created | Light / dark / system theme switcher |
| 2026-08-23T18:00:00Z | i-9 | item_created | Keyboard shortcut cheat-sheet |
| 2026-08-23T11:00:00Z | i-10 | item_created | Hover peek popover for rows |
| 2026-08-23T04:00:00Z | i-11 | item_created | Home renovation |

<!-- SECTION: INTEGRITY -->

<!-- saved: 2026-08-26T09:00:00Z | checksum: sha256:71805003fee82b92e0727cf36f22d30cb4c83a921d54c0630534097909ef2b3c | entries: 56 | history: 12 -->
