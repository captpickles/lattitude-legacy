use std::cmp::max;
use std::env;
use ab_glyph::{Font, PxScale};
use bmp::Image;
use chrono::{Datelike, DateTime, Days, Local, Timelike, TimeZone, Utc, Weekday};
use glyph_brush_layout::{FontId, GlyphPositioner, HorizontalAlign, Layout, SectionGeometry, SectionText, VerticalAlign};
use crate::accuweather::daily_forecast::DailyForecast;
use crate::accuweather::hourly_forecast::HourlyForecast;
use crate::art::{aqi, arrow_down, arrow_level, arrow_small_down, arrow_small_up, arrow_up, logo, moon_full, moon_new, moon_first_quarter, sunrise, sunset, moon_waning_gibbous, moon_waxing_gibbous, moon_waning_crescent, moon_waxing_crescent, moon_third_quarter, weather, wind};
use crate::calendar::Event;
use crate::data::data::{DisplayData, NowData};
use crate::font::{typewriter, sanserif, sanserif_bold, typewriter_bold, script};
use crate::graphics::{Color, Darkness, Graphics, lighten_bmp, rotate_bmp, Thickness, trim_bmp, ViewPort};
use crate::netatmo::Trend;

//const WIDTH: usize = 1072;
//const HEIGHT: usize = 1448;

const WIDTH: usize = 1404;
const HEIGHT: usize = 1872;


pub struct Display {
    graphics: Graphics<WIDTH, HEIGHT>,
}

impl Display {
    pub fn new() -> Self {
        Self {
            graphics: Graphics::new()
        }
    }

    #[cfg(feature = "linux-embedded-hal")]
    pub fn paint(&self) {
        use linux_embedded_hal::gpio_cdev::{Chip, LineRequestFlags};
        use linux_embedded_hal::spidev::{SpiModeFlags, SpidevOptions};
        use linux_embedded_hal::{CdevPin, Delay, Spidev};

        let mut spi = Spidev::open("/dev/spidev0.0")?;
        let spi_options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(12_000_000)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        spi.configure(&spi_options)?;

        let mut chip = Chip::new("/dev/gpiochip0")?;
        // RST: 17
        let rst_output = chip.get_line(17)?;
        let rst_output_handle = rst_output.request(LineRequestFlags::OUTPUT, 0, "meeting-room")?;
        let rst = CdevPin::new(rst_output_handle)?;
        // BUSY / HDRY: 24
        let busy_input = chip.get_line(24)?;
        let busy_input_handle = busy_input.request(LineRequestFlags::INPUT, 0, "meeting-room")?;
        let busy = CdevPin::new(busy_input_handle)?;

        let driver = it8951::interface::IT8951SPIInterface::new(spi, busy, rst, Delay);
        let mut epd = it8951::IT8951::new(driver).init(1670).unwrap();

        epd.display(it8951::WaveformMode::GrayscaleClearing16).unwrap();



    }

    pub fn draw_splash_screen(&self) -> Result<(), anyhow::Error> {
        self.graphics.default_viewport()
            .shift_down(1400)
            .bmp(
                    &trim_bmp(&logo()?),
                HorizontalAlign::Center,
                VerticalAlign::Top
            );

        self.graphics.default_viewport()
            .shift_down(1800)
            .text(
                "copyright 2024.",
                24.0,
                &typewriter()?,
                HorizontalAlign::Center,
                VerticalAlign::Top,
                Darkness::Medium
            );

        self.graphics.default_viewport()
            .shift_down(400)
            .text(
                "L'åttitüdé",
                    144.0,
                &typewriter()?,
                HorizontalAlign::Center,
                VerticalAlign::Top,
                Darkness::Dark
            );

        self.graphics.default_viewport()
            .shift_down(560)
            .text(
                "weather • time • aqi • calendar",
                56.0,
                &typewriter()?,
                HorizontalAlign::Center,
                VerticalAlign::Top,
                Darkness::Light
            );


        let bmp = self.graphics.to_bmp();
        let res = env::current_dir().unwrap();
        let res = res.join("splash.bmp");
        let result = bmp.save(res);

        Ok(())

    }

    pub fn draw_clear_screen(&self) -> Result<(), anyhow::Error> {
        let bmp = self.graphics.to_bmp();
        let res = env::current_dir().unwrap();
        let res = res.join("clear.bmp");
        let result = bmp.save(res);
        Ok(())
    }

