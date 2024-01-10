use crate::accuweather::daily_forecast::DailyForecast;
use crate::accuweather::hourly_forecast::HourlyForecast;
use crate::art::{aqi, arrow_down, arrow_level, arrow_small_down, arrow_small_up, arrow_up, logo, moon_first_quarter, moon_full, moon_new, moon_third_quarter, moon_waning_crescent, moon_waning_gibbous, moon_waxing_crescent, moon_waxing_gibbous, usb, weather, wind};
use crate::calendar::Event;
use crate::data::data::{DisplayData, NowData};
use crate::font::{sanserif, sanserif_bold, typewriter, typewriter_bold};
use crate::graphics::{lighten_bmp, rotate_bmp, trim_bmp, Color, Darkness, Graphics, Thickness, ViewPort, scale_bmp};
use crate::netatmo::Trend;
use crate::paint::Paint;
use bmp::Image;
use chrono::{DateTime, Datelike, Local, Timelike, Weekday, Utc};
use glyph_brush_layout::{HorizontalAlign, VerticalAlign};
use std::env;
use crate::graphics::Darkness::Dark;

pub const WIDTH: usize = 1404;
pub const HEIGHT: usize = 1872;

pub struct Display<'p, P: Paint> {
    graphics: Graphics<WIDTH, HEIGHT>,
    paint: &'p mut P,
}

impl<'p, P: Paint> Display<'p, P> {
    pub fn new(paint: &'p mut P) -> Self {
        Self {
            graphics: Graphics::new(),
            paint,
        }
    }

    pub fn paint(&mut self) -> Result<(), anyhow::Error> {
        let bmp = self.graphics.to_bmp();
        let res = env::current_dir().unwrap();
        let res = res.join("lattitude.bmp");
        let _ = bmp.save(res);
        self.paint.paint(&self.graphics)
    }

    pub fn paint_partial(&mut self, origin: (usize, usize), dimensions: (usize, usize)) -> Result<(), anyhow::Error> {
        let bmp = self.graphics.to_bmp();
        let res = env::current_dir().unwrap();
        let res = res.join("lattitude.bmp");
        let _ = bmp.save(res);
        self.paint.paint_partial(&self.graphics, origin, dimensions)
        //self.paint.paint(&self.graphics)
    }
    pub fn draw_unbox_screen(&mut self) -> Result<(), anyhow::Error> {
        self.graphics.default_viewport().shift_down(400).bmp(
            &trim_bmp(&usb()?),
            HorizontalAlign::Center,
            VerticalAlign::Top,
        );
        self.graphics.default_viewport().shift_down(600).text(
            "plug me in.",
            48.0,
            &typewriter()?,
            HorizontalAlign::Center,
            VerticalAlign::Top,
            Darkness::Medium,
        );

        self.graphics.default_viewport().shift_down(1300).bmp(
            &trim_bmp(&lighten_bmp(&logo()?, 0.5, false)),
            HorizontalAlign::Center,
            VerticalAlign::Top,
        );

        self.graphics.default_viewport().shift_down(1700).text(
            "\"always innovative; sometimes overly ambitious\"",
            44.0,
            &sanserif_bold()?,
            HorizontalAlign::Center,
            VerticalAlign::Top,
            Darkness::Medium,
        );

        self.paint()
    }

    pub fn draw_splash_screen(&mut self) -> Result<(), anyhow::Error> {
        self.graphics.default_viewport().shift_down(1300).bmp(
            &trim_bmp(&lighten_bmp(&logo()?, 0.5, false)),
            HorizontalAlign::Center,
            VerticalAlign::Top,
        );

        self.graphics.default_viewport().shift_down(1700).text(
            "Domestic Info Hub Division",
            24.0,
            &typewriter_bold()?,
            HorizontalAlign::Center,
            VerticalAlign::Top,
            Darkness::Medium,
        );

        self.graphics.default_viewport().shift_down(1800).text(
            "copyright 2024.",
            24.0,
            &typewriter()?,
            HorizontalAlign::Center,
            VerticalAlign::Top,
            Darkness::Medium,
        );

        self.graphics.default_viewport().shift_down(400).text(
            "L'åttitüdé",
            144.0,
            &typewriter()?,
            HorizontalAlign::Center,
            VerticalAlign::Top,
            Darkness::Dark,
        );

        self.graphics.default_viewport().shift_down(560).text(
            "weather • time • aqi • calendar",
            56.0,
            &typewriter()?,
            HorizontalAlign::Center,
            VerticalAlign::Top,
            Darkness::Light,
        );

        self.paint()?;

        Ok(())
    }

