use image::{ImageBuffer, Rgb};
use imageproc::{
    drawing::{draw_antialiased_line_segment_mut, draw_filled_rect_mut},
    rect::Rect,
};
use rand::Rng;

mod bezier;
pub use bezier::{get_bezier_curve_points, Point};

fn fill_background_mut(imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, color: Rgb<u8>) {
    let rect = Rect::at(0, 0).of_size(imgbuf.width(), imgbuf.height());
    draw_filled_rect_mut(imgbuf, rect, color);
}

fn generate_random_points(num: u8, width: u32, height: u32) -> Vec<Point> {
    let mut r = rand::thread_rng();
    let mut points: Vec<Point> = Vec::new();
    for _ in 0..num {
        let x = r.gen_range(0, width) as f32;
        let y = r.gen_range(0, height) as f32;
        points.push(Point(x, y));
    }
    points
}

fn draw_line_for_points(
    imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    points: &Vec<Point>,
    color: Rgb<u8>,
) {
    let last = points.len() - 1;
    for i in 0..last {
        let p1 = &points[i];
        let p2 = &points[i + 1];
        draw_antialiased_line_segment_mut(
            imgbuf,
            (p1.0 as i32, p1.1 as i32),
            (p2.0 as i32, p2.1 as i32),
            color,
            imageproc::pixelops::interpolate,
        );
    }
}

fn draw_bezier_line(
    imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    width: u32,
    height: u32,
    color: Rgb<u8>,
) {
    let points = generate_random_points(5, width, height);
    // println!("Points: {:?}", points);

    let bezier_points = get_bezier_curve_points(&points, 1000);
    draw_line_for_points(imgbuf, &bezier_points, color);
}

fn main() {
    let width = 2000;
    let height = 2000;

    let mut imgbuf: image::ImageBuffer<Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::new(width as u32, height as u32);

    fill_background_mut(&mut imgbuf, Rgb([0, 0, 100]));
    for _ in 0..1000 {
        draw_bezier_line(&mut imgbuf, width, height, Rgb([255, 100, 100]));
    }
    imgbuf.save("bezier.png").unwrap();
}
