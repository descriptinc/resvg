use std::ops::Range;
use std::sync::Arc;

use crate::{ImageKind, Size};

/// A replacement for a complete UTF-8 text range, measured in SVG user units
/// relative to the alphabetic baseline at the requested font size.
#[derive(Clone, Debug)]
pub struct RasterGlyph {
    /// UTF-8 byte offsets in the callback's original string.
    pub text_range: Range<usize>,
    /// Encoded pixels; the host retains responsibility for supplying valid image data.
    pub image: ImageKind,
    /// Intrinsic pixel dimensions of the encoded image.
    pub size: Size,
    /// SVG user units per image pixel.
    pub image_scale: f32,
    /// Horizontal bearing in SVG user units from the text cursor.
    pub left: f32,
    /// Vertical bearing in SVG user units from the alphabetic baseline, positive downwards.
    pub top: f32,
    /// Cursor advance in SVG user units.
    pub advance: f32,
}

/// Returns nonoverlapping replacements in increasing byte order. The font size is
/// in SVG user units. Unreplaced text retains ordinary font shaping and fallback.
#[cfg(target_arch = "wasm32")]
pub type RasterGlyphsFn<'a> = Box<dyn Fn(&str, f32) -> Vec<Arc<RasterGlyph>> + 'a>;
/// Returns nonoverlapping replacements in increasing byte order. The font size is
/// in SVG user units. Unreplaced text retains ordinary font shaping and fallback.
#[cfg(not(target_arch = "wasm32"))]
pub type RasterGlyphsFn<'a> = Box<dyn Fn(&str, f32) -> Vec<Arc<RasterGlyph>> + Send + Sync + 'a>;

pub(super) fn valid_glyphs(text: &str, glyphs: Vec<Arc<RasterGlyph>>) -> Vec<Arc<RasterGlyph>> {
    let mut end = 0;
    glyphs
        .into_iter()
        .filter(|glyph| {
            let range = &glyph.text_range;
            let valid = range.start >= end
                && range.start < range.end
                && text.get(range.clone()).is_some()
                && glyph.left.is_finite()
                && glyph.top.is_finite()
                && glyph.advance.is_finite()
                && glyph.advance >= 0.0
                && glyph.image_scale.is_finite()
                && glyph.image_scale > 0.0;
            if valid {
                end = range.end;
            }
            valid
        })
        .collect()
}

pub(super) fn segments(
    range: Range<usize>,
    glyphs: &[Arc<RasterGlyph>],
) -> Vec<(Range<usize>, Option<Arc<RasterGlyph>>)> {
    let mut output = Vec::new();
    let mut start = range.start;
    for glyph in glyphs {
        if glyph.text_range.start < start || glyph.text_range.end > range.end {
            continue;
        }
        if start < glyph.text_range.start {
            output.push((start..glyph.text_range.start, None));
        }
        output.push((glyph.text_range.clone(), Some(glyph.clone())));
        start = glyph.text_range.end;
    }
    if start < range.end {
        output.push((start..range.end, None));
    }
    output
}
