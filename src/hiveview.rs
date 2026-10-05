use bevy::prelude::*;
use bevy::image::ImageSampler;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use rand::Rng;
use crate::colony::Colony;
use crate::comb::{self, FrameCell, CELLS, HEX_H, HEX_W, paint_hex};
use crate::meadow::{DayNight, Weather};
use crate::pixel_assets::GamePixelAssets;

// Living hive interior: the beekeeper watches every caste habit.
//
// - Queen parades the brood nest and visibly lays (flash + egg pops in).
// - Foragers shuttle comb -> flowers -> comb, glowing gold (nectar) or
//   orange (pollen); every home arrival fattens the stores.
// - Nurses pause at brood cells to feed larvae.
// - Builders haunt the comb edge; new wax cells pop in behind them.
// - Drones drift aimlessly and get evicted first in a dearth.
// The colony runs itself; watching is the game.

/// Interior anchor: the hive IS the world now (single screen at origin).
pub const ANCHOR: Vec2 = Vec2::new(0.0, 0.0);

#[derive(Component)]
pub struct InteriorRoot;

#[derive(Component)]
pub struct WaxCell {
    pub idx: usize,
    pub kind: FrameCell,
}

#[derive(Component)]
pub struct Entrance;

/// Cuteness layer markers.
#[derive(Component)]
pub struct CuteMote {
    pub phase: f32,
    pub speed: f32,
}

#[derive(Component)]
pub struct SkyIcon;

#[derive(Component)]
pub struct NightTint;

#[derive(Component)]
pub struct HoneyDrip {
    pub phase: f32,
}

#[derive(Component)]
pub struct DecoSticker;

/// Growth glyph floating over a brood cell: 0 egg rice, 1 grub, 2 capped.
#[derive(Component)]
pub struct BroodGlyph {
    pub idx: usize,
    pub stage: u8,
}

fn glyph_size_color(stage: u8) -> (Vec2, Color) {
    match stage {
        0 => (
            Vec2::new(5.0, 7.0),
            Color::srgb(1.0, 1.0, 0.96),
        ), // standing rice grain
        1 => (
            Vec2::new(11.0, 7.0),
            Color::srgb(1.0, 0.93, 0.75),
        ), // plump grub
        _ => (
            Vec2::new(12.0, 8.0),
            Color::srgb(0.85, 0.6, 0.3),
        ), // capped dome
    }
}

