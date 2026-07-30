//! Lightweight, dependency-free XML syntax highlighter for the in-app editor.
//!
//! It turns an XML string into an egui `LayoutJob` with coloured runs for tags,
//! attribute names, quoted values, comments, the declaration and plain text.
//!
//! It is deliberately *tolerant*: it never panics and colours as best it can,
//! even while the user is mid-typing and the document is temporarily malformed.
//! Crucially, the concatenation of every appended run reproduces the input text
//! byte-for-byte, so egui's cursor and selection stay correct.

use eframe::egui::{text::LayoutJob, Color32, FontId, TextFormat};

#[derive(Clone, Copy)]
enum Tok {
    Punct,   // < > / = and the quotes around values
    Tag,     // element name
    Attr,    // attribute name
    Value,   // quoted attribute value
    Comment, // <!-- ... -->
    Decl,    // <?xml ... ?>, <!DOCTYPE ...>, <![CDATA[ ... ]]>
    Text,    // character data between tags
}

#[derive(Clone, Copy)]
struct Palette {
    punct: Color32,
    tag: Color32,
    attr: Color32,
    value: Color32,
    comment: Color32,
    decl: Color32,
    text: Color32,
}

impl Palette {
    fn new(dark: bool, text_color: Color32) -> Self {
        if dark {
            Palette {
                punct: Color32::from_rgb(140, 140, 140),
                tag: Color32::from_rgb(86, 156, 214),
                attr: Color32::from_rgb(156, 220, 254),
                value: Color32::from_rgb(206, 145, 120),
                comment: Color32::from_rgb(106, 153, 85),
                decl: Color32::from_rgb(197, 134, 192),
                text: text_color,
            }
        } else {
            Palette {
                punct: Color32::from_rgb(90, 90, 90),
                tag: Color32::from_rgb(0, 0, 180),
                attr: Color32::from_rgb(150, 60, 0),
                value: Color32::from_rgb(163, 21, 21),
                comment: Color32::from_rgb(0, 128, 0),
                decl: Color32::from_rgb(128, 0, 128),
                text: text_color,
            }
        }
    }

    fn color(&self, k: Tok) -> Color32 {
        match k {
            Tok::Punct => self.punct,
            Tok::Tag => self.tag,
            Tok::Attr => self.attr,
            Tok::Value => self.value,
            Tok::Comment => self.comment,
            Tok::Decl => self.decl,
            Tok::Text => self.text,
        }
    }
}

const ERROR_COLOR: Color32 = Color32::from_rgb(230, 80, 80);

#[allow(clippy::too_many_arguments)]
fn emit(
    job: &mut LayoutJob,
    text: &str,
    start: usize,
    end: usize,
    kind: Tok,
    pal: &Palette,
    font_id: &FontId,
    error_byte: Option<usize>,
) {
    if start >= end {
        return;
    }
    let mut fmt = TextFormat {
        font_id: font_id.clone(),
        color: pal.color(kind),
        ..Default::default()
    };
    // Underline the run that contains the reported error position.
    if let Some(b) = error_byte {
        if start <= b && b < end {
            fmt.underline = eframe::egui::Stroke::new(2.0_f32, ERROR_COLOR);
        }
    }
    job.append(&text[start..end], 0.0, fmt);
}

fn is_ascii_ws(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r')
}

/// Find `pat` at or after byte offset `from` (both are char boundaries here).
fn find_from(text: &str, from: usize, pat: &str) -> Option<usize> {
    text[from..].find(pat).map(|p| from + p)
}

/// Find byte `needle` at or after `from`.
fn byte_from(text: &str, from: usize, needle: u8) -> Option<usize> {
    text.as_bytes()[from..]
        .iter()
        .position(|&c| c == needle)
        .map(|p| from + p)
}