    pub fn draw_data_screen(&self, data: DisplayData) -> Result<(), anyhow::Error> {

        let viewport = self.graphics.viewport((10, 32), (1400, 300));
        self.current(viewport, &data.now)?;

        let viewport = self.graphics.default_viewport()
            .shift_down(420);

        self.hourly_forecast(viewport, &data.hourly_forecast)?;

        self.daily_forecast(&data)?;

        self.header(&data)?;

        let bmp = self.graphics.to_bmp();
        let res = env::current_dir().unwrap();
        let res = res.join("screen.bmp");
        let result = bmp.save(res);
        Ok(())
    }

    fn current<const WIDTH: usize, const HEIGHT: usize>(&self, viewport: ViewPort<'_, WIDTH, HEIGHT>, data: &NowData) -> Result<(), anyhow::Error> {
        //viewport.outline(Color::Black);
        if let Some(temp) = &data.temp {
            let temp_vp = viewport.viewport((0,90), (600, 250));
            let rect = temp_vp
                .text(
                    &format!("{:.1}°", c_to_f(temp.temperature as f64)),
                    160.0,
                    &typewriter()?,
                    HorizontalAlign::Center,
                    VerticalAlign::Center,
                    Darkness::Dark);

            let trend_vp = self.graphics.viewport( (rect.min.x as usize - 100, rect.min.y as usize + 30), (300, (rect.max.y - rect.min.y) as usize - 30));

            match temp.temp_trend {
                Trend::Up => {
                    trend_vp
                        .bmp(
                            &trim_bmp(&arrow_up()?),
                            HorizontalAlign::Left,
                            VerticalAlign::Center,
                        );
                }
                Trend::Down => {
                    trend_vp
                        .bmp(
                            &trim_bmp(&arrow_down()?),
                            HorizontalAlign::Left,
                            VerticalAlign::Center,
                        );
                }
                Trend::Stable => {
                    trend_vp
                        .bmp(
                            &trim_bmp(&arrow_level()?),
                            HorizontalAlign::Left,
                            VerticalAlign::Center,
                        );
                }
            }
        }

        if let Some(aqi_data) = &data.aqi {
            let aqi_vp = viewport.viewport((550, 60), (300, 300));

            aqi_vp
                .bmp(
                    &lighten_bmp(&trim_bmp(&aqi()?), 0.22, false),
                    HorizontalAlign::Center,
                    VerticalAlign::Center,
                );

            aqi_vp
                .text(
                    &format!("{:.0}", aqi_data.current),
                    140.0,
                    &typewriter()?,
                    HorizontalAlign::Center,
                    VerticalAlign::Center,
                    Darkness::Dark);
        }

        if let Some(wind) = &data.wind {
            let wind_vp = viewport.viewport((1000, 0), (300, 300));

            wind_vp.bmp(
                &lighten_bmp(
                    &trim_bmp(
                        &gust_direction_icon(wind.gust_angle)?,
                    ),
                    0.009,
                    true,
                ),
                HorizontalAlign::Center,
                VerticalAlign::Center,
            );

            wind_vp.bmp(
                &trim_bmp(
                    &wind_direction_icon(wind.wind_angle)?
                ),
                HorizontalAlign::Center,
                VerticalAlign::Center,
            );

            let windspeed_vp = viewport.viewport((1000, 250), (300, 300));

            let wind_speed = format!("{} - {} mph",
                                     wind.wind_strength, wind.max_wind_strength
            );
            windspeed_vp.shift_down(30)
                .text(
                    &wind_speed,
                    36.0,
                    &sanserif_bold()?,
                    HorizontalAlign::Center,
                    VerticalAlign::Top,
                    Darkness::Dark,
                );

            windspeed_vp
                .shift_down(70)
                .text(
                    &format!("{} mph", wind.gust_strength),
                    30.0,
                    &sanserif_bold()?,
                    HorizontalAlign::Center,
                    VerticalAlign::Top,
                    Darkness::Light,
                );
        }

        /*
        let viewport = self.graphics.default_viewport()
            .shift_down(120)
            .padded(40);

        if let Some(aqi_data) = &data.now.aqi {
            viewport.bmp((550, 100), &aqi()?);

            viewport.text(
                (670, 120),
                &format!("{:.0}", aqi_data.current),
                64.0,
                &typewriter()?,
                HorizontalAlign::Left);
        }

        if let Some(temp) = &data.now.temp {
            viewport.text(
                (100, 0),
                &format!("{:.0}°", c_to_f(temp.temperature as f64)),
                210.0,
                &typewriter()?,
                HorizontalAlign::Left);

            let arrow = match temp.temp_trend {
                Trend::Up => {
                    arrow_up()?
                }
                Trend::Down => {
                    arrow_down()?
                }
                Trend::Stable => {
                    arrow_level()?
                }
            };

            viewport.bmp((0, 60), &arrow);
        }

        if let Some(humidity) = &data.now.humidity {
            viewport.text(
                (120, 190),
                &format!("{:.0}%", humidity.humidity),
                64.0,
                &typewriter()?,
                HorizontalAlign::Left);
        }


        viewport.circle(
            (1000, 120), 100, Thickness::Heavy, Color::Gray2,
        );

         */
        Ok(())
    }

