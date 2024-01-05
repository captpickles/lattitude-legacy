use ab_glyph::FontRef;

pub fn typewriter() -> Result<FontRef<'static>, anyhow::Error> {
    let font = FontRef::try_from_slice(include_bytes!("../../fonts/JMH Typewriter dry.otf"))?;
    Ok(font)
}

pub fn typewriter_bold() -> Result<FontRef<'static>, anyhow::Error> {
    let font = FontRef::try_from_slice(include_bytes!("../../fonts/JMH Typewriter dry-Bold.otf"))?;
    Ok(font)
}

pub fn sanserif() -> Result<FontRef<'static>, anyhow::Error> {
    Ok(FontRef::try_from_slice(include_bytes!(
        "../../fonts/Comfortaa-Regular.ttf"
    ))?)
}

pub fn sanserif_bold() -> Result<FontRef<'static>, anyhow::Error> {
    Ok(FontRef::try_from_slice(include_bytes!(
        "../../fonts/Comfortaa-Bold.ttf"
    ))?)
}

