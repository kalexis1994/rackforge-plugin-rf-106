#!/usr/bin/env python3
"""Generate the canonical RF-106 branding PNGs used by RackForge.

The artwork is deliberately code-native: it reuses the panel palette and the
visual language of the plugin's faders, LEDs, section strips and display.  No
runtime or host state is baked into the images.
"""

from __future__ import annotations

import math
from pathlib import Path

from PIL import Image, ImageDraw, ImageFilter, ImageFont


ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "plugin" / "package" / "branding"
SEGMENT_FONT = ROOT / "plugin" / "package" / "web" / "assets" / "Segment14.otf"
REGULAR_FONT_CANDIDATES = (
    Path("C:/Windows/Fonts/arial.ttf"),
    Path("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"),
)
BOLD_FONT_CANDIDATES = (
    Path("C:/Windows/Fonts/arialbd.ttf"),
    Path("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf"),
)

METAL = (59, 57, 58)
METAL_HIGHLIGHT = (90, 91, 102)
PANEL_DARK = (22, 22, 24)
PANEL_MID = (43, 43, 47)
WHITE = (242, 242, 238)
MUTED = (185, 186, 190)
RED = (231, 48, 32)
RED_DARK = (144, 24, 26)
BLUE = (59, 164, 219)
BLUE_DARK = (31, 77, 174)
LED_RED = (255, 63, 77)
LED_GREEN = (72, 218, 138)
DISPLAY_RED = (255, 49, 52)


def font(size: int, *, bold: bool = False, segment: bool = False) -> ImageFont.FreeTypeFont:
    if segment:
        return ImageFont.truetype(str(SEGMENT_FONT), size)
    candidates = BOLD_FONT_CANDIDATES if bold else REGULAR_FONT_CANDIDATES
    for candidate in candidates:
        if candidate.is_file():
            return ImageFont.truetype(str(candidate), size)
    return ImageFont.load_default(size=size)


def anisotropic_background(width: int, height: int, strength: float = 0.34) -> Image.Image:
    """Paint the panel metal with one broad, fixed-angle anisotropic highlight."""
    image = Image.new("RGB", (width, height), METAL)
    pixels = image.load()
    mu = width * 0.47
    sigma = width * 0.34
    for x in range(width):
        gaussian = math.exp(-((x - mu) ** 2) / (2.0 * sigma * sigma))
        for y in range(height):
            vertical = 0.94 + 0.06 * math.cos((y / max(height - 1, 1)) * math.pi)
            brushed = 0.012 * math.sin(x * 0.21 + y * 0.031)
            mix = max(0.0, min(1.0, strength * gaussian * vertical + brushed))
            pixels[x, y] = tuple(
                round(METAL[channel] + (METAL_HIGHLIGHT[channel] - METAL[channel]) * mix)
                for channel in range(3)
            )
    draw = ImageDraw.Draw(image, "RGBA")
    for y in range(1, height, 4):
        draw.line((0, y, width, y), fill=(255, 255, 255, 5), width=1)
    return image


def centered_text(
    draw: ImageDraw.ImageDraw,
    box: tuple[int, int, int, int],
    text: str,
    text_font: ImageFont.FreeTypeFont,
    fill: tuple[int, int, int] | tuple[int, int, int, int] = WHITE,
) -> None:
    left, top, right, bottom = box
    bounds = draw.textbbox((0, 0), text, font=text_font)
    width = bounds[2] - bounds[0]
    height = bounds[3] - bounds[1]
    x = left + (right - left - width) / 2
    y = top + (bottom - top - height) / 2 - bounds[1]
    draw.text((x, y), text, font=text_font, fill=fill)


def led(image: Image.Image, center: tuple[int, int], color: tuple[int, int, int], radius: int) -> None:
    x, y = center
    glow = Image.new("RGBA", image.size, (0, 0, 0, 0))
    glow_draw = ImageDraw.Draw(glow)
    glow_draw.ellipse(
        (x - radius * 4, y - radius * 4, x + radius * 4, y + radius * 4),
        fill=(*color, 65),
    )
    glow = glow.filter(ImageFilter.GaussianBlur(radius * 2.2))
    image.paste(glow, (0, 0), glow)
    draw = ImageDraw.Draw(image, "RGBA")
    draw.ellipse((x - radius - 2, y - radius - 2, x + radius + 2, y + radius + 2), fill=(8, 8, 9, 220))
    draw.ellipse((x - radius, y - radius, x + radius, y + radius), fill=(*color, 255))
    draw.ellipse(
        (x - radius * 0.45, y - radius * 0.65, x + radius * 0.15, y - radius * 0.05),
        fill=(255, 255, 255, 155),
    )


