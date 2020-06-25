extern crate image;
extern crate rand;

use rand::random;
use std::env;
use std::path::Path;

#[derive(Copy, Clone)]
struct RGBColour(u8, u8, u8);

impl RGBColour {
    fn as_pixel(self) -> image::Rgb<u8> {
        image::Rgb::<u8> {
            data: [self.0, self.1, self.2],
        }
    }

    fn random() -> Self {
        Self(random::<u8>(), random::<u8>(), random::<u8>())
    }
}

fn fib(n: u32) -> u32 {
    match n {
        0..=1 => n,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Call this program as fibonacci <n> <path>")
    }
    let n = args[1].parse::<u32>().unwrap();
    let mut colours: Vec<RGBColour> = vec![];
    for _ in 0..n {
        colours.push(RGBColour::random());
    }
    let size_x = match n % 2 {
        0 => fib(n),
        _ => fib(n) + fib(n - 1),
    };
    let size_y = match n % 2 {
        0 => fib(n) + fib(n - 1),
        _ => fib(n),
    };
    let mut img: image::RgbImage = image::ImageBuffer::new(size_x, size_y);
    let mut start_x = 0;
    let mut start_y = 0;
    for i in 0..n {
        let size = fib(n - i);
        match i % 4 {
            0 => {
                for x in 0..size {
                    for y in 0..size {
                        img.put_pixel(start_x + x, start_y + y, colours[i as usize].as_pixel());
                    }
                }
                start_x += size - 1;
                start_y += size;
            }
            1 => {
                for x in 0..size {
                    for y in 0..size {
                        img.put_pixel(start_x - x, start_y + y, colours[i as usize].as_pixel());
                    }
                }
                start_x -= size;
                start_y += size - 1;
            }
            2 => {
                for x in 0..size {
                    for y in 0..size {
                        img.put_pixel(start_x - x, start_y - y, colours[i as usize].as_pixel());
                    }
                }
                start_x -= size - 1;
                start_y -= size;
            }
            _ => {
                for x in 0..size {
                    for y in 0..size {
                        img.put_pixel(start_x + x, start_y - y, colours[i as usize].as_pixel());
                    }
                }
                start_x += size;
                start_y -= size - 1;
            }
        }
    }

    let p = Path::new(&args[2]);
    let _ = img.save(p);
}
