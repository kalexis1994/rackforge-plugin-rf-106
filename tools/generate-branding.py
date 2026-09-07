#!/usr/bin/env python3
"""Validate the final RF-106 branding PNGs required by RackForge.

The selected artwork lives directly in plugin/package/branding. Keeping only
those package assets avoids storing duplicate source renders in the repository.
Requires Pillow.
"""

from pathlib import Path

from PIL import Image


ROOT = Path(__file__).resolve().parents[1]
BRANDING = ROOT / "plugin" / "package" / "branding"
ASSETS = {
    "icon.png": ((512, 512), "RGBA", 2 * 1024 * 1024),
    "banner.png": ((1600, 400), "RGB", 4 * 1024 * 1024),
    "splash.png": ((1920, 1080), "RGB", 8 * 1024 * 1024),
}


def main() -> None:
    for name, (expected_size, expected_mode, byte_limit) in ASSETS.items():
        path = BRANDING / name
        with Image.open(path) as image:
            image.load()
            if image.size != expected_size:
                raise ValueError(
                    f"{name} must be {expected_size[0]}x{expected_size[1]}, "
                    f"found {image.width}x{image.height}"
                )
            if image.mode != expected_mode:
                raise ValueError(
                    f"{name} must use {expected_mode}, found {image.mode}"
                )
            if name == "icon.png" and image.getchannel("A").getextrema() != (0, 255):
                raise ValueError("icon.png must contain real alpha transparency")
        if path.stat().st_size > byte_limit:
            raise ValueError(f"{name} exceeds RackForge's file-size limit")
        print(
            f"BRANDING_VALID path={path} size={expected_size[0]}x{expected_size[1]} "
            f"mode={expected_mode} bytes={path.stat().st_size}"
        )


if __name__ == "__main__":
    main()
