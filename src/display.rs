use std::time::Duration;
use actix_web::delete;
use anyhow::Result;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

pub const DISPLAY_HEIGHT: u32 = 480;
pub const DISPLAY_WIDTH: u32 = 800;

const BITS_PER_PIXEL: u32 = 1;
const BUFFER_SIZE: usize = (DISPLAY_WIDTH * DISPLAY_HEIGHT / (8 * BITS_PER_PIXEL)) as usize;

pub struct Display {
    pub sdl: Sdl,
    pub canvas: WindowCanvas,
    pixels: Vec<u8>
}

#[derive(Debug)]
pub enum BinaryColor {
    On, Off
}

impl Display {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("Virtual Display", DISPLAY_WIDTH, DISPLAY_HEIGHT)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .software()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Display {
            sdl: sdl_context,
            canvas,
            pixels: vec![0; BUFFER_SIZE],
        })
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.canvas.clear();
        Ok(())
    }

    fn get_pixel(&self, x: u32, y: u32) -> BinaryColor {
        let index = (x + y * DISPLAY_WIDTH) as usize;
        let byte_index = index / 8;
        let bit_index = index % 8;

        if self.pixels[byte_index] & (0x80 >> bit_index) != 0 {
            BinaryColor::On
        } else {
            BinaryColor::Off
        }
    }

    pub fn flush(&mut self, buffer: Vec<u8>) -> Result<()> {
        self.pixels = buffer;
        log::info!("Setting buffer");
        Ok(())
    }

    pub fn refresh(&mut self) -> Result<()> {
        for x in 0..DISPLAY_WIDTH {
            for y in 0..DISPLAY_HEIGHT {
                let color = self.get_pixel(x, y);

                match color {
                    BinaryColor::On => { self.canvas.set_draw_color(Color::BLACK); }
                    BinaryColor::Off => { self.canvas.set_draw_color(Color::WHITE); }
                };

                self.canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
            }
        }
        self.canvas.present();
        Ok(())
    }

}