def display(image: Image.Image, box: tuple[int, int, int, int], text: str, size: int) -> None:
    draw = ImageDraw.Draw(image, "RGBA")
    left, top, right, bottom = box
    draw.rounded_rectangle(box, radius=max(4, (bottom - top) // 12), fill=(10, 6, 7, 255), outline=(76, 30, 32, 210), width=2)
    glow = Image.new("RGBA", image.size, (0, 0, 0, 0))
    glow_draw = ImageDraw.Draw(glow)
    centered_text(glow_draw, box, text, font(size, segment=True), (*DISPLAY_RED, 150))
    glow = glow.filter(ImageFilter.GaussianBlur(max(2, size // 18)))
    image.paste(glow, (0, 0), glow)
    centered_text(draw, box, text, font(size, segment=True), DISPLAY_RED)


def section_strip(
    draw: ImageDraw.ImageDraw,
    box: tuple[int, int, int, int],
    label: str,
    color: tuple[int, int, int],
    label_size: int,
) -> None:
    draw.rectangle(box, fill=color)
    centered_text(draw, box, label, font(label_size, bold=True), WHITE)


def fader_group(
    image: Image.Image,
    box: tuple[int, int, int, int],
    labels: list[str],
    positions: list[float],
    scale: float = 1.0,
) -> None:
    draw = ImageDraw.Draw(image, "RGBA")
    left, top, right, bottom = box
    width = right - left
    count = len(labels)
    label_height = round(32 * scale)
    track_top = top + label_height + round(12 * scale)
    track_bottom = bottom - round(14 * scale)
    margin = round(25 * scale)
    x_positions = [left + margin + (width - margin * 2) * (index + 0.5) / count for index in range(count)]

    line_left = x_positions[0] - round(25 * scale)
    line_right = x_positions[-1] + round(25 * scale)
    for index in range(11):
        y = track_bottom - (track_bottom - track_top) * index / 10
        major = index in (0, 5, 10)
        draw.line(
            (line_left, y, line_right, y),
            fill=(242, 242, 238, 210 if major else 155),
            width=max(1, round((2.4 if major else 1.5) * scale)),
        )

    for x, label, position in zip(x_positions, labels, positions, strict=True):
        centered_text(
            draw,
            (round(x - 55 * scale), top, round(x + 55 * scale), top + label_height),
            label,
            font(max(10, round(14 * scale)), bold=True),
            WHITE,
        )
        track_width = max(7, round(11 * scale))
        draw.rounded_rectangle(
            (x - track_width / 2, track_top - 5 * scale, x + track_width / 2, track_bottom + 5 * scale),
            radius=track_width / 2,
            fill=(8, 8, 9, 245),
        )
        thumb_y = track_bottom - (track_bottom - track_top) * max(0.0, min(1.0, position))
        thumb_w = round(39 * scale)
        thumb_h = round(22 * scale)
        shadow = (x - thumb_w / 2 + 2, thumb_y - thumb_h / 2 + 3, x + thumb_w / 2 + 2, thumb_y + thumb_h / 2 + 3)
        draw.rounded_rectangle(shadow, radius=max(2, round(3 * scale)), fill=(0, 0, 0, 150))
        cap = (x - thumb_w / 2, thumb_y - thumb_h / 2, x + thumb_w / 2, thumb_y + thumb_h / 2)
        draw.rounded_rectangle(cap, radius=max(2, round(3 * scale)), fill=(30, 30, 32, 255), outline=(8, 8, 9, 255), width=max(1, round(scale)))
        gap = round(7 * scale)
        line_y = thumb_y
        draw.line((x - thumb_w / 2 + 1, line_y, x - gap, line_y), fill=WHITE, width=max(2, round(3 * scale)))
        draw.line((x + gap, line_y, x + thumb_w / 2 - 1, line_y), fill=WHITE, width=max(2, round(3 * scale)))


def wordmark(
    image: Image.Image,
    origin: tuple[int, int],
    height: int,
    *,
    subtitle: bool = True,
) -> tuple[int, int, int, int]:
    draw = ImageDraw.Draw(image, "RGBA")
    x, y = origin
    mark_font = font(height, bold=True)
    bounds = draw.textbbox((x, y), "RF-106", font=mark_font)
    draw.text((x + 3, y + 4), "RF-106", font=mark_font, fill=(0, 0, 0, 120))
    draw.text((x, y), "RF-106", font=mark_font, fill=WHITE)
    if subtitle:
        subtitle_font = font(max(12, round(height * 0.19)), bold=True)
        subtitle_y = bounds[3] + round(height * 0.10)
        draw.text(
            (x + 3, subtitle_y),
            "PROGRAMMABLE POLYPHONIC SYNTHESIZER",
            font=subtitle_font,
            fill=MUTED,
        )
    return bounds


def save(image: Image.Image, name: str) -> None:
    OUTPUT.mkdir(parents=True, exist_ok=True)
    image.save(OUTPUT / name, format="PNG", optimize=True, compress_level=9)


def make_icon() -> None:
    image = anisotropic_background(512, 512, 0.38)
    draw = ImageDraw.Draw(image, "RGBA")
    draw.rounded_rectangle((42, 42, 470, 470), radius=42, fill=(*PANEL_DARK, 246), outline=(8, 8, 9, 255), width=5)
    draw.rectangle((72, 74, 440, 92), fill=RED)
    draw.rectangle((320, 92, 440, 101), fill=BLUE)
    centered_text(draw, (70, 110, 442, 205), "RF-106", font(67, bold=True), WHITE)
    display(image, (130, 219, 382, 318), "106", 82)
    led(image, (172, 363), LED_RED, 7)
    led(image, (256, 363), LED_RED, 7)
    led(image, (340, 363), LED_GREEN, 7)
    fader_group(image, (108, 376, 404, 447), ["LFO", "VCF", "ENV"], [0.28, 0.57, 0.74], 0.57)
    save(image, "icon.png")


def make_banner() -> None:
    image = anisotropic_background(1600, 400, 0.30)
    draw = ImageDraw.Draw(image, "RGBA")
    draw.rectangle((0, 0, 1600, 400), outline=(12, 12, 14, 255), width=8)
    draw.rectangle((0, 0, 1600, 22), fill=RED)
    draw.rectangle((1030, 22, 1592, 31), fill=BLUE)
    draw.rounded_rectangle((70, 67, 1530, 342), radius=24, fill=(*PANEL_DARK, 228), outline=(9, 9, 10, 230), width=3)

    # Leave the left overlay area calm; the identity starts just beyond it.
    draw.rectangle((102, 104, 311, 305), fill=(13, 13, 15, 190), outline=(78, 78, 84, 180), width=2)
    display(image, (132, 151, 281, 256), "106", 70)
    led(image, (155, 282), LED_RED, 6)
    led(image, (206, 282), LED_RED, 6)
    led(image, (257, 282), LED_GREEN, 6)

    wordmark(image, (380, 94), 93, subtitle=True)
    draw.rectangle((382, 250, 830, 264), fill=RED)
    draw.rectangle((688, 264, 830, 272), fill=BLUE)

    section_strip(draw, (910, 91, 1078, 119), "DCO", RED_DARK, 16)
    section_strip(draw, (1078, 91, 1246, 119), "VCF", RED_DARK, 16)
    section_strip(draw, (1246, 91, 1414, 119), "ENV", RED_DARK, 16)
    section_strip(draw, (1414, 91, 1502, 119), "CH", BLUE_DARK, 16)
    fader_group(image, (916, 126, 1496, 314), ["LFO", "PWM", "FREQ", "RES", "A", "D", "S", "R"], [0.22, 0.52, 0.66, 0.38, 0.44, 0.73, 0.62, 0.31], 0.74)
    save(image, "banner.png")


def make_splash() -> None:
    image = anisotropic_background(1920, 1080, 0.27)
    draw = ImageDraw.Draw(image, "RGBA")
    draw.rectangle((0, 0, 1920, 1080), outline=(11, 11, 13, 255), width=12)

    # Crop-safe central product panel.
    draw.rounded_rectangle((154, 154, 1766, 926), radius=38, fill=(*PANEL_DARK, 238), outline=(9, 9, 10, 255), width=5)
    draw.rectangle((190, 184, 1730, 208), fill=RED)
    draw.rectangle((1220, 208, 1730, 220), fill=BLUE)
    wordmark(image, (224, 242), 112, subtitle=True)
    display(image, (1405, 246, 1662, 373), "106", 90)

    # The section headers and faders mirror the live RF-106 panel without
    # embedding a patch, version, loading message or host control.
    headers = [
        (224, 438, 438, "LFO", RED_DARK),
        (438, 438, 770, "DCO", RED_DARK),
        (770, 438, 934, "HPF", RED_DARK),
        (934, 438, 1242, "VCF", RED_DARK),
        (1242, 438, 1410, "VCA", RED_DARK),
        (1410, 438, 1636, "ENV", RED_DARK),
        (1636, 438, 1696, "CH", BLUE_DARK),
    ]
    for left, top, right, label, color in headers:
        section_strip(draw, (left, top, right, top + 35), label, color, 19)

    draw.rounded_rectangle((210, 420, 1710, 790), radius=14, outline=(9, 9, 10, 255), width=3)
    fader_group(image, (232, 494, 1668, 755), ["RATE", "DELAY", "LFO", "PWM", "SUB", "NOISE", "FREQ", "RES", "ENV", "KYBD", "LEVEL", "A", "D", "S", "R"], [0.72, 0.28, 0.36, 0.61, 0.42, 0.18, 0.58, 0.38, 0.66, 0.55, 0.63, 0.43, 0.71, 0.58, 0.29], 0.88)

    # Reusable front-panel status language: light points and flat color rules.
    for index, color in enumerate((LED_RED, LED_RED, LED_GREEN, LED_RED, LED_GREEN)):
        led(image, (1290 + index * 72, 842), color, 7)
    draw.rectangle((224, 837, 1125, 852), fill=RED)
    draw.rectangle((824, 852, 1125, 861), fill=BLUE)
    save(image, "splash.png")


def main() -> None:
    make_icon()
    make_banner()
    make_splash()
    for path in sorted(OUTPUT.glob("*.png")):
        with Image.open(path) as image:
            print(f"BRANDING_WRITTEN path={path} size={image.width}x{image.height} bytes={path.stat().st_size}")


if __name__ == "__main__":
    main()
