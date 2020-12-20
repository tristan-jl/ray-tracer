use crate::{IMAGE_HEIGHT, IMAGE_WIDTH};

use std::{fs::File, io::prelude::*};
#[derive(Copy, Clone)]
pub struct Pixel {
    pub r: i32,
    pub g: i32,
    pub b: i32,
}
impl Pixel {
    fn new() -> Pixel {
        Self { r: 0, g: 0, b: 0 }
    }
    fn from(r: i32, g: i32, b: i32) -> Pixel {
        Self { r, g, b }
    }

    fn to_tuple(&self) -> (i32, i32, i32) {
        (self.r, self.g, self.b)
    }
}

pub struct Image {
    pub pixels: [[Pixel; IMAGE_WIDTH as usize]; IMAGE_HEIGHT as usize],
}
impl Image {
    pub fn new() -> Self {
        Self {
            pixels: [[Pixel::new(); IMAGE_WIDTH as usize]; IMAGE_HEIGHT as usize],
        }
    }

    pub fn write_image(&self, mut file: File) {
        write!(file, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)
            .expect("Unable to write to file");

        for i in (0..IMAGE_HEIGHT).rev() {
            eprint!("\rWriting Image: {}/{} ", IMAGE_HEIGHT - i, IMAGE_HEIGHT);
            for j in 0..IMAGE_WIDTH {
                let pixel = self.pixels[i as usize][j as usize];
                write!(file, "{} {} {}\n", pixel.r, pixel.g, pixel.b)
                    .expect("Unable to write to file");
            }
        }
        eprintln!("\nDone.")
    }
}
