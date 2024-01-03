use std::cell::RefCell;
use std::f64::consts::PI;
use std::fmt::{Debug, Formatter};
use ab_glyph::{Font, FontRef, PxScale, Rect};
use anyhow::anyhow;
use bmp::{Image, Pixel};
use glyph_brush_layout::{BuiltInLineBreaker, FontId, GlyphPositioner, HorizontalAlign, Layout, SectionGeometry, SectionGlyph, SectionText, VerticalAlign};

const BPP: usize = 2;

#[derive(Copy, Clone, Debug)]
pub enum Color {
    Black,
    Gray1,
    Gray2,
    Gray3,
    Gray4,
    Gray5,
    Gray6,
    Gray7,
    Gray8,
    Gray9,
    Gray10,
    Gray11,
    Gray12,
    Gray13,
    Gray14,
    White,
}

#[derive(Copy, Clone, Debug)]
pub enum Thickness {
    Thin = 2,
    Medium = 3,
    Heavy = 4,
}

#[derive(Copy, Clone, Debug)]
pub enum Darkness {
    Dark,
    Medium,
    Light,
}

impl Darkness {
    pub fn coverage_to_color(&self, c: f32) -> Color {
        match self {
            Darkness::Dark => {
                if c > 0.9 {
                    Color::Black
                } else if c > 0.7 {
                    Color::Gray7
                } else if c > 0.5 {
                    Color::Gray13
                } else {
                    Color::White
                }
            }
            Darkness::Medium => {
                if c > 0.9 {
                    Color::Gray4
                } else if c > 0.7 {
                    Color::Gray8
                } else if c > 0.5 {
                    Color::Gray13
                } else {
                    Color::White
                }
            }
            Darkness::Light => {
                if c > 0.9 {
                    Color::Gray9
                } else if c > 0.7 {
                    Color::Gray11
                } else if c > 0.5 {
                    Color::Gray13
                } else {
                    Color::White
                }
            }
        }
    }
}

impl From<&Color> for Pixel {
    fn from(value: &Color) -> Self {
        match value {
            Color::Black => Pixel {
                r: 0,
                g: 0,
                b: 0,
            },
            Color::Gray1 => Pixel {
                r: 15,
                g: 15,
                b: 15,
            },
            Color::Gray2 => Pixel {
                r: 31,
                g: 31,
                b: 31,
            },
            Color::Gray3 => Pixel {
                r: 47,
                g: 47,
                b: 47,
            },
            Color::Gray4 => Pixel {
                r: 63,
                g: 63,
                b: 63,
            },
            Color::Gray5 => Pixel {
                r: 79,
                g: 79,
                b: 79,
            },
            Color::Gray6 => Pixel {
                r: 95,
                g: 95,
                b: 95,
            },
            Color::Gray7 => Pixel {
                r: 111,
                g: 111,
                b: 111,
            },
            Color::Gray8 => Pixel {
                r: 127,
                g: 127,
                b: 127,
            },
            Color::Gray9 => Pixel {
                r: 143,
                g: 143,
                b: 143,
            },
            Color::Gray10 => Pixel {
                r: 159,
                g: 159,
                b: 159,
            },
            Color::Gray11 => Pixel {
                r: 175,
                g: 175,
                b: 175,
            },
            Color::Gray12 => Pixel {
                r: 191,
                g: 191,
                b: 191,
            },
            Color::Gray13 => Pixel {
                r: 207,
                g: 207,
                b: 207,
            },
            Color::Gray14 => Pixel {
                r: 223,
                g: 223,
                b: 223,
            },
            Color::White => Pixel {
                r: 255,
                g: 255,
                b: 255,
            }
        }
    }
}