    fn daily_forecast(&self, data: &DisplayData) -> Result<(), anyhow::Error> {
        let mut viewport = self.graphics.default_viewport()
            .shift_down(640);

        for (i, forecast) in data.daily_forecast.iter().enumerate() {
            if i != 0 {
                viewport = viewport.shift_down(220);
                viewport.hline((10, 0), WIDTH - 20, Thickness::Medium, Color::Gray13);
                viewport.hline((180, 0), WIDTH - 360, Thickness::Medium, Color::Gray8);
                viewport = viewport.shift_down(30);
            }
            self.day_forecast(
                viewport,
                forecast,
                &data.events,
            )?;
        }
        Ok(())
    }

    fn hourly_forecast<const WIDTH: usize, const HEIGHT: usize>(&self, viewport: ViewPort<'_, WIDTH, HEIGHT>, forecast: &Vec<HourlyForecast>) -> Result<(), anyhow::Error> {
        for (i, f) in forecast.iter().enumerate() {
            let hour_vp = viewport.viewport(((112 * i) + 12, 0), (110, 200));
            //hour_vp.outline(Color::Black);
            let hour = if f.date_time.hour() >= 12 {
                if f.date_time.hour() == 12 {
                    "Noon".to_string()
                } else {
                   format!( "{}p", f.date_time.hour() - 12)
                }
            } else {
                if f.date_time.hour() == 0 {
                    "Midnight".to_string()
                } else {
                    format!("{}a", f.date_time.hour())
                }
            };
            hour_vp.text(
                &hour,
                24.0,
                &typewriter_bold()?,
                HorizontalAlign::Center,
                VerticalAlign::Top,
                Darkness::Dark,
            );

            let hour_vp = hour_vp.shift_down(32);

            if let Some(Ok(icon)) = weather_icon(f.weather_icon) {
                hour_vp.bmp(&trim_bmp(&icon),
                            HorizontalAlign::Center,
                            VerticalAlign::Top);
            }

            let hour_vp = hour_vp.shift_down(86);
            hour_vp.text(
                &format!("{}°", f.temperature.value),
                30.0,
                &sanserif_bold()?,
                HorizontalAlign::Center,
                VerticalAlign::Top,
                Darkness::Dark,
            );
        }

        Ok(())
    }

