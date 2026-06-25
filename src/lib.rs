pub mod input;
pub mod scene;
pub mod visuals;

use bevy::{
    app::plugin_group,
    camera::{
        Camera,
        Camera2d,
        ClearColorConfig,
        OrthographicProjection,
        Projection,
        ScalingMode,
    },
    color::Color,
    ecs::system::Commands,
    scene::CommandsSceneExt,
};

use crate::{
    input::UpToChanceInputPlugin,
    scene::sample_scene,
    visuals::UpToChanceVisualsPlugin,
};

plugin_group! {
    pub struct UpToChancePlugins {
        :UpToChanceInputPlugin,
        :UpToChanceVisualsPlugin,
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb_u8(49, 77, 121)),
            ..Default::default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: 1280f32,
                min_height: 720f32,
            },
            ..OrthographicProjection::default_2d()
        }),
        Camera2d,
    ));
    commands.spawn_scene(sample_scene());
}
