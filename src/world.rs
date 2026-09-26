use bevy::prelude::*;
use crate::bees::BiomeType;
use crate::items::FlowerColor;
use crate::pixel_assets::GamePixelAssets;

#[derive(Component)]
pub struct IslandEntity;

#[derive(Component)]
pub struct IslandTile;

#[derive(Component)]
pub struct HarvestableFlora {
    pub flower_color: FlowerColor,
    pub growth_stage: f32, // 0.0 to 1.0
}

#[derive(Component)]
pub struct HarvestableTree {
    pub wood_remaining: u32,
    pub max_wood: u32,
}

#[derive(Component)]
pub struct HarvestableRock {
    pub stone_remaining: u32,
}

#[derive(Component)]
pub struct WildBeeHive {
    pub species_id: String,
    pub harvested: bool,
}

#[derive(Component)]
pub struct FerryBoat {
    pub destination_name: &'static str,
    pub target_biome: BiomeType,
}

#[derive(Component)]
pub struct RepopulationShrine {
    pub biome: BiomeType,
    pub target_species: [&'static str; 3],
    pub current_repopulated: [bool; 3],
    pub restored: bool,
}

#[derive(Component)]
pub struct WifeEungNpc;

#[derive(Resource)]
pub struct CurrentIsland {
    pub biome: BiomeType,
    pub island_name: String,
}

impl Default for CurrentIsland {
    fn default() -> Self {
        Self {
            biome: BiomeType::PortHoneyBeaFather,
            island_name: "Port Gijang (Home)".to_string(),
        }
    }
}

#[derive(Resource, Default)]
pub struct IslandDiscovery {
    pub unlocked_meadows: bool,
    pub unlocked_forest: bool,
    pub unlocked_coastal: bool,
    pub unlocked_marsh: bool,
    pub unlocked_peaks: bool,
}

pub fn spawn_island(
    commands: &mut Commands,
    biome: BiomeType,
    pixel_assets: &GamePixelAssets,
) {
    // Generate an atmospheric island layout
    let (center_color, water_color, accent_color) = match biome {
        BiomeType::PortHoneyBeaFather => (
            Color::srgb(0.42, 0.68, 0.35), // Cozy lawn
            Color::srgb(0.25, 0.55, 0.75), // Harbor blue
            Color::srgb(0.72, 0.58, 0.42), // Wooden docks
        ),
        BiomeType::EmeraldMeadows => (
            Color::srgb(0.35, 0.75, 0.35), // Vibrant lush emerald green
            Color::srgb(0.3, 0.6, 0.8),
            Color::srgb(0.85, 0.82, 0.4),
        ),
        BiomeType::WhisperwoodForest => (
            Color::srgb(0.22, 0.48, 0.28), // Deep evergreen
            Color::srgb(0.2, 0.45, 0.6),
            Color::srgb(0.45, 0.32, 0.2),
        ),
        BiomeType::SunkenShore => (
            Color::srgb(0.88, 0.82, 0.58), // Warm sandy beach
            Color::srgb(0.2, 0.7, 0.85),   // Aquamarine tide
            Color::srgb(0.75, 0.65, 0.5),
        ),
        BiomeType::MangroveMarsh => (
            Color::srgb(0.28, 0.42, 0.3),  // Murky wetland moss
            Color::srgb(0.18, 0.38, 0.42), // Brackish water
            Color::srgb(0.35, 0.28, 0.22),
        ),
        BiomeType::ShimmeringPeaks => (
            Color::srgb(0.65, 0.72, 0.78), // Cold alpine stone
            Color::srgb(0.45, 0.65, 0.85), // Glacial melt
            Color::srgb(0.85, 0.9, 0.98),  // Snow frost
        ),
        _ => (
            Color::srgb(0.4, 0.6, 0.4),
            Color::srgb(0.3, 0.5, 0.7),
            Color::srgb(0.6, 0.5, 0.4),
        ),
    };

    // Island ground base
    commands.spawn((
        Sprite {
            color: center_color,
            custom_size: Some(Vec2::new(960.0, 720.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -10.0),
        IslandTile,
        IslandEntity,
    ));

    // Water border ring
    for &(x, y, w, h) in &[
        (0.0, 420.0, 1200.0, 160.0),
        (0.0, -420.0, 1200.0, 160.0),
        (-560.0, 0.0, 180.0, 960.0),
        (560.0, 0.0, 180.0, 960.0),
    ] {
        commands.spawn((
            Sprite {
                color: water_color,
                custom_size: Some(Vec2::new(w, h)),
                ..default()
            },
            Transform::from_xyz(x, y, -9.0),
            IslandTile,
            IslandEntity,
        ));
    }

    // Cobblestone / Dock pathway
    commands.spawn((
        Sprite {
            color: accent_color,
            custom_size: Some(Vec2::new(120.0, 500.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -100.0, -8.0),
        IslandTile,
        IslandEntity,
    ));

    // Spawn Ferry Boat at the southern dock
    commands.spawn((
        Sprite {
            image: pixel_assets.ferry_boat.clone(),
            custom_size: Some(Vec2::new(72.0, 48.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -320.0, 1.0),
        FerryBoat {
            destination_name: "Ferry to Gijang Archipelago",
            target_biome: BiomeType::EmeraldMeadows,
        },
        IslandEntity,
    ));

    // Spawn Repopulation Shrine
    let (s1, s2, s3) = match biome {
        BiomeType::PortHoneyBeaFather => ("common", "meadow", "blossom"),
        BiomeType::EmeraldMeadows => ("meadow", "verdant", "golden"),
        BiomeType::WhisperwoodForest => ("forest", "bark", "ancient"),
        BiomeType::SunkenShore => ("coastal", "dewdrop", "coral"),
        BiomeType::MangroveMarsh => ("swamp", "reed", "mist"),
        BiomeType::ShimmeringPeaks => ("mountain", "frosty", "crystal"),
        _ => ("common", "meadow", "blossom"),
    };

    commands.spawn((
        Sprite {
            image: pixel_assets.shrine.clone(),
            custom_size: Some(Vec2::new(56.0, 56.0)),
            ..default()
        },
        Transform::from_xyz(280.0, 220.0, 2.0),
        RepopulationShrine {
            biome,
            target_species: [s1, s2, s3],
            current_repopulated: [false; 3],
            restored: false,
        },
        IslandEntity,
    ));

    // Spawn 와이프응 (Wife-Eung) in Port Gijang
    if biome == BiomeType::PortHoneyBeaFather {
        commands.spawn((
            Sprite {
                image: pixel_assets.wife_eung.clone(),
                custom_size: Some(Vec2::new(32.0, 44.0)),
                ..default()
            },
            Transform::from_xyz(80.0, 30.0, 2.5),
            WifeEungNpc,
            IslandEntity,
        ));
    }

    // Spawn flora patches
    let flower_types = [
        FlowerColor::Red,
        FlowerColor::Yellow,
        FlowerColor::Blue,
        FlowerColor::Purple,
        FlowerColor::White,
        FlowerColor::Golden,
    ];

    let flower_positions = [
        (-220.0, 160.0), (-180.0, 180.0), (-250.0, 120.0),
        (-320.0, -80.0), (-280.0, -120.0), (-340.0, -150.0),
        (220.0, -120.0), (260.0, -80.0), (300.0, -140.0),
        (180.0, 180.0), (220.0, 140.0), (160.0, 230.0),
        (-140.0, -220.0), (-100.0, -250.0), (140.0, -230.0),
    ];

    for (i, &(fx, fy)) in flower_positions.iter().enumerate() {
        let fcol = flower_types[i % flower_types.len()];
        let f_img = match fcol {
            FlowerColor::Red => pixel_assets.flower_red.clone(),
            FlowerColor::Yellow => pixel_assets.flower_yellow.clone(),
            FlowerColor::Blue => pixel_assets.flower_blue.clone(),
            FlowerColor::Purple => pixel_assets.flower_purple.clone(),
            FlowerColor::White => pixel_assets.flower_white.clone(),
            FlowerColor::Golden => pixel_assets.flower_gold.clone(),
        };

        commands.spawn((
            Sprite {
                image: f_img,
                custom_size: Some(Vec2::new(26.0, 26.0)),
                ..default()
            },
            Transform::from_xyz(fx, fy, 1.0),
            HarvestableFlora {
                flower_color: fcol,
                growth_stage: 1.0,
            },
            IslandEntity,
        ));
    }

    // Spawn Trees
    let tree_positions = [
        (-380.0, 240.0), (-320.0, 280.0), (-420.0, 160.0),
        (-380.0, -240.0), (-420.0, -180.0),
        (380.0, 260.0), (420.0, 180.0), (360.0, -220.0),
    ];

    for &(tx, ty) in &tree_positions {
        commands.spawn((
            Sprite {
                image: pixel_assets.tree.clone(),
                custom_size: Some(Vec2::new(64.0, 80.0)),
                ..default()
            },
            Transform::from_xyz(tx, ty, 3.0),
            HarvestableTree {
                wood_remaining: 5,
                max_wood: 5,
            },
            IslandEntity,
        ));
    }

    // Spawn Rocks
    let rock_positions = [
        (-260.0, 280.0), (-160.0, 310.0), (240.0, -260.0), (380.0, -120.0),
    ];

    for &(rx, ry) in &rock_positions {
        commands.spawn((
            Sprite {
                image: pixel_assets.rock.clone(),
                custom_size: Some(Vec2::new(36.0, 30.0)),
                ..default()
            },
            Transform::from_xyz(rx, ry, 2.0),
            HarvestableRock {
                stone_remaining: 4,
            },
            IslandEntity,
        ));
    }

    // Spawn wild hive depending on biome
    let wild_bee_sp = match biome {
        BiomeType::PortHoneyBeaFather => "common",
        BiomeType::EmeraldMeadows => "meadow",
        BiomeType::WhisperwoodForest => "forest",
        BiomeType::SunkenShore => "coastal",
        BiomeType::MangroveMarsh => "swamp",
        BiomeType::ShimmeringPeaks => "mountain",
        _ => "common",
    };

    commands.spawn((
        Sprite {
            image: pixel_assets.wild_hive.clone(),
            custom_size: Some(Vec2::new(36.0, 40.0)),
            ..default()
        },
        Transform::from_xyz(-280.0, 40.0, 2.5),
        WildBeeHive {
            species_id: wild_bee_sp.to_string(),
            harvested: false,
        },
        IslandEntity,
    ));
}
