use bevy::prelude::*;
use crate::apiary::Apiary;
use crate::ui::GameNotification;
use crate::minigames::CentrifugeMinigame;

#[derive(Component)]
pub struct BeeSwarmParticle {
    pub center: Vec2,
    pub orbit_angle: f32,
    pub orbit_radius: f32,
    pub speed: f32,
}

use crate::pixel_assets::GamePixelAssets;

pub fn spawn_apiary_bee_particles(
    mut commands: Commands,
    apiaries: Query<(Entity, &Transform), Added<Apiary>>,
    pixel_assets: Res<GamePixelAssets>,
) {
    for (_, tf) in apiaries.iter() {
        let center = tf.translation.truncate();
        for i in 0..4 {
            let angle = (i as f32) * 1.57;
            commands.spawn((
                Sprite {
                    image: pixel_assets.bee.clone(),
                    custom_size: Some(Vec2::new(18.0, 15.0)),
                    ..default()
                },
                Transform::from_xyz(center.x, center.y, 4.0),
                BeeSwarmParticle {
                    center,
                    orbit_angle: angle,
                    orbit_radius: 26.0 + (i as f32) * 8.0,
                    speed: 3.0 + (i as f32) * 0.5,
                },
            ));
        }
    }
}

pub fn animate_bee_particles(
    time: Res<Time>,
    mut particles: Query<(&mut Transform, &mut BeeSwarmParticle)>,
) {
    let delta = time.delta_secs();
    for (mut transform, mut p) in particles.iter_mut() {
        p.orbit_angle += p.speed * delta;
        let x = p.center.x + p.orbit_angle.cos() * p.orbit_radius;
        let y = p.center.y + p.orbit_angle.sin() * p.orbit_radius * 0.7;
        transform.translation.x = x;
        transform.translation.y = y;
    }
}

pub fn tick_notification_timer(
    time: Res<Time>,
    mut notification: ResMut<GameNotification>,
) {
    if notification.timer > 0.0 {
        notification.timer -= time.delta_secs();
        if notification.timer <= 0.0 {
            notification.message.clear();
        }
    }
}

pub fn tick_centrifuge(
    time: Res<Time>,
    mut centrifuge: ResMut<CentrifugeMinigame>,
) {
    centrifuge.tick(time.delta_secs());
}
