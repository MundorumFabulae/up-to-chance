use std::process::ExitCode;

#[cfg(feature = "dev-full")]
use avian2d::debug_render::PhysicsDebugPlugin;
#[cfg(feature = "dev")]
use avian2d::diagnostics::PhysicsDiagnosticsPlugin;
use avian2d::{
    PhysicsPlugins,
    dynamics::integrator::Gravity,
    physics_transform::PhysicsTransformConfig,
};
use bevy::{
    DefaultPlugins,
    app::{
        App,
        AppExit,
        PluginGroup,
        Startup,
        Update,
    },
    diagnostic::FrameCount,
    ecs::{
        query::With,
        schedule::IntoScheduleConfigs,
        system::{
            Res,
            Single,
        },
    },
    log::{
        DEFAULT_FILTER,
        LogPlugin,
    },
    window::{
        PrimaryWindow,
        Window,
        WindowPlugin,
        WindowResizeConstraints,
    },
};
use up_to_chance::UpToChancePlugins;

fn main() -> ExitCode {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(LogPlugin {
                filter: (if cfg!(feature = "dev") {
                    DEFAULT_FILTER
                } else {
                    concat!("warn,", "wgpu=error,", "up_to_chance=info",)
                })
                .to_owned(),
                ..Default::default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Up To Chance".to_owned(),
                    resizable: true,
                    resize_constraints: WindowResizeConstraints {
                        min_width: 640.0,
                        min_height: 480.0,
                        ..Default::default()
                    },
                    visible: false,
                    ..Default::default()
                }),
                ..Default::default()
            }),
    );

    app.add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity::ZERO)
        .insert_resource(PhysicsTransformConfig {
            transform_to_position: false,
            position_to_transform: false,
            ..Default::default()
        });

    #[cfg(feature = "dev")]
    app.add_plugins(PhysicsDiagnosticsPlugin);

    #[cfg(feature = "dev-full")]
    app.add_plugins(PhysicsDebugPlugin);

    app.add_plugins(UpToChancePlugins);

    app.add_systems(Startup, up_to_chance::setup);
    app.add_systems(
        Update,
        show_window.run_if(|frames: Res<FrameCount>| frames.0 == 5),
    );

    match app.run() {
        AppExit::Success => ExitCode::from(0),
        AppExit::Error(exit_code) => ExitCode::from(exit_code.get()),
    }
}

fn show_window(mut window: Single<&mut Window, With<PrimaryWindow>>) {
    window.visible = true;
}
