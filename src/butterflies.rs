use bevy::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButterflyEffect {
    pub name: &'static str,
    pub description: &'static str,
    pub speed_bonus: f32,
    pub mutation_bonus: f32,
    pub sell_bonus: f32,
}

#[derive(Component)]
pub struct Butterfly {
    pub species_name: &'static str,
    pub base_position: Vec2,
    pub flutter_time: f32,
    pub color: Color,
    pub speed: f32,
}

#[derive(Resource, Default)]
pub struct ActiveButterflyBuffs {
    pub speed_multiplier: f32,
    pub mutation_bonus: f32,
    pub sell_multiplier: f32,
    pub collected_species: Vec<String>,
}

impl ActiveButterflyBuffs {
    pub fn recalculate(&mut self) {
        self.speed_multiplier = 1.0;
        self.mutation_bonus = 0.0;
        self.sell_multiplier = 1.0;

        for sp in &self.collected_species {
            match sp.as_str() {
                "Brimstone" => self.speed_multiplier += 0.25,
                "Peacock" => self.mutation_bonus += 0.20,
                "Rainbow Swallowtail" => self.sell_multiplier += 0.30,
                _ => {}
            }
        }
    }
}

use crate::pixel_assets::GamePixelAssets;

pub fn spawn_wild_butterflies(commands: &mut Commands, pixel_assets: &GamePixelAssets) {
    let species_list = [
        ("Brimstone", pixel_assets.butterfly_brimstone.clone(), 40.0),
        ("Peacock", pixel_assets.butterfly_peacock.clone(), 35.0),
        ("Blue Morpho", pixel_assets.butterfly_morpho.clone(), 45.0),
        ("Monarch", pixel_assets.butterfly_monarch.clone(), 38.0),
        ("Moon Moth", pixel_assets.butterfly_moon.clone(), 32.0),
        ("Rainbow Swallowtail", pixel_assets.butterfly_rainbow.clone(), 42.0),
    ];

    let mut rng = rand::thread_rng();

    for (name, texture, speed) in species_list {
        let x = rng.gen_range(-300.0..300.0);
        let y = rng.gen_range(-200.0..200.0);

        commands.spawn((
            Sprite {
                image: texture,
                custom_size: Some(Vec2::new(24.0, 21.0)),
                ..default()
            },
            Transform::from_xyz(x, y, 4.0),
            Butterfly {
                species_name: name,
                base_position: Vec2::new(x, y),
                flutter_time: rng.gen_range(0.0..10.0),
                color: Color::WHITE,
                speed,
            },
        ));
    }
}

pub fn animate_butterflies(
    time: Res<Time>,
    mut butterflies: Query<(&mut Transform, &mut Butterfly)>,
) {
    let delta = time.delta_secs();

    for (mut transform, mut butterfly) in butterflies.iter_mut() {
        butterfly.flutter_time += delta * 2.0;

        // Circular wandering + fast vertical fluttering wing wave
        let wander_x = (butterfly.flutter_time * 0.5).cos() * 60.0;
        let wander_y = (butterfly.flutter_time * 0.7).sin() * 40.0;
        let flap_offset = (butterfly.flutter_time * 12.0).sin() * 3.0;

        transform.translation.x = butterfly.base_position.x + wander_x;
        transform.translation.y = butterfly.base_position.y + wander_y + flap_offset;

        // Scale x slightly to simulate flapping wings
        let wing_scale = (butterfly.flutter_time * 15.0).cos().abs() * 0.6 + 0.4;
        transform.scale.x = wing_scale;
    }
}
