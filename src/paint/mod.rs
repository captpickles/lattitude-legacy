use anyhow::Error;
use crate::graphics::Graphics;

pub trait Paint {

    fn paint<const WIDTH: usize, const HEIGHT: usize>(&mut self, graphics: &Graphics<WIDTH, HEIGHT>) -> Result<(), anyhow::Error>;

}

pub struct NoOpPaint;

impl Paint for NoOpPaint {
    fn paint<const WIDTH: usize, const HEIGHT: usize>(&mut self, graphics: &Graphics<WIDTH, HEIGHT>) -> Result<(), Error> {
        todo!()
    }
}

#[cfg(feature = "linux-embedded-hal")]
pub mod epd {
    use anyhow::Error;
    use embedded_graphics::pixelcolor::{Gray4, GrayColor};
    use it8951::interface::IT8951SPIInterface;
    use it8951::{AreaImgInfo, IT8951, memory_converter_settings, Run};
    use it8951::memory_converter_settings::MemoryConverterSetting;
    use crate::graphics::{Color, Graphics};
    use crate::paint::Paint;
    use linux_embedded_hal::gpio_cdev::{Chip, LineRequestFlags};
    use linux_embedded_hal::spidev::{SpiModeFlags, SpidevOptions};
    use linux_embedded_hal::{CdevPin, Delay, Spidev};

    pub struct EpdPaint {
        pub epd:  IT8951<IT8951SPIInterface<Spidev, CdevPin, CdevPin, Delay>, Run>,
    }

    impl EpdPaint {
        pub fn new() -> Self {
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
            let mut epd = it8951::IT8951::new(driver).init(1550).unwrap();

            Self {
                epd
            }
        }
    }

    impl Paint for EpdPaint {
        fn paint<const WIDTH: usize, const HEIGHT: usize>(&mut self, graphics: &Graphics<WIDTH, HEIGHT>) -> Result<(), Error> {

            let buffer = graphics.pixels.borrow();

            const CHUNK_SIZE: usize = 2;

            let chunks = buffer.chunks(CHUNK_SIZE);

            for (chunk, rows) in chunks.enumerate() {
                let mut data = [0; (WIDTH * CHUNK_SIZE)/4];
                println!("addr {}", self.epd.get_dev_info().memory_address);
                println!("h-w {}x{}", self.epd.get_dev_info().panel_height, self.epd.get_dev_info().panel_width);
                println!("chunk {chunk} data len {}", data.len());
                let mut cur = 0;
                for row in rows.iter() {
                    for (x, color) in row.iter().rev().enumerate() {
                        let color: Gray4 = color.into();
                        data[cur] = data[cur] | (color.luma() as u16) << ((x % 4) * 4);
                        if x % 4 == 3 {
                            cur += 1;
                        }
                    }
                }
                if let Err(err) = self.epd.load_image_area(
                    self.epd.get_dev_info().memory_address,
                    MemoryConverterSetting {
                        endianness: memory_converter_settings::MemoryConverterEndianness::LittleEndian,
                        bit_per_pixel: memory_converter_settings::MemoryConverterBitPerPixel::BitsPerPixel4,
                        rotation: memory_converter_settings::MemoryConverterRotation::Rotate270,
                    },
                    &AreaImgInfo {
                        area_x: 0,
                        area_y: (chunk * CHUNK_SIZE) as u16,
                        area_w: WIDTH as u16,
                        area_h: CHUNK_SIZE as u16,
                    },
                    &data,
                ) {
                    println!("{:#?}", err);
                }
            }

            println!("display()");
            self.epd.display(it8951::WaveformMode::GrayscaleClearing16).unwrap();
            println!("display() done");
            Ok(())
        }
    }

    impl From<&Color> for Gray4 {
        fn from(value: &Color) -> Self {
            match value {
                Color::Black => Gray4::new(0),
                Color::Gray1 => Gray4::new(1),
                Color::Gray2 => Gray4::new(2),
                Color::Gray3 => Gray4::new(3),
                Color::Gray4 => Gray4::new(4),
                Color::Gray5 => Gray4::new(5),
                Color::Gray6 => Gray4::new(6),
                Color::Gray7 => Gray4::new(7),
                Color::Gray8 => Gray4::new(8),
                Color::Gray9 => Gray4::new(9),
                Color::Gray10 => Gray4::new(10),
                Color::Gray11 => Gray4::new(11),
                Color::Gray12 => Gray4::new(12),
                Color::Gray13 => Gray4::new(13),
                Color::Gray14 => Gray4::new(14),
                Color::White => Gray4::new(15)
            }
        }
    }

}