impl From<Pixel> for Color {
    fn from(value: Pixel) -> Self {
        let r = value.r as f32;
        let g = value.g as f32;
        let b = value.b as f32;

        let y = (0.299 * r + 0.587 * g + 0.114 * b).round() as u8;

        if y >= 239 {
            Color::White
        } else if y >= 223 {
            Color::Gray14
        } else if y >= 207 {
            Color::Gray13
        } else if y >= 191 {
            Color::Gray12
        } else if y >= 175 {
            Color::Gray11
        } else if y >= 159 {
            Color::Gray10
        } else if y >= 143 {
            Color::Gray9
        } else if y >= 127 {
            Color::Gray8
        } else if y >= 111 {
            Color::Gray7
        } else if y >= 95 {
            Color::Gray6
        } else if y >= 79 {
            Color::Gray5
        } else if y >= 63 {
            Color::Gray4
        } else if y >= 47 {
            Color::Gray3
        } else if y >= 31 {
            Color::Gray2
        } else if y >= 15 {
            Color::Gray1
        } else {
            Color::Black
        }
    }
}

pub struct Graphics<const WIDTH: usize, const HEIGHT: usize> {
    pixels: RefCell<Vec<Vec<Color>>>,
}

impl<const WIDTH: usize, const HEIGHT: usize> Graphics<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        Self {
            pixels: RefCell::new(
                vec![vec![Color::White; WIDTH]; HEIGHT]
            ),
        }
        //[[Color::White; WIDTH]; HEIGHT]
    }

    fn set(&self, (x, y): (usize, usize), color: Color) {
        if x >= WIDTH || y >= HEIGHT {
            return;
        }
        self.pixels.borrow_mut()[y][x] = color;
    }

    /*
    pub fn hline(&mut self, x: usize, y: usize, width: usize, thickness: Thickness, color: Color) {
        for row in 0..(thickness as u8) {
            for col in 0..width {
                self.set(
                    x + col,
                    y + row as usize,
                    color
                );
            }
        }
    }

     */


    pub fn to_bmp(&self) -> Image {
        let mut image = Image::new(WIDTH as u32, HEIGHT as u32);

        for (y, row) in self.pixels.borrow().iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                image.set_pixel(x as u32, y as u32, pixel.into());
            }
        }

        image
    }

    pub fn default_viewport(&self) -> ViewPort<'_, WIDTH, HEIGHT> {
        self.viewport(
            (0, 0),
            (WIDTH, HEIGHT),
        )
    }

    pub fn viewport(&self, (x_offset, y_offset): (usize, usize), (width, height): (usize, usize)) -> ViewPort<'_, WIDTH, HEIGHT> {
        ViewPort {
            graphics: self,
            x_offset,
            y_offset,
            width,
            height,
        }
    }
}

#[derive(Copy, Clone)]
pub struct ViewPort<'g, const WIDTH: usize, const HEIGHT: usize> {
    pub graphics: &'g Graphics<WIDTH, HEIGHT>,
    pub x_offset: usize,
    pub y_offset: usize,
    pub width: usize,
    pub height: usize,
}

impl<const WIDTH: usize, const HEIGHT: usize> Debug for ViewPort<'_, WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{}) w:{} h:{}", self.x_offset, self.y_offset, self.width, self.height)
    }
}

impl<'g, const WIDTH: usize, const HEIGHT: usize> ViewPort<'g, WIDTH, HEIGHT> {
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn set(&self, (x, y): (usize, usize), color: Color) {
        self.graphics.set((x + self.x_offset, y + self.y_offset), color);
    }

    pub fn hline(&self, (x, y): (usize, usize), length: usize, thickness: Thickness, color: Color) {
        for row in 0..(thickness as u8) {
            for col in 0..length {
                self.set(
                    (x + col, y + row as usize),
                    color,
                );
            }
        }
    }

    pub fn bmp(&self, image: &Image, h_align: HorizontalAlign, v_align: VerticalAlign) {
        let x_offset = match h_align {
            HorizontalAlign::Left => self.x_offset,
            HorizontalAlign::Center => ((self.width - image.get_width() as usize) / 2) + self.x_offset,
            HorizontalAlign::Right => self.width - image.get_width() as usize
        };

        let y_offset = match v_align {
            VerticalAlign::Top => self.y_offset,
            VerticalAlign::Center => ((self.height - image.get_height() as usize) / 2) + self.y_offset,
            VerticalAlign::Bottom => self.height - image.get_height() as usize
        };

        for x in 0..image.get_width() {
            for y in 0..image.get_height() {
                let pixel = image.get_pixel(x, y).into();
                if ! matches!( pixel, Color::White ) {
                    self.graphics.set((x as usize + x_offset, y as usize + y_offset),
                                      pixel,
                    )
                }
            }
        }
    }

