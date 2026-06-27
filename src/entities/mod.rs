use avian2d::dynamics::rigid_body::LinearVelocity;
use bevy::{
    app::{
        App,
        FixedUpdate,
        Plugin,
    },
    ecs::{
        component::Component,
        system::Query,
    },
    math::Vec2,
};

pub mod scenes;

#[derive(Component, Default, Clone)]
pub struct EntityMovement {
    pub direction: Vec2,
    pub speed: f32,
}

fn handle_entity_movement(entities: Query<(&mut LinearVelocity, &EntityMovement)>) {
    for (mut linear_velocity, movement) in entities {
        linear_velocity.0 = movement.direction * movement.speed;
    }
}

#[derive(Default)]
pub struct UpToChanceEntityPlugin;

impl Plugin for UpToChanceEntityPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_systems(FixedUpdate, handle_entity_movement);
    }
}
