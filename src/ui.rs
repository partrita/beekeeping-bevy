use bevy::prelude::*;
use crate::player::{PlayerInventory, Player};
use crate::market::MarketEconomy;
use crate::conservation::ConservationTracker;
use crate::world::CurrentIsland;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum ActiveUI {
    #[default]
    None,
    Beedex,
    Crafting,
    Centrifuge,
    Microscope,
    Market,
    Journal,
    Ferry,
    ApiaryInspection,
}

#[derive(Component)]
pub struct NotificationText;

#[derive(Component)]
pub struct HudCoinsText;

#[derive(Component)]
pub struct HudIslandText;

#[derive(Component)]
pub struct HudHotbarSlot(pub usize);

#[derive(Component)]
pub struct ModalRoot;

#[derive(Resource, Default)]
pub struct SelectedApiary(pub Option<Entity>);

#[derive(Resource, Default)]
pub struct GameNotification {
    pub message: String,
    pub timer: f32,
}

impl GameNotification {
    pub fn send(&mut self, msg: impl Into<String>) {
        self.message = msg.into();
        self.timer = 4.0;
    }
}

#[derive(Resource, Default)]
pub struct DiscoveredSpecies {
    pub discovered: Vec<String>,
}

impl DiscoveredSpecies {
    pub fn discover(&mut self, sp: &str) -> bool {
        if !self.discovered.contains(&sp.to_string()) {
            self.discovered.push(sp.to_string());
            true
        } else {
            false
        }
    }
}

#[derive(Component)]
pub struct HudHealthText;

pub fn setup_hud(
    mut commands: Commands,
    fonts: Res<crate::GameFonts>,
    pixel_assets: Res<crate::pixel_assets::GamePixelAssets>,
) {
    let kfont = |size: f32| TextFont {
        font: fonts.korean.clone(),
        font_size: size,
        ..default()
    };

    // Root container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(12.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 50.0),
        ))
        .with_children(|parent| {
            // Top HUD Bar
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(52.0),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::axes(Val::Px(16.0), Val::Px(6.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.1, 0.14, 0.12, 0.92)),
                    BorderColor(Color::srgb(0.85, 0.65, 0.2)),
                ))
                .with_children(|top| {
                    // Left: Island Name with Island Icon
                    top.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        ..default()
                    })
                    .with_children(|island_box| {
                        island_box.spawn((
                            ImageNode::new(pixel_assets.icons.island.clone()),
                            Node { width: Val::Px(22.0), height: Val::Px(22.0), ..default() },
                        ));
                        island_box.spawn((
                            Text::new("기장 포구 (Port Gijang)"),
                            kfont(17.0),
                            TextColor(Color::srgb(0.95, 0.9, 0.7)),
                            HudIslandText,
                        ));
                    });

                    // Center: Nav Shortcuts with Mini Icons
                    top.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(12.0),
                        ..default()
                    })
                    .with_children(|nav| {
                        let mut add_shortcut = |icon: Handle<Image>, label: &str| {
                            nav.spawn((
                                Node {
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    column_gap: Val::Px(4.0),
                                    padding: UiRect::axes(Val::Px(6.0), Val::Px(3.0)),
                                    border: UiRect::all(Val::Px(1.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0.15, 0.2, 0.18, 0.8)),
                                BorderColor(Color::srgb(0.4, 0.5, 0.45)),
                            ))
                            .with_children(|item| {
                                item.spawn((
                                    ImageNode::new(icon),
                                    Node { width: Val::Px(16.0), height: Val::Px(16.0), ..default() },
                                ));
                                item.spawn((
                                    Text::new(label),
                                    kfont(12.5),
                                    TextColor(Color::srgb(0.85, 0.92, 0.88)),
                                ));
                            });
                        };

                        add_shortcut(pixel_assets.icons.book.clone(), "도감[B]");
                        add_shortcut(pixel_assets.icons.craft.clone(), "제작[C]");
                        add_shortcut(pixel_assets.icons.gear.clone(), "채밀[M]");
                        add_shortcut(pixel_assets.icons.microscope.clone(), "현미경[G]");
                        add_shortcut(pixel_assets.icons.market.clone(), "상점[P]");
                        add_shortcut(pixel_assets.icons.exit.clone(), "종료[ESC]");
                    });

                    // Right: Coins & Island Health with Icons
                    top.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(14.0),
                        ..default()
                    })
                    .with_children(|stats| {
                        // Coin box
                        stats.spawn((
                            Node {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(5.0),
                                padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                                border: UiRect::all(Val::Px(1.5)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.2, 0.18, 0.1, 0.85)),
                            BorderColor(Color::srgb(0.9, 0.75, 0.2)),
                        ))
                        .with_children(|coin_box| {
                            coin_box.spawn((
                                ImageNode::new(pixel_assets.icons.coin.clone()),
                                Node { width: Val::Px(18.0), height: Val::Px(18.0), ..default() },
                            ));
                            coin_box.spawn((
                                Text::new("50"),
                                kfont(15.0),
                                TextColor(Color::srgb(1.0, 0.88, 0.3)),
                                HudCoinsText,
                            ));
                        });

                        // Health box
                        stats.spawn((
                            Node {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(5.0),
                                padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                                border: UiRect::all(Val::Px(1.5)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.1, 0.22, 0.14, 0.85)),
                            BorderColor(Color::srgb(0.3, 0.8, 0.4)),
                        ))
                        .with_children(|health_box| {
                            health_box.spawn((
                                ImageNode::new(pixel_assets.icons.leaf.clone()),
                                Node { width: Val::Px(18.0), height: Val::Px(18.0), ..default() },
                            ));
                            health_box.spawn((
                                Text::new("25%"),
                                kfont(15.0),
                                TextColor(Color::srgb(0.45, 0.95, 0.55)),
                                HudHealthText,
                            ));
                        });
                    });
                });

            // Center Notification Banner
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                })
                .with_children(|banner| {
                    banner.spawn((
                        Text::new("기장 포구에 오신 것을 환영합니다! [E] 상호작용, [1-8] 핫바, [ESC] 게임 종료"),
                        kfont(15.0),
                        TextColor(Color::srgb(0.95, 0.95, 0.8)),
                        NotificationText,
                    ));
                });

            // Bottom Hotbar
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(64.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.08, 0.1, 0.09, 0.85)),
                ))
                .with_children(|bar| {
                    for i in 0..8 {
                        bar.spawn((
                            Node {
                                width: Val::Px(110.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(1.5)),
                                padding: UiRect::all(Val::Px(4.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.18, 0.22, 0.2, 0.9)),
                            BorderColor(if i == 0 { Color::srgb(1.0, 0.8, 0.2) } else { Color::srgb(0.4, 0.45, 0.4) }),
                            HudHotbarSlot(i),
                        ))
                        .with_children(|slot| {
                            slot.spawn((
                                Text::new(format!("[{}] 빈 슬롯", i + 1)),
                                kfont(11.0),
                                TextColor(Color::WHITE),
                            ));
                        });
                    }
                });
        });
}