    pub fn draw_clear_screen(&mut self) -> Result<(), anyhow::Error> {
        self.paint()?;

        Ok(())
    }

    pub fn draw_data_screen(&mut self, data: &DisplayData, time: DateTime<Utc>) -> Result<(), anyhow::Error> {
        let viewport = self.graphics.viewport((10, 20), (1400, 300));
        self.current(viewport, &data.now, &data.birds)?;

        let viewport = self.graphics.default_viewport().shift_down(420);

        self.hourly_forecast(viewport, &data.hourly_forecast)?;

        self.daily_forecast(&data)?;

        self.header(time)?;

        self.paint()?;

        Ok(())
    }

    fn current<const WIDTH: usize, const HEIGHT: usize>(
        &self,
        viewport: ViewPort<'_, WIDTH, HEIGHT>,
        data: &NowData,
        birds: &Vec<String>,
    ) -> Result<(), anyhow::Error> {
        //viewport.outline(Color::Black);
        if let Some(temp) = &data.temp {
            let temp_vp = viewport.viewport((0, 40), (600, 250));
            if let Some(temperature) = temp.temperature {
                let rect = temp_vp.text(
                    &format!("{:.1}°", c_to_f(temperature as f64)),
                    100.0,
                    &typewriter()?,
                    HorizontalAlign::Center,
                    VerticalAlign::Center,
                    Darkness::Dark,
                );

                let trend_vp = self.graphics.viewport(
                    (rect.min.x as usize - 100, rect.min.y as usize + 30),
                    (300, (rect.max.y - rect.min.y) as usize - 30),
                );

                match &temp.temp_trend {
                    Some(Trend::Up) => {
                        trend_vp.bmp(
                            &trim_bmp(&arrow_up()?),
                            HorizontalAlign::Left,
                            VerticalAlign::Center,
                        );
                    }
                    Some(Trend::Down) => {
                        trend_vp.bmp(
                            &trim_bmp(&arrow_down()?),
                            HorizontalAlign::Left,
                            VerticalAlign::Center,
                        );
                    }
                    Some(Trend::Stable) => {
                        trend_vp.bmp(
                            &trim_bmp(&arrow_level()?),
                            HorizontalAlign::Left,
                            VerticalAlign::Center,
                        );
                    }
                    None => { /* nothing */ }
                }
            }
        }

        if let Some(aqi_data) = &data.aqi {
            let aqi_vp = viewport.viewport((400, 0), (300, 300));

            aqi_vp.bmp(
                &scale_bmp(
                    &lighten_bmp(&trim_bmp(&aqi()?), 0.22, false),
                    0.5,
                ),
                HorizontalAlign::Center,
                VerticalAlign::Center,
            );

            aqi_vp.text(
                &format!("{:.0}", aqi_data.current),
                80.0,
                &typewriter()?,
                HorizontalAlign::Center,
                VerticalAlign::Center,
                Darkness::Dark,
            );
        }

        if let Some(wind) = &data.wind {
            let wind_vp = viewport.viewport((560, 110), (300, 300));

            wind_vp.bmp(
                &scale_bmp(
                    &lighten_bmp(
                        &trim_bmp(&gust_direction_icon(wind.gust_angle)?),
                        0.009,
                        true,
                    ),
                    0.7,
                ),
                HorizontalAlign::Center,
                VerticalAlign::Center,
            );

            wind_vp.bmp(
                &scale_bmp(
                    &trim_bmp(&wind_direction_icon(wind.wind_angle)?),
                    0.7,
                ),
                HorizontalAlign::Center,
                VerticalAlign::Center,
            );

            let windspeed_vp = viewport.viewport((400, 160), (300, 300));

            let wind_speed = format!("{}-{}", wind.wind_strength, wind.max_wind_strength);
            windspeed_vp.shift_down(50).text(
                &wind_speed,
                40.0,
                &typewriter_bold()?,
                HorizontalAlign::Center,
                VerticalAlign::Top,
                Darkness::Dark,
            );

            windspeed_vp.shift_down(100).text(
                &format!("{}\nmph", wind.gust_strength),
                30.0,
                &typewriter()?,
                HorizontalAlign::Center,
                VerticalAlign::Top,
                Darkness::Light,
            );
        }

        let mut bird_vp = viewport.viewport((880, 100), (500, 300));

        for bird in birds {
            bird_vp.text(
                &format!("•{}",bird),
                30.0,
                &typewriter()?,
                HorizontalAlign::Left,
                VerticalAlign::Top,
                Darkness::Dark,
            );
            bird_vp = bird_vp.shift_down(36);
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
        let mut viewport = self.graphics.default_viewport().shift_down(640);

        for (i, forecast) in data.daily_forecast.iter().enumerate() {
            if i != 0 {
                viewport = viewport.shift_down(220);
                viewport.hline((10, 0), WIDTH - 20, Thickness::Medium, Color::Gray13);
                viewport.hline((180, 0), WIDTH - 360, Thickness::Medium, Color::Gray8);
                viewport = viewport.shift_down(30);
            }
            self.day_forecast(viewport, forecast, &data.events)?;
        }
        Ok(())
    }

    fn hourly_forecast<const WIDTH: usize, const HEIGHT: usize>(
        &self,
        viewport: ViewPort<'_, WIDTH, HEIGHT>,
        forecast: &[HourlyForecast],
    ) -> Result<(), anyhow::Error> {
        for (i, f) in forecast.iter().enumerate() {
            let hour_vp = viewport.viewport(((112 * i) + 12, 0), (110, 200));
            //hour_vp.outline(Color::Black);
            let hour = if f.date_time.hour() >= 12 {
                if f.date_time.hour() == 12 {
                    "Noon".to_string()
                } else {
                    format!("{}p", f.date_time.hour() - 12)
                }
            } else if f.date_time.hour() == 0 {
                "Midnight".to_string()
            } else {
                format!("{}a", f.date_time.hour())
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
                hour_vp.bmp(
                    &trim_bmp(&icon),
                    HorizontalAlign::Center,
                    VerticalAlign::Top,
                );
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

    fn day_forecast<const WIDTH: usize, const HEIGHT: usize>(
        &self,
        viewport: ViewPort<'_, WIDTH, HEIGHT>,
        forecast: &DailyForecast,
        events: &[Event],
    ) -> Result<(), anyhow::Error> {
        let viewport = viewport.shift_right(30);

        let moonphase_vp = viewport.viewport((1230, 0), (200, 150));

        if let Some(Ok(moonphase_icon)) = moonphase_icon(&forecast.moon.phase) {
            moonphase_vp.bmp(
                &trim_bmp(&moonphase_icon),
                HorizontalAlign::Left,
                VerticalAlign::Top,
            );
        }

        let moonphase_vp = moonphase_vp.shift_down(120);

        let (rise_shift, set_shift) = match (forecast.moon.rise, forecast.moon.set) {
            (Some(rise), Some(set)) => {
                if rise < set {
                    (0, 25)
                } else {
                    (25, 0)
                }
            }
            (Some(_rise), None) => (0, 25),
            (None, Some(_set)) => (0, 25),

            (None, None) => (0, 25),
        };

        let rise_vp = moonphase_vp.shift_down(rise_shift);

        rise_vp.bmp(
            &trim_bmp(&arrow_small_up()?),
            HorizontalAlign::Left,
            VerticalAlign::Top,
        );
        rise_vp.shift_right(35).text(
            &forecast
                .moon
                .rise
                .map(human_time)
                .unwrap_or("--".to_string()),
            24.0,
            &sanserif()?,
            HorizontalAlign::Left,
            VerticalAlign::Top,
            Darkness::Dark,
        );

        let set_vp = moonphase_vp.shift_down(set_shift);
        set_vp.bmp(
            &trim_bmp(&arrow_small_down()?),
            HorizontalAlign::Left,
            VerticalAlign::Top,
        );
        set_vp.shift_right(35).text(
            &forecast
                .moon
                .set
                .map(human_time)
                .unwrap_or("--".to_string()),
            24.0,
            &sanserif()?,
            HorizontalAlign::Left,
            VerticalAlign::Top,
            Darkness::Dark,
        );

        let today_vp = viewport.shift_right(150);
        today_vp.text(
            &relative_day_name(forecast.date),
            40.0,
            &typewriter_bold()?,
            HorizontalAlign::Left,
            VerticalAlign::Top,
            Darkness::Dark,
        );

        let cleaned_phrase = forecast
            .day
            .long_phrase
            .replace("; check AccuWeather frequently", "");

        today_vp.shift_down(114).padded_right(200).text(
            &cleaned_phrase,
            30.0,
            &typewriter()?,
            HorizontalAlign::Left,
            VerticalAlign::Top,
            Darkness::Dark,
        );

        let stats_vp = today_vp.viewport((240, 0), (500, 500));

        let temp_vp = stats_vp.viewport((0, 0), (140, 50));

        let min_rect = temp_vp.text(
            &format!("{}°", forecast.temperature.minimum.value),
            42.0,
            &typewriter_bold()?,
            HorizontalAlign::Right,
            VerticalAlign::Center,
            Darkness::Light,
        );

        temp_vp
            .padded_right(min_rect.width() as usize + 10)
            .text(
                &format!("{}°", forecast.temperature.maximum.value),
                42.0,
                &typewriter_bold()?,
                HorizontalAlign::Right,
                VerticalAlign::Center,
                Darkness::Dark,
            );

        let rain_vp = stats_vp.viewport((170, 0), (200, 50));

        let prob_rect = rain_vp.text(
            &format!("{}%", forecast.day.precipitation_probability),
            32.0,
            &typewriter()?,
            HorizontalAlign::Left,
            VerticalAlign::Center,
            Darkness::Dark,
        );

        let total_precip = forecast.day.snow.value
            + forecast.night.snow.value
            + forecast.day.rain.value
            + forecast.night.rain.value
            + forecast.day.ice.value
            + forecast.night.ice.value;

        rain_vp.shift_right(prob_rect.width() as usize + 10).text(
            &format!("{:.2}\"", total_precip),
            32.0,
            &typewriter()?,
            HorizontalAlign::Left,
            VerticalAlign::Center,
            Darkness::Light,
        );

        let sun_vp = viewport.viewport((0, 0), (100, 150));

        if let Some(Ok(weather_icon)) = weather_icon(forecast.day.icon) {
            //let icon_vp = viewport.viewport((500, 0), (200, 200));
            sun_vp.bmp(
                &trim_bmp(&weather_icon),
                HorizontalAlign::Center,
                VerticalAlign::Top,
            );
        }

        let sun_vp = sun_vp.shift_down(120);
        sun_vp.bmp(
            &trim_bmp(&arrow_small_up()?),
            HorizontalAlign::Left,
            VerticalAlign::Top,
        );

        sun_vp.shift_right(35).text(
            &human_time(forecast.sun.rise),
            24.0,
            &sanserif()?,
            HorizontalAlign::Left,
            VerticalAlign::Top,
            Darkness::Dark,
        );

        let sun_vp = sun_vp.shift_down(25);
        sun_vp.bmp(
            &trim_bmp(&arrow_small_down()?),
            HorizontalAlign::Left,
            VerticalAlign::Top,
        );

        sun_vp.shift_right(35).text(
            &human_time(forecast.sun.set),
            24.0,
            &sanserif()?,
            HorizontalAlign::Left,
            VerticalAlign::Top,
            Darkness::Dark,
        );

        let todays_events: Vec<_> = events
            .iter()
            .filter(|e| e.date == forecast.date.date_naive())
            .collect();

        let mut event_vp = viewport.viewport((720, 0), (600, 100));
        for event in todays_events {
            event_vp.text(
                &format!("• {}", event.summary),
                22.0,
                &typewriter_bold()?,
                HorizontalAlign::Left,
                VerticalAlign::Top,
                Darkness::Dark,
            );
            event_vp = event_vp.shift_down(26);
        }

        Ok(())
    }

    pub fn draw_header_only(&mut self, time: DateTime<Utc>) -> Result<(), anyhow::Error> {
        println!("{:?}", time);
        self.header(time)?;
        self.paint_partial((0, 0), (WIDTH, 76))?;
        Ok(())
    }

    fn header(&self, time: DateTime<Utc>) -> Result<(), anyhow::Error> {
        const FONT_SIZE: f32 = 36.0;

        let vp = self
            .graphics
            .default_viewport()
            .padded_left(30)
            .padded_right(30)
            .shift_down(30);

        let local: DateTime<Local> = DateTime::from(time);

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
            _ => unreachable!(),
        };

        let day = local.day();

        let mut hour = local.hour();
        let minute = local.minute();

        let mut ampm = "a";

        if hour >= 12 {
            ampm = "p";
            if hour > 12 {
                hour = hour - 12
            }
        }

        let date = format!("{}, {} {}", day_name, day, month);
        vp.text(
            &date,
            FONT_SIZE,
            &typewriter()?,
            HorizontalAlign::Left,
            VerticalAlign::Top,
            Darkness::Dark,
        );

        let time = format!("{}:{:0>2}{ampm}", hour, minute);
        vp.text(
            &time,
            FONT_SIZE,
            &typewriter()?,
            HorizontalAlign::Right,
            VerticalAlign::Top,
            Darkness::Dark,
        );

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
    }
        .to_string()
}

pub fn relative_day_name(date: DateTime<Local>) -> String {
    let today = Local::now();
    if today.date_naive() == date.date_naive() {
        "Today".to_string()
    } else {
        day_name(date.weekday())
    }
}

pub fn human_time(date_time: DateTime<Local>) -> String {
    let mut hour = date_time.hour();
    let minute = date_time.minute();

    let ampm = if hour >= 12 {
        if hour > 12 {
            hour -= 12;
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

pub fn c_to_f(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

pub fn moonphase_icon(phase: &str) -> Option<Result<Image, anyhow::Error>> {
    match phase {
        "WaningGibbous" => Some(moon_waning_gibbous()),
        "WaxingGibbous" => Some(moon_waxing_gibbous()),
        "WaningCrescent" => Some(moon_waning_crescent()),
        "WaxingCrescent" => Some(moon_waxing_crescent()),
        "FirstQuarter" => Some(moon_first_quarter()),
        "ThirdQuarter" => Some(moon_third_quarter()),
        "Full" => Some(moon_full()),
        "New" => Some(moon_new()),
        "Last" => Some(moon_third_quarter()),
        _ => None,
    }
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
        _ => None,
    }
}

fn wind_direction_icon(wind_angle: i16) -> Result<Image, anyhow::Error> {
    let wind = wind::wind()?;

    let wind_angle = wind_angle + 180;
    let wind_angle = wind_angle % 360;

    if wind_angle < 90 {
        Ok(rotate_bmp(&wind, (90 - wind_angle) as f32))
    } else {
        Ok(rotate_bmp(&wind, (wind_angle - 90) as f32))
    }
}

fn gust_direction_icon(wind_angle: i16) -> Result<Image, anyhow::Error> {
    let wind = wind::gust()?;

    let wind_angle = wind_angle + 180;
    let wind_angle = wind_angle % 360;

    if wind_angle < 90 {
        Ok(rotate_bmp(&wind, (90 - wind_angle) as f32))
    } else {
        Ok(rotate_bmp(&wind, (wind_angle - 90) as f32))
    }
}