    pub fn text(&self, text: &str, size: f32, font: &FontRef, h_align: HorizontalAlign, v_align: VerticalAlign, darkness: Darkness) -> Rect {
        let scale = PxScale::from(size);

        let layout = Layout::Wrap {
            line_breaker: BuiltInLineBreaker::default(),
            h_align,
            v_align,
        };

        let screen_x = match h_align {
            HorizontalAlign::Left => { self.x_offset }
            HorizontalAlign::Center => { (self.width / 2) + self.x_offset }
            HorizontalAlign::Right => { self.x_offset + self.width }
        };

        let screen_y = match v_align {
            VerticalAlign::Top => { self.y_offset }
            VerticalAlign::Center => { (self.height / 2) + self.y_offset }
            VerticalAlign::Bottom => { self.y_offset + self.height }
        };

        let screen_position = (screen_x as f32, screen_y as f32);

        let bounds = (
            self.width as f32,
            self.height as f32,
        );

        let glpyhs = layout
            .calculate_glyphs(
                &[font.clone()],
                &SectionGeometry {
                    screen_position,
                    bounds,
                },
                &[
                    SectionText {
                        text,
                        scale,
                        font_id: FontId(0),
                    }
                ],
            );

        let mut min_x = 5000.0;
        let mut min_y = 5000.0;

        let mut max_x = 0.0;
        let mut max_y = 0.0;

        for glyph in &glpyhs {
            let rect = self.glyph(glyph, font, darkness);
            if rect.min.x < min_x {
                min_x = rect.min.x
            }
            if rect.min.y < min_y {
                min_y = rect.min.y
            }

            if rect.max.x > max_x {
                max_x = rect.max.x
            }

            if rect.max.y > max_y {
                max_y = rect.max.y
            }
        }

        Rect {
            min: (min_x, min_y).into(),
            max: (max_x, max_y).into(),
        }
        //((min_x, min_y), (max_x, max_y)).into()
    }

    pub fn glyph(&self, glyph: &SectionGlyph, font: &FontRef, darkness: Darkness) -> Rect {
        if let Some(glyph) = font.outline_glyph(glyph.glyph.clone()) {
            let x_offset = glyph.px_bounds().min.x;
            let y_offset = glyph.px_bounds().min.y;
            glyph.draw(|x, y, c| {
                let color = darkness.coverage_to_color(c);
                if !matches!(color, Color::White) {
                    self.graphics.set((
                                          (x as f32 + x_offset) as usize,
                                          (y as f32 + y_offset) as usize,
                                      ), color);
                }
            });
            glyph.px_bounds()
        } else {
            Rect::default()
        }
    }

