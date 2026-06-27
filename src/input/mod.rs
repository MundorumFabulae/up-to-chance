use bevy::{
    app::{
        App,
        FixedUpdate,
        Plugin,
    },
    ecs::{
        bundle::Bundle,
        component::Component,
        lifecycle::HookContext,
        observer::On,
        query::With,
        resource::Resource,
        spawn::SpawnRelated,
        system::{
            Res,
            ResMut,
            Single,
        },
        world::DeferredWorld,
    },
    math::Vec2,
};
use bevy_enhanced_input::{
    action::{
        Action,
        events::{
            Complete,
            Fire,
        },
    },
    actions,
    binding::relationship::Bindings,
    context::InputContextAppExt,
    prelude::InputAction,
    preset::{
        axial::Axial,
        cardinal::Cardinal,
    },
};

use crate::entities::EntityMovement;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct MovementAction;

#[derive(Component, Default, Debug, Clone, Copy)]
#[component(on_add = on_player_actions_added)]
pub struct PlayerActions;

#[derive(Default)]
pub struct UpToChanceInputPlugin;

#[derive(Resource, Default, Debug, Clone, Copy)]
pub struct PlayerMovement(pub Vec2);

fn handle_movement(
    action: On<Fire<MovementAction>>,
    mut movement: ResMut<PlayerMovement>,
) {
    movement.0 = action.value;
}

fn reset_movement(
    _action: On<Complete<MovementAction>>,
    mut movement: ResMut<PlayerMovement>,
) {
    movement.0 = Vec2::ZERO;
}

fn player_action_bundle() -> impl Bundle {
    actions!(
        PlayerActions[(
            Action::<MovementAction>::new(),
            Bindings::spawn((Cardinal::wasd_keys(), Axial::left_stick()))
        )]
    )
}

fn on_player_actions_added(
    mut world: DeferredWorld,
    context: HookContext,
) {
    let entity = context.entity;

    world
        .commands()
        .entity(entity)
        .insert(player_action_bundle());
}

fn handle_player_movement(
    mut player: Single<&mut EntityMovement, With<PlayerActions>>,
    movement: Res<PlayerMovement>,
) {
    let direction = movement.0;
    player.direction = if direction.length() < 1.0 {
        direction
    } else {
        direction.normalize_or_zero()
    }
}

impl Plugin for UpToChanceInputPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_input_context::<PlayerActions>();
        app.insert_resource(PlayerMovement::default());

        app.add_observer(handle_movement);
        app.add_observer(reset_movement);

        app.add_systems(FixedUpdate, handle_player_movement);
    }
}
