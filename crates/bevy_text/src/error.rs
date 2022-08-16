use ab_glyph::GlyphId;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum TextError {
    #[error("font not found")]
    NoSuchFont,
    #[error("failed to add glyph to newly-created atlas {0:?}")]
    FailedToAddGlyph(GlyphId),
    #[error("exceeded {0:?} availble TextAltases for font. This can be caused by using an excessive number of font sizes. Consider using Transform::scale to modify font size dynamically. If you need more font sizes modify TextSettings.max_font_atlases." )]
    ExceedMaxTextAtlases(usize),
}