/// First not-yet-built cell (brood front): where builders chew next.
fn frontier_point(built: f32, rng: &mut rand::rngs::ThreadRng) -> Vec2 {
    let mask = comb::built_mask(built);
    let order: Vec<usize> = {
        // center-out priority so the nursery visibly grows.
        let mut v: Vec<usize> = (0..comb::CELLS).collect();
        let (cx, cy) = (comb::COLS as f32 / 2.0 - 0.5, comb::ROWS as f32 - 3.0);
        v.sort_by(|&a, &b| {
            let da = {
                let x = (a % comb::COLS) as f32;
                let y = (a / comb::COLS) as f32;
                ((x - cx) * (x - cx) + (y - cy) * (y - cy) * 1.4).sqrt()
            };
            let db = {
                let x = (b % comb::COLS) as f32;
                let y = (b / comb::COLS) as f32;
                ((x - cx) * (x - cx) + (y - cy) * (y - cy) * 1.4).sqrt()
            };
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        });
        v
    };
    for idx in order {
        if !mask[idx] {
            let p = comb_origin() + comb::cell_pos(idx);
            return p + Vec2::new(rng.gen_range(-8.0..8.0), rng.gen_range(-8.0..8.0));
        }
    }
    comb_point(rng)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    Forager,
    Nurse,
    Builder,
    Drone,
    Queen,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Load {
    Empty,
    Nectar,
    Pollen,
}

#[derive(Component)]
pub struct HiveBee {
    pub role: Role,
    pub waypoint: Vec2,
    pub speed: f32,
    pub loaded: Load,
    pub wait: f32,
    pub lay: f32,
    pub trips: u32,
    /// Outside wander legs done this trip (0 = heading out).
    pub outside: u8,
}

#[derive(Component)]
pub struct Flash {
    pub ttl: f32,
}

/// Shared white hex tile; per-cell tint carries the content color.
#[derive(Resource)]
pub struct CombKit {
    pub tile: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct SavedCam(pub Vec3);

/// Nectar/pollen hauled home by visual foragers, drained by the sim tick.
#[derive(Resource, Default)]
pub struct PendingDrops {
    pub nectar: f32,
    pub pollen: f32,
}

fn comb_origin() -> Vec2 {
    let size = comb::comb_size();
    Vec2::new(
        ANCHOR.x - size.x / 2.0,
        ANCHOR.y + size.y / 2.0 - HEX_H as f32 / 2.0,
    )
}

fn comb_point(rng: &mut rand::rngs::ThreadRng) -> Vec2 {
    let o = comb_origin();
    let size = comb::comb_size();
    Vec2::new(
        o.x + rng.gen_range(10.0..size.x - 10.0),
        o.y - rng.gen_range(10.0..size.y - 10.0),
    )
}

fn flower_spots() -> [Vec2; 2] {
    // Bottom entrance hole + just-below-screen staging point.
    // Foragers exit through the hole, return loaded — no exterior map.
    [
        Vec2::new(ANCHOR.x, ANCHOR.y - 300.0),
        Vec2::new(ANCHOR.x, ANCHOR.y - 360.0),
    ]
}

fn entrance_point() -> Vec2 {
    Vec2::new(ANCHOR.x, ANCHOR.y - 300.0)
}

/// Gathering spot below the screen: bees visibly leave through the hole,
/// linger outside, then climb back loaded.
fn outside_point(rng: &mut rand::rngs::ThreadRng) -> Vec2 {
    Vec2::new(
        ANCHOR.x + rng.gen_range(-260.0..260.0),
        ANCHOR.y - rng.gen_range(430.0..520.0),
    )
}

fn hex_tile_image() -> Image {
    let mut buf = vec![0u8; (HEX_W * HEX_H * 4) as usize];
    paint_hex(&mut buf);
    let mut img = Image::new(
        Extent3d {
            width: HEX_W,
            height: HEX_H,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        buf,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    img.sampler = ImageSampler::nearest();
    img
}

/// Carry the camera inside the hive and raise the comb.
pub fn spawn_interior(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    pixel_assets: Res<GamePixelAssets>,
    mut cameras: Query<&mut Transform, With<Camera>>,
    mut saved: ResMut<SavedCam>,
    hive: Query<&Colony>,
) {
    for mut tf in cameras.iter_mut() {
        saved.0 = tf.translation;
        tf.translation.x = ANCHOR.x;
        tf.translation.y = ANCHOR.y;
    }

    // Hollow-tree backdrop: wooden rim + dark heartwood.
    commands.spawn((
        Sprite {
            color: Color::srgb(0.40, 0.26, 0.13),
            custom_size: Some(Vec2::new(1000.0, 760.0)),
            ..default()
        },
        Transform::from_xyz(ANCHOR.x, ANCHOR.y - 10.0, -11.0),
        InteriorRoot,
    ));
    commands.spawn((
        Sprite {
            color: Color::srgb(0.11, 0.075, 0.045),
            custom_size: Some(Vec2::new(940.0, 700.0)),
            ..default()
        },
        Transform::from_xyz(ANCHOR.x, ANCHOR.y - 10.0, -10.0),
        InteriorRoot,
    ));

    let tile = images.add(hex_tile_image());
    commands.insert_resource(CombKit { tile: tile.clone() });

    // Comb painted from the live colony (or fresh wax).
    let layout = hive
        .iter()
        .next()
        .map(comb::layout_for_colony)
        .unwrap_or([FrameCell::Unbuilt; CELLS]);
    let origin = comb_origin();
    for (idx, kind) in layout.iter().enumerate() {
        let p = origin + comb::cell_pos(idx);
        commands.spawn((
            Sprite {
                image: tile.clone(),
                color: kind.tint(),
                ..default()
            },
            Transform::from_xyz(p.x, p.y, 0.0),
            WaxCell { idx, kind: *kind },
            InteriorRoot,
        ));
    }

    // Bottom entrance hole: foragers exit/enter here (no exterior map).
    // Dark wooden rim + black hole.
    commands.spawn((
        Sprite {
            color: Color::srgb(0.30, 0.19, 0.10),
            custom_size: Some(Vec2::new(220.0, 40.0)),
            ..default()
        },
        Transform::from_xyz(entrance_point().x, entrance_point().y + 10.0, 1.0),
        Entrance,
        InteriorRoot,
    ));
    commands.spawn((
        Sprite {
            color: Color::srgb(0.02, 0.015, 0.01),
            custom_size: Some(Vec2::new(180.0, 22.0)),
            ..default()
        },
        Transform::from_xyz(entrance_point().x, entrance_point().y + 8.0, 1.5),
        Entrance,
        InteriorRoot,
    ));
    // Honey drips under the entrance (cute + tells "honey lives here").
    for (i, dx) in [-40.0_f32, 0.0, 40.0].iter().enumerate() {
        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 0.72, 0.2),
                custom_size: Some(Vec2::new(8.0, 12.0 + i as f32 * 3.0)),
                ..default()
            },
            Transform::from_xyz(
                entrance_point().x + dx,
                entrance_point().y - 8.0,
                1.6,
            ),
            HoneyDrip { phase: i as f32 * 2.1 },
            InteriorRoot,
        ));
    }
    // Bunting flags across the top rim (instant coziness).
    let flag_colors = [
        Color::srgb(1.0, 0.55, 0.65),
        Color::srgb(1.0, 0.85, 0.3),
        Color::srgb(0.55, 0.9, 0.7),
        Color::srgb(0.55, 0.75, 1.0),
    ];
    for i in 0..14 {
        commands.spawn((
            Sprite {
                color: flag_colors[i % flag_colors.len()],
                custom_size: Some(Vec2::new(18.0, 14.0)),
                ..default()
            },
            Transform::from_xyz(
                ANCHOR.x - 280.0 + i as f32 * 43.0,
                ANCHOR.y + 322.0,
                2.0,
            ),
            DecoSticker,
            InteriorRoot,
        ));
    }
    // Sky-slit window: little round window showing weather at a glance.
    commands.spawn((
        Sprite {
            color: Color::srgb(0.16, 0.12, 0.08),
            custom_size: Some(Vec2::new(84.0, 84.0)),
            ..default()
        },
        Transform::from_xyz(ANCHOR.x + 400.0, ANCHOR.y + 250.0, 2.0),
        DecoSticker,
        InteriorRoot,
    ));
    commands.spawn((
        Sprite {
            image: pixel_assets.flower_gold.clone(),
            custom_size: Some(Vec2::new(56.0, 56.0)),
            ..default()
        },
        Transform::from_xyz(ANCHOR.x + 400.0, ANCHOR.y + 250.0, 2.5),
        SkyIcon,
        InteriorRoot,
    ));
    // Night tint overlay (soft blue veil after dark).
    commands.spawn((
        Sprite {
            color: Color::srgba(0.1, 0.15, 0.35, 0.0),
            custom_size: Some(Vec2::new(940.0, 700.0)),
            ..default()
        },
        Transform::from_xyz(ANCHOR.x, ANCHOR.y - 10.0, 6.0),
        NightTint,
        InteriorRoot,
    ));
    // Floating dust motes / pollen sparkles (slow drift + twinkle).
    let mut rng = rand::thread_rng();
    for _ in 0..22 {
        let gold = rng.gen_bool(0.4);
        commands.spawn((
            Sprite {
                color: if gold {
                    Color::srgba(1.0, 0.9, 0.5, 0.5)
                } else {
                    Color::srgba(1.0, 1.0, 1.0, 0.35)
                },
                custom_size: Some(Vec2::new(5.0, 5.0)),
                ..default()
            },
            Transform::from_xyz(
                ANCHOR.x + rng.gen_range(-440.0..440.0),
                ANCHOR.y + rng.gen_range(-320.0..320.0),
                5.5,
            ),
            CuteMote {
                phase: rng.gen_range(0.0..6.28),
                speed: rng.gen_range(8.0..22.0),
            },
            InteriorRoot,
        ));
    }
    let _ = pixel_assets;
}