    pub fn old_text(&self, (mut origin_x, mut origin_y): (usize, usize), text: &str, size: f32, font: &FontRef, h_align: HorizontalAlign) {
        let scale = PxScale::from(size);

        let layout = Layout::SingleLine {
            line_breaker: BuiltInLineBreaker::default(),
            h_align,
            v_align: VerticalAlign::Top,
        };

        let screen_position = match h_align {
            HorizontalAlign::Left => (0 as f32, 0 as f32),
            HorizontalAlign::Center => ((self.width() / 2) as f32, 0 as f32),
            HorizontalAlign::Right => ((self.width() - 1) as f32, 0 as f32),
        };

        let glpyhs = layout
            .calculate_glyphs(
                &[font.clone()],
                &SectionGeometry {
                    screen_position,
                    bounds: ((self.width() - 1) as f32, (self.height() - 1) as f32),
                },
                &[
                    SectionText {
                        text,
                        scale,
                        font_id: FontId(0),
                    }
                ],
            );

        let right_align_fix = if matches!( h_align, HorizontalAlign::Right) {
            let mut max = 0.0;
            for metrics in &glpyhs {
                if let Some(glyph) = font.outline_glyph(metrics.glyph.clone()) {
                    if glyph.px_bounds().max.x > max {
                        max = glyph.px_bounds().max.x
                    }
                }
            }

            if max.ceil() as usize >= (self.width() - 1) {
                max.ceil() as usize - self.width()
            } else {
                0
            }
        } else {
            0
        };

        for metrics in glpyhs {

            if let Some(glyph) = font.outline_glyph(metrics.glyph) {
                let x_offset = if glyph.px_bounds().min.x >= 0.0 {
                    glyph.px_bounds().min.x as usize
                } else {
                    (self.width() - glyph.px_bounds().min.x as usize)
                };

                let x_offset = x_offset - right_align_fix;

                let y_offset = if glyph.px_bounds().min.y > 0.0 {
                    glyph.px_bounds().min.y as usize
                } else {
                    self.height() - glyph.px_bounds().min.y as usize
                };

                glyph.draw(|x, y, c| {
                    let color = if c > 0.9 {
                        Color::Black
                    } else if c > 0.7 {
                        Color::Gray7
                    } else if c > 0.5 {
                        Color::Gray13
                    } else {
                        Color::White
                    };
                    if !matches!(color, Color::White) {
                        self.set((
                                     x as usize + x_offset + origin_x,
                                     y as usize + y_offset + origin_y,
                                 ), color);
                    }
                })
            }
        }
    }

    pub fn viewport(&self, (x_offset, y_offset): (usize, usize), (width, height): (usize, usize)) -> ViewPort<'g, WIDTH, HEIGHT> {
        Self {
            graphics: self.graphics,
            x_offset: self.x_offset + x_offset,
            y_offset: self.y_offset + y_offset,
            width,
            height,
        }
    }

    pub fn padded(&self, pixels: usize) -> ViewPort<'g, WIDTH, HEIGHT> {
        Self {
            graphics: &self.graphics,
            x_offset: self.x_offset + pixels,
            y_offset: self.y_offset + pixels,
            width: self.width - (pixels * 2),
            height: self.height - (pixels * 2),
        }
    }

    pub fn padded_left(&self, pixels: usize) -> ViewPort<'g, WIDTH, HEIGHT> {
        Self {
            graphics: &self.graphics,
            x_offset: self.x_offset + pixels,
            y_offset: self.y_offset,
            width: self.width - pixels,
            height: self.height,
        }
    }

    pub fn padded_right(&self, pixels: usize) -> ViewPort<'g, WIDTH, HEIGHT> {
        Self {
            graphics: &self.graphics,
            x_offset: self.x_offset,
            y_offset: self.y_offset,
            width: self.width - pixels,
            height: self.height,
        }
    }

    pub fn shift_down(&self, pixels: usize) -> ViewPort<'g, WIDTH, HEIGHT> {
        Self {
            graphics: &self.graphics,
            x_offset: self.x_offset,
            y_offset: self.y_offset + pixels,
            width: self.width,
            height: self.height - pixels,
        }
    }

    pub fn shift_right(&self, pixels: usize) -> ViewPort<'g, WIDTH, HEIGHT> {
        Self {
            graphics: &self.graphics,
            x_offset: self.x_offset + pixels,
            y_offset: self.y_offset,
            width: self.width - pixels,
            height: self.height,
        }
    }

    pub fn shift_left(&self, pixels: usize) -> ViewPort<'g, WIDTH, HEIGHT> {
        Self {
            graphics: &self.graphics,
            x_offset: self.x_offset - pixels,
            y_offset: self.y_offset,
            width: self.width + pixels,
            height: self.height,
        }
    }

    pub fn outline(&self, color: Color) {
        for y in 0..self.height {
            self.set((0, y), color);
            self.set((self.width, y), color);
        }

        for x in 0..self.width {
            self.set((x, 0), color);
            self.set((x, self.height), color);
        }
    }

    pub fn relative(&self, rect: Rect) -> Rect {
        Rect {
            min: (rect.min.x - self.x_offset as f32, rect.min.y - self.y_offset as f32).into(),
            max: (rect.max.x - self.x_offset as f32, rect.max.y - self.y_offset as f32).into(),
        }
    }
}

