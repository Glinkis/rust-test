mod point;
use point::Point;

fn create_point() -> Point<f32> {
    Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }
}

fn print_point(point: &Point<f32>) {
    println!("{}", point.to_string())
}

fn increment_point(point: &mut Point<f32>) {
    point.x += 1.0;
    point.y += 1.0;
    point.z += 1.0;
}

fn main() {
    let mut point = create_point();

    print_point(&point);

    increment_point(&mut point);
    print_point(&point);

    increment_point(&mut point);
    print_point(&point);
}