/// Tear the interior down and return the camera to the meadow.
pub fn despawn_interior(
    mut commands: Commands,
    roots: Query<Entity, With<InteriorRoot>>,
    mut cameras: Query<&mut Transform, With<Camera>>,
    saved: Res<SavedCam>,
) {
    for e in roots.iter() {
        commands.entity(e).despawn();
    }
    commands.remove_resource::<CombKit>();
    for mut tf in cameras.iter_mut() {
        tf.translation = saved.0;
    }
}

fn role_size(role: Role) -> Vec2 {
    match role {
        Role::Forager | Role::Nurse | Role::Builder => Vec2::new(20.0, 17.0),
        Role::Drone => Vec2::new(24.0, 20.0),
        Role::Queen => Vec2::new(27.0, 22.0),
    }
}

fn role_color(role: Role) -> Color {
    match role {
        Role::Queen => Color::srgb(1.0, 0.84, 0.4),
        Role::Drone => Color::srgb(0.72, 0.78, 0.9),
        _ => Color::WHITE,
    }
}

/// Bustle: every caste walks its habit. Drop-offs land in PendingDrops.
pub fn walk_hive(
    mut commands: Commands,
    time: Res<Time>,
    night: Res<DayNight>,
    colony: Query<&Colony>,
    brood: Query<&WaxCell>,
    mut bees: Query<(Entity, &mut Transform, &mut HiveBee, &mut Sprite)>,
    mut pending: ResMut<PendingDrops>,
) {
    let delta = time.delta_secs();
    if delta <= 0.0 {
        return;
    }
    let Ok(col) = colony.get_single() else {
        return;
    };
    let slow = if night.is_night() { 0.4 } else { 1.0 };
    let mut rng = rand::thread_rng();

    for (_, mut tf, mut bee, mut sprite) in bees.iter_mut() {
        if bee.wait > 0.0 {
            bee.wait -= delta;
            continue;
        }
        let pos = tf.translation.truncate();
        let to = bee.waypoint - pos;
        let dist = to.length();
        if dist < 12.0 {
            match bee.role {
                Role::Forager => {
                    if night.is_night() {
                        // Nights are for clustering, not flying.
                        bee.loaded = Load::Empty;
                        sprite.color = Color::WHITE;
                        bee.wait = 1.0;
                        bee.waypoint = comb_point(&mut rng);
                    } else if bee.loaded == Load::Empty {
                        let hole = entrance_point();
                        let below = pos.y < hole.y - 40.0;
                        if below {
                            // Outside: visit a couple of flower patches
                            // (visible wandering), then load up and queue
                            // back through the hole.
                            if bee.outside < 2 {
                                bee.outside += 1;
                                bee.wait = rng.gen_range(1.6..2.6);
                                bee.waypoint = outside_point(&mut rng);
                            } else {
                                bee.outside = 0;
                                bee.trips += 1;
                                bee.loaded = if bee.trips % 2 == 0 {
                                    Load::Pollen
                                } else {
                                    Load::Nectar
                                };
                                sprite.color = if bee.loaded == Load::Nectar {
                                    Color::srgb(1.0, 0.8, 0.3)
                                } else {
                                    Color::srgb(1.0, 0.55, 0.2)
                                };
                                bee.wait = rng.gen_range(0.6..1.0);
                                bee.waypoint = hole;
                            }
                        } else {
                            let at_hole = pos.distance(hole) < 24.0;
                            if at_hole {
                                // Queue a breath at the hole, then dart out
                                // single-file through its center.
                                bee.outside = 0;
                                bee.wait = rng.gen_range(0.4..0.9);
                                bee.waypoint = outside_point(&mut rng);
                            } else if bee.waypoint.y < hole.y - 40.0 {
                                // Already heading out: keep going.
                            } else {
                                bee.waypoint = hole;
                            }
                        }
                    } else {
                        // Loaded: re-enter through the hole first, then
                        // carry the drop to the comb.
                        let hole = entrance_point();
                        if pos.distance(hole) < 24.0 {
                            // Just came in through the hole: head up.
                            bee.waypoint = comb_point(&mut rng);
                        } else if pos.y < hole.y - 40.0 {
                            // Still outside: climb to the hole.
                            bee.waypoint = hole;
                        } else {
                            // At the comb: unload into the stores.
                            match bee.loaded {
                                Load::Nectar => pending.nectar += 1.5,
                                Load::Pollen => pending.pollen += 1.0,
                                Load::Empty => {}
                            }
                            bee.loaded = Load::Empty;
                            sprite.color = Color::WHITE;
                            bee.wait = 0.4;
                            bee.waypoint = hole;
                        }
                    }
                }
                Role::Nurse => {
                    bee.wait = rng.gen_range(0.5..1.5); // feeding a larva
                    let mut spots = Vec::new();
                    for cell in brood.iter() {
                        if matches!(cell.kind, FrameCell::Egg | FrameCell::Larva) {
                            spots.push(comb_origin() + comb::cell_pos(cell.idx));
                        }
                    }
                    bee.waypoint = if spots.is_empty() {
                        comb_point(&mut rng)
                    } else {
                        spots[rng.gen_range(0..spots.len())]
                    };
                }
                Role::Builder => {
                    bee.wait = rng.gen_range(0.5..1.2); // chewing wax
                    // Haunt the rising edge so 증축 reads as work, not magic.
                    bee.waypoint = frontier_point(col.built, &mut rng);
                }
                Role::Drone => {
                    let o = comb_origin();
                    let size = comb::comb_size();
                    bee.waypoint = Vec2::new(
                        o.x + rng.gen_range(-50.0..size.x + 50.0),
                        o.y - rng.gen_range(-50.0..size.y + 50.0),
                    );
                }
                Role::Queen => {
                    // Lay an egg where she stands: white pop + gold ring,
                    // then the wax sync below paints it into the comb.
                    bee.lay += delta;
                    let interval = 1.0 / col.lay_display.max(0.5);
                    if bee.lay >= interval {
                        bee.lay = 0.0;
                        commands.spawn((
                            Sprite {
                                color: Color::srgba(1.0, 1.0, 1.0, 0.95),
                                custom_size: Some(Vec2::new(14.0, 14.0)),
                                ..default()
                            },
                            Transform::from_xyz(pos.x, pos.y, 5.0),
                            Flash { ttl: 0.45 },
                            InteriorRoot,
                        ));
                        commands.spawn((
                            Sprite {
                                color: Color::srgba(1.0, 0.85, 0.4, 0.8),
                                custom_size: Some(Vec2::new(22.0, 22.0)),
                                ..default()
                            },
                            Transform::from_xyz(pos.x, pos.y, 4.9),
                            Flash { ttl: 0.7 },
                            InteriorRoot,
                        ));
                    }
                    let o = comb_origin();
                    let size = comb::comb_size();
                    bee.waypoint = Vec2::new(
                        o.x + size.x * 0.5 + rng.gen_range(-110.0..110.0),
                        o.y - size.y * 0.68 + rng.gen_range(-60.0..60.0),
                    );
                    bee.wait = rng.gen_range(0.8..1.8);
                }
            }
        } else {
            let step = (bee.speed * slow * delta).min(dist);
            tf.translation.x += to.x / dist * step;
            tf.translation.y += to.y / dist * step;
            sprite.flip_x = to.x < 0.0;
            // Slip INTO the hole, not over it: dip behind the dark rim
            // while crossing so exits/entries read as in/out.
            if bee.role == Role::Forager {
                let hole = entrance_point();
                let over_hole = (tf.translation.x - hole.x).abs() < 90.0
                    && (tf.translation.y - hole.y).abs() < 28.0;
                tf.translation.z = if over_hole { 1.2 } else { 4.0 };
            }
        }
    }
}

