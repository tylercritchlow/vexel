<h1 align='center'>
<code>vexel</code>
</h1>
<div align='center'>
linear algebra in Rust
</div>

## Overview
`vexel` is a linear algebra library for Rust, designed to be used for the Impulse Physics Engine. We are designing it to be fast and simple to use in other use cases as well. 

It will provide a variety of features, including:
- [x] Vector Operations (Add, Sub, Mult, Div, Dot, Cross, Normalize, Length, Lerp, Projection/Rejection, Angle Between Vectors, Swizzling for Vector2, Vector3, Vector4)
- [ ] Matrix Operations (Matrix Multiplication, Transpose, Determinant, Inverse, Identity Matrix, Matrix-Vector Multiplication, Row/Column Access, Matrix Decomposition, LookAt Matrix, Perspective Projection, Orthographic Projection for Mat2, Mat3, Mat4)
- [ ] Utility Functions (Constants like PI, Clamping, Interpolation, Random Generation, Comparison)
- [ ] Transformations (Translation, Rotation, Scaling, Shear, LookAt for camera transformation, Perspective/Orthographic Projection, Quaternion Support e.g. slerp, normalization, conjugation)
- [ ] Quaternion Operations (Normalization, Conjugation, Multiplication, Slerp, From/To Rotation Matrices, From/To Axis-Angle, Dot Product, Angle Between Quaternions)
- [ ] Geometric Operations (Plane Operations, Line/Line Segment Operations, Intersection Tests, Bounding Volumes, Raycasting, Frustum Culling, Distance Between Points/Shapes)
- [ ] Extras (Frustum Culling, Color Space Conversion, Spline Interpolation, Noise Functions)


## License
Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.