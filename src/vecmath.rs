
pub type Vector3 = [f64, ..3];
pub type Matrix4x3 = [[f64, ..3], ..4];

#[inline(always)]
pub fn vec3_sub(a: Vector3, b: Vector3) -> Vector3 {
    [
        a[0] - b[0],
        a[1] - b[1],
        a[2] - b[2],
    ]
}

#[inline(always)]
pub fn vec3_len(a: Vector3) -> f64 {
    a[0] * a[0] + a[1] * a[1] + a[2] * a[2]
}

#[inline(always)]
pub fn vec3_inv_len(a: Vector3) -> f64 {
    1.0 / vec3_len(a)
}

#[inline(always)]
pub fn vec3_normalized(a: Vector3) -> Vector3 {
    let inv_len = vec3_inv_len(a);
    [
        a[0] * inv_len,
        a[1] * inv_len,
        a[2] * inv_len,
    ]
}

#[inline(always)]
pub fn vec3_normalized_sub(a: Vector3, b: Vector3) -> Vector3 {
    vec3_normalized(vec3_sub(a, b))
}

#[inline(always)]
pub fn mat4x3_transform(mat: Matrix4x3, a: Vector3) -> Vector3 {
    [
        mat[0][0] * a[0] + mat[1][0] * a[1] + mat[2][0] * a[2] + mat[3][0],
        mat[0][1] * a[0] + mat[1][1] * a[1] + mat[2][1] * a[2] + mat[3][1],
        mat[0][2] * a[0] + mat[1][2] * a[1] + mat[2][2] * a[2] + mat[3][2], 
    ]
}

