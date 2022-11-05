extern crate sdl2;

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window, Sdl};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

const BLACK: Color = Color::RGB(0, 0, 0);
const WHITE: Color = Color::RGB(255, 255, 255);

#[derive(Debug)]
pub struct Position {
    x: usize,
    y: usize,
}

pub struct Display {
    pixels: [[bool; WIDTH]; HEIGHT],
    canvas: Canvas<Window>,
}

impl Display {
    pub fn new(sdl_context: &Sdl) -> Display {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Chip-8-rs", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(BLACK);
        canvas.clear();
        canvas.present();

        Display {
            pixels: [[false; WIDTH]; HEIGHT],
            canvas,
        }
    }
    pub fn clear(&mut self) {
        self.pixels.fill([false; 64]);

        self.canvas.set_draw_color(BLACK);
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn refresh(&mut self) {
        self.canvas.set_draw_color(BLACK);
        self.canvas.clear();
        self.canvas.present();

        self.canvas.set_draw_color(WHITE);

        let mut row_index = 0;
        let mut col_index = 0;

        let window_size = self.canvas.window().size();
        let rect_width = window_size.0 / WIDTH as u32;
        let rect_height = window_size.1 / HEIGHT as u32;

        while row_index < HEIGHT {
            while col_index < WIDTH {
                if self.pixels[row_index][col_index] {
                    let x = col_index as u32 * rect_width;
                    let y = row_index as u32 * rect_width;

                    self.canvas
                        .fill_rect(Rect::new(x as i32, y as i32, rect_width, rect_height));
                }

                col_index += 1;
            }

            row_index += 1;
        }

        self.canvas.present();
    }

    pub fn draw_character_from_font(
        &mut self,
        character: u8,
        position: Position,
    ) -> Result<(), String> {
        let character_starting_index: usize = (character * 5).into();

        if character_starting_index >= FONT_DATA.len() {
            return Err(format!(
                "Could not find character {:#04X?} to draw at position {:?}.",
                character, position
            ));
        }

        let sprite = [
            FONT_DATA[character_starting_index],
            FONT_DATA[character_starting_index + 1],
            FONT_DATA[character_starting_index + 2],
            FONT_DATA[character_starting_index + 3],
            FONT_DATA[character_starting_index + 4],
        ];

        self.draw_sprite(&sprite, position)?;

        Ok(())
    }

    pub fn draw_sprite(&mut self, sprite: &[u8], position: Position) -> Result<(), String> {
        if position.x + 7 >= WIDTH || position.y + sprite.len() >= HEIGHT {
            return Err(
                format!("Attempting to draw sprite {:?} at position {:?} would exceed screen bounds of {} by {} pixels.", sprite, position, WIDTH, HEIGHT),
            );
        }

        let mut row = position.y;

        for line in sprite {
            self.pixels[row][position.x] = line & 0x80 == 0x80;
            self.pixels[row][position.x + 1] = line & 0x40 == 0x40;
            self.pixels[row][position.x + 2] = line & 0x20 == 0x20;
            self.pixels[row][position.x + 3] = line & 0x10 == 0x10;
            self.pixels[row][position.x + 4] = line & 0x08 == 0x08;
            self.pixels[row][position.x + 5] = line & 0x04 == 0x04;
            self.pixels[row][position.x + 6] = line & 0x02 == 0x02;
            self.pixels[row][position.x + 7] = line & 0x01 == 0x01;

            row += 1;
        }

        self.refresh();

        Ok(())
    }
}

#[rustfmt::skip]
pub static FONT_DATA: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, /* "0" */
    0x20, 0x60, 0x20, 0x20, 0x70, /* "1" */
    0xF0, 0x10, 0xF0, 0x80, 0xF0, /* "2" */
    0xF0, 0x10, 0xF0, 0x10, 0xF0, /* "3" */
    0x90, 0x90, 0xF0, 0x10, 0x10, /* "4" */
    0xF0, 0x80, 0xF0, 0x10, 0xF0, /* "5" */
    0xF0, 0x80, 0xF0, 0x90, 0xF0, /* "6" */
    0xF0, 0x10, 0x20, 0x40, 0x40, /* "7" */
    0xF0, 0x90, 0xF0, 0x90, 0xF0, /* "8" */
    0xF0, 0x90, 0xF0, 0x10, 0xF0, /* "9" */
    0xF0, 0x90, 0xF0, 0x90, 0x90, /* "A" */
    0xE0, 0x90, 0xE0, 0x90, 0xE0, /* "B" */
    0xF0, 0x80, 0x80, 0x80, 0xF0, /* "C" */
    0xE0, 0x90, 0x90, 0x90, 0xE0, /* "D" */
    0xF0, 0x80, 0xF0, 0x80, 0xF0, /* "E" */
    0xF0, 0x80, 0xF0, 0x80, 0x80, /* "F" */
];
