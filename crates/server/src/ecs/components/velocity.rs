use shipyard::Component;
use vek::Vec3;

#[derive(Component)]
pub struct Vel(pub Vec3<f32>);
