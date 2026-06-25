use avian2d::{
    physics_transform::Position,
    schedule::PhysicsSystems,
};
use bevy::{
    app::{
        App,
        FixedPostUpdate,
        Plugin,
    },
    ecs::{
        component::Component,
        query::With,
        schedule::IntoScheduleConfigs,
        system::Query,
    },
    transform::components::Transform,
};

#[derive(Component, Default, Clone, Copy)]
pub struct IsometricSprite;

const TILE_WIDTH: f32 = 64.0;
const TILE_HEIGHT: f32 = 32.0;

fn sync_isometric_sprite(mut sprites: Query<(&Position, &mut Transform), With<IsometricSprite>>) {
    for (position, mut transform) in sprites.iter_mut() {
        let pos_x = position.x;
        let pos_y = position.y;

        let iso_x = (pos_x - pos_y) * (TILE_WIDTH / 2.0);
        let iso_y = (pos_x + pos_y) * (TILE_HEIGHT / 2.0);

        transform.translation.x = iso_x;
        transform.translation.y = iso_y;

        transform.translation.z = -pos_y / 64.0;
    }
}

#[derive(Default)]
pub struct UpToChanceVisualsPlugin;

impl Plugin for UpToChanceVisualsPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_systems(
            FixedPostUpdate,
            sync_isometric_sprite.after(PhysicsSystems::Writeback),
        );
    }
}