/// Build a coloured `LayoutJob` for the given XML text.
///
/// `error_byte`, when set, underlines the token containing that byte offset.
pub fn highlight_xml(
    text: &str,
    font_id: FontId,
    dark: bool,
    text_color: Color32,
    error_byte: Option<usize>,
) -> LayoutJob {
    let pal = Palette::new(dark, text_color);
    let mut job = LayoutJob::default();
    let b = text.as_bytes();
    let n = text.len();
    let mut i = 0;

    while i < n {
        if text[i..].starts_with("<!--") {
            let end = find_from(text, i + 4, "-->").map(|e| e + 3).unwrap_or(n);
            emit(&mut job, text, i, end, Tok::Comment, &pal, &font_id, error_byte);
            i = end;
        } else if text[i..].starts_with("<![CDATA[") {
            let end = find_from(text, i + 9, "]]>").map(|e| e + 3).unwrap_or(n);
            emit(&mut job, text, i, end, Tok::Decl, &pal, &font_id, error_byte);
            i = end;
        } else if text[i..].starts_with("<?") {
            let end = find_from(text, i + 2, "?>").map(|e| e + 2).unwrap_or(n);
            emit(&mut job, text, i, end, Tok::Decl, &pal, &font_id, error_byte);
            i = end;
        } else if text[i..].starts_with("<!") {
            let end = byte_from(text, i + 2, b'>').map(|e| e + 1).unwrap_or(n);
            emit(&mut job, text, i, end, Tok::Decl, &pal, &font_id, error_byte);
            i = end;
        } else if b[i] == b'<' {
            let end = byte_from(text, i + 1, b'>').map(|e| e + 1).unwrap_or(n);
            highlight_tag(&mut job, text, i, end, &pal, &font_id, error_byte);
            i = end;
        } else {
            let end = byte_from(text, i, b'<').unwrap_or(n);
            emit(&mut job, text, i, end, Tok::Text, &pal, &font_id, error_byte);
            i = end;
        }
    }

    job
}

/// Colour a single `<...>` tag, given its absolute byte range `[ts, te)` in
/// `text` (leading `<` and trailing `>` included when present).
#[allow(clippy::too_many_arguments)]
fn highlight_tag(
    job: &mut LayoutJob,
    text: &str,
    ts: usize,
    te: usize,
    pal: &Palette,
    font_id: &FontId,
    error_byte: Option<usize>,
) {
    let tag = &text[ts..te];
    let b = tag.as_bytes();
    let n = tag.len();
    let mut j = 0;

    // Opening '<' and an optional '/'.
    let start = j;
    j += 1; // '<'
    if j < n && b[j] == b'/' {
        j += 1;
    }
    emit(job, text, ts + start, ts + j, Tok::Punct, pal, font_id, error_byte);

    // Element name.
    let ns = j;
    while j < n {
        let c = b[j];
        if is_ascii_ws(c) || c == b'/' || c == b'>' {
            break;
        }
        j += 1;
    }
    emit(job, text, ts + ns, ts + j, Tok::Tag, pal, font_id, error_byte);

    // Attributes / punctuation until the end of the slice.
    while j < n {
        let c = b[j];
        if is_ascii_ws(c) {
            let s = j;
            while j < n && is_ascii_ws(b[j]) {
                j += 1;
            }
            emit(job, text, ts + s, ts + j, Tok::Text, pal, font_id, error_byte);
        } else if c == b'/' || c == b'>' || c == b'=' {
            let s = j;
            j += 1;
            emit(job, text, ts + s, ts + j, Tok::Punct, pal, font_id, error_byte);
        } else if c == b'"' || c == b'\'' {
            let quote = c;
            let s = j;
            j += 1;
            while j < n && b[j] != quote {
                j += 1;
            }
            if j < n {
                j += 1; // include the closing quote
            }
            emit(job, text, ts + s, ts + j, Tok::Value, pal, font_id, error_byte);
        } else {
            // Attribute name.
            let s = j;
            while j < n {
                let d = b[j];
                if is_ascii_ws(d)
                    || d == b'='
                    || d == b'/'
                    || d == b'>'
                    || d == b'"'
                    || d == b'\''
                {
                    break;
                }
                j += 1;
            }
            emit(job, text, ts + s, ts + j, Tok::Attr, pal, font_id, error_byte);
        }
    }
}
