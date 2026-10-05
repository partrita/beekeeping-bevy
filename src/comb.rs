use bevy::prelude::*;
use crate::colony::Colony;

// Hexagonal comb: layout + guaranteed-correct rasterizer.
//
// Real comb hangs with a vertex at the top (pointy-top hexagons), so the
// sprite AND the grid math below use the same pointy-top convention:
// horizontal step sqrt(3)*S, vertical step 1.5*S, odd rows shifted half
// a step. Unit tests pin both the pixels and the spacing, so the comb
// can never stack wrong again.

pub const COLS: usize = 14;
pub const ROWS: usize = 9;
pub const CELLS: usize = COLS * ROWS; // 126

/// Center-to-vertex radius of one wax cell.
pub const HEX_S: f32 = 20.0;
/// Sprite canvas (hex + transparent margin for gaps).
pub const HEX_W: u32 = 40;
pub const HEX_H: u32 = 44;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameCell {
    Empty,
    Egg,
    Larva,
    Pupa,
    Honey,
    Pollen,
    Unbuilt,
}

impl FrameCell {
    /// Wax tint per content (multiplied over the white hex sprite).
    pub fn tint(self) -> Color {
        match self {
            FrameCell::Empty => Color::srgb(0.42, 0.30, 0.18),
            FrameCell::Egg => Color::srgb(0.96, 0.94, 0.88),
            FrameCell::Larva => Color::srgb(0.93, 0.84, 0.66),
            FrameCell::Pupa => Color::srgb(0.74, 0.56, 0.32),
            FrameCell::Honey => Color::srgb(1.0, 0.68, 0.15),
            FrameCell::Pollen => Color::srgb(0.9, 0.44, 0.16),
            FrameCell::Unbuilt => Color::srgb(0.10, 0.07, 0.04),
        }
    }
}

/// Grid position of a cell (comb origin at 0,0, rows grow downward).
pub fn cell_pos(idx: usize) -> Vec2 {
    let col = (idx % COLS) as f32;
    let row = (idx / COLS) as f32;
    let x = 1.7320508 * HEX_S * (col + 0.5 * ((row as i32) & 1) as f32);
    Vec2::new(x, -1.5 * HEX_S * row)
}

/// Comb extents in world units.
pub fn comb_size() -> Vec2 {
    Vec2::new(
        1.7320508 * HEX_S * (COLS as f32 - 1.0 + 0.5) + HEX_W as f32,
        1.5 * HEX_S * (ROWS as f32 - 1.0) + HEX_H as f32,
    )
}

/// Rasterize one white pointy-top hex tile (transparent outside).
/// Border ring stays dark so tinting keeps cell edges readable.
pub fn paint_hex(buf: &mut [u8]) {
    debug_assert_eq!(buf.len(), (HEX_W * HEX_H * 4) as usize);
    let cx = HEX_W as f32 / 2.0;
    let cy = HEX_H as f32 / 2.0;
    for y in 0..HEX_H {
        for x in 0..HEX_W {
            let dx = (x as f32 + 0.5 - cx).abs();
            let dy = (y as f32 + 0.5 - cy).abs();
            let px = if inside_hex(dx, dy, 0.0) {
                if inside_hex(dx, dy, 2.4) {
                    [255, 255, 255, 255]
                } else {
                    [74, 54, 30, 255]
                }
            } else {
                [0, 0, 0, 0]
            };
            let i = ((y * HEX_W + x) * 4) as usize;
            buf[i..i + 4].copy_from_slice(&px);
        }
    }
}

/// Pointy-top hex containment with an inset margin (exact cell math).
fn inside_hex(dx: f32, dy: f32, margin: f32) -> bool {
    let s = HEX_S - margin;
    if s <= 0.0 {
        return false;
    }
    dx <= 0.8660254 * s && dy <= s - dx / 1.7320508
}

/// Cells ordered from the warm brood center outward (egg innermost).
fn brood_order() -> [usize; CELLS] {
    let mut order = [0usize; CELLS];
    for (i, v) in order.iter_mut().enumerate() {
        *v = i;
    }
    let (cx, cy) = (COLS as f32 / 2.0 - 0.5, ROWS as f32 - 3.0);
    order.sort_by(|&a, &b| {
        let da = cell_dist(a, cx, cy);
        let db = cell_dist(b, cx, cy);
        da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
    });
    order
}

fn cell_dist(idx: usize, cx: f32, cy: f32) -> f32 {
    let x = (idx % COLS) as f32;
    let y = (idx / COLS) as f32;
    ((x - cx) * (x - cx) + (y - cy) * (y - cy) * 1.4).sqrt()
}

