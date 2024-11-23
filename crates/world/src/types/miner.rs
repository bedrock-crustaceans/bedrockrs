use bedrockrs_core::Vec3;

#[allow(dead_code)]
pub fn idx_3_to_1<T: Clone + Copy + Into<usize>>(vec: Vec3<T>, width: T, height: T) -> usize
where
    usize: From<T>,
{
    (usize::from(vec.x)
        + usize::from(vec.y) * usize::from(width)
        + usize::from(vec.z) * usize::from(width) * usize::from(height))
    .into()
}

#[allow(dead_code)]
pub fn in_bounds<T: std::cmp::PartialOrd>(min: T, max: T, val: T) -> bool {
    val >= min && val <= max
}
