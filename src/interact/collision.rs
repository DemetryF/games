use crate::cosmos_object::{CosmosObject, Vec2};

use super::Interact;

pub struct Collision;
impl Interact for Collision {
    fn interact(a: &mut CosmosObject, b: &mut CosmosObject, _dt: f32) {
        if !Self::detect_collision(a, b) {
            return;
        }

        let (new_a_velocity, new_b_velocity) = Self::velocies_after_hit(a, b);

        a.velocity = new_a_velocity;
        b.velocity = new_b_velocity;
    }
}

impl Collision {
    fn detect_collision(a: &mut CosmosObject, b: &mut CosmosObject) -> bool {
        let dist_sq = (a.position - b.position).length_sq();

        dist_sq <= a.radius + b.radius
    }

    fn velocies_after_hit(a: &mut CosmosObject, b: &mut CosmosObject) -> (Vec2, Vec2) {
        return (calc_velocity(a, b), calc_velocity(b, a));

        fn calc_velocity(a: &mut CosmosObject, b: &mut CosmosObject) -> Vec2 {
            (a.velocity * a.mass * 2.0 + b.velocity * (b.mass - a.mass)) / (a.mass + b.mass)
        }
    }
}