pub fn update_hud_elements(
    economy: Res<MarketEconomy>,
    island: Res<CurrentIsland>,
    conservation: Res<ConservationTracker>,
    notification: Res<GameNotification>,
    inventory: Res<PlayerInventory>,
    player_q: Query<&Player>,
    mut coins_text: Query<&mut Text, (With<HudCoinsText>, Without<HudIslandText>, Without<HudHealthText>, Without<NotificationText>)>,
    mut health_text: Query<&mut Text, (With<HudHealthText>, Without<HudCoinsText>, Without<HudIslandText>, Without<NotificationText>)>,
    mut island_text: Query<&mut Text, (With<HudIslandText>, Without<HudCoinsText>, Without<HudHealthText>, Without<NotificationText>)>,
    mut notify_text: Query<&mut Text, (With<NotificationText>, Without<HudCoinsText>, Without<HudHealthText>, Without<HudIslandText>)>,
    mut hotbar_slots: Query<(&HudHotbarSlot, &mut BorderColor, &Children)>,
    mut text_children: Query<&mut Text, (Without<HudCoinsText>, Without<HudHealthText>, Without<HudIslandText>, Without<NotificationText>)>,
) {
    let health = conservation.island_health.get(&island.biome).copied().unwrap_or(0.0);

    for mut text in coins_text.iter_mut() {
        text.0 = format!("{}", economy.player_coins);
    }

    for mut text in health_text.iter_mut() {
        text.0 = format!("{:.0}%", health);
    }

    for mut text in island_text.iter_mut() {
        text.0 = format!("{} (Gijang)", island.island_name);
    }

    for mut text in notify_text.iter_mut() {
        if !notification.message.is_empty() {
            text.0 = notification.message.clone();
        }
    }

    let active_slot = player_q.iter().next().map(|p| p.selected_slot).unwrap_or(0);

    for (slot_idx, mut border, children) in hotbar_slots.iter_mut() {
        if slot_idx.0 == active_slot {
            border.0 = Color::srgb(1.0, 0.85, 0.2);
        } else {
            border.0 = Color::srgb(0.35, 0.4, 0.38);
        }

        if let Some(&child) = children.first() {
            if let Ok(mut text) = text_children.get_mut(child) {
                if let Some(stack) = inventory.slots.get(slot_idx.0).and_then(|s| s.as_ref()) {
                    text.0 = format!("[{}] {} x{}", slot_idx.0 + 1, stack.item.display_name(), stack.count);
                } else {
                    text.0 = format!("[{}] 빈 슬롯", slot_idx.0 + 1);
                }
            }
        }
    }
}