/// Wax sync + caste staffing, a few times per second.
pub fn sync_comb(
    mut commands: Commands,
    time: Res<Time>,
    pixel_assets: Res<GamePixelAssets>,
    colony: Query<&Colony>,
    mut cells: Query<(&mut WaxCell, &mut Sprite, &Transform)>,
    staff: Query<(Entity, &HiveBee)>,
    glyphs: Query<(Entity, &BroodGlyph)>,
    mut timer: Local<f32>,
) {
    *timer += time.delta_secs();
    if *timer < 0.4 {
        return;
    }
    *timer = 0.0;
    let Ok(col) = colony.get_single() else {
        return;
    };

    let layout = comb::layout_for_colony(&col);
    // Starving: dim the comb slightly so hunger reads without alarms (Q33=B).
    let dim = if col.starving { 0.75 } else { 1.0 };
    // Desired growth stage per cell for the glyph pass below.
    let mut want_stage = [None; comb::CELLS];
    for (mut cell, mut sprite, tf) in cells.iter_mut() {
        let kind = layout[cell.idx];
        if kind != cell.kind {
            // Fresh wax pops gold so 증축 is an event, not a silent swap.
            if cell.kind == FrameCell::Unbuilt && kind != FrameCell::Unbuilt {
                commands.spawn((
                    Sprite {
                        color: Color::srgba(1.0, 0.85, 0.4, 0.85),
                        custom_size: Some(Vec2::new(26.0, 26.0)),
                        ..default()
                    },
                    Transform::from_xyz(tf.translation.x, tf.translation.y, 5.0),
                    Flash { ttl: 0.6 },
                    InteriorRoot,
                ));
            }
            cell.kind = kind;
        }
        let mut tint = kind.tint();
        // Apply dim by scaling sRGB channels.
        let c = tint.to_srgba();
        tint = Color::srgb(c.red * dim, c.green * dim, c.blue * dim);
        if sprite.color != tint {
            sprite.color = tint;
        }
        want_stage[cell.idx] = match kind {
            FrameCell::Egg => Some(0),
            FrameCell::Larva => Some(1),
            FrameCell::Pupa => Some(2),
            _ => None,
        };
    }

    // Growth glyphs: one sprite per brood cell, stage-colored.
    // Despawn stale, spawn missing (a few per 0.4s tick at most).
    for (e, g) in glyphs.iter() {
        if want_stage[g.idx] != Some(g.stage) {
            commands.entity(e).despawn();
        }
    }
    // Rebuild the live set each tick (cheap: <=126 cells, changes are rare).
    let mut live = [false; comb::CELLS];
    for (_, g) in glyphs.iter() {
        if want_stage[g.idx] == Some(g.stage) {
            live[g.idx] = true;
        }
    }
    let origin = comb_origin();
    for idx in 0..comb::CELLS {
        if let Some(stage) = want_stage[idx] {
            if !live[idx] {
                let (size, color) = glyph_size_color(stage);
                let p = origin + comb::cell_pos(idx);
                commands.spawn((
                    Sprite {
                        color,
                        custom_size: Some(size),
                        ..default()
                    },
                    Transform::from_xyz(p.x, p.y + 2.0, 2.0),
                    BroodGlyph { idx, stage },
                    InteriorRoot,
                ));
            }
        }
    }

    // ~30-45 visible bees (Q12=A symbolic): scale up from old tiny crew.
    let want_f = ((col.foragers() / 12.0) as u32).clamp(4, 16);
    let want_n = ((col.nurses() / 2.0 / 12.0) as u32).clamp(3, 10);
    let want_b = ((col.nurses() / 2.0 / 12.0) as u32).clamp(3, 8);
    let want_d = ((col.drones / 8.0) as u32).clamp(1, 4);
    let want = [want_f, want_n, want_b, want_d, 1];
    let roles = [
        Role::Forager,
        Role::Nurse,
        Role::Builder,
        Role::Drone,
        Role::Queen,
    ];
    let mut have = [0u32; 5];
    for (e, bee) in staff.iter() {
        let i = bee.role as usize;
        have[i] += 1;
        if have[i] > want[i] {
            commands.entity(e).despawn();
            have[i] -= 1;
        }
    }

    let mut rng = rand::thread_rng();
    for (i, role) in roles.iter().enumerate() {
        for _ in have[i]..want[i] {
            let home = comb_point(&mut rng);
            commands.spawn((
                Sprite {
                    image: pixel_assets.bee.clone(),
                    custom_size: Some(role_size(*role)),
                    color: role_color(*role),
                    ..default()
                },
                Transform::from_xyz(home.x, home.y, 4.0),
                HiveBee {
                    role: *role,
                    waypoint: home,
                    speed: match role {
                        Role::Forager => 150.0,
                        Role::Nurse => 70.0,
                        Role::Builder => 60.0,
                        Role::Drone => 35.0,
                        Role::Queen => 26.0,
                    },
                    loaded: Load::Empty,
                    wait: rng.gen_range(0.0..1.0),
                    lay: 0.0,
                    trips: rng.gen_range(0..2),
                    outside: 0,
                },
                InteriorRoot,
            ));
        }
    }
}

