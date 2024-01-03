use bmp::Image;

pub fn sunrise() -> Result<Image, anyhow::Error> {
    println!("image:sunrise");
    let image = bmp::open("art/icons8-sunrise-50.bmp")?;
    Ok(image)
}

pub fn cloud_lightning() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/weather/cloud-lightning.bmp")?)
}

pub fn clouds() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/weather/clouds.bmp")?)
}

pub fn flurries() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/weather/flurries.bmp")?)
}

pub fn fog() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/weather/fog.bmp")?)
}
pub fn heavy_rain() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/weather/heavy-rain.bmp")?)
}

pub fn ice() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/weather/ice.bmp")?)
}
pub fn partly_cloudy_thunderstorm() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/weather/partly-cloud-tstorm.bmp")?)
}
pub fn partly_cloudy_rain() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/weather/partly-cloudy-rain.bmp")?)
}
pub fn partly_cloudy() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/weather/partly-cloudy.bmp")?)
}

pub fn scattered_clouds() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/weather/scattered-clouds.bmp")?)
}

pub fn sleet() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/weather/sleet.bmp")?)
}

pub fn snow() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/weather/snow.bmp")?)
}

pub fn sunny() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/weather/sunny.bmp")?)
}

pub fn thunderstorms() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/weather/tstorms.bmp")?)
}

pub fn windy() -> Result<Image, anyhow::Error> {
    Ok(bmp::open("art/weather/windy.bmp")?)
}