mod point;
use point::Point;

fn create_point() -> Point {
    Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }
}

fn print_point(point: &Point) {
    println!("The vector is: {} {} {}", point.x, point.y, point.z);
}

fn increment_point(point: &mut Point) {
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
