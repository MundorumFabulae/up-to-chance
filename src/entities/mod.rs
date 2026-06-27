use avian2d::dynamics::rigid_body::LinearVelocity;
use bevy::{
    app::{
        App,
        FixedUpdate,
        Plugin,
    },
    ecs::system::Query,
    math::Vec2,
    scene::{
        Scene,
        SceneComponent,
        bsn,
    },
};

pub mod scenes;

#[derive(SceneComponent, Default, Clone)]
#[scene(EntityMovementProps)]
pub struct EntityMovement {
    pub direction: Vec2,
    pub speed: f32,
}

#[derive(Default)]
pub struct EntityMovementProps {
    pub speed: f32,
}

impl EntityMovement {
    fn scene(props: EntityMovementProps) -> impl Scene {
        bsn! {
            EntityMovement {
                speed: { props.speed }
            }
        }
    }
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
