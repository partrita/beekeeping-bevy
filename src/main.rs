#![allow(dead_code)]

mod colony;
mod comb;
mod hiveview;
mod meadow;
mod pixel_assets;
mod save;
mod ui;

use bevy::prelude::*;
use bevy::window::WindowCloseRequested;
use colony::{Colony, HoneyJars};
use hiveview::{
    DecoSticker, Entrance, HiveBee, InteriorRoot, PendingDrops, Role, SavedCam, cute_anim,
    fade_flashes, spawn_interior, sync_comb, walk_hive,
};
use meadow::{DayNight, Weather, setup_hive, tick_colony, tick_daynight};
use pixel_assets::GamePixelAssets;
use ui::{AppState, Notice, harvest_honey, setup_hud, spawn_panel, update_hud, update_panel};

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

/// Camera follow target (entity of bee being watched).
#[derive(Resource, Default)]
pub struct FollowTarget(pub Option<Entity>);

/// HUD hidden flag (Tab).
#[derive(Resource, Default)]
pub struct HudHidden(pub bool);

/// Mute flag (M). Audio files land in assets/audio later; logic first.
#[derive(Resource)]
pub struct Muted(pub bool);

impl Default for Muted {
    fn default() -> Self {
        Self(false)
    }
}

/// Offline catch-up summary shown after fast-forward.
#[derive(Resource, Default)]
pub struct OfflineSummary {
    pub text: String,
    pub timer: f32,
}

/// Decoration tier from jars (Q10=A, Q23=B): wood 3 + sticker 3.
#[derive(Resource, Default)]
pub struct DecoLevel(pub u32);

fn main() {
    let mut app = App::new();
    // CARGO_MANIFEST_DIR 절대경로로 에셋을 고정: `cargo run`이든
    // `target/debug/바이너리` 직접 실행이든 fonts가 항상 보임.
    let asset_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/assets").to_string();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "꿀벌 내검 일기 - Hive Heart".into(),
                    resolution: (1280.0_f32, 720.0_f32).into(),
                    ..default()
                }),
                ..default()
            })
            .set(bevy::asset::AssetPlugin {
                file_path: asset_dir,
                ..default()
            }),
    );
    configure(&mut app);
    app.run();
}

/// One registration path for the game and the headless tests.
pub fn configure(app: &mut App) -> &mut App {
    app.insert_resource(ClearColor(Color::srgb(0.05, 0.04, 0.03)))
        .init_resource::<HoneyJars>()
        .init_resource::<Notice>()
        .init_resource::<DayNight>()
        .init_resource::<Weather>()
        .init_resource::<PendingDrops>()
        .init_resource::<SavedCam>()
        .init_resource::<FollowTarget>()
        .init_resource::<HudHidden>()
        .init_resource::<Muted>()
        .init_resource::<OfflineSummary>()
        .init_resource::<DecoLevel>()
        .init_resource::<GameFonts>()
        .init_resource::<GamePixelAssets>()
        .init_state::<AppState>()
        .add_systems(
            Startup,
            (
                setup_hive,
                apply_offline_catchup,
                setup_hud,
                spawn_interior,
                spawn_panel,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                tick_daynight,
                tick_colony,
                walk_hive,
                sync_comb,
                cute_anim,
                fade_flashes,
                update_hud,
                update_panel,
                harvest_honey,
                camera_control,
                click_follow,
                find_queen,
                toggle_hud,
                toggle_mute,
                autosave,
                update_deco,
                dismiss_tutorial,
                exit_system,
                close_requested,
            ),
        )
}

