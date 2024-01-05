use bmp::Image;

pub mod weather;

pub fn logo() -> Result<Image, anyhow::Error> {
    let image = bmp::open("art/captpickles.bmp")?;
    Ok(image)
}

pub fn sunrise() -> Result<Image, anyhow::Error> {
    let image = bmp::open("art/icons8-sunrise-50.bmp")?;
    Ok(image)
}

pub fn sunset() -> Result<Image, anyhow::Error> {
    let image = bmp::open("art/icons8-sunset-50.bmp")?;
    Ok(image)
}

pub fn moonrise() -> Result<Image, anyhow::Error> {
    let image = bmp::open("art/icons8-moonrise-50.bmp")?;
    Ok(image)
}

pub fn moon_full() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/moon/full.bmp")?)
}

pub fn moon_new() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/moon/new.bmp")?)
}

pub fn moon_first_quarter() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/moon/first-quarter.bmp")?)
}

pub fn moon_third_quarter() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/moon/third-quarter.bmp")?)
}

pub fn moon_waxing_gibbous() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/moon/waxing-gibbous.bmp")?)
}

pub fn moon_waning_gibbous() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/moon/waning-gibbous.bmp")?)
}

pub fn moon_waxing_crescent() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/moon/waxing-crescent.bmp")?)
}

pub fn moon_waning_crescent() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/moon/waning-crescent.bmp")?)
}

pub fn arrow_up() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/arrow/arrow-up.bmp")?)
}

pub fn arrow_level() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/arrow/arrow-level.bmp")?)
}

pub fn arrow_down() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/arrow/arrow-down.bmp")?)
}

pub fn arrow_small_down() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/arrow/small-down.bmp")?)
}

pub fn arrow_small_up() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/arrow/small-up.bmp")?)
}

pub fn aqi() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/aqi.bmp")?)
}

pub mod wind {
    use bmp::Image;
    pub fn wind() -> Result<Image, anyhow::Error> {
        Ok(bmp::open("art/wind/wind.bmp")?)
    }

    pub fn gust() -> Result<Image, anyhow::Error> {
        Ok(bmp::open("art/wind/gust.bmp")?)
    }
}
