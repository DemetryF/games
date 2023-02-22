use crate::cosmos_object::CosmosObject;

use super::Interact;

pub struct Gravity;
impl Interact for Gravity {
    fn interact(a: &mut CosmosObject, b: &mut CosmosObject, dt: f32) {
        let rect = b.position - a.position;
        let dist = rect.length_sq().sqrt();

        let direction = rect / dist;

        let force = a.mass * b.mass / dist.powi(2);

        let a_acceleration = force / a.mass;
        let b_acceleration = force / b.mass;

        a.velocity += direction * a_acceleration * dt;
        b.velocity += -direction * b_acceleration * dt;
    }
}