/// Which cells are built yet (exactly `cap` of them).
///
/// Two fronts grow together so both stories stay visible from day one:
/// - stores front: top-down rows (honey crown always has a roof),
/// - brood front: warm center outward (queen's eggs always have a bed).
/// Union of the two, trimmed to exactly `cap` cells.
pub fn built_mask(built: f32) -> [bool; CELLS] {
    let cap = (built.round() as usize).min(CELLS);
    let mut mask = [false; CELLS];
    if cap == 0 {
        return mask;
    }
    // Stores front: first 40% of the budget, top-down.
    let top_n = ((cap as f32 * 0.4).round() as usize).clamp(1, cap);
    for i in 0..top_n.min(CELLS) {
        mask[i] = true;
    }
    let mut added = mask.iter().filter(|&&b| b).count();
    // Brood front: center-out until the budget is spent.
    for &idx in brood_order().iter() {
        if added >= cap {
            break;
        }
        if !mask[idx] {
            mask[idx] = true;
            added += 1;
        }
    }
    mask
}
/// Convert colony stores/brood into display cells.
///
/// Stores take built cells top-down (honey crown, then pollen band);
/// brood takes built cells center-out and NEVER lands on a store cell,
/// so honey/pollen cells are barren by construction. Unbuilt cells
/// render as dark wax-to-come.
pub fn layout_cells(
    eggs: f32,
    larvae: f32,
    pupae: f32,
    honey: f32,
    pollen: f32,
    built: f32,
) -> [FrameCell; CELLS] {
    let mask = built_mask(built);
    let cap = mask.iter().filter(|&&b| b).count();
    let total = (eggs + larvae + pupae + honey + pollen).max(0.0);
    let scale = if total > cap as f32 && cap > 0 {
        cap as f32 / total
    } else {
        1.0
    };
    let mut remaining = cap;
    let take = |want: f32, remaining: &mut usize| -> usize {
        let n = ((want * scale).round() as usize).min(*remaining);
        *remaining -= n;
        n
    };
    let n_honey = take(honey, &mut remaining);
    let n_pollen = take(pollen, &mut remaining);
    let n_egg = take(eggs, &mut remaining);
    let n_larva = take(larvae, &mut remaining);
    let n_pupa = take(pupae, &mut remaining);
    let _ = remaining;

    let mut cells = [FrameCell::Unbuilt; CELLS];
    for (i, c) in cells.iter_mut().enumerate() {
        if mask[i] {
            *c = FrameCell::Empty;
        }
    }
    // Honey crown from the top, pollen band beneath (built cells only).
    let mut placed = 0;
    for i in 0..CELLS {
        if placed >= n_honey {
            break;
        }
        if mask[i] && cells[i] == FrameCell::Empty {
            cells[i] = FrameCell::Honey;
            placed += 1;
        }
    }
    placed = 0;
    for i in 0..CELLS {
        if placed >= n_pollen {
            break;
        }
        if mask[i] && cells[i] == FrameCell::Empty {
            cells[i] = FrameCell::Pollen;
            placed += 1;
        }
    }
    // Brood from the warm center outward (built + still empty only).
    let brood = brood_order();
    let mut fill_brood = |count: usize, kind: FrameCell| {
        let mut done = 0;
        for &idx in brood.iter() {
            if done >= count {
                break;
            }
            if mask[idx] && cells[idx] == FrameCell::Empty {
                cells[idx] = kind;
                done += 1;
            }
        }
    };
    fill_brood(n_egg, FrameCell::Egg);
    fill_brood(n_larva, FrameCell::Larva);
    fill_brood(n_pupa, FrameCell::Pupa);
    cells
}

/// Count cells: [Empty, Egg, Larva, Pupa, Honey, Pollen, Unbuilt].
pub fn count_cells(cells: &[FrameCell; CELLS]) -> [usize; 7] {
    let mut counts = [0usize; 7];
    for c in cells.iter() {
        counts[*c as usize] += 1;
    }
    counts
}

