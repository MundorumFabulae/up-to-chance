use bevy::{
    app::{
        App,
        Plugin,
    },
    ecs::{
        component::Component,
        entity::Entity,
        system::{
            Commands,
            Query,
        },
    },
    scene::{
        CommandsSceneExt,
        SceneList,
        bsn_list,
    },
    state::{
        app::AppExtStates,
        state::{
            OnEnter,
            OnExit,
            States,
        },
    },
};

use crate::entities::scenes::PlayerComponent;

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Screens {
    #[default]
    SampleScreen,
}

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssociatedScreen(pub Screens);

pub fn sample_screen() -> impl SceneList {
    bsn_list![
        (
            @PlayerComponent {
                @speed: 2.5
            }
            AssociatedScreen(Screens::SampleScreen)
        )
    ]
}

pub fn spawn_sample_screen(mut commands: Commands) {
    commands.spawn_scene_list(sample_screen());
}

pub fn despawn_sample_screen(
    mut commands: Commands,
    mut screen_entities: Query<(Entity, &AssociatedScreen)>,
) {
    for (entity, screen) in screen_entities.iter_mut() {
        if screen.0 != Screens::SampleScreen {
            continue;
        }

        commands.entity(entity).despawn();
    }
}

#[derive(Default)]
pub struct UpToChanceSceneManagementPlugin;

impl Plugin for UpToChanceSceneManagementPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.init_state::<Screens>();

        app.add_systems(OnEnter(Screens::SampleScreen), spawn_sample_screen);
        app.add_systems(OnExit(Screens::SampleScreen), despawn_sample_screen);
    }
}
