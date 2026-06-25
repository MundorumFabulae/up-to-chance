use avian2d::{
    collision::collider::Collider,
    dynamics::rigid_body::{
        Friction,
        Restitution,
        RigidBody,
    },
};
use bevy::{
    scene::{
        Scene,
        bsn,
        template_value,
    },
    sprite::Sprite,
};

use crate::visuals::IsometricSprite;

pub fn sample_scene() -> impl Scene {
    let player_collider = Collider::circle(32.0);

    bsn! {
        IsometricSprite
        Sprite {
            image: "sprites/spr_square.ktx2",
        }
        template_value(RigidBody::Dynamic)
        template_value(player_collider)
        template_value(Friction::ZERO)
        template_value(Restitution::ZERO)
    }
}
