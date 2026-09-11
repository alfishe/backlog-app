#!/usr/bin/env python3
"""
Generate app icon for Backlog.

Usage:
    python scripts/generate-icon.py

    # Then generate Tauri icons:
    cd backlog-app && npx tauri icon app-icon.png

Requirements:
    pip install Pillow
"""

from PIL import Image, ImageDraw, ImageFont
import math
import os

def draw_rounded_rect(draw, xy, radius, fill=None, outline=None, width=1):
    """Draw a rounded rectangle."""
    x1, y1, x2, y2 = xy
    r = radius

    # Main rectangle (without corners)
    draw.rectangle([x1 + r, y1, x2 - r, y2], fill=fill)
    draw.rectangle([x1, y1 + r, x2, y2 - r], fill=fill)

    # Four corners
    draw.pieslice([x1, y1, x1 + 2*r, y1 + 2*r], 180, 270, fill=fill)
    draw.pieslice([x2 - 2*r, y1, x2, y1 + 2*r], 270, 360, fill=fill)
    draw.pieslice([x1, y2 - 2*r, x1 + 2*r, y2], 90, 180, fill=fill)
    draw.pieslice([x2 - 2*r, y2 - 2*r, x2, y2], 0, 90, fill=fill)

    if outline:
        # Draw outline
        draw.arc([x1, y1, x1 + 2*r, y1 + 2*r], 180, 270, fill=outline, width=width)
        draw.arc([x2 - 2*r, y1, x2, y1 + 2*r], 270, 360, fill=outline, width=width)
        draw.arc([x1, y2 - 2*r, x1 + 2*r, y2], 90, 180, fill=outline, width=width)
        draw.arc([x2 - 2*r, y2 - 2*r, x2, y2], 0, 90, fill=outline, width=width)
        draw.line([x1 + r, y1, x2 - r, y1], fill=outline, width=width)
        draw.line([x1 + r, y2, x2 - r, y2], fill=outline, width=width)
        draw.line([x1, y1 + r, x1, y2 - r], fill=outline, width=width)
        draw.line([x2, y1 + r, x2, y2 - r], fill=outline, width=width)


def draw_checkmark(draw, center, size, color, width):
    """Draw a checkmark."""
    cx, cy = center
    s = size

    # Checkmark points (relative to center)
    points = [
        (cx - s * 0.4, cy),
        (cx - s * 0.1, cy + s * 0.3),
        (cx + s * 0.45, cy - s * 0.35),
    ]
    draw.line(points, fill=color, width=width, joint="curve")


def draw_checkbox(draw, xy, size, checked=False, fill_color=None, border_color=None, check_color=None):
    """Draw a checkbox."""
    x, y = xy
    s = size
    r = s // 6  # corner radius

    if checked and fill_color:
        draw_rounded_rect(draw, [x, y, x + s, y + s], r, fill=fill_color)
        if check_color:
            draw_checkmark(draw, (x + s//2, y + s//2), s * 0.4, check_color, max(2, s // 10))
    else:
        draw_rounded_rect(draw, [x, y, x + s, y + s], r, outline=border_color, width=max(2, s // 16))


def generate_icon(size=1024, output_path="app-icon.png"):
    """Generate the backlog app icon."""

    # Colors (warm paper theme)
    bg_color = (247, 243, 236)        # Warm cream background
    paper_color = (255, 255, 255)      # White paper
    ink_color = (58, 53, 48)           # Dark brown ink
    accent_color = (180, 130, 70)      # Warm amber accent
    success_color = (90, 140, 90)      # Muted green for checks
    shadow_color = (220, 215, 205)     # Subtle shadow

    # Create image with transparent background
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    # Paper card (no background circle - just the card)
    card_margin = size // 10
    card_x1 = card_margin
    card_y1 = card_margin
    card_x2 = size - card_margin
    card_y2 = size - card_margin
    card_radius = size // 16

    # Shadow
    shadow_offset = size // 64
    draw_rounded_rect(draw,
        [card_x1 + shadow_offset, card_y1 + shadow_offset,
         card_x2 + shadow_offset, card_y2 + shadow_offset],
        card_radius, fill=shadow_color)

    # Paper card
    draw_rounded_rect(draw, [card_x1, card_y1, card_x2, card_y2], card_radius, fill=paper_color)

    # Header bar (accent stripe at top)
    header_height = size // 12
    draw_rounded_rect(draw,
        [card_x1, card_y1, card_x2, card_y1 + header_height + card_radius],
        card_radius, fill=accent_color)
    draw.rectangle([card_x1, card_y1 + card_radius, card_x2, card_y1 + header_height + card_radius],
                   fill=accent_color)

    # Task rows
    row_start_y = card_y1 + header_height + size // 16
    row_height = size // 8
    checkbox_size = size // 14
    row_padding = size // 20
    line_start_x = card_x1 + row_padding + checkbox_size + size // 24
    line_end_x = card_x2 - row_padding

    tasks = [
        (True, 0.95),   # Done task, full width
        (True, 0.75),   # Done task, shorter
        (False, 0.85),  # Open task
        (False, 0.6),   # Open task, shorter
    ]

    for i, (checked, width_ratio) in enumerate(tasks):
        y = row_start_y + i * row_height
        checkbox_y = y + (row_height - checkbox_size) // 2

        # Checkbox
        if checked:
            draw_checkbox(draw,
                (card_x1 + row_padding, checkbox_y),
                checkbox_size,
                checked=True,
                fill_color=success_color,
                check_color=paper_color)
        else:
            draw_checkbox(draw,
                (card_x1 + row_padding, checkbox_y),
                checkbox_size,
                checked=False,
                border_color=ink_color)

        # Task line
        line_y = y + row_height // 2
        line_width = int((line_end_x - line_start_x) * width_ratio)
        line_thickness = max(3, size // 80)

        line_color = (*ink_color[:3], 100) if checked else ink_color
        draw.rounded_rectangle(
            [line_start_x, line_y - line_thickness//2,
             line_start_x + line_width, line_y + line_thickness//2],
            radius=line_thickness//2,
            fill=line_color if not checked else (*ink_color, 120)
        )

        # Strikethrough for completed tasks
        if checked:
            draw.line(
                [line_start_x, line_y, line_start_x + line_width, line_y],
                fill=(*ink_color, 80),
                width=max(2, size // 128)
            )

    # Save
    img.save(output_path, 'PNG')
    print(f"✓ Generated {output_path} ({size}x{size})")
    return output_path


def main():
    # Find project root
    script_dir = os.path.dirname(os.path.abspath(__file__))
    project_root = os.path.dirname(script_dir)

    output_path = os.path.join(project_root, "app-icon.png")
    generate_icon(1024, output_path)

    print(f"\nTo generate all Tauri icons:")
    print(f"  cd {project_root}")
    print(f"  npx tauri icon app-icon.png")


if __name__ == "__main__":
    main()