pub fn layout_for_colony(colony: &Colony) -> [FrameCell; CELLS] {
    layout_cells(
        colony.eggs,
        colony.larvae,
        colony.pupae,
        colony.honey,
        colony.pollen,
        colony.built,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_colony_all_empty() {
        let cells = layout_cells(0.0, 0.0, 0.0, 0.0, 0.0, 126.0);
        assert!(cells.iter().all(|c| *c == FrameCell::Empty));
    }

    #[test]
    fn test_unbuilt_stays_dark() {
        let cells = layout_cells(50.0, 50.0, 50.0, 50.0, 20.0, 20.0);
        let counts = count_cells(&cells);
        assert_eq!(counts[6], CELLS - 20);
        // Far corner stays unbuilt at small budgets.
        assert_eq!(cells[125], FrameCell::Unbuilt);
    }

    #[test]
    fn test_built_mask_exact_and_two_fronts() {
        // Exactly cap cells, and both fronts visible from the start.
        let mask = built_mask(40.0);
        assert_eq!(mask.iter().filter(|&&b| b).count(), 40);
        assert!(mask[0], "honey crown needs a roof from day one");
        let center = (ROWS - 3) * COLS + COLS / 2;
        assert!(mask[center], "brood center must be built from day one");
        let full = built_mask(126.0);
        assert!(full.iter().all(|&b| b));
        assert!(built_mask(0.0).iter().all(|&b| !b));
    }

    #[test]
    fn test_stores_and_brood_never_share() {
        // Flood everything: every built cell takes exactly one role.
        let cells = layout_cells(60.0, 60.0, 60.0, 60.0, 60.0, 40.0);
        let mask = built_mask(40.0);
        for (i, c) in cells.iter().enumerate() {
            if !mask[i] {
                assert_eq!(*c, FrameCell::Unbuilt);
            } else {
                assert_ne!(*c, FrameCell::Unbuilt);
            }
        }
        // Brood visible alongside stores even at starting budget.
        assert!(cells.contains(&FrameCell::Honey));
        assert!(
            cells.contains(&FrameCell::Egg)
                || cells.contains(&FrameCell::Larva)
                || cells.contains(&FrameCell::Pupa),
            "brood must show at 40 cells: {cells:?}"
        );
    }

    #[test]
    fn test_never_overflows_grid() {
        let cells = layout_cells(5000.0, 5000.0, 5000.0, 5000.0, 500.0, 126.0);
        assert_eq!(count_cells(&cells).iter().sum::<usize>(), CELLS);
    }

    #[test]
    fn test_honey_crowns_top_row() {
        let cells = layout_cells(0.0, 0.0, 0.0, 500.0, 0.0, 126.0);
        assert!(cells[..COLS].iter().all(|c| *c == FrameCell::Honey));
    }

    #[test]
    fn test_brood_nests_center() {
        let cells = layout_cells(300.0, 0.0, 0.0, 0.0, 0.0, 126.0);
        let center = (ROWS - 3) * COLS + COLS / 2;
        assert_eq!(cells[center], FrameCell::Egg);
    }

    #[test]
    fn test_grid_spacing_matches_sprite() {
        // Neighbor step must equal the rasterized hex pitch, or the comb
        // visibly stacks wrong.
        let step_x = cell_pos(1).x - cell_pos(0).x;
        assert!((step_x - 1.7320508 * HEX_S).abs() < 1e-4);
        let step_y = cell_pos(0).y - cell_pos(COLS).y;
        assert!((step_y - 1.5 * HEX_S).abs() < 1e-4);
        // Odd rows shift half a step (interlocking): compare column 0
        // across row 0 and row 1.
        let odd_shift = cell_pos(COLS).x - cell_pos(0).x;
        assert!((odd_shift - step_x * 0.5).abs() < 1e-3);
    }

    #[test]
    fn test_hex_sprite_is_symmetric_hexagon() {
        let mut buf = vec![0u8; (HEX_W * HEX_H * 4) as usize];
        paint_hex(&mut buf);
        let px = |x: u32, y: u32| -> [u8; 4] {
            let i = ((y * HEX_W + x) * 4) as usize;
            [buf[i], buf[i + 1], buf[i + 2], buf[i + 3]]
        };
        // Corners transparent, center filled, mirror symmetry.
        assert_eq!(px(0, 0)[3], 0);
        assert_eq!(px(HEX_W - 1, HEX_H - 1)[3], 0);
        assert_eq!(px(HEX_W / 2, HEX_H / 2), [255, 255, 255, 255]);
        for y in 0..HEX_H {
            for x in 0..HEX_W / 2 {
                assert_eq!(px(x, y), px(HEX_W - 1 - x, y), "asymmetric at {x},{y}");
            }
        }
        // Top vertex present, side middles present (pointy-top profile).
        // (Hex spans x 2.68..37.32 and y 2..42: outermost pixels are margin.)
        assert_ne!(px(HEX_W / 2, 2)[3], 0);
        assert_ne!(px(3, HEX_H / 2)[3], 0);
    }

    #[test]
    fn test_cell_tints_differ() {
        assert_ne!(FrameCell::Honey.tint(), FrameCell::Empty.tint());
        assert_ne!(FrameCell::Egg.tint(), FrameCell::Larva.tint());
    }
}