    fn day_forecast<const WIDTH: usize, const HEIGHT: usize>(&self, viewport: ViewPort<'_, WIDTH, HEIGHT>, forecast: &DailyForecast, events: &Vec<Event>) -> Result<(), anyhow::Error> {
        let viewport = viewport.shift_right(30);

        let moonphase_vp = viewport.viewport((1230, 0), (200, 150));

        if let Some(Ok(moonphase_icon)) = moonphase_icon(&forecast.moon.phase) {
            moonphase_vp.bmp(&trim_bmp(&moonphase_icon), HorizontalAlign::Left, VerticalAlign::Top);
        }

        let moonphase_vp = moonphase_vp.shift_down(120);

        let (rise_shift, set_shift)  = match (forecast.moon.rise, forecast.moon.set) {
            (Some(rise), Some(set)) => {
                if rise < set {
                    (0, 25)
                } else {
                    (25, 0)
                }
            },
            (Some(rise), None) => {
                (0, 25)
            }
            (None, Some(set)) => {
                (0, 25)
            }

            (None, None) => {
                (0, 25)
            }
        };

        let rise_vp = moonphase_vp.shift_down(rise_shift);

        rise_vp.bmp(&trim_bmp(&arrow_small_up()?), HorizontalAlign::Left, VerticalAlign::Top);
        rise_vp
            .shift_right(35)
            .text(
                &forecast.moon.rise.map(|inner| human_time(inner)).unwrap_or("--".to_string()),
                24.0,
                &sanserif()?,
                HorizontalAlign::Left,
                VerticalAlign::Top,
                Darkness::Dark,
            );

        let set_vp = moonphase_vp.shift_down(set_shift);
        set_vp.bmp(&trim_bmp(&arrow_small_down()?), HorizontalAlign::Left, VerticalAlign::Top);
        set_vp
            .shift_right(35)
            .text(
                &forecast.moon.set.map(|inner| human_time(inner)).unwrap_or("--".to_string()),
                24.0,
                &sanserif()?,
                HorizontalAlign::Left,
                VerticalAlign::Top,
                Darkness::Dark,
            );

        let today_vp = viewport.shift_right(150);
        today_vp.text(
            &relative_day_name(forecast.date),
            64.0,
            &typewriter_bold()?,
            HorizontalAlign::Left,
            VerticalAlign::Top,
            Darkness::Dark,
        );

        let cleaned_phrase = forecast.day.long_phrase.replace( "; check AccuWeather frequently", "");

        today_vp.shift_down(114)
            .padded_right(200)
            .text(
                &cleaned_phrase,
                30.0,
                &typewriter()?,
                HorizontalAlign::Left,
                VerticalAlign::Top,
                Darkness::Dark,
            );

        let stats_vp = today_vp.viewport(
            (300, 0),
            (500, 500),
        );

        //stats_vp.outline(Color::Black);

        let temp_vp = stats_vp.viewport(
            (0, 0),
            (140, 100),
        );

        let rect = temp_vp.text(
            &format!("{}°", forecast.temperature.maximum.value),
            40.0,
            &sanserif_bold()?,
            HorizontalAlign::Right,
            VerticalAlign::Top,
            Darkness::Dark,
        );

        let rect = temp_vp.relative(rect);

        temp_vp
            .shift_down(50)
            .text(
                &format!("{}°", forecast.temperature.minimum.value),
                32.0,
                &sanserif_bold()?,
                HorizontalAlign::Right,
                VerticalAlign::Top,
                Darkness::Light,
            );

        let rain_vp = stats_vp.viewport(
            (160, 0),
            (200, 80),
        );

        let rect = rain_vp
            .text(
                &format!("{}%", forecast.day.precipitation_probability),
                40.0,
                &sanserif_bold()?,
                HorizontalAlign::Left,
                VerticalAlign::Top,
                Darkness::Dark,
            );

        let rect = rain_vp.relative(rect);

        let total_precip = forecast.day.snow.value + forecast.night.snow.value
        + forecast.day.rain.value + forecast.night.rain.value
            + forecast.day.ice.value + forecast.night.ice.value;

        rain_vp
            .shift_down(50)
            .text(
                &format!("{:.2}\"", total_precip),
                32.0,
                &sanserif_bold()?,
                HorizontalAlign::Left,
                VerticalAlign::Top,
                Darkness::Light,
            );

        let sun_vp = viewport.viewport((0, 0), (100, 150));

        if let Some(Ok(weather_icon)) = weather_icon(forecast.day.icon) {
            //let icon_vp = viewport.viewport((500, 0), (200, 200));
            sun_vp
                .bmp(&trim_bmp(&weather_icon),
                     HorizontalAlign::Center,
                     VerticalAlign::Top);
        }

        let sun_vp = sun_vp.shift_down(120);
        sun_vp.bmp(&trim_bmp(&arrow_small_up()?), HorizontalAlign::Left, VerticalAlign::Top);

        sun_vp
            .shift_right(35)
            .text(
                &human_time(forecast.sun.rise),
                24.0,
                &sanserif()?,
                HorizontalAlign::Left,
                VerticalAlign::Top,
                Darkness::Dark,
            );

        let sun_vp = sun_vp.shift_down(25);
        sun_vp.bmp(&trim_bmp(&arrow_small_down()?), HorizontalAlign::Left, VerticalAlign::Top);

        sun_vp
            .shift_right(35)
            .text(
                &human_time(forecast.sun.set),
                24.0,
                &sanserif()?,
                HorizontalAlign::Left,
                VerticalAlign::Top,
                Darkness::Dark,
            );

        let todays_events: Vec<_> = events.iter().filter(|e| e.date == forecast.date.date_naive()).collect();

        let mut event_vp = viewport.viewport((720, 0), (600, 100));
        for event in todays_events {
            event_vp
                .text(
                    &format!("• {}", event.summary),
                    22.0,
                    &typewriter_bold()?,
                    HorizontalAlign::Left,
                    VerticalAlign::Top,
                    Darkness::Dark,
                );
            event_vp = event_vp.shift_down(26);
        }


        /*
        viewport.text((230, 70),
                      &format!("{}%", forecast.precip_probability.round()),
                      40.0,
                      &lemon_milk_medium()?,
                      HorizontalAlign::Left,
        );

        viewport.text((230, 110),
                      &format!("{:.2}\"", mm_to_inches(forecast.precip)),
                      24.0,
                      &lemon_milk_medium()?,
                      HorizontalAlign::Left,
        );

        viewport.bmp((0, 76), &sunrise()?);
        viewport.text((0, 110),
                      &human_time(forecast.sunrise_ts),
                      22.0,
                      &lemon_milk_medium()?,
                      HorizontalAlign::Left,
        );
        viewport.bmp((90, 76), &sunset()?);
        viewport.text((90, 110),
                      &human_time(forecast.sunset_ts),
                      22.0,
                      &lemon_milk_medium()?,
                      HorizontalAlign::Left,
        );

        let icon = icon_for(&forecast.weather.icon)?;
        viewport.bmp((320, 0), &icon);

        viewport.text(
            (320, 110),
            &forecast.weather.description,
            24.0,
            &lemon_milk_medium()?,
            HorizontalAlign::Left);

        viewport.text(
            (500, 34),
            &format!("{:.0}°", c_to_f(forecast.high_temp)),
            40.0,
            &lemon_milk_medium()?,
            HorizontalAlign::Left);

        viewport.text(
            (500, 74),
            &format!("{:.0}°", c_to_f(forecast.low_temp)),
            32.0,
            &lemon_milk_medium()?,
            HorizontalAlign::Left);

        let lunation = forecast.moon_phase_lunation;

        println!("LUNATION: {}", lunation);
        let phase = if lunation < 0.02 {
            Some(moon_new()?)
        } else if lunation >= 0.49 && lunation <= 0.51 {
            Some(moon_quarter()?)
        } else if lunation >= 0.98 {
            Some(moon_full()?)
        } else {
            None
        };

        if let Some(phase) = phase {
            viewport.bmp((580, 20), &phase);
        }

         */


        Ok(())
    }