/// Offline catch-up: load save.json, fast-forward up to 8h in 1s steps.
fn apply_offline_catchup(
    mut colony_q: Query<&mut Colony>,
    mut jars: ResMut<HoneyJars>,
    mut day: ResMut<DayNight>,
    weather: Res<Weather>,
    mut summary: ResMut<OfflineSummary>,
    mut notice: ResMut<Notice>,
) {
    let Ok(mut col) = colony_q.get_single_mut() else {
        return;
    };
    if let Some(saved) = save::load() {
        col.eggs = saved.eggs;
        col.larvae = saved.larvae;
        col.pupae = saved.pupae;
        col.workers = saved.workers;
        col.drones = saved.drones;
        col.honey = saved.honey;
        col.pollen = saved.pollen;
        col.built = saved.built;
        col.total_emerged = saved.total_emerged;
        jars.0 = saved.jars;
        day.day_count = saved.day_count;
        // Offline seconds capped at 8h (Q15=A, Q26=C).
        let away = saved.away_secs().min(8.0 * 3600.0);
        if away > 60.0 {
            let before_workers = col.workers;
            let before_honey = col.honey;
            let mut drops = colony::Deliveries::default();
            // Night assumed off for catch-up richness average; weather held.
            let mut t = 0.0;
            let richness = 1.0;
            while t < away {
                let dt = 1.0_f32.min(away - t);
                col.tick(dt, false, richness, &mut drops);
                t += dt;
                if t > 28800.0 {
                    break;
                }
            }
            let msg = format!(
                "잘 자고 있었어요 ☀️ {}시간 만에 복귀! (일벌 {:.0}→{:.0}, 꿀 {:.0}→{:.0})",
                (away / 3600.0 * 10.0).round() / 10.0,
                before_workers,
                col.workers,
                before_honey,
                col.honey
            );
            summary.text = msg.clone();
            summary.timer = 10.0;
            notice.send(msg);
        }
        let _ = weather;
    }
}

/// Zoom 1.0-2.5 wheel, drag pan clamped, follow overrides pan.
fn camera_control(
    mut cameras: Query<&mut Transform, With<Camera>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut wheel: EventReader<bevy::input::mouse::MouseWheel>,
    mut drag: Local<Option<Vec2>>,
    windows: Query<&Window>,
    follow: Res<FollowTarget>,
    bees: Query<&Transform, (With<HiveBee>, Without<Camera>)>,
) {
    let Ok(mut cam) = cameras.get_single_mut() else {
        return;
    };
    for ev in wheel.read() {
        cam.scale.x = (cam.scale.x - ev.y * 0.1).clamp(0.4, 1.0);
        cam.scale.y = cam.scale.x;
    }
    if let Some(e) = follow.0 {
        if let Ok(bt) = bees.get(e) {
            cam.translation.x = bt.translation.x;
            cam.translation.y = bt.translation.y;
        }
        return;
    }
    let Ok(win) = windows.get_single() else {
        return;
    };
    if mouse.just_pressed(MouseButton::Right) || mouse.just_pressed(MouseButton::Left) {
        // Left is reserved for click-follow (handled elsewhere); right drags.
    }
    if mouse.pressed(MouseButton::Right) {
        if let Some(pos) = win.cursor_position() {
            if let Some(prev) = *drag {
                let dx = (pos.x - prev.x) * cam.scale.x;
                let dy = (prev.y - pos.y) * cam.scale.x;
                cam.translation.x = (cam.translation.x - dx).clamp(-450.0, 450.0);
                cam.translation.y = (cam.translation.y - dy).clamp(-380.0, 380.0);
            }
            *drag = Some(pos);
        }
    } else {
        *drag = None;
    }
}

/// Click a bee to follow; Esc handled in exit_system to release first.
fn click_follow(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    bees: Query<(Entity, &Transform), With<HiveBee>>,
    mut follow: ResMut<FollowTarget>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    let Ok(win) = windows.get_single() else {
        return;
    };
    let Some(cursor) = win.cursor_position() else {
        return;
    };
    let Ok((cam, gtf)) = cameras.get_single() else {
        return;
    };
    let Ok(world) = cam.viewport_to_world_2d(gtf, cursor) else {
        return;
    };
    let mut best: Option<(Entity, f32)> = None;
    for (e, tf) in bees.iter() {
        let d = tf.translation.truncate().distance(world);
        if d < 24.0 && best.map(|(_, bd)| d < bd).unwrap_or(true) {
            best = Some((e, d));
        }
    }
    if let Some((e, _)) = best {
        follow.0 = Some(e);
    }
}

/// Spacebar: jump to the queen.
fn find_queen(
    keyboard: Res<ButtonInput<KeyCode>>,
    bees: Query<(Entity, &HiveBee)>,
    mut follow: ResMut<FollowTarget>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }
    for (e, bee) in bees.iter() {
        if bee.role == Role::Queen {
            follow.0 = Some(e);
            return;
        }
    }
}

