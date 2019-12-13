mod vector;
use vector::Vector2;

fn create_vector2() -> Vector2 {
	Vector2 { x: 0.0, y: 0.0 }
}

fn print_vector2(vector: &Vector2) {
	println!("The vector is: {} {}", vector.x, vector.y);
}

fn increment_vector2(vector: &mut Vector2) {
	vector.x += 1.0;
	vector.y += 1.0;
}

fn main() {
	let mut vector = create_vector2();

	print_vector2(&vector);
	increment_vector2(&mut vector);
	print_vector2(&vector);
	print_vector2(&vector);
}
