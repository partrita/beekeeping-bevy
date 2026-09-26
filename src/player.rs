use bevy::prelude::*;
use crate::items::{ItemStack, ItemType};

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub selected_slot: usize,
}

#[derive(Resource)]
pub struct PlayerInventory {
    pub slots: Vec<Option<ItemStack>>,
    pub max_slots: usize,
}

impl Default for PlayerInventory {
    fn default() -> Self {
        let mut slots = vec![None; 16];
        // Give starting gear: Smoker, Net, starter Queen & Drone, some empty frames
        slots[0] = Some(ItemStack::new(ItemType::BeekeeperSmoker, 1));
        slots[1] = Some(ItemStack::new(ItemType::ButterflyNet, 1));
        slots[2] = Some(ItemStack::new(ItemType::QueenBee("common".to_string()), 1));
        slots[3] = Some(ItemStack::new(ItemType::DroneBee("common".to_string()), 3));
        slots[4] = Some(ItemStack::new(ItemType::EmptyFrame, 4));
        slots[5] = Some(ItemStack::new(ItemType::FlowerSeed(crate::items::FlowerColor::Yellow), 5));
        slots[6] = Some(ItemStack::new(ItemType::FlowerSeed(crate::items::FlowerColor::Red), 5));
        slots[7] = Some(ItemStack::new(ItemType::ApiaryBox, 1));
        slots[8] = Some(ItemStack::new(ItemType::GlassJar, 5));
        slots[9] = Some(ItemStack::new(ItemType::Wood, 10));
        slots[10] = Some(ItemStack::new(ItemType::SugarSyrup, 3));
        slots[11] = Some(ItemStack::new(ItemType::PollenPatty, 2));

        Self {
            slots,
            max_slots: 16,
        }
    }
}

impl PlayerInventory {
    pub fn add_item(&mut self, item: ItemType, count: u32) -> bool {
        // Try to stack with existing
        for slot in self.slots.iter_mut() {
            if let Some(stack) = slot {
                if stack.item == item {
                    stack.count += count;
                    return true;
                }
            }
        }

        // Find empty slot
        for slot in self.slots.iter_mut() {
            if slot.is_none() {
                *slot = Some(ItemStack::new(item, count));
                return true;
            }
        }
        false
    }

    pub fn remove_item(&mut self, item: &ItemType, count: u32) -> bool {
        let mut remaining = count;
        for slot in self.slots.iter_mut() {
            if let Some(stack) = slot {
                if &stack.item == item {
                    if stack.count <= remaining {
                        remaining -= stack.count;
                        *slot = None;
                    } else {
                        stack.count -= remaining;
                        remaining = 0;
                    }
                    if remaining == 0 {
                        return true;
                    }
                }
            }
        }
        remaining == 0
    }

    pub fn count_item(&self, item: &ItemType) -> u32 {
        let mut total = 0;
        for slot in self.slots.iter().flatten() {
            if &slot.item == item {
                total += slot.count;
            }
        }
        total
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum PlayerFacing {
    #[default]
    Down,
    Up,
    Left,
    Right,
}

#[derive(Component)]
pub struct PlayerAnimation {
    pub facing: PlayerFacing,
    pub is_moving: bool,
    pub timer: Timer,
    pub cycle_step: usize,
}

impl Default for PlayerAnimation {
    fn default() -> Self {
        Self {
            facing: PlayerFacing::Down,
            is_moving: false,
            timer: Timer::from_seconds(0.12, TimerMode::Repeating),
            cycle_step: 0,
        }
    }
}

pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &mut Transform, &mut PlayerAnimation)>,
) {
    for (player, mut transform, mut anim) in query.iter_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
            transform.translation.x += direction.x * player.speed * time.delta_secs();
            transform.translation.y += direction.y * player.speed * time.delta_secs();

            // Island boundary clamping
            transform.translation.x = transform.translation.x.clamp(-460.0, 460.0);
            transform.translation.y = transform.translation.y.clamp(-340.0, 340.0);

            anim.is_moving = true;
            if direction.y.abs() > direction.x.abs() {
                if direction.y > 0.0 {
                    anim.facing = PlayerFacing::Up;
                } else {
                    anim.facing = PlayerFacing::Down;
                }
            } else {
                if direction.x > 0.0 {
                    anim.facing = PlayerFacing::Right;
                } else {
                    anim.facing = PlayerFacing::Left;
                }
            }

            anim.timer.tick(time.delta());
            if anim.timer.just_finished() {
                anim.cycle_step = (anim.cycle_step + 1) % 4;
            }
        } else {
            anim.is_moving = false;
            anim.cycle_step = 0;
        }
    }
}

pub fn update_player_sprite(
    pixel_assets: Res<crate::pixel_assets::GamePixelAssets>,
    mut query: Query<(&PlayerAnimation, &mut Sprite)>,
) {
    for (anim, mut sprite) in query.iter_mut() {
        // Walk cycle pattern: 0 (neutral) -> 1 (step 1) -> 2 (neutral) -> 3 (step 2)
        let frame_num = if anim.is_moving {
            match anim.cycle_step {
                1 => 1,
                3 => 2,
                _ => 0,
            }
        } else {
            0
        };

        let (image, flip_x) = match anim.facing {
            PlayerFacing::Down => {
                let img = match frame_num {
                    1 => &pixel_assets.player_sprites.down_step1,
                    2 => &pixel_assets.player_sprites.down_step2,
                    _ => &pixel_assets.player_sprites.down_idle,
                };
                (img.clone(), false)
            }
            PlayerFacing::Up => {
                let img = match frame_num {
                    1 => &pixel_assets.player_sprites.up_step1,
                    2 => &pixel_assets.player_sprites.up_step2,
                    _ => &pixel_assets.player_sprites.up_idle,
                };
                (img.clone(), false)
            }
            PlayerFacing::Right => {
                let img = match frame_num {
                    1 => &pixel_assets.player_sprites.side_step1,
                    2 => &pixel_assets.player_sprites.side_step2,
                    _ => &pixel_assets.player_sprites.side_idle,
                };
                (img.clone(), false)
            }
            PlayerFacing::Left => {
                let img = match frame_num {
                    1 => &pixel_assets.player_sprites.side_step1,
                    2 => &pixel_assets.player_sprites.side_step2,
                    _ => &pixel_assets.player_sprites.side_idle,
                };
                (img.clone(), true)
            }
        };

        if sprite.image != image {
            sprite.image = image;
        }
        if sprite.flip_x != flip_x {
            sprite.flip_x = flip_x;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_animation_default() {
        let anim = PlayerAnimation::default();
        assert_eq!(anim.facing, PlayerFacing::Down);
        assert!(!anim.is_moving);
        assert_eq!(anim.cycle_step, 0);
    }

    #[test]
    fn test_player_animation_cycle_progression() {
        let mut anim = PlayerAnimation::default();
        anim.is_moving = true;

        for step in 1..=4 {
            anim.timer.tick(std::time::Duration::from_millis(150));
            if anim.timer.just_finished() {
                anim.cycle_step = (anim.cycle_step + 1) % 4;
            }
            assert_eq!(anim.cycle_step, step % 4);
        }
    }

    #[test]
    fn test_noto_sans_font_exists_and_valid() {
        let font_path = std::path::Path::new("assets/fonts/NotoSansKR-Regular.ttf");
        assert!(font_path.exists(), "NotoSansKR-Regular.ttf font file must exist in assets/fonts/");
        let data = std::fs::read(font_path).expect("Failed to read font file");
        assert!(!data.is_empty(), "Font data must not be empty");
        assert!(data.len() > 1000);
    }
}
