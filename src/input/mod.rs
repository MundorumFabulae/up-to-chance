use bevy::{
    app::{
        App,
        Plugin,
    },
    ecs::{
        component::Component,
        observer::On,
        resource::Resource,
        system::ResMut,
    },
    math::Vec2,
};
use bevy_enhanced_input::{
    action::events::{
        Complete,
        Fire,
    },
    context::InputContextAppExt,
    prelude::InputAction,
};

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct MovementAction;

#[derive(Component, Default, Debug, Clone, Copy)]
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

impl Plugin for UpToChanceInputPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_input_context::<PlayerActions>();
        app.insert_resource(PlayerMovement::default());

        app.add_observer(handle_movement);
        app.add_observer(reset_movement);
    }
}