    fn header(&self, data: &DisplayData) -> Result<(), anyhow::Error> {
        const FONT_SIZE: f32 = 36.0;

        let vp = self.graphics.default_viewport()
            .padded_left(30)
            .padded_right(30)
            .shift_down(30);

        let local: DateTime<Local> = DateTime::from(data.time);

        let day_name = day_name(local.weekday());

        let month = match local.month() {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => unreachable!()
        };

        let day = local.day();

        let mut hour = local.hour();
        let minute = local.minute();

        let mut ampm = "a";

        if hour >= 12 {
            ampm = "p";
            if hour > 12 {
                hour = hour - 12;
            }
        }

        let date = format!("{}, {} {}", day_name, day, month);
        let bounds1 = vp.text(
            &date,
            FONT_SIZE,
            &typewriter()?,
            HorizontalAlign::Left,
            VerticalAlign::Top,
            Darkness::Dark,
        );

        let time = format!("{}:{:0>2}{ampm}", hour, minute);
        let bounds2 = vp.text(
            &time,
            FONT_SIZE,
            &typewriter()?,
            HorizontalAlign::Right,
            VerticalAlign::Top,
            Darkness::Dark,
        );

        let max_y = max(bounds1.max.y as usize, bounds2.max.y as usize);

        Ok(())
    }
}

pub fn day_name(weekday: Weekday) -> String {
    match weekday {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    }.to_string()
}

pub fn relative_day_name(date: DateTime<Local>) -> String {
    let today = Local::now();
    if today.date_naive() == date.date_naive() {
        "Today".to_string()
    } else if (today.date_naive().checked_add_days(Days::new(1)).unwrap()) == date.date_naive() {
        "Tomorrow".to_string()
    } else {
        day_name(date.weekday())
    }
}

