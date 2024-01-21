use std::cmp::max;

use image::{ImageBuffer, Rgb};
use imageproc::{
    drawing::{
        draw_antialiased_line_segment_mut, draw_filled_circle_mut, draw_filled_rect_mut,
        draw_polygon_mut,
    },
    point::Point,
    rect::Rect,
};
use rand::{seq::SliceRandom, thread_rng, Rng};

mod bezier;
pub use bezier::get_bezier_curve_points;

const TWO_POINTS_THRESHOLD: f32 = 5.0;

fn fill_background_mut(imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, color: Rgb<u8>) {
    let rect = Rect::at(0, 0).of_size(imgbuf.width(), imgbuf.height());
    draw_filled_rect_mut(imgbuf, rect, color);
}

fn generate_random_points(num: u8, width: u32, height: u32) -> Vec<bezier::Point> {
    let mut r = rand::thread_rng();
    let mut points: Vec<bezier::Point> = Vec::new();
    for _ in 0..num {
        let x = r.gen_range(0, width) as f32;
        let y = r.gen_range(0, height) as f32;
        points.push(bezier::Point(x, y));
    }
    points
}

/// Returns 4 points for thicker line drawing
fn create_polygon_for_line(
    start: &bezier::Point,
    end: &bezier::Point,
    width: u16,
) -> Result<[Point<i32>; 4], String> {
    // calculate a, b, c with some math and pythogorean theorem
    let line_width = (end.0 - start.0).abs() as f32;
    let line_height = (end.1 - start.1).abs() as f32;
    let line_len = two_points_distance(start, end);
    if line_len <= TWO_POINTS_THRESHOLD {
        return Err("Points are too close".to_string());
    }
    let tg = line_height / line_len;
    let ctg = line_width / line_len;
    let half_width = width as f32 / 2.0;
    let points = [
        Point {
            x: (start.0 - half_width * tg) as i32,
            y: (start.1 - half_width * ctg) as i32,
        },
        Point {
            x: (start.0 + half_width * tg) as i32,
            y: (start.1 + half_width * ctg) as i32,
        },
        Point {
            x: (end.0 + half_width * tg) as i32,
            y: (end.1 + half_width * ctg) as i32,
        },
        Point {
            x: (end.0 - half_width * tg) as i32,
            y: (end.1 - half_width * ctg) as i32,
        },
    ];
    if points[0] == points[points.len() - 1] {
        return Err("Bad values for end and start".to_string());
    }
    return Ok(points);
}

fn two_points_distance(start: &bezier::Point, end: &bezier::Point) -> f32 {
    let a = (end.0 - start.0).abs() as f32;
    let b = (end.1 - start.1).abs() as f32;
    let c = (a.powi(2) + b.powi(2)).sqrt();
    return c;
}

fn draw_line_for_points(
    imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    points: &Vec<bezier::Point>,
    color: Rgb<u8>,
    line_width: u16,
) {
    let last = points.len() - 1;
    for i in 0..last {
        let p1 = &points[i];
        let p2 = &points[i + 1];

        let poly = create_polygon_for_line(p1, p2, line_width);
        match poly {
            Ok(points) => draw_polygon_mut(imgbuf, &points, color),
            Err(_) => {
                draw_filled_circle_mut(
                    imgbuf,
                    (p1.0 as i32, p1.1 as i32),
                    (line_width / 2) as i32,
                    color,
                );
                draw_filled_circle_mut(
                    imgbuf,
                    (p2.0 as i32, p2.1 as i32),
                    (line_width / 2) as i32,
                    color,
                );
            }
        }

        // draw_antialiased_line_segment_mut(
        //     imgbuf,
        //     (p1.0 as i32, p1.1 as i32),
        //     (p2.0 as i32, p2.1 as i32),
        //     color,
        //     imageproc::pixelops::interpolate,
        // );
    }
}

fn draw_random_bezier_line(
    imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    width: u32,
    height: u32,
    color: Rgb<u8>,
    precision: u32,
    line_width: u16,
) -> Vec<bezier::Point> {
    let points = generate_random_points(5, width, height);
    // println!("Points: {:?}", points);

    let bezier_points = get_bezier_curve_points(&points, precision);
    draw_line_for_points(imgbuf, &bezier_points, color, line_width);
    return bezier_points;
}

/// Draws random lines for curve points
fn draw_random_lines(
    imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    points: Vec<bezier::Point>,
    color: Rgb<u8>,
    line_width: u16,
    max_lines: u32,
) {
    let mut random_points = points;
    random_points.shuffle(&mut thread_rng());
    let mut random_points_to_draw: Vec<bezier::Point> = vec![];
    for _ in 0..max_lines {
        match random_points.pop() {
            Some(val) => random_points_to_draw.push(val),
            None => {
                println!("Item doesn't exist!");
                break;
            }
        }
    }
    draw_line_for_points(imgbuf, &random_points_to_draw, color, line_width);
}

fn main() {
    let width = 4000;
    let height = 4000;
    let precision = 500;
    let color = Rgb([255, 100, 100]);
    let line_width = 10;
    let iterations = 10;
    let secondary_width = 2;
    let max_secondary_lines = 100;

    let mut imgbuf: image::ImageBuffer<Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::new(width as u32, height as u32);

    fill_background_mut(&mut imgbuf, Rgb([0, 0, 100]));
    for _ in 0..iterations {
        let bezier_points =
            draw_random_bezier_line(&mut imgbuf, width, height, color, precision, line_width);
        draw_random_lines(
            &mut imgbuf,
            bezier_points,
            color,
            secondary_width,
            max_secondary_lines,
        );
    }
    imgbuf.save("bezier.png").unwrap();
}