/// Cuteness + readability animation, every frame (cheap sprite math).
/// - motes drift up and twinkle, drips bob, bees waddle (queen glows).
/// - night veil fades, sky-slit icon follows the weather dice.
pub fn cute_anim(
    time: Res<Time>,
    night: Res<DayNight>,
    weather: Res<Weather>,
    pixel_assets: Res<GamePixelAssets>,
    mut motes: Query<
        (&mut Transform, &mut Sprite, &CuteMote),
        (Without<HoneyDrip>, Without<HiveBee>, Without<BroodGlyph>),
    >,
    mut drips: Query<
        (&mut Transform, &HoneyDrip),
        (Without<CuteMote>, Without<HiveBee>, Without<BroodGlyph>),
    >,
    mut bees: Query<
        (&HiveBee, &mut Transform),
        (Without<CuteMote>, Without<HoneyDrip>, Without<BroodGlyph>),
    >,
    mut sky: Query<
        &mut Sprite,
        (
            With<SkyIcon>,
            Without<CuteMote>,
            Without<HiveBee>,
            Without<NightTint>,
            Without<BroodGlyph>,
        ),
    >,
    mut veil: Query<
        &mut Sprite,
        (
            With<NightTint>,
            Without<SkyIcon>,
            Without<CuteMote>,
            Without<HiveBee>,
            Without<BroodGlyph>,
        ),
    >,
    mut grubs: Query<
        (&BroodGlyph, &mut Transform),
        (
            Without<CuteMote>,
            Without<HiveBee>,
            Without<HoneyDrip>,
            Without<SkyIcon>,
            Without<NightTint>,
        ),
    >,
) {
    let t = time.elapsed_secs();
    for (mut tf, mut s, m) in motes.iter_mut() {
        tf.translation.y += m.speed * time.delta_secs();
        if tf.translation.y > ANCHOR.y + 330.0 {
            tf.translation.y = ANCHOR.y - 330.0;
        }
        tf.translation.x += (t * 0.7 + m.phase).sin() * 6.0 * time.delta_secs();
        let a = 0.28 + 0.22 * (t * 2.0 + m.phase).sin();
        s.color.set_alpha(a.clamp(0.05, 0.6));
    }
    for (mut tf, d) in drips.iter_mut() {
        let base = 12.0;
        tf.scale.y = 1.0 + 0.18 * (t * 2.4 + d.phase).sin();
        let _ = base;
    }
    for (bee, mut tf) in bees.iter_mut() {
        let phase = bee.waypoint.x * 0.05 + bee.waypoint.y * 0.03;
        tf.rotation = Quat::from_rotation_z(0.12 * (t * 6.0 + phase).sin());
        // Outside = farther = smaller: depth cue for the foraging trip.
        let outside = bee.role == Role::Forager
            && tf.translation.y < entrance_point().y - 40.0;
        let base = if outside { 0.72 } else { 1.0 };
        let pulse = if bee.role == Role::Queen {
            1.0 + 0.08 * (t * 3.0).sin()
        } else {
            1.0 + 0.05 * (t * 8.0 + phase).sin()
        };
        tf.scale = Vec3::splat(base * pulse);
    }
    // Brood growth: eggs bob, grubs visibly wiggle-and-swell, pupae rest.
    for (g, mut tf) in grubs.iter_mut() {
        let ph = g.idx as f32 * 1.7;
        match g.stage {
            0 => {
                tf.scale = Vec3::splat(1.0 + 0.05 * (t * 3.0 + ph).sin());
                tf.rotation = Quat::IDENTITY;
            }
            1 => {
                // 0.8 → 1.2 swell + sideways wiggle: "크는 중"이 읽힘.
                let swell = 1.0 + 0.18 * (t * 2.2 + ph).sin();
                tf.scale = Vec3::new(swell + 0.08, 1.0 + 0.10 * (t * 2.2 + ph).sin(), 1.0);
                tf.rotation =
                    Quat::from_rotation_z(0.35 * (t * 7.0 + ph).sin());
            }
            _ => {
                tf.scale = Vec3::splat(1.0 + 0.02 * (t * 1.5 + ph).sin());
                tf.rotation = Quat::IDENTITY;
            }
        }
    }
    // Sky-slit icon.
    let (img, tint) = match *weather {
        Weather::Sunny => (pixel_assets.flower_gold.clone(), Color::WHITE),
        Weather::Cloudy => (pixel_assets.flower_white.clone(), Color::srgb(0.85, 0.88, 0.95)),
        Weather::Rain => (pixel_assets.flower_blue.clone(), Color::srgb(0.7, 0.85, 1.0)),
    };
    for mut s in sky.iter_mut() {
        if s.image != img {
            s.image = img.clone();
        }
        s.color = tint;
    }
    // Night veil: soft blue after dark, extra grey when raining.
    let mut alpha = if night.is_night() { 0.32 } else { 0.0 };
    if *weather == Weather::Rain {
        alpha += 0.10;
    }
    for mut s in veil.iter_mut() {
        s.color.set_alpha(alpha);
    }
}

/// Fade laying flashes.
pub fn fade_flashes(
    time: Res<Time>,
    mut commands: Commands,
    mut flashes: Query<(Entity, &mut Flash, &mut Sprite)>,
) {
    let delta = time.delta_secs();
    for (e, mut f, mut s) in flashes.iter_mut() {
        f.ttl -= delta;
        if f.ttl <= 0.0 {
            commands.entity(e).despawn();
        } else {
            s.color.set_alpha((f.ttl / 0.45).clamp(0.0, 1.0));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interior_at_origin() {
        assert!(ANCHOR.x.abs() < 1.0);
    }

    #[test]
    fn test_comb_fits_backdrop() {
        let size = comb::comb_size();
        assert!(size.x < 900.0 && size.y < 640.0);
    }

    #[test]
    fn test_load_tints_read_at_a_glance() {
        // Gold nectar vs orange pollen must differ.
        assert_ne!(
            Color::srgb(1.0, 0.8, 0.3),
            Color::srgb(1.0, 0.55, 0.2)
        );
    }
}