pub fn lighten_bmp(image: &Image, percentage: f32, debug: bool) -> Image {
    let mut lightened = Image::new(
        image.get_width(),
        image.get_height(),
    );

    for x in 0..image.get_width() {
        for y in 0..image.get_height() {
            let pixel = image.get_pixel(x, y);
            let new_pixel =  Pixel::new(
                ((pixel.r as f32 + 2.0) * (1.0 / percentage)) as u8,
                ((pixel.g as f32 + 2.0) * (1.0 / percentage)) as u8,
                ((pixel.b as f32 + 2.0) * (1.0 / percentage)) as u8,
            );

            if debug {
                //println!("{:?} -> {:?}", pixel, new_pixel);
            }
            lightened.set_pixel(
                x, y,
                new_pixel
            );
        }
    }

    lightened
}

pub fn trim_bmp(image: &Image) -> Image {
    let mut first_row = 0;
    let mut last_row = image.get_height();

    let mut first_column = 0;
    let mut last_column = image.get_width();

    'outer:
    for y in 0..image.get_height() {
        for x in 0..image.get_width() {
            let color: Color = image.get_pixel(x, y).into();

            if !matches!( color, Color::White ) {
                first_row = y;
                break 'outer;
            }
        }
    }

    'outer:
    for y in (0..image.get_height()).rev() {
        for x in 0..image.get_width() {
            let color: Color = image.get_pixel(x, y).into();

            if !matches!( color, Color::White ) {
                last_row = y;
                break 'outer;
            }
        }
    }

    'outer:
    for x in 0..image.get_width() {
        for y in 0..image.get_height() {
            let color: Color = image.get_pixel(x, y).into();

            if !matches!( color, Color::White ) {
                first_column = x;
                break 'outer;
            }
        }
    }

    'outer:
    for x in (0..image.get_width()).rev() {
        for y in 0..image.get_height() {
            let color: Color = image.get_pixel(x, y).into();

            if !matches!( color, Color::White ) {
                last_column = x;
                break 'outer;
            }
        }
    }

    let trimmed_width = last_column - first_column;
    let trimmed_height = last_row - first_row;

    let mut trimmed = Image::new(
        trimmed_width,
        trimmed_height,
    );

    for (new_x, x) in (first_column..last_column).enumerate() {
        for (new_y, y) in (first_row..last_row).enumerate() {
            trimmed.set_pixel(new_x as u32, new_y as u32,
                              image.get_pixel(x, y),
            )
        }
    }

    trimmed
}

pub fn rotate_bmp(image: &Image, degrees: f32) -> Image {
    let radians = (degrees * PI as f32) / 180.0;

    let mut rotated = Image::new(image.get_width(), image.get_height());

    let cos = radians.cos();
    let sin = radians.sin();

    let cx = (image.get_width() - 1) / 2;
    let cy = (image.get_height() - 1) / 2;

    for x in 0..rotated.get_width() {
        for y in 0..rotated.get_height() {
            let ox = (cos * (x as f32 - cx as f32) + (sin * (y as f32 - cy as f32)) + rotated.get_width() as f32 / 2.0) as u32;
            let oy = (cos * (y as f32 - cy as f32) - (sin * (x as f32 - cx as f32)) + rotated.get_height() as f32 / 2.0) as u32;

            if ox < image.get_width() && oy < image.get_height() {
                let pixel = image.get_pixel(
                    ox, oy,
                );

                rotated.set_pixel(
                    x, y,
                    pixel,
                );
            } else {
                rotated.set_pixel(
                    x, y,
                    Pixel::new(255, 255, 255)
                )

            }
        }
    }

    rotated
}