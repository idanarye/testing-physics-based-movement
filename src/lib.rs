mod arena;
mod camera;
mod player;
mod ui;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_rapier2d::physics::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::render::RapierRenderPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_plugin(RapierRenderPlugin);
        app.add_plugin(EguiPlugin);
        app.add_plugin(camera::CameraPlugin);
        app.add_plugin(arena::ArenaPlugin);
        app.add_plugin(player::PlayerPlugin);
        app.add_plugin(ui::UiPlugin);

        app.insert_resource(PlayerMovementSettings {
            max_speed: 20.0,
            impulse_exponent: 4.0,
            impulse_coefficient: 40_000.0,
            jump_power_coefficient: 1000.0,
            jump_brake_coefficient: 0.02,
            start_fall_before_peak: 10.0,
            start_of_fall_range: 10.0,
            start_of_fall_gravity_boost: 30.0,
            fall_boost_coefficient: 1.06,
            stood_on_time_coefficient: 10.0,
            uphill_move_exponent: 0.5,
            downhill_brake_exponent: 1.0,
        });
    }
}

pub struct PlayerMovementSettings {
    pub max_speed: f32,
    pub impulse_exponent: f32,
    pub impulse_coefficient: f32,
    pub jump_power_coefficient: f32,
    pub jump_brake_coefficient: f32,
    pub start_fall_before_peak: f32,
    pub start_of_fall_range: f32,
    pub start_of_fall_gravity_boost: f32,
    pub fall_boost_coefficient: f32,
    pub stood_on_time_coefficient: f32,
    pub uphill_move_exponent: f32,
    pub downhill_brake_exponent: f32,
}