pub fn human_time_epoch(ts: i64) -> String {
    //let date_time = Utc.timestamp_opt(timestamp_u64 as i64, 0).unwrap();
    let date_time = Local.timestamp_opt(ts, 0).unwrap();
    let mut hour = date_time.hour();
    let minute = date_time.minute();

    let ampm = if hour >= 12 {
        if hour > 12 {
            hour = hour - 12;
        }
        "pm"
    } else {
        "am"
    };
    format!("{}:{:0>2}{ampm}", hour, minute)
}

pub fn human_time(date_time: DateTime<Local>) -> String {
    let mut hour = date_time.hour();
    let minute = date_time.minute();

    let ampm = if hour >= 12 {
        if hour > 12 {
            hour = hour - 12;
        }
        "p"
    } else if hour == 0 {
        hour = 12;
        "a"
    } else {
        "a"
    };
    format!("{}:{:0>2}{ampm}", hour, minute)
}

pub fn mm_to_inches(size: f64) -> f64 {
    size / 25.4
}

pub fn c_to_f(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}


pub fn moonphase_icon(phase: &str) -> Option<Result<Image, anyhow::Error>> {
    let icon = match phase {
        "WaningGibbous" => Some(moon_waning_gibbous()),
        "WaxingGibbous" => Some(moon_waxing_gibbous()),
        "WaningCrescent" => Some(moon_waning_crescent()),
        "WaxingCrescent" => Some(moon_waxing_crescent()),
        "FirstQuarter" => Some(moon_first_quarter()),
        "ThirdQuarter" => Some(moon_third_quarter()),
        "Full" => Some(moon_full()),
        "New" => Some(moon_new()),
        "Last" => Some(moon_third_quarter()),
        _ => None
    };

    icon
}

pub fn weather_icon(icon: u8) -> Option<Result<Image, anyhow::Error>> {
    match icon {
        1 => Some(weather::sunny()),
        2 => Some(weather::partly_cloudy()),
        3 => Some(weather::partly_cloudy()),
        4 => Some(weather::scattered_clouds()),
        5 => Some(weather::sunny()),
        6 => Some(weather::partly_cloudy()),
        7 => Some(weather::clouds()),
        8 => Some(weather::clouds()),
        11 => Some(weather::fog()),
        12 => Some(weather::heavy_rain()),
        13 => Some(weather::partly_cloudy_rain()),
        14 => Some(weather::partly_cloudy_rain()),
        15 => Some(weather::thunderstorms()),
        16 => Some(weather::partly_cloudy_thunderstorm()),
        17 => Some(weather::partly_cloudy_thunderstorm()),
        18 => Some(weather::heavy_rain()),
        19 => Some(weather::flurries()),
        20 => Some(weather::flurries()),
        21 => Some(weather::flurries()),
        22 => Some(weather::snow()),
        23 => Some(weather::snow()),
        24 => Some(weather::ice()),
        25 => Some(weather::sleet()),
        26 => Some(weather::sleet()),
        29 => Some(weather::sleet()),
        32 => Some(weather::windy()),
        33 => Some(weather::sunny()),
        34 => Some(weather::partly_cloudy()),
        35 => Some(weather::partly_cloudy()),
        36 => Some(weather::partly_cloudy()),
        37 => Some(weather::partly_cloudy()),
        38 => Some(weather::partly_cloudy()),
        39 => Some(weather::partly_cloudy_rain()),
        40 => Some(weather::partly_cloudy_rain()),
        41 => Some(weather::partly_cloudy_thunderstorm()),
        42 => Some(weather::partly_cloudy_thunderstorm()),
        43 => Some(weather::flurries()),
        44 => Some(weather::snow()),
        _ => None
    }
}

fn wind_direction_icon(wind_angle: u16) -> Result<Image, anyhow::Error> {
    let wind = wind::wind()?;

    let wind_angle = wind_angle + 180;
    let wind_angle = wind_angle % 360;

    if wind_angle < 90 {
        Ok(rotate_bmp(&wind, (90 - wind_angle) as f32))
    } else {
        Ok(rotate_bmp(&wind, (wind_angle - 90) as f32))
    }
}

fn gust_direction_icon(wind_angle: u16) -> Result<Image, anyhow::Error> {
    let wind = wind::gust()?;

    let wind_angle = wind_angle + 180;
    let wind_angle = wind_angle % 360;


    if wind_angle < 90 {
        Ok(rotate_bmp(&wind, (90 - wind_angle) as f32))
    } else {
        Ok(rotate_bmp(&wind, (wind_angle - 90) as f32))
    }
}