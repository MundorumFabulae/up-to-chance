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

#[derive(SceneComponent, Default, Clone)]
#[scene(PlayerProps)]
pub struct PlayerComponent;

#[derive(Default)]
pub struct PlayerProps {
    pub speed: f32,
}

impl PlayerComponent {
    fn scene(props: PlayerProps) -> impl Scene {
        let player_collider = Collider::circle(32.0);

        bsn! {
            #Player
            @EntityMovement {
                @speed: { props.speed }
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
