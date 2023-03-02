pub mod collision;
pub mod gravity;

use crate::cosmos_object::CosmosObject;

pub use self::{collision::Collision, gravity::Gravity};

pub trait Interact {
    fn interact(a: &mut CosmosObject, b: &mut CosmosObject, dt: f32);
}
