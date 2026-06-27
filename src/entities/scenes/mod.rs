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
        SceneComponent,
        bsn,
        template_value,
    },
    sprite::Sprite,
};

use crate::{
    entities::EntityMovement,
    input::PlayerActions,
    visuals::IsometricSprite,
};

#[derive(SceneComponent, Clone)]
pub struct PlayerComponent {
    pub speed: f32,
}

impl Default for PlayerComponent {
    fn default() -> Self {
        Self { speed: 2.5 }
    }
}

impl PlayerComponent {
    fn scene() -> impl Scene {
        let player_collider = Collider::circle(32.0);

        bsn! {
            #Player
            EntityMovement {
                speed: 2.5
            }
            PlayerActions
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
}
