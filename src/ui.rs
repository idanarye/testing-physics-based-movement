use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_rapier2d::physics::RapierConfiguration;
use bevy_rapier2d::prelude::{ColliderMaterialComponent, RigidBodyMassPropsComponent};

use crate::player::PlayerControl;
use crate::PlayerMovementSettings;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ui_system);
    }
}

fn ui_system(
    mut egui_context: ResMut<EguiContext>,
    mut player_movement_settings: ResMut<PlayerMovementSettings>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
    mut player_query: Query<
        (
            &mut RigidBodyMassPropsComponent,
            &mut ColliderMaterialComponent,
        ),
        With<PlayerControl>,
    >,
) {
    egui::Window::new("Physical Properties Tweaking").show(egui_context.ctx_mut(), |ui| {
        let mut gravity = -rapier_configuration.gravity.y;

        let (mut player_mass_properties, mut player_collider_material) =
            player_query.get_single_mut().unwrap();
        let mut mass = player_mass_properties.mass();

        let player_movement_settings = &mut *player_movement_settings;
        for (caption, property, range) in [
            ("Gravity", &mut gravity, 0.1..=20.0),
            ("Mass", &mut mass, 0.1..=200.0),
            (
                "Friction",
                &mut player_collider_material.friction,
                0.0..=10.0,
            ),
            (
                "Max Speed",
                &mut player_movement_settings.max_speed,
                1.0..=100.0,
            ),
            (
                "Impulse Exponent",
                &mut player_movement_settings.impulse_exponent,
                1.0..=10.0,
            ),
            (
                "Impulse Coefficient",
                &mut player_movement_settings.impulse_coefficient,
                1.0..=100_000.0,
            ),
            (
                "Jump Power Coefficient",
                &mut player_movement_settings.jump_power_coefficient,
                1.0..=2000.0,
            ),
            (
                "Jump Time Coefficient",
                &mut player_movement_settings.jump_time_coefficient,
                2.0..=20.0,
            ),
            (
                "Stood On Time Coefficient",
                &mut player_movement_settings.stood_on_time_coefficient,
                1.0..=100.0,
            ),
            (
                "Uphill Move Exponent",
                &mut player_movement_settings.uphill_move_exponent,
                0.01..=200.0,
            ),
            (
                "Downhill Stop Exponent",
                &mut player_movement_settings.downhill_stop_exponent,
                0.01..=200.0,
            ),
        ] {
            ui.add(egui::Slider::new(property, range).text(caption));
        }

        rapier_configuration.gravity.y = -gravity;
        player_mass_properties.local_mprops.set_mass(mass, true);
    });
}
