use bevy::prelude::*;
use bevy_rapier2d::na::Vector2;
use bevy_rapier2d::prelude::*;

use crate::PlayerMovementSettings;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player);
        app.add_system(control_player);
    }
}

fn setup_player(mut commands: Commands) {
    let mut cmd = commands.spawn();
    cmd.insert_bundle(RigidBodyBundle {
        body_type: RigidBodyType::Dynamic.into(),
        position: point![0.0, 0.0].into(),
        mass_properties: RigidBodyMassProps {
            flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
            local_mprops: MassProperties {
                local_com: point![0.0, 0.0],
                inv_mass: 1.0 / 80.0,
                inv_principal_inertia_sqrt: 0.0,
            },
            ..Default::default()
        }
        .into(),
        ..Default::default()
    });
    cmd.insert_bundle(ColliderBundle {
        shape: ColliderShape::cuboid(0.25, 1.0).into(),
        ..Default::default()
    });
    cmd.insert(ColliderPositionSync::Discrete);
    cmd.insert(ColliderDebugRender::with_id(2));
    cmd.insert(PlayerControl {
        jump_potential: 0.0,
        last_stood_on: vector![0.0, 1.0],
        stood_on_potential: 0.0,
    });
}

#[derive(Component)]
pub struct PlayerControl {
    jump_potential: f32,
    last_stood_on: Vector2<f32>,
    stood_on_potential: f32,
}

fn control_player(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(
        Entity,
        &mut RigidBodyVelocityComponent,
        &RigidBodyMassPropsComponent,
        &mut PlayerControl,
    )>,
    player_movement_settings: Res<PlayerMovementSettings>,
    narrow_phase: Res<NarrowPhase>,
) {
    let is_jumping = input.pressed(KeyCode::Up);
    let mut target_speed: f32 = 0.0;
    if input.pressed(KeyCode::Left) {
        target_speed -= 1.0;
    }
    if input.pressed(KeyCode::Right) {
        target_speed += 1.0;
    }
    for (player_entity, mut velocity, mass_props, mut player_control) in query.iter_mut() {
        let standing_on = narrow_phase
            .contacts_with(player_entity.handle())
            .filter(|contact| contact.has_any_active_contact)
            .flat_map(|contact| {
                contact.manifolds.iter().filter_map(|contact_manifold| {
                    let player_handle = player_entity.handle();
                    if contact_manifold.data.rigid_body1 == Some(player_handle) {
                        Some(-contact_manifold.data.normal)
                    } else if contact_manifold.data.rigid_body2 == Some(player_handle) {
                        Some(contact_manifold.data.normal)
                    } else {
                        None
                    }
                })
            })
            .max_by_key(|normal| float_ord::FloatOrd(normal.dot(&vector![0.0, 1.0])));
        if let Some(standing_on) = standing_on {
            let refill_percentage = standing_on.dot(&vector![0.0, 1.0]);
            if player_control.jump_potential < refill_percentage {
                player_control.jump_potential = refill_percentage;
            }

            player_control.last_stood_on = standing_on;
            player_control.stood_on_potential = 1.0;
        } else {
            if !is_jumping {
                player_control.jump_potential = 0.0;
            }

            player_control.stood_on_potential = (player_control.stood_on_potential
                - time.delta().as_secs_f32() * player_movement_settings.stood_on_time_coefficient)
                .max(0.0);
        }
        if is_jumping {
            let to_deplete = player_control
                .jump_potential
                .min(time.delta().as_secs_f32() * player_movement_settings.jump_time_coefficient);
            if 0.0 < to_deplete {
                let before_depletion = player_control.jump_potential;
                let after_depletion = before_depletion - to_deplete;
                player_control.jump_potential = after_depletion;
                let integrate = |x: f32| {
                    let exponent = player_movement_settings.jump_potential_exponent + 1.0;
                    x.powf(exponent) / exponent
                };
                let area_under_graph =
                    (integrate(before_depletion) - integrate(after_depletion)) / integrate(1.0);
                velocity.apply_impulse(
                    mass_props,
                    vector![0.0, 1.0]
                        * player_movement_settings.jump_power_coefficient
                        * area_under_graph,
                );
            }
        }

        let mut up_now = vector![0.0, 1.0];
        up_now = (1.0 - player_control.stood_on_potential) * up_now
            + player_control.stood_on_potential * player_control.last_stood_on;

        let movement_vector = Isometry::rotation(-std::f32::consts::FRAC_PI_2) * up_now;

        let current_speed =
            velocity.linvel.dot(&movement_vector) / player_movement_settings.max_speed;

        if (0.0 < target_speed && target_speed <= current_speed)
            || (target_speed < 0.0 && current_speed <= target_speed)
        {
            continue;
        }
        let impulse = target_speed - current_speed;
        let impulse = if 1.0 < impulse.abs() {
            impulse.signum()
        } else {
            impulse.signum()
                * impulse
                    .abs()
                    .powf(player_movement_settings.impulse_exponent)
        };
        let mut impulse = movement_vector
            * time.delta().as_secs_f32()
            * player_movement_settings.impulse_coefficient
            * impulse;
        let uphill = impulse.normalize().dot(&vector![0.0, 1.0]);
        if 0.01 <= uphill {
            let efficiency = if target_speed.signum() as i32 == current_speed.signum() as i32 {
                player_movement_settings.uphill_move_exponent
            } else {
                player_movement_settings.downhill_stop_exponent
            };
            impulse *= 1.0 - uphill.powf(efficiency);
        }
        velocity.apply_impulse(mass_props, impulse);
    }
}
