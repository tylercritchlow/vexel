use vexel::vectors::{vector2::Vector2, vector3::Vector3, vector4::Vector4};

fn main() {
    // Example usage
    let v2_a = Vector2 { x: 1.0, y: 2.0 };
    let v2_b = Vector2 { x: 3.0, y: 4.0 };
    let v3_a = Vector3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let v3_b = Vector3 {
        x: 4.0,
        y: 5.0,
        z: 6.0,
    };
    let v4_a = Vector4 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        w: 4.0,
    };
    let v4_b = Vector4 {
        x: 5.0,
        y: 6.0,
        z: 7.0,
        w: 8.0,
    };

    println!("========================================");

    println!("v2_a + v2_b = {:?}", v2_a + v2_b);
    println!("v3_a + v3_b = {:?}", v3_a + v3_b);
    println!("v4_a + v4_b = {:?}", v4_a + v4_b);

    println!("========================================");

    println!("v2_a - v2_b = {:?}", v2_a - v2_b);
    println!("v3_a - v3_b = {:?}", v3_a - v3_b);
    println!("v4_a - v4_b = {:?}", v4_a - v4_b);

    println!("========================================");

    println!("v2_a * v2_b = {:?}", v2_a - v2_b);
    println!("v3_a * v3_b = {:?}", v3_a - v3_b);
    println!("v4_a * v4_b = {:?}", v4_a - v4_b);

    println!("========================================");

    println!("v2_a / v2_b = {:?}", v2_a / v2_b);
    println!("v3_a / v3_b = {:?}", v3_a / v3_b);
    println!("v4_a / v4_b = {:?}", v4_a / v4_b);

    println!("========================================");

    // Example of length calculation
    println!("Length of Vector2: {}", v2_a.length());
    println!("Length of Vector3: {}", v3_a.length());
    println!("Length of Vector4: {}", v4_a.length());

    println!("========================================");

    // Example of dot product calculation
    println!("Dot Product of Vector2: {}", v2_a.dot(&v2_b));
    println!("Dot Product of Vector3: {}", v3_a.dot(&v3_b));
    println!("Dot Product of Vector4: {}", v4_a.dot(&v4_b));

    println!("========================================");

    // Example of cross product calculation
    println!("Cross Product of Vector2: {:?}", v2_a.cross(&v2_a));
    println!("Cross Product of Vector3: {:?}", v3_a.cross(&v3_b));
    println!("Cross Product of Vector4: {:?}", v4_a.cross(&v4_b));

    println!("========================================");

    // Example of projection
    println!("Projection onto of Vector2: {:?}", v2_a.project_onto(&v2_b));
    println!("Projection onto of Vector3: {:?}", v3_a.project_onto(&v3_b));
    println!("Projection onto of Vector4: {:?}", v4_a.project_onto(&v4_b));

    println!("========================================");

    // Example of Linear Interpolation
    println!("Lerp for Vector2 (t = 0.5): {:?}", v2_a.lerp(&v2_b, 0.5));
    println!("Lerp for Vector3 (t = 0.5): {:?}", v3_a.lerp(&v3_b, 0.5));
    println!("Lerp for Vector4 (t = 0.5): {:?}", v4_a.lerp(&v4_b, 0.5));

    println!("Lerp for Vector2 (t = 0.0): {:?}", v2_a.lerp(&v2_b, 0.0)); // Should return v2_a
    println!("Lerp for Vector3 (t = 1.0): {:?}", v3_a.lerp(&v3_b, 1.0)); // Should return v3_b

    println!("========================================");

    // Example of Swizzling
    println!("Swizzled Vector2 (yx): {:?}", v2_a.swizzle(1, 0)); // Swap x and y
    println!("Swizzled Vector3 (yxz): {:?}", v3_a.swizzle(1, 0, 2)); // Swap x and y
    println!("Swizzled Vector4 (wzyx): {:?}", v4_a.swizzle(3, 2, 1, 0)); // Reverse order

    println!("========================================");

    // Example of Angle Between Vectors
    println!(
        "Angle between Vector2 (radians): {}",
        v2_a.angle_between(&v2_b)
    );
    println!(
        "Angle between Vector3 (radians): {}",
        v3_a.angle_between(&v3_b)
    );
    println!(
        "Angle between Vector4 (radians): {}",
        v4_a.angle_between(&v4_b)
    );

    println!("========================================");

    let v1 = Vector3::new(1.0, 0.0, -3.0);
    let v2 = Vector3::new(0.0, 1.0, 2.0);
    let angle = v1.angle_between(&v2);
    println!("Angle between Vector3 (radians): {}", angle);
}
