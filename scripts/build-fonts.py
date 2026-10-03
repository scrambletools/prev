"""Builds the fonts bundled in crates/prev/assets/fonts.

Run with: uv run --with fonttools scripts/build-fonts.py

- Roboto Flex: every axis pinned to its default except weight, which
  iced's text stack drives, and a static bold instance beside it, as iced
  picks a face only at a weight it was registered with, and the variable
  font registers as regular alone. Without it, bold text, as in Markdown,
  falls back to a system font (a monospaced one on macOS).
- Dancing Script: a static semibold instance, for typed signatures.
- Material Symbols Rounded: static outlined and filled instances at 24 px
  optical size, subset to the icons listed in crates/prev/src/ui/icon.rs.
"""

import io
import re
import sys
import tempfile
import urllib.request
from pathlib import Path

from fontTools.pens.transformPen import TransformPen
from fontTools.pens.ttGlyphPen import TTGlyphPen
from fontTools.subset import Options, Subsetter
from fontTools.ttLib import TTFont
from fontTools.varLib.instancer import instantiateVariableFont

FONTS_COMMIT = "23e54b51ddffbc7713c583748e3bd86f62b1fa4a"
ICONS_COMMIT = "bd8cb85bd4bad964fe6918f79665bb40c3a8efef"
ROBOTO_FLEX = (
    f"https://github.com/google/fonts/raw/{FONTS_COMMIT}/ofl/robotoflex/"
    "RobotoFlex%5BGRAD,XOPQ,XTRA,YOPQ,YTAS,YTDE,YTFI,YTLC,YTUC,opsz,slnt,wdth,wght%5D.ttf"
)
ROBOTO_FLEX_LICENSE = f"https://github.com/google/fonts/raw/{FONTS_COMMIT}/ofl/robotoflex/OFL.txt"
DANCING_SCRIPT = (
    f"https://github.com/google/fonts/raw/{FONTS_COMMIT}/ofl/dancingscript/DancingScript%5Bwght%5D.ttf"
)
DANCING_SCRIPT_LICENSE = f"https://github.com/google/fonts/raw/{FONTS_COMMIT}/ofl/dancingscript/OFL.txt"
SYMBOLS = (
    f"https://github.com/google/material-design-icons/raw/{ICONS_COMMIT}/variablefont/"
    "MaterialSymbolsRounded%5BFILL,GRAD,opsz,wght%5D.ttf"
)
SYMBOLS_LICENSE = f"https://github.com/google/material-design-icons/raw/{ICONS_COMMIT}/LICENSE"

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "crates/prev/assets/fonts"
ICON_TABLE = ROOT / "crates/prev/src/ui/icon.rs"

# Glyphs the font lacks, drawn by transforming another glyph:
# (made-up name in icon.rs, source codepoint, transform).
DERIVED = {
    "flip rotated a quarter turn": (0xE3E8, "quarter_turn"),
}


def fetch(url: str, into: Path) -> Path:
    path = into / url.rsplit("/", 1)[-1].replace("%5B", "[").replace("%5D", "]")
    with urllib.request.urlopen(url) as response:
        path.write_bytes(response.read())
    return path


def icons() -> list[tuple[int, str]]:
    table = ICON_TABLE.read_text()
    entries = re.findall(r"^\s+\w+ = (0x[0-9a-f]+), \"([^\"]+)\";", table, re.MULTILINE)
    if not entries:
        sys.exit(f"no icons found in {ICON_TABLE}")
    return [(int(codepoint, 16), name) for codepoint, name in entries]


def rename(font: TTFont, family: str) -> None:
    postscript = family.replace(" ", "")
    for record in font["name"].names:
        if record.nameID in (1, 16):
            record.string = family
        elif record.nameID == 4:
            record.string = family
        elif record.nameID == 6:
            record.string = postscript