fn toggle_hud(keyboard: Res<ButtonInput<KeyCode>>, mut hidden: ResMut<HudHidden>) {
    if keyboard.just_pressed(KeyCode::Tab) {
        hidden.0 = !hidden.0;
    }
}

fn toggle_mute(keyboard: Res<ButtonInput<KeyCode>>, mut muted: ResMut<Muted>) {
    if keyboard.just_pressed(KeyCode::KeyM) {
        muted.0 = !muted.0;
    }
}

/// Autosave every 30s + decoration tier update.
fn autosave(
    time: Res<Time>,
    colony_q: Query<&Colony>,
    jars: Res<HoneyJars>,
    day: Res<DayNight>,
    mut timer: Local<f32>,
) {
    *timer += time.delta_secs();
    if *timer < 30.0 {
        return;
    }
    *timer = 0.0;
    if let Ok(col) = colony_q.get_single() {
        save::save(col, jars.0, day.day_count);
    }
}

/// Deco tiers (Q23=B): jars 0/5/15/30 → rim color + real stickers + notice.
fn update_deco(
    mut commands: Commands,
    jars: Res<HoneyJars>,
    assets: Res<GamePixelAssets>,
    mut deco: ResMut<DecoLevel>,
    mut notice: ResMut<Notice>,
    mut rims: Query<&mut Sprite, (With<InteriorRoot>, Without<HiveBee>, Without<Entrance>)>,
) {
    let level = if jars.0 >= 30 {
        5
    } else if jars.0 >= 15 {
        4
    } else if jars.0 >= 5 {
        3
    } else if jars.0 >= 1 {
        2
    } else {
        1
    };
    if level != deco.0 {
        deco.0 = level;
        let rim = match level {
            1 => Color::srgb(0.40, 0.26, 0.13),
            2 => Color::srgb(0.48, 0.32, 0.16),
            3 => Color::srgb(0.55, 0.38, 0.20),
            4 => Color::srgb(0.62, 0.45, 0.24),
            _ => Color::srgb(0.72, 0.55, 0.28),
        };
        for mut s in rims.iter_mut() {
            // Only the large wooden rim (custom_size ~1000x760) gets retinted.
            if let Some(size) = s.custom_size {
                if size.x > 900.0 {
                    s.color = rim;
                }
            }
        }
        // Real stickers on the rim (cute payoff for each tier).
        let bx = 0.0_f32;
        let by = 330.0_f32;
        match level {
            3 => {
                for (i, img) in [
                    assets.flower_red.clone(),
                    assets.flower_yellow.clone(),
                    assets.flower_blue.clone(),
                ]
                .iter()
                .enumerate()
                {
                    commands.spawn((
                        Sprite {
                            image: img.clone(),
                            custom_size: Some(Vec2::new(30.0, 30.0)),
                            ..default()
                        },
                        Transform::from_xyz(bx - 360.0 + i as f32 * 36.0, by, 3.0),
                        DecoSticker,
                        InteriorRoot,
                    ));
                }
                notice.send("🌸 들꽃 스티커가 붙었어요 (5단지)");
            }
            4 => {
                commands.spawn((
                    Sprite {
                        color: Color::srgb(1.0, 0.45, 0.6),
                        custom_size: Some(Vec2::new(150.0, 18.0)),
                        ..default()
                    },
                    Transform::from_xyz(bx + 200.0, by, 3.0),
                    DecoSticker,
                    InteriorRoot,
                ));
                commands.spawn((
                    Sprite {
                        image: assets.flower_white.clone(),
                        custom_size: Some(Vec2::new(30.0, 30.0)),
                        ..default()
                    },
                    Transform::from_xyz(bx + 200.0, by, 3.5),
                    DecoSticker,
                    InteriorRoot,
                ));
                notice.send("🎀 리본 장식이 달렸어요 (15단지)");
            }
            5 => {
                for dx in [-440.0_f32, 440.0] {
                    commands.spawn((
                        Sprite {
                            image: assets.flower_gold.clone(),
                            custom_size: Some(Vec2::new(34.0, 34.0)),
                            ..default()
                        },
                        Transform::from_xyz(dx, by - 40.0, 3.0),
                        DecoSticker,
                        InteriorRoot,
                    ));
                }
                notice.send("✨ 황금 테두리! 대들보가 반짝여요 (30단지)");
            }
            _ => {}
        }
    }
}

