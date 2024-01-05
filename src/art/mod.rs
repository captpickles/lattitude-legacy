use bmp::Image;
use bytes::Buf;

pub mod weather;

pub fn logo() -> Result<Image, anyhow::Error> {
    Ok( bmp::from_reader( &mut include_bytes!("../../art/captpickles.bmp").reader())?)
}

pub fn usb() -> Result<Image, anyhow::Error> {
    Ok( bmp::from_reader( &mut include_bytes!("../../art/usb.bmp").reader())?)
}

pub fn moon_full() -> Result<Image, anyhow::Error> {
    Ok( bmp::from_reader( &mut include_bytes!("../../art/moon/full.bmp").reader())?)
}

pub fn moon_new() -> Result<Image, anyhow::Error> {
    Ok( bmp::from_reader( &mut include_bytes!("../../art/moon/new.bmp").reader())?)
}

pub fn moon_first_quarter() -> Result<Image, anyhow::Error> {
    Ok( bmp::from_reader( &mut include_bytes!("../../art/moon/first-quarter.bmp").reader())?)
}

pub fn moon_third_quarter() -> Result<Image, anyhow::Error> {
    Ok( bmp::from_reader( &mut include_bytes!("../../art/moon/third-quarter.bmp").reader())?)
}

pub fn moon_waxing_gibbous() -> Result<Image, anyhow::Error> {
    Ok( bmp::from_reader( &mut include_bytes!("../../art/moon/waxing-gibbous.bmp").reader())?)
}

pub fn moon_waning_gibbous() -> Result<Image, anyhow::Error> {
    Ok( bmp::from_reader( &mut include_bytes!("../../art/moon/waning-gibbous.bmp").reader())?)
}

pub fn moon_waxing_crescent() -> Result<Image, anyhow::Error> {
    Ok( bmp::from_reader( &mut include_bytes!("../../art/moon/waxing-crescent.bmp").reader())?)
}

pub fn moon_waning_crescent() -> Result<Image, anyhow::Error> {
    Ok( bmp::from_reader( &mut include_bytes!("../../art/moon/waning-crescent.bmp").reader())?)
}

pub fn arrow_up() -> Result<Image, anyhow::Error> {
    Ok( bmp::from_reader( &mut include_bytes!("../../art/arrow/arrow-up.bmp").reader())?)
}

pub fn arrow_level() -> Result<Image, anyhow::Error> {
    Ok( bmp::from_reader( &mut include_bytes!("../../art/arrow/arrow-level.bmp").reader())?)
}

pub fn arrow_down() -> Result<Image, anyhow::Error> {
    Ok( bmp::from_reader( &mut include_bytes!("../../art/arrow/arrow-down.bmp").reader())?)
}

pub fn arrow_small_down() -> Result<Image, anyhow::Error> {
    Ok( bmp::from_reader( &mut include_bytes!("../../art/arrow/small-down.bmp").reader())?)
}

pub fn arrow_small_up() -> Result<Image, anyhow::Error> {
    Ok( bmp::from_reader( &mut include_bytes!("../../art/arrow/small-up.bmp").reader())?)
}

pub fn aqi() -> Result<Image, anyhow::Error> {
    Ok( bmp::from_reader( &mut include_bytes!("../../art/aqi.bmp").reader())?)
}

pub mod wind {
    use bmp::Image;
    use bytes::Buf;

    pub fn wind() -> Result<Image, anyhow::Error> {
        Ok( bmp::from_reader( &mut include_bytes!("../../art/wind/wind.bmp").reader())?)
    }

    pub fn gust() -> Result<Image, anyhow::Error> {
        Ok( bmp::from_reader( &mut include_bytes!("../../art/wind/gust.bmp").reader())?)
    }
}
