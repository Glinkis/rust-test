mod point;
use point::Point;

fn create_point() -> Point {
    Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }
}

fn print_point(vector: &Point) {
    println!("The vector is: {} {} {}", vector.x, vector.y, vector.z);
}

fn increment_point(vector: &mut Point) {
    vector.x += 1.0;
    vector.y += 1.0;
    vector.z += 1.0;
}

fn main() {
    let mut vector = create_point();

    print_point(&vector);
    increment_point(&mut vector);
    print_point(&vector);
    increment_point(&mut vector);
    print_point(&vector);
}
