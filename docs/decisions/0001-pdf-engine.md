# ADR 0001: PDF engine

Date: 2026-09-25. Status: accepted.

## Decision

Use MuPDF (via the `mupdf` crate) behind an internal `PdfEngine` trait.
prev is licensed AGPL-3.0-or-later as a result.

## Options considered

| Engine | License | Outcome |
|---|---|---|
| MuPDF | AGPL-3.0 | Chosen |
| PDFium + lopdf | BSD-3 / Apache-2.0 | Close second |
| Poppler | GPL | Rejected: slower on complex pages, no page operations or redaction |
| hayro (pure Rust) | MIT / Apache-2.0 | Rejected for now: too young for top-tier fidelity |

## Reasons

- Renders more documents correctly across a wider range of PDF features
  (comparison below).
- Built-in true redaction; with PDFium we would have had to build it.
- Handles page editing, annotations, forms, encryption and export itself, so
  no separate low-level PDF crate is needed.
- Fastest renderer tested, with the smallest worst case.
- Linux only scope removed PDFium's cross-platform advantage.

## Consequences

- prev is AGPL-3.0-or-later. LGPL dependencies (rawler, libheif) are fine.
- Artifex holds the MuPDF copyright and could relicense future versions.
  Released AGPL versions stay AGPL; the engine trait allows adding a PDFium
  implementation if needed.
- Known weakness: advanced JBIG2 (refinement, halftone) decoding.
- The `mupdf` 0.8 crate builds its vendored MuPDF 1.27.2 from source; the
  comparison below used 1.28.2.

## Rendering comparison (2026-09-25)

**Engines:** PDFium 153.0.7999.0 (pypdfium2 5.13.0) and MuPDF 1.28.2 (PyMuPDF
1.28.2), with Poppler 26.08.0 and Ghostscript 10.07.1 as arbiters.

**Corpus:** 1,332 PDFs (pdf.js `test/pdfs`, PDFium `testing/resources`,
32 local technical standards and datasheets), first 5 pages at 100 dpi,
1,632 pages rendered by both candidates. About 45 disagreements were judged
manually.

### Robustness

| | PDFium | MuPDF |
|---|---|---|
| Open failure (non-password) | 29 | 12, plus 38 that report 0 pages |
| Crash | 1 (no engine opens that file) | 0 |
| Timeout (60 s) | 0 | 0 |
| Blank pages where others show content | 17 | 15 |
| Files only this engine rendered with content | about 11 | 4 |

PDFium recovers structurally broken files slightly better, partly because
many of those files come from its own test suite.

### Correctness

About 95.6 % of pages agreed within 5 % pixel difference. The 32 real-world
documents had no content differences, only hairline weight.

| Scope | PDFium correct | MuPDF correct |
|---|---|---|
| All significant disagreements | 32 | 25 |
| Excluding one cluster of 24 JBIG2 test files | 8 | 25 |

- **MuPDF correct, PDFium wrong:** non-embedded CID fonts without Microsoft
  fonts installed (Cyrillic, Polish, CJK text missing), embedded font
  encodings, transparency knockout groups, CalGray, CMYK shading, tiling
  patterns, `/UserUnit`, Brotli streams.
- **PDFium correct, MuPDF wrong:** JBIG2 refinement, halftone and custom
  Huffman tables; a few broken fonts, a soft mask transfer function, stroked
  text, page box parsing.

### Speed (per page, 100 dpi, sequential)

| Corpus | PDFium median / p95 / max | MuPDF median / p95 / max |
|---|---|---|
| Real documents | 2.5 / 21.3 / 98 ms | 1.9 / 21.8 / 101 ms |
| pdf.js corpus | 1.1 / 14.6 ms / 7.9 s | 1.0 / 19.2 ms / 1.2 s |

### Limitations

Edge-case heavy corpus with clusters; few real-world documents, all English
technical PDFs; verdicts judged by eye without reference images; font results
depend on installed fonts; only first 5 pages at 100 dpi.

## Later: Windows (2026-09)

prev now also runs on Windows, so the reason above that a Linux only
scope removed PDFium's cross-platform advantage no longer holds. MuPDF
built with MSVC through `mupdf-sys` without changes, and the decision
stands.
