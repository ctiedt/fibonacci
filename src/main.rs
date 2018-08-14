extern crate image;
extern crate rand;


use rand::random;
use std::env;
use std::path::Path;

fn fib(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse::<u32>().unwrap();
    let mut colours: Vec<[u8; 3]> = vec![];
    for _ in 0..n {
        colours.push([random::<u8>(), random::<u8>(), random::<u8>()]);
    }
    let size_x = if n % 2 == 0 {
            fib(n)
        } else {
            fib(n) + fib(n - 1)
        };
    let size_y = if n % 2 == 0 {
            fib(n) + fib(n - 1)
        } else {
            fib(n)
        };
    let mut img: image::RgbImage = image::ImageBuffer::new(size_x, size_y);
    for i in 0..n {
        let size = fib(n - i);
        for x in 0..size {
            for y in 0..size {
                img.put_pixel(x, y, image::Rgb::<u8> {data: colours[i as usize]});
            }
        }
    }
    let p = Path::new(&args[2]);
    img.save(p);
}