def add_derived(font: TTFont, codepoint: int, source: int, transform: str) -> None:
    cmap = font.getBestCmap()
    if codepoint in cmap:
        sys.exit(f"U+{codepoint:X} is already used in the icon font")
    glyph_set = font.getGlyphSet()
    source_name = cmap[source]
    width, _ = font["hmtx"][source_name]
    ascent = font["hhea"].ascent
    descent = font["hhea"].descent
    if transform != "quarter_turn":
        sys.exit(f"unknown transform {transform}")
    # Rotate 90 degrees about the centre of the icon's em box.
    centre_x = width / 2
    centre_y = (ascent + descent) / 2
    matrix = (0, 1, -1, 0, centre_x + centre_y, centre_y - centre_x)
    pen = TTGlyphPen(glyph_set)
    glyph_set[source_name].draw(TransformPen(pen, matrix))
    name = f"derived{codepoint:x}"
    font["glyf"][name] = pen.glyph()
    font["hmtx"][name] = (width, 0)
    for table in font["cmap"].tables:
        if table.isUnicode():
            table.cmap[codepoint] = name


def build_symbols(source: Path, fill: int, family: str, out: Path) -> None:
    font = TTFont(source)
    instantiateVariableFont(font, {"FILL": fill, "GRAD": 0, "opsz": 24, "wght": 400}, inplace=True)
    wanted = []
    for codepoint, name in icons():
        if name in DERIVED:
            base, transform = DERIVED[name]
            add_derived(font, codepoint, base, transform)
        wanted.append(codepoint)
    # Reload so the subsetter sees the added glyphs.
    buffer = io.BytesIO()
    font.save(buffer)
    buffer.seek(0)
    font = TTFont(buffer)
    missing = [f"U+{c:X}" for c in wanted if c not in font.getBestCmap()]
    if missing:
        sys.exit(f"icons missing from the font: {', '.join(missing)}")
    options = Options()
    options.layout_features = []
    options.name_IDs = ["*"]
    options.notdef_outline = True
    subsetter = Subsetter(options)
    subsetter.populate(unicodes=wanted)
    subsetter.subset(font)
    rename(font, family)
    font.save(out)


def build_roboto_flex(source: Path, out: Path) -> None:
    font = TTFont(source)
    pinned = {axis.axisTag: axis.defaultValue for axis in font["fvar"].axes if axis.axisTag != "wght"}
    instantiateVariableFont(font, pinned, inplace=True)
    font.save(out)


def build_roboto_flex_bold(variable: Path, out: Path) -> None:
    font = TTFont(variable)
    instantiateVariableFont(font, {"wght": 700}, inplace=True, updateFontNames=False)
    font["OS/2"].usWeightClass = 700
    # Bold, not regular, in the style bits.
    font["OS/2"].fsSelection = (font["OS/2"].fsSelection & ~0x40) | 0x20
    font["head"].macStyle |= 0x1
    names = {2: "Bold", 4: "Roboto Flex Bold", 6: "RobotoFlex-Bold", 17: "Bold"}
    for record in font["name"].names:
        if record.nameID in names:
            record.string = names[record.nameID]
    font.save(out)


def build_dancing_script(source: Path, out: Path) -> None:
    font = TTFont(source)
    instantiateVariableFont(font, {"wght": 600}, inplace=True)
    font.save(out)


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory() as scratch:
        scratch = Path(scratch)
        build_roboto_flex(fetch(ROBOTO_FLEX, scratch), OUT / "RobotoFlex.ttf")
        build_roboto_flex_bold(OUT / "RobotoFlex.ttf", OUT / "RobotoFlexBold.ttf")
        (OUT / "OFL.txt").write_bytes(fetch(ROBOTO_FLEX_LICENSE, scratch).read_bytes())
        build_dancing_script(fetch(DANCING_SCRIPT, scratch), OUT / "DancingScript.ttf")
        (OUT / "OFL-DancingScript.txt").write_bytes(
            fetch(DANCING_SCRIPT_LICENSE, scratch).read_bytes()
        )
        symbols = fetch(SYMBOLS, scratch)
        build_symbols(symbols, 0, "Material Symbols Rounded", OUT / "MaterialSymbolsRounded.ttf")
        build_symbols(symbols, 1, "Material Symbols Rounded Filled", OUT / "MaterialSymbolsRoundedFilled.ttf")
        (OUT / "LICENSE-MaterialSymbols.txt").write_bytes(fetch(SYMBOLS_LICENSE, scratch).read_bytes())
    for path in sorted(OUT.iterdir()):
        print(f"{path.stat().st_size:>9}  {path.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
