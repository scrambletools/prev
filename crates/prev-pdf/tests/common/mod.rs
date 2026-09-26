//! Hand-built PDF fixtures.

#![allow(dead_code)]

/// Assembles a PDF from numbered object bodies, computing the xref table.
/// `trailer_extra` is added to the trailer dictionary.
pub fn assemble_pdf_with_trailer(objects: &[Vec<u8>], trailer_extra: &str) -> Vec<u8> {
    let mut out = b"%PDF-1.7\n".to_vec();
    let mut offsets = Vec::with_capacity(objects.len());
    for (index, body) in objects.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n", index + 1).as_bytes());
        out.extend_from_slice(body);
        out.extend_from_slice(b"\nendobj\n");
    }
    let xref_offset = out.len();
    out.extend_from_slice(
        format!("xref\n0 {}\n0000000000 65535 f \n", objects.len() + 1).as_bytes(),
    );
    for offset in offsets {
        out.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R {trailer_extra} >>\nstartxref\n{xref_offset}\n%%EOF\n",
            objects.len() + 1
        )
        .as_bytes(),
    );
    out
}

pub fn assemble_pdf(objects: &[Vec<u8>]) -> Vec<u8> {
    assemble_pdf_with_trailer(objects, "")
}

pub fn stream_object(dict: &str, data: &[u8]) -> Vec<u8> {
    let mut body = format!("<< {dict} /Length {} >>\nstream\n", data.len()).into_bytes();
    body.extend_from_slice(data);
    body.extend_from_slice(b"\nendstream");
    body
}

/// Three pages: portrait with two text lines and two links, landscape, and
/// a portrait page rotated 90 degrees. Has a title, page labels
/// (i, ii, 1) and a two-level outline.
pub fn navigation_fixture() -> Vec<u8> {
    let objects: Vec<Vec<u8>> = vec![
        // 1
        b"<< /Type /Catalog /Pages 2 0 R /Outlines 9 0 R \
/PageLabels << /Nums [0 << /S /r >> 2 << /S /D /St 1 >>] >> >>"
            .to_vec(),
        // 2
        b"<< /Type /Pages /Kids [3 0 R 4 0 R 5 0 R] /Count 3 >>".to_vec(),
        // 3
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 6 0 R \
/Resources << /Font << /F1 16 0 R >> >> /Annots [10 0 R 11 0 R] >>"
            .to_vec(),
        // 4
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 842 595] /Contents 7 0 R \
/Resources << /Font << /F1 16 0 R >> >> >>"
            .to_vec(),
        // 5
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Rotate 90 /Contents 8 0 R \
/Resources << /Font << /F1 16 0 R >> >> >>"
            .to_vec(),
        // 6
        stream_object(
            "",
            b"BT /F1 24 Tf 72 700 Td (Hello world) Tj ET\nBT /F1 24 Tf 72 670 Td (Second line here) Tj ET\n",
        ),
        // 7
        stream_object("", b"BT /F1 24 Tf 72 500 Td (Page two text) Tj ET\n"),
        // 8
        stream_object("", b"BT /F1 24 Tf 72 700 Td (Rotated page) Tj ET\n"),
        // 9
        b"<< /Type /Outlines /First 12 0 R /Last 13 0 R /Count 3 >>".to_vec(),
        // 10
        b"<< /Type /Annot /Subtype /Link /Rect [72 690 200 720] /Border [0 0 0] \
/Dest [4 0 R /XYZ 0 400 0] >>"
            .to_vec(),
        // 11
        b"<< /Type /Annot /Subtype /Link /Rect [72 660 250 690] /Border [0 0 0] \
/A << /S /URI /URI (https://example.org/) >> >>"
            .to_vec(),
        // 12
        b"<< /Title (Chapter One) /Parent 9 0 R /Next 13 0 R /First 14 0 R /Last 14 0 R /Count 1 \
/Dest [3 0 R /XYZ 0 792 0] >>"
            .to_vec(),
        // 13
        b"<< /Title (Chapter Two) /Parent 9 0 R /Prev 12 0 R /Dest [4 0 R /Fit] >>".to_vec(),
        // 14
        b"<< /Title (Section 1.1) /Parent 12 0 R /Dest [5 0 R /XYZ 100 500 0] >>".to_vec(),
        // 15
        b"<< /Title (Engine Fixture) >>".to_vec(),
        // 16
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_vec(),
    ];
    assemble_pdf_with_trailer(&objects, "/Info 15 0 R")
}
