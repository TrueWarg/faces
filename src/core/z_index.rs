pub const FLOOR_Z: f32 = 0.0;
pub const WALL_Z: f32 = 1.0;
pub const ON_WALL_OBJECT_Z: f32 = 2.0;
pub const MIN_RANGE_Z: f32 = 15.0;
pub const MAX_RABGE_Z: f32 = 3.0;
pub const DEFAULT_OBJECT_Z: f32 = 5.0;

pub fn calculate_z(y: f32, y_max: f32) -> f32 {
    return (MAX_RABGE_Z - MIN_RANGE_Z) * (y / y_max) + MIN_RANGE_Z;
}