fn dismiss_tutorial() {}

/// Esc releases follow first, then quits. M/H/Tab handled elsewhere.
fn exit_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut follow: ResMut<FollowTarget>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if follow.0.is_some() {
            follow.0 = None;
        } else {
            exit.send(AppExit::Success);
        }
    }
}

fn close_requested(
    mut close: EventReader<WindowCloseRequested>,
    colony_q: Query<&Colony>,
    jars: Res<HoneyJars>,
    day: Res<DayNight>,
    mut exit: EventWriter<AppExit>,
) {
    for _ in close.read() {
        if let Ok(col) = colony_q.get_single() {
            save::save(col, jars.0, day.day_count);
        }
        exit.send(AppExit::Success);
    }
}

#[cfg(test)]
mod boot_tests {
    use super::*;
    use bevy::asset::{AssetApp, AssetPlugin};
    use bevy::image::Image;
    use bevy::input::ButtonInput;
    use bevy::state::app::StatesPlugin;
    use bevy::tasks::{IoTaskPool, TaskPool};
    use bevy::text::Font;
    use bevy::time::TimePlugin;

    fn headless_app() -> App {
        let mut app = App::new();
        IoTaskPool::get_or_init(TaskPool::new);
        app.add_plugins((TimePlugin, AssetPlugin::default(), StatesPlugin));
        app.init_asset::<Image>();
        app.init_asset::<Font>();
        app.add_event::<AppExit>();
        app.add_event::<WindowCloseRequested>();
        app.init_resource::<ButtonInput<KeyCode>>();
        app.init_resource::<ButtonInput<MouseButton>>();
        app.add_event::<bevy::input::mouse::MouseWheel>();
        configure(&mut app);
        app
    }

    fn press_key(app: &mut App, key: KeyCode) {
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .reset_all();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(key);
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .reset_all();
        app.update();
    }

    #[test]
    fn test_boot_ticks() {
        let mut app = headless_app();
        for _ in 0..5 {
            app.update();
        }
        assert!(app.world().contains_resource::<Notice>());
        let mut hives = app.world_mut().query::<&Colony>();
        assert_eq!(hives.iter(app.world()).len(), 1);
    }

    #[test]
    fn test_harvest_flow() {
        let mut app = headless_app();
        for _ in 0..5 {
            app.update();
        }
        // Integration check: H wires to Colony::harvest. The live sim
        // keeps delivering, so assert direction + lower bound, not exact.
        {
            let mut q = app.world_mut().query::<&mut Colony>();
            let mut col = q.iter_mut(app.world_mut()).next().unwrap();
            col.honey = 200.0;
            col.built = Colony::MAX_CELLS;
        }
        press_key(&mut app, KeyCode::KeyH);
        // floor((200-12)/10) = 18 minimum; forager drops only add more.
        assert!(
            app.world().resource::<HoneyJars>().0 >= 18,
            "H should press jars"
        );
        let honey = app
            .world_mut()
            .query::<&Colony>()
            .iter(app.world())
            .next()
            .unwrap()
            .honey;
        assert!(honey < 200.0, "harvest must drain honey");
    }

    #[test]
    fn test_esc_quits_when_not_following() {
        let mut app = headless_app();
        for _ in 0..3 {
            app.update();
        }
        press_key(&mut app, KeyCode::Escape);
        let n = app.world().resource::<Events<AppExit>>().len();
        assert_eq!(n, 1, "Esc with no follow must request quit");
    }

    #[test]
    fn test_close_button_quits() {
        let mut app = headless_app();
        for _ in 0..3 {
            app.update();
        }
        app.world_mut()
            .resource_mut::<Events<WindowCloseRequested>>()
            .send(WindowCloseRequested { window: Entity::PLACEHOLDER });
        app.update();
        app.update();
        let n = app.world().resource::<Events<AppExit>>().len();
        assert_eq!(n, 1, "window X must request quit");
    }
}
