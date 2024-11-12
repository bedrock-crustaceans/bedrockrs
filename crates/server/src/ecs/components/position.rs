use shipyard::Component;
use vek::Vec3;

#[derive(Component)]
pub struct Pos(pub Vec3<f32>);
