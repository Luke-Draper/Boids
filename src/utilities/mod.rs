
pub const MAX_HEIGHT: f32 = 100.0;
pub const MAX_HEIGHT_SCALE: f32 = 2.0;
pub const MIN_HEIGHT_SCALE: f32 = 1.0;

pub fn scale_from_height(height: f32) -> Vector3<f32> {
    let scale_factor: f32 = ((height / MAX_HEIGHT) * (MAX_HEIGHT_SCALE - MIN_HEIGHT_SCALE) + MIN_HEIGHT_SCALE);
    Vector3::new(scale_factor, scale_factor, scale_factor)
}