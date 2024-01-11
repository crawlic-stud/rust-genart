use factorial::Factorial;

#[derive(Debug, Clone)]
pub struct Point(pub f32, pub f32);

fn binomial_coefficient(n: u32, k: u32) -> u32 {
    n.factorial() / (k.factorial() * (n - k).factorial())
}

fn bezier_point_np(t: f32, points: &Vec<Point>) -> Point {
    let n = points.len() as i32 - 1;
    let mut x = 0f32;
    let mut y = 0f32;
    for (i, p) in points.iter().enumerate() {
        let i = i as i32;
        let varx = (binomial_coefficient(n as u32, i as u32) as f32)
            * (1_f32 - t).powi(n - i)
            * t.powi(i)
            * p.0;
        let vary = (binomial_coefficient(n as u32, i as u32) as f32)
            * (1_f32 - t).powi(n - i)
            * t.powi(i)
            * p.1;
        x += varx;
        y += vary;
    }
    Point(x, y)
}

pub fn get_bezier_curve_points(points: &Vec<Point>, precision: u32) -> Vec<Point> {
    let mut bezier_points: Vec<Point> = Vec::new();
    for t in (0..precision).map(|x| x as f32 / precision as f32) {
        let p = bezier_point_np(t, &points);
        bezier_points.push(p);
    }
    bezier_points
}
