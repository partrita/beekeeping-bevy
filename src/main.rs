#![allow(dead_code)]

mod items;
mod bees;
mod world;
mod apiary;
mod butterflies;
mod crafting;
mod minigames;
mod conservation;
mod market;
mod player;
mod ui;
mod modals;
mod render_effects;
mod pixel_assets;

use bevy::prelude::*;
use bees::{BeeCatalog, BiomeType};
use crafting::CraftingRegistry;
use player::{Player, PlayerInventory, PlayerAnimation, player_movement, update_player_sprite};
use market::MarketEconomy;
use conservation::ConservationTracker;
use butterflies::{ActiveButterflyBuffs, spawn_wild_butterflies, animate_butterflies};
use minigames::{CentrifugeMinigame, MicroscopeMinigame};
use world::{CurrentIsland, spawn_island};
use ui::{ActiveUI, GameNotification, DiscoveredSpecies, SelectedApiary, setup_hud, update_hud_elements};
use modals::*;
use interactions::*;
use render_effects::*;
use apiary::{Apiary, SolitaryBeeHotel, update_apiaries, update_solitary_hotels};
use pixel_assets::GamePixelAssets;

#[derive(Resource, Clone)]
pub struct GameFonts {
    pub korean: Handle<Font>,
}

impl FromWorld for GameFonts {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let korean = asset_server.load("fonts/NotoSansKR-Regular.ttf");
        Self { korean }
    }
}

pub fn apply_korean_font(
    fonts: Option<Res<GameFonts>>,
    mut text_query: Query<&mut TextFont, Added<TextFont>>,
) {
    let Some(fonts) = fonts else { return; };
    for mut text_font in text_query.iter_mut() {
        if text_font.font == Handle::default() {
            text_font.font = fonts.korean.clone();
        }
    }
}

mod interactions;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "HoneyBeaFather - Laid-back Beekeeping & Conservation Sim (Gijang Islands)".into(),
                resolution: (1280.0_f32, 720.0_f32).into(),
                ..default()
            }),
            ..default()
        }))
        // Resources
        .insert_resource(BeeCatalog::new())
        .insert_resource(CraftingRegistry::new())
        .init_resource::<PlayerInventory>()
        .init_resource::<MarketEconomy>()
        .init_resource::<ConservationTracker>()
        .init_resource::<ActiveButterflyBuffs>()
        .init_resource::<CentrifugeMinigame>()
        .init_resource::<MicroscopeMinigame>()
        .init_resource::<CurrentIsland>()
        .init_resource::<GameNotification>()
        .init_resource::<DiscoveredSpecies>()
        .init_resource::<SelectedApiary>()
        .init_resource::<GameFonts>()
        .init_resource::<GamePixelAssets>()
        // State
        .init_state::<ActiveUI>()
        // Systems
        .add_systems(Startup, (setup_game, setup_hud).chain())
        .add_systems(
            Update,
            (
                player_movement,
                update_player_sprite,
                apply_korean_font,
                player_hotbar_selection,
                handle_harvesting,
                handle_wild_interactions,
                handle_hive_tending,
                handle_flower_planting_and_placement,
                handle_apiary_inspection_inputs,
                open_or_close_modals,
                handle_modal_inputs,
            ),
        )
        .add_systems(
            Update,
            (
                update_apiaries,
                update_solitary_hotels,
                animate_butterflies,
                animate_bee_particles,
                spawn_apiary_bee_particles,
                tick_notification_timer,
                tick_centrifuge,
                update_hud_elements,
                update_inspection_modal.run_if(in_state(ActiveUI::ApiaryInspection)),
            ),
        )
        // Modals
        .add_systems(OnEnter(ActiveUI::Beedex), spawn_beedex_modal)
        .add_systems(OnExit(ActiveUI::Beedex), cleanup_modal)
        .add_systems(OnEnter(ActiveUI::Crafting), spawn_crafting_modal)
        .add_systems(OnExit(ActiveUI::Crafting), cleanup_modal)
        .add_systems(OnEnter(ActiveUI::Centrifuge), spawn_centrifuge_modal)
        .add_systems(OnExit(ActiveUI::Centrifuge), cleanup_modal)
        .add_systems(OnEnter(ActiveUI::Microscope), spawn_microscope_modal)
        .add_systems(OnExit(ActiveUI::Microscope), cleanup_modal)
        .add_systems(OnEnter(ActiveUI::Market), spawn_market_modal)
        .add_systems(OnExit(ActiveUI::Market), cleanup_modal)
        .add_systems(OnEnter(ActiveUI::Journal), spawn_journal_modal)
        .add_systems(OnExit(ActiveUI::Journal), cleanup_modal)
        .add_systems(OnEnter(ActiveUI::Ferry), spawn_ferry_modal)
        .add_systems(OnExit(ActiveUI::Ferry), cleanup_modal)
        .add_systems(OnEnter(ActiveUI::ApiaryInspection), spawn_apiary_inspection_modal)
        .add_systems(OnExit(ActiveUI::ApiaryInspection), cleanup_modal)
        .run();
}

fn setup_game(
    mut commands: Commands,
    mut discovered: ResMut<DiscoveredSpecies>,
    pixel_assets: Res<GamePixelAssets>,
) {
    // 2D Camera
    commands.spawn(Camera2d);

    // Initial starter discoveries
    discovered.discover("common");
    discovered.discover("meadow");

    // Spawn Port Gijang Island with pixel art
    spawn_island(&mut commands, BiomeType::PortHoneyBeaFather, &pixel_assets);

    // Spawn Chibi Pokemon Trainer Beekeeper
    commands.spawn((
        Sprite {
            image: pixel_assets.player_sprites.down_idle.clone(),
            custom_size: Some(Vec2::new(32.0, 44.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 10.0),
        Player {
            speed: 180.0,
            selected_slot: 0,
        },
        PlayerAnimation::default(),
    ));

    // Spawn Starter Apiary in Port yard
    commands.spawn((
        Sprite {
            image: pixel_assets.apiary.clone(),
            custom_size: Some(Vec2::new(48.0, 52.0)),
            ..default()
        },
        Transform::from_xyz(-100.0, 80.0, 2.0),
        Apiary {
            id: 1,
            ..default()
        },
    ));

    // Spawn Solitary Bee Hotel
    commands.spawn((
        Sprite {
            image: pixel_assets.solitary_hotel.clone(),
            custom_size: Some(Vec2::new(40.0, 48.0)),
            ..default()
        },
        Transform::from_xyz(100.0, 80.0, 2.0),
        SolitaryBeeHotel {
            bee_species: Some("solitary_carpenter".to_string()),
            occupancy: 2,
            propolis_stored: 1,
            ..default()
        },
    ));

    // Spawn cute butterflies
    spawn_wild_butterflies(&mut commands, &pixel_assets);
}
