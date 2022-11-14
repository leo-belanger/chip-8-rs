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

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
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
            .resizable()
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

    pub fn refresh(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(WHITE);

        let window_size = self.canvas.window().size();
        let rect_width = window_size.0 / WIDTH as u32;
        let rect_height = window_size.1 / HEIGHT as u32;

        let mut row_index = 0;

        while row_index < HEIGHT {
            let y = (row_index as u32) * rect_height;
            let mut col_index = 0;

            while col_index < WIDTH {
                if self.pixels[row_index][col_index] {
                    let x = (col_index as u32) * rect_width;

                    self.canvas.fill_rect(Rect::new(
                        x as i32,
                        y as i32,
                        rect_width,
                        rect_height,
                    ))?;
                }

                col_index += 1;
            }

            row_index += 1;
        }

        self.canvas.present();

        Ok(())
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

    pub fn draw_sprite(&mut self, sprite: &[u8], position: Position) -> Result<bool, String> {
        let mut collided = false;

        let mut row = position.y;

        let masks = [0x80, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01];

        for line in sprite {
            if row >= HEIGHT {
                row = 0;
            }

            masks.iter().enumerate().for_each(|(i, mask)| {
                let mut column_index = position.x + i;

                if column_index >= WIDTH {
                    column_index -= WIDTH;
                }

                // Sprites are XORed onto the screen, if a pixel has been erased, then there was a collision
                let current_pixel = self.pixels[row][column_index];
                let new_pixel = (line & mask) != 0;
                let updated_pixel = current_pixel ^ new_pixel;

                if current_pixel && !updated_pixel {
                    collided = true;
                }

                self.pixels[row][column_index] = updated_pixel;
            });

            row += 1;
        }

        Ok(collided)
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
