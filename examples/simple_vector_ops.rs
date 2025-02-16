use vexel::vectors::{vector2::Vector2, vector3::Vector3, vector4::Vector4};

fn main() {
    // Example usage
    let v2_a = Vector2 { x: 1.0, y: 2.0 };
    let v2_b = Vector2 { x: 3.0, y: 4.0 };
    let v2_sum = v2_a + v2_b;
    println!("Vector2 Sum: {:?}", v2_sum);

    let v3_a = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
    let v3_b = Vector3 { x: 4.0, y: 5.0, z: 6.0 };
    let v3_product = v3_a * v3_b;
    println!("Vector3 Product: {:?}", v3_product);

    let v4_a = Vector4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
    let v4_b = Vector4 { x: 5.0, y: 6.0, z: 7.0, w: 8.0 };
    let v4_diff = v4_a - v4_b;
    println!("Vector4 Difference: {:?}", v4_diff);

    // Example of length calculation
    println!("Length of Vector2: {}", v2_a.length());
    println!("Length of Vector3: {}", v3_a.length());
    println!("Length of Vector4: {}", v4_a.length());

    // Example of dot product calculation
    println!("Dot Product of Vector2: {}", v2_a.dot(&v2_b));
    println!("Dot Product of Vector3: {}", v3_a.dot(&v3_b));
    println!("Dot Product of Vector4: {}", v4_a.dot(&v4_b));

    // Example of cross product calculation (3D vectors only)
    println!("Cross Product of Vector3: {:?}", v3_a.cross(&v3_b));
}