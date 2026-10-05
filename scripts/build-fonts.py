"""Builds the font prev bundles itself in crates/prev/assets/fonts.

Run with: uv run --with fonttools scripts/build-fonts.py

- Dancing Script: a static semibold instance, for typed signatures.

The interface and icon fonts, Roboto Flex and Material Symbols Rounded,
come with scramble-ui, which builds them. Their license texts stay here,
as the packages install them with prev's.
"""

import tempfile
import urllib.request
from pathlib import Path

from fontTools.ttLib import TTFont
from fontTools.varLib.instancer import instantiateVariableFont

FONTS_COMMIT = "23e54b51ddffbc7713c583748e3bd86f62b1fa4a"
ICONS_COMMIT = "bd8cb85bd4bad964fe6918f79665bb40c3a8efef"
ROBOTO_FLEX_LICENSE = f"https://github.com/google/fonts/raw/{FONTS_COMMIT}/ofl/robotoflex/OFL.txt"
DANCING_SCRIPT = (
    f"https://github.com/google/fonts/raw/{FONTS_COMMIT}/ofl/dancingscript/DancingScript%5Bwght%5D.ttf"
)
DANCING_SCRIPT_LICENSE = f"https://github.com/google/fonts/raw/{FONTS_COMMIT}/ofl/dancingscript/OFL.txt"
SYMBOLS_LICENSE = f"https://github.com/google/material-design-icons/raw/{ICONS_COMMIT}/LICENSE"

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "crates/prev/assets/fonts"


def fetch(url: str, into: Path) -> Path:
    path = into / url.rsplit("/", 1)[-1].replace("%5B", "[").replace("%5D", "]")
    with urllib.request.urlopen(url) as response:
        path.write_bytes(response.read())
    return path


def build_dancing_script(source: Path, out: Path) -> None:
    font = TTFont(source)
    instantiateVariableFont(font, {"wght": 600}, inplace=True)
    font.save(out)


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory() as scratch:
        scratch = Path(scratch)
        build_dancing_script(fetch(DANCING_SCRIPT, scratch), OUT / "DancingScript.ttf")
        (OUT / "OFL-DancingScript.txt").write_bytes(
            fetch(DANCING_SCRIPT_LICENSE, scratch).read_bytes()
        )
        (OUT / "OFL.txt").write_bytes(fetch(ROBOTO_FLEX_LICENSE, scratch).read_bytes())
        (OUT / "LICENSE-MaterialSymbols.txt").write_bytes(fetch(SYMBOLS_LICENSE, scratch).read_bytes())
    for path in sorted(OUT.iterdir()):
        print(f"{path.stat().st_size:>9}  {path.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
