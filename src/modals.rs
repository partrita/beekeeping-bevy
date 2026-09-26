use bevy::prelude::*;
use crate::bees::{BeeCatalog, BeeTier, BiomeType};
use crate::ui::{ActiveUI, ModalRoot, DiscoveredSpecies};
use crate::crafting::CraftingRegistry;
use crate::market::MarketEconomy;
use crate::conservation::ConservationTracker;
use crate::minigames::{CentrifugeMinigame, MicroscopeMinigame};
use crate::player::PlayerInventory;
use crate::world::CurrentIsland;
use crate::apiary::Apiary;
use crate::ui::SelectedApiary;

pub fn open_or_close_modals(
    keyboard: Res<ButtonInput<KeyCode>>,
    active_ui: Res<State<ActiveUI>>,
    mut next_ui: ResMut<NextState<ActiveUI>>,
    mut app_exit: EventWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if *active_ui.get() != ActiveUI::None {
            next_ui.set(ActiveUI::None);
        } else {
            app_exit.send(AppExit::Success);
        }
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyQ) && (keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::SuperLeft)) {
        app_exit.send(AppExit::Success);
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyB) {
        if *active_ui.get() == ActiveUI::Beedex {
            next_ui.set(ActiveUI::None);
        } else {
            next_ui.set(ActiveUI::Beedex);
        }
    } else if keyboard.just_pressed(KeyCode::KeyC) {
        if *active_ui.get() == ActiveUI::Crafting {
            next_ui.set(ActiveUI::None);
        } else {
            next_ui.set(ActiveUI::Crafting);
        }
    } else if keyboard.just_pressed(KeyCode::KeyM) {
        if *active_ui.get() == ActiveUI::Centrifuge {
            next_ui.set(ActiveUI::None);
        } else {
            next_ui.set(ActiveUI::Centrifuge);
        }
    } else if keyboard.just_pressed(KeyCode::KeyG) {
        if *active_ui.get() == ActiveUI::Microscope {
            next_ui.set(ActiveUI::None);
        } else {
            next_ui.set(ActiveUI::Microscope);
        }
    } else if keyboard.just_pressed(KeyCode::KeyJ) {
        if *active_ui.get() == ActiveUI::Journal {
            next_ui.set(ActiveUI::None);
        } else {
            next_ui.set(ActiveUI::Journal);
        }
    } else if keyboard.just_pressed(KeyCode::KeyP) {
        if *active_ui.get() == ActiveUI::Market {
            next_ui.set(ActiveUI::None);
        } else {
            next_ui.set(ActiveUI::Market);
        }
    }
}

pub fn cleanup_modal(
    mut commands: Commands,
    query: Query<Entity, With<ModalRoot>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// -------------------------------------------------------------
// BEEDEX (COMPENDIUM) UI
// -------------------------------------------------------------
pub fn spawn_beedex_modal(
    mut commands: Commands,
    catalog: Res<BeeCatalog>,
    discovered: Res<DiscoveredSpecies>,
) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(10.0),
                top: Val::Percent(8.0),
                width: Val::Percent(80.0),
                height: Val::Percent(84.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(3.0)),
                row_gap: Val::Px(12.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.08, 0.12, 0.1, 0.96)),
            BorderColor(Color::srgb(0.9, 0.75, 0.2)),
            ModalRoot,
        ))
        .with_children(|parent| {
            // Header
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|header| {
                    header.spawn((
                        Text::new("비덱스 도감 (Beedex: Gijang Islands Compendium)"),
                        TextFont { font_size: 24.0, ..default() },
                        TextColor(Color::srgb(1.0, 0.85, 0.3)),
                    ));
                    header.spawn((
                        Text::new(format!("발견: {} / 46종 ([Esc] 닫기)", discovered.discovered.len())),
                        TextFont { font_size: 15.0, ..default() },
                        TextColor(Color::srgb(0.8, 0.9, 0.85)),
                    ));
                });

            // Scrollable / grid listing of bees
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    overflow: Overflow::clip_y(),
                    ..default()
                })
                .with_children(|list| {
                    // Show a rich preview of discovered & discoverable species
                    for (id, sp) in catalog.species.iter() {
                        let is_known = discovered.discovered.contains(id);
                        let tier_str = match sp.tier {
                            BeeTier::Tier1Wild => "Tier 1 Wild",
                            BeeTier::Tier2Hybrid => "Tier 2 Hybrid",
                            BeeTier::Tier3Refined => "Tier 3 Refined",
                            BeeTier::Tier4Exotic => "Tier 4 Exotic",
                            BeeTier::Tier5Mythic => "Tier 5 Mythic",
                            BeeTier::Solitary => "Solitary Pollinator",
                        };

                        list.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                border: UiRect::all(Val::Px(1.0)),
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(if is_known {
                                Color::srgba(0.14, 0.2, 0.16, 0.9)
                            } else {
                                Color::srgba(0.1, 0.1, 0.1, 0.7)
                            }),
                            BorderColor(if is_known {
                                Color::srgb(sp.color_primary[0], sp.color_primary[1], sp.color_primary[2])
                            } else {
                                Color::srgb(0.3, 0.3, 0.3)
                            }),
                        ))
                        .with_children(|row| {
                            if is_known {
                                row.spawn((
                                    Text::new(format!("[O] {} ({}) - {}", sp.common_name, sp.latin_name, tier_str)),
                                    TextFont { font_size: 14.0, ..default() },
                                    TextColor(Color::srgb(1.0, 0.9, 0.4)),
                                ));
                                row.spawn((
                                    Text::new(format!("꿀: {} | 수명: {:.0}s | 속도: {:.1}s | 번식력: {}", sp.honey_type, sp.base_lifespan, sp.base_speed, sp.fertility)),
                                    TextFont { font_size: 13.0, ..default() },
                                    TextColor(Color::srgb(0.7, 0.85, 0.7)),
                                ));
                            } else {
                                row.spawn((
                                    Text::new(format!("[?] ??? [{}] - 미발견 꿀벌 종", tier_str)),
                                    TextFont { font_size: 14.0, ..default() },
                                    TextColor(Color::srgb(0.5, 0.5, 0.5)),
                                ));
                                row.spawn((
                                    Text::new("Cross-breed parent species in an apiary to discover!"),
                                    TextFont { font_size: 12.0, ..default() },
                                    TextColor(Color::srgb(0.4, 0.5, 0.4)),
                                ));
                            }
                        });
                    }
                });
        });
}

// -------------------------------------------------------------
// CRAFTING WORKBENCH UI
// -------------------------------------------------------------
pub fn spawn_crafting_modal(
    mut commands: Commands,
    crafting: Res<CraftingRegistry>,
    inventory: Res<PlayerInventory>,
) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(15.0),
                top: Val::Percent(12.0),
                width: Val::Percent(70.0),
                height: Val::Percent(76.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(3.0)),
                row_gap: Val::Px(12.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.12, 0.1, 0.08, 0.96)),
            BorderColor(Color::srgb(0.85, 0.6, 0.25)),
            ModalRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                })
                .with_children(|header| {
                    header.spawn((
                        Text::new("기장 공방 제작대 (Workbench: [1-9] 제작)"),
                        TextFont { font_size: 22.0, ..default() },
                        TextColor(Color::srgb(1.0, 0.8, 0.3)),
                    ));
                    header.spawn((
                        Text::new("[Esc] 닫기"),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));
                });

            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    overflow: Overflow::clip_y(),
                    ..default()
                })
                .with_children(|list| {
                    for (idx, recipe) in crafting.recipes.iter().enumerate() {
                        let can_craft = recipe.inputs.iter().all(|input| {
                            inventory.count_item(&input.item) >= input.count
                        });

                        let cost_str = recipe.inputs.iter().map(|input| {
                            format!("{} x{}", input.item.display_name(), input.count)
                        }).collect::<Vec<_>>().join(", ");

                        list.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                border: UiRect::all(Val::Px(1.5)),
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(if can_craft {
                                Color::srgba(0.2, 0.25, 0.18, 0.9)
                            } else {
                                Color::srgba(0.15, 0.12, 0.12, 0.7)
                            }),
                            BorderColor(if can_craft {
                                Color::srgb(0.5, 0.8, 0.3)
                            } else {
                                Color::srgb(0.4, 0.3, 0.3)
                            }),
                        ))
                        .with_children(|row| {
                            row.spawn((
                                Text::new(format!("[{}] {} -> Yields: {} x{}", idx + 1, recipe.name, recipe.output.item.display_name(), recipe.output.count)),
                                TextFont { font_size: 14.0, ..default() },
                                TextColor(if can_craft { Color::WHITE } else { Color::srgb(0.6, 0.6, 0.6) }),
                            ));
                            row.spawn((
                                Text::new(format!("Cost: {}", cost_str)),
                                TextFont { font_size: 12.0, ..default() },
                                TextColor(if can_craft { Color::srgb(0.4, 0.9, 0.4) } else { Color::srgb(0.9, 0.4, 0.4) }),
                            ));
                        });
                    }
                });
        });
}

// -------------------------------------------------------------
// CENTRIFUGE HONEY EXTRACTOR MINIGAME UI
// -------------------------------------------------------------
pub fn spawn_centrifuge_modal(
    mut commands: Commands,
    minigame: Res<CentrifugeMinigame>,
) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(20.0),
                top: Val::Percent(15.0),
                width: Val::Percent(60.0),
                height: Val::Percent(70.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(24.0)),
                border: UiRect::all(Val::Px(3.0)),
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.12, 0.14, 0.96)),
            BorderColor(Color::srgb(0.3, 0.7, 0.9)),
            ModalRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("원심분리 채밀기 (Honey Centrifuge Extractor)"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::srgb(0.4, 0.85, 1.0)),
            ));

            parent.spawn((
                Text::new("[SPACEBAR]를 눌러 크랭크를 회전시키세요. 바늘을 녹색 구간(55-80 RPM)에 3.5초 유지!"),
                TextFont { font_size: 14.0, ..default() },
                TextColor(Color::srgb(0.85, 0.9, 0.95)),
            ));

            // Extractor RPM Gauge bar
            parent
                .spawn((
                    Node {
                        width: Val::Percent(80.0),
                        height: Val::Px(40.0),
                        border: UiRect::all(Val::Px(2.0)),
                        padding: UiRect::all(Val::Px(4.0)),
                        justify_content: JustifyContent::FlexStart,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.05, 0.05, 0.08)),
                    BorderColor(Color::srgb(0.4, 0.5, 0.6)),
                ))
                .with_children(|gauge| {
                    gauge.spawn((
                        Node {
                            width: Val::Percent(minigame.current_rpm),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(
                            if minigame.current_rpm >= minigame.target_min && minigame.current_rpm <= minigame.target_max {
                                Color::srgb(0.2, 0.9, 0.3)
                            } else {
                                Color::srgb(0.9, 0.3, 0.2)
                            }
                        ),
                    ));
                });

            parent.spawn((
                Text::new(format!("현재 RPM: {:.1} / 100.0 | 채밀 진행도: {:.1}s / {:.1}s", minigame.current_rpm, minigame.sweet_spot_timer, minigame.required_time)),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::srgb(1.0, 0.85, 0.3)),
            ));

            if minigame.success {
                parent.spawn((
                    Text::new("[성공] 채밀 완료! 벌꿀 단지와 밀랍을 추출했습니다! [ESC]를 눌러 수령하세요."),
                    TextFont { font_size: 16.0, ..default() },
                    TextColor(Color::srgb(0.3, 1.0, 0.5)),
                ));
            } else {
                parent.spawn((
                    Text::new("[SPACEBAR]를 적절히 펌프질하세요! 너무 빠르거나 느리면 안 됩니다."),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                ));
            }
        });
}

// -------------------------------------------------------------
// GENETICS MICROSCOPE MINIGAME UI
// -------------------------------------------------------------
pub fn spawn_microscope_modal(
    mut commands: Commands,
    microscope: Res<MicroscopeMinigame>,
) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(20.0),
                top: Val::Percent(15.0),
                width: Val::Percent(60.0),
                height: Val::Percent(70.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(24.0)),
                border: UiRect::all(Val::Px(3.0)),
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.09, 0.1, 0.15, 0.96)),
            BorderColor(Color::srgb(0.6, 0.4, 0.9)),
            ModalRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("유전 연구 현미경 (Genetics Microscope)"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::srgb(0.8, 0.6, 1.0)),
            ));

            parent.spawn((
                Text::new("[A]/[D]로 초점 조절, [W]/[S]로 배율 조절. 렌즈를 정렬하여 열성 유전 형질을 해독하세요!"),
                TextFont { font_size: 14.0, ..default() },
                TextColor(Color::srgb(0.85, 0.9, 0.95)),
            ));

            parent.spawn((
                Text::new(format!("초점: {:.0}% (목표 ~65%) | 배율: {:.0}% (목표 ~50%)", microscope.focus_dial, microscope.magnification_dial)),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::srgb(1.0, 0.85, 0.3)),
            ));

            let is_aligned = microscope.check_alignment();

            if is_aligned {
                parent.spawn((
                    Text::new("[성공] 염색체 정렬 완료! 새로운 열성 유전 형질을 발견했습니다! [ESC]로 종료."),
                    TextFont { font_size: 16.0, ..default() },
                    TextColor(Color::srgb(0.3, 1.0, 0.6)),
                ));
            } else {
                parent.spawn((
                    Text::new("광학 선명도가 100%가 될 때까지 다이얼을 미세 조정하세요."),
                    TextFont { font_size: 14.0, ..default() },
                    TextColor(Color::srgb(0.7, 0.7, 0.7)),
                ));
            }
        });
}

// -------------------------------------------------------------
// PORT MARKET STALL UI
// -------------------------------------------------------------
pub fn spawn_market_modal(
    mut commands: Commands,
    economy: Res<MarketEconomy>,
    _inventory: Res<PlayerInventory>,
) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(15.0),
                top: Val::Percent(10.0),
                width: Val::Percent(70.0),
                height: Val::Percent(80.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(3.0)),
                row_gap: Val::Px(12.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.14, 0.12, 0.08, 0.96)),
            BorderColor(Color::srgb(1.0, 0.8, 0.2)),
            ModalRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                })
                .with_children(|header| {
                    header.spawn((
                        Text::new("기장 포구 장터 (Market Stall)"),
                        TextFont { font_size: 24.0, ..default() },
                        TextColor(Color::srgb(1.0, 0.85, 0.3)),
                    ));
                    header.spawn((
                        Text::new(format!("보유 코인: {} 코인 ([S] 빠른 판매 | [Esc] 닫기)", economy.player_coins)),
                        TextFont { font_size: 15.0, ..default() },
                        TextColor(Color::srgb(0.9, 0.9, 0.8)),
                    ));
                });

            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    overflow: Overflow::clip_y(),
                    ..default()
                })
                .with_children(|list| {
                    for (i, entry) in economy.shop_items.iter().enumerate() {
                        let can_afford = economy.player_coins >= entry.price;
                        list.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                border: UiRect::all(Val::Px(1.5)),
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.2, 0.18, 0.14, 0.9)),
                            BorderColor(if can_afford { Color::srgb(0.8, 0.7, 0.2) } else { Color::srgb(0.4, 0.3, 0.3) }),
                        ))
                        .with_children(|row| {
                            row.spawn((
                                Text::new(format!("[{}] {} - {}", i + 1, entry.name, entry.description)),
                                TextFont { font_size: 13.0, ..default() },
                                TextColor(Color::WHITE),
                            ));
                            row.spawn((
                                Text::new(format!("가격: {} 코인", entry.price)),
                                TextFont { font_size: 14.0, ..default() },
                                TextColor(if can_afford { Color::srgb(1.0, 0.85, 0.3) } else { Color::srgb(0.8, 0.3, 0.3) }),
                            ));
                        });
                    }
                });
        });
}

// -------------------------------------------------------------
// GRANDFATHER'S LORE JOURNAL UI
// -------------------------------------------------------------
pub fn spawn_journal_modal(
    mut commands: Commands,
    conservation: Res<ConservationTracker>,
) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(18.0),
                top: Val::Percent(12.0),
                width: Val::Percent(64.0),
                height: Val::Percent(76.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(24.0)),
                border: UiRect::all(Val::Px(3.0)),
                row_gap: Val::Px(14.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.12, 0.1, 0.08, 0.97)),
            BorderColor(Color::srgb(0.8, 0.65, 0.3)),
            ModalRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                })
                .with_children(|header| {
                    header.spawn((
                        Text::new("할아버지의 양봉 일지 (Field Journal)"),
                        TextFont { font_size: 24.0, ..default() },
                        TextColor(Color::srgb(0.95, 0.8, 0.4)),
                    ));
                    header.spawn((
                        Text::new("Press [Esc] to Close"),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
                });

            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    overflow: Overflow::clip_y(),
                    ..default()
                })
                .with_children(|list| {
                    for entry in &conservation.lore_entries {
                        list.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                padding: UiRect::all(Val::Px(12.0)),
                                border: UiRect::all(Val::Px(1.0)),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(4.0),
                                ..default()
                            },
                            BackgroundColor(if entry.unlocked {
                                Color::srgba(0.2, 0.17, 0.12, 0.95)
                            } else {
                                Color::srgba(0.1, 0.1, 0.1, 0.6)
                            }),
                            BorderColor(if entry.unlocked { Color::srgb(0.85, 0.65, 0.25) } else { Color::srgb(0.3, 0.3, 0.3) }),
                        ))
                        .with_children(|item| {
                            if entry.unlocked {
                                item.spawn((
                                    Text::new(entry.title),
                                    TextFont { font_size: 16.0, ..default() },
                                    TextColor(Color::srgb(1.0, 0.85, 0.3)),
                                ));
                                item.spawn((
                                    Text::new(entry.content),
                                    TextFont { font_size: 13.0, ..default() },
                                    TextColor(Color::srgb(0.9, 0.88, 0.82)),
                                ));
                            } else {
                                item.spawn((
                                    Text::new(format!("[잠김] 일지 기록 - 생태 복원도 100% 달성 시 해독 가능 ({:?})", entry.biome)),
                                    TextFont { font_size: 14.0, ..default() },
                                    TextColor(Color::srgb(0.5, 0.5, 0.5)),
                                ));
                            }
                        });
                    }
                });
        });
}

// -------------------------------------------------------------
// FERRY BOAT FAST TRAVEL UI
// -------------------------------------------------------------
pub fn spawn_ferry_modal(
    mut commands: Commands,
    current: Res<CurrentIsland>,
) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(25.0),
                top: Val::Percent(20.0),
                width: Val::Percent(50.0),
                height: Val::Percent(60.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(24.0)),
                border: UiRect::all(Val::Px(3.0)),
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.08, 0.14, 0.18, 0.96)),
            BorderColor(Color::srgb(0.2, 0.7, 0.9)),
            ModalRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("기장 군도 여객선 (Ferry Passage)"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::srgb(0.4, 0.8, 1.0)),
            ));

            parent.spawn((
                Text::new(format!("Current Port: {} - Press [1-6] to sail to island!", current.island_name)),
                TextFont { font_size: 14.0, ..default() },
                TextColor(Color::WHITE),
            ));

            let destinations = [
                ("1. Port Gijang (Home Dock)", BiomeType::PortHoneyBeaFather),
                ("2. Emerald Meadows (Wildflowers & Mason Bees)", BiomeType::EmeraldMeadows),
                ("3. Whisperwood Forest (Ancient Trees & Canopy)", BiomeType::WhisperwoodForest),
                ("4. Sunken Shore (Tidepools & Coastal Bees)", BiomeType::SunkenShore),
                ("5. Mangrove Marsh (Wetlands & Orchid Bees)", BiomeType::MangroveMarsh),
                ("6. Shimmering Peaks (Alpine Caverns & Cosmic Bees)", BiomeType::ShimmeringPeaks),
            ];

            for (desc, _) in &destinations {
                parent.spawn((
                    Text::new(*desc),
                    TextFont { font_size: 15.0, ..default() },
                    TextColor(Color::srgb(0.85, 0.95, 0.9)),
                ));
            }
        });
}

// -------------------------------------------------------------
// REALISTIC APIARY INSPECTION (내검, 증소, 분봉열, 사양꿀 급여)
// -------------------------------------------------------------
#[derive(Component)]
pub struct InspectionQueenText;

#[derive(Component)]
pub struct InspectionColonyText;

#[derive(Component)]
pub struct InspectionFeverText;

#[derive(Component)]
pub struct InspectionFeederText;

#[derive(Component)]
pub struct InspectionFloraText;

#[derive(Component)]
pub struct InspectionCombsText;

pub fn spawn_apiary_inspection_modal(
    mut commands: Commands,
    fonts: Res<crate::GameFonts>,
    pixel_assets: Res<crate::pixel_assets::GamePixelAssets>,
) {
    let kfont = |sz: f32| TextFont {
        font: fonts.korean.clone(),
        font_size: sz,
        ..default()
    };

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(10.0),
                top: Val::Percent(6.0),
                width: Val::Percent(80.0),
                height: Val::Percent(88.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(18.0)),
                border: UiRect::all(Val::Px(3.0)),
                row_gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.08, 0.12, 0.1, 0.97)),
            BorderColor(Color::srgb(0.9, 0.75, 0.2)),
            ModalRoot,
        ))
        .with_children(|parent| {
            // Header
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|hdr| {
                    hdr.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(8.0),
                        ..default()
                    }).with_children(|title_box| {
                        title_box.spawn((
                            ImageNode::new(pixel_assets.icons.bee.clone()),
                            Node { width: Val::Px(24.0), height: Val::Px(24.0), ..default() },
                        ));
                        title_box.spawn((
                            Text::new("벌통 정밀 내검 & 사양 관리"),
                            kfont(22.0),
                            TextColor(Color::srgb(1.0, 0.85, 0.2)),
                        ));
                    });

                    hdr.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(6.0),
                        padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    }).with_children(|close_box| {
                        close_box.spawn((
                            ImageNode::new(pixel_assets.icons.exit.clone()),
                            Node { width: Val::Px(16.0), height: Val::Px(16.0), ..default() },
                        ));
                        close_box.spawn((
                            Text::new("[ESC] 내검 닫기"),
                            kfont(13.0),
                            TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        ));
                    });
                });

            // Subtitle
            parent.spawn((
                Text::new("기장 군도 양봉 관리: 소비장 증소/축소, 분봉열 왕대 제거, 사양 관리, 밀원 채밀"),
                kfont(12.5),
                TextColor(Color::srgb(0.7, 0.9, 0.8)),
            ));

            // Content Split (2 columns)
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(66.0),
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(14.0),
                    ..default()
                })
                .with_children(|cols| {
                    // Left Column: 벌통 군세, 내검, 사양기
                    cols.spawn((
                        Node {
                            width: Val::Percent(50.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(12.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            row_gap: Val::Px(8.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.12, 0.16, 0.14, 0.92)),
                        BorderColor(Color::srgb(0.4, 0.6, 0.5)),
                    ))
                    .with_children(|col1| {
                        // Title
                        col1.spawn(Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(6.0),
                            ..default()
                        }).with_children(|t| {
                            t.spawn((
                                ImageNode::new(pixel_assets.icons.frame.clone()),
                                Node { width: Val::Px(18.0), height: Val::Px(18.0), ..default() },
                            ));
                            t.spawn((
                                Text::new("군세 & 내검 상태"),
                                kfont(14.5),
                                TextColor(Color::srgb(0.9, 0.8, 0.3)),
                            ));
                        });

                        // Queen
                        col1.spawn(Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::FlexStart,
                            column_gap: Val::Px(6.0),
                            padding: UiRect::all(Val::Px(6.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        }).with_children(|card| {
                            card.spawn((
                                ImageNode::new(pixel_assets.icons.crown.clone()),
                                Node { width: Val::Px(16.0), height: Val::Px(16.0), margin: UiRect::top(Val::Px(2.0)), ..default() },
                            ));
                            card.spawn((
                                Text::new("여왕벌 정보 불러오는 중..."),
                                kfont(12.0),
                                TextColor(Color::WHITE),
                                InspectionQueenText,
                            ));
                        });

                        // Colony
                        col1.spawn(Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::FlexStart,
                            column_gap: Val::Px(6.0),
                            padding: UiRect::all(Val::Px(6.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        }).with_children(|card| {
                            card.spawn((
                                ImageNode::new(pixel_assets.icons.bee.clone()),
                                Node { width: Val::Px(16.0), height: Val::Px(16.0), margin: UiRect::top(Val::Px(2.0)), ..default() },
                            ));
                            card.spawn((
                                Text::new("벌 군세 계산 중..."),
                                kfont(12.0),
                                TextColor(Color::srgb(0.85, 0.92, 0.85)),
                                InspectionColonyText,
                            ));
                        });

                        // Swarm fever
                        col1.spawn(Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::FlexStart,
                            column_gap: Val::Px(6.0),
                            padding: UiRect::all(Val::Px(6.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        }).with_children(|card| {
                            card.spawn((
                                ImageNode::new(pixel_assets.icons.warning.clone()),
                                Node { width: Val::Px(16.0), height: Val::Px(16.0), margin: UiRect::top(Val::Px(2.0)), ..default() },
                            ));
                            card.spawn((
                                Text::new("분봉열 및 왕대 점검 중..."),
                                kfont(12.0),
                                TextColor(Color::srgb(1.0, 0.65, 0.5)),
                                InspectionFeverText,
                            ));
                        });

                        // Feeder
                        col1.spawn(Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::FlexStart,
                            column_gap: Val::Px(6.0),
                            padding: UiRect::all(Val::Px(6.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        }).with_children(|card| {
                            card.spawn((
                                ImageNode::new(pixel_assets.icons.feeder.clone()),
                                Node { width: Val::Px(16.0), height: Val::Px(16.0), margin: UiRect::top(Val::Px(2.0)), ..default() },
                            ));
                            card.spawn((
                                Text::new("사양기 및 화분떡 상태 확인 중..."),
                                kfont(12.0),
                                TextColor(Color::srgb(1.0, 0.88, 0.4)),
                                InspectionFeederText,
                            ));
                        });
                    });

                    // Right Column: 밀원 꽃밭 및 채밀 상태
                    cols.spawn((
                        Node {
                            width: Val::Percent(50.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(12.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            row_gap: Val::Px(8.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.12, 0.16, 0.14, 0.92)),
                        BorderColor(Color::srgb(0.4, 0.6, 0.5)),
                    ))
                    .with_children(|col2| {
                        // Title
                        col2.spawn(Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(6.0),
                            ..default()
                        }).with_children(|t| {
                            t.spawn((
                                ImageNode::new(pixel_assets.icons.flower.clone()),
                                Node { width: Val::Px(18.0), height: Val::Px(18.0), ..default() },
                            ));
                            t.spawn((
                                Text::new("밀원 꽃밭 & 벌꿀 수확"),
                                kfont(14.5),
                                TextColor(Color::srgb(0.4, 0.8, 0.9)),
                            ));
                        });

                        // Flora Card
                        col2.spawn(Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::FlexStart,
                            column_gap: Val::Px(6.0),
                            padding: UiRect::all(Val::Px(6.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        }).with_children(|card| {
                            card.spawn((
                                ImageNode::new(pixel_assets.icons.flower.clone()),
                                Node { width: Val::Px(16.0), height: Val::Px(16.0), margin: UiRect::top(Val::Px(2.0)), ..default() },
                            ));
                            card.spawn((
                                Text::new("밀원 꽃밭 탐색 중..."),
                                kfont(12.0),
                                TextColor(Color::srgb(0.9, 0.95, 0.9)),
                                InspectionFloraText,
                            ));
                        });

                        // Combs Card
                        col2.spawn(Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::FlexStart,
                            column_gap: Val::Px(6.0),
                            padding: UiRect::all(Val::Px(6.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        }).with_children(|card| {
                            card.spawn((
                                ImageNode::new(pixel_assets.icons.honey.clone()),
                                Node { width: Val::Px(16.0), height: Val::Px(16.0), margin: UiRect::top(Val::Px(2.0)), ..default() },
                            ));
                            card.spawn((
                                Text::new("저장된 꿀 소비 확인 중..."),
                                kfont(12.0),
                                TextColor(Color::srgb(1.0, 0.9, 0.6)),
                                InspectionCombsText,
                            ));
                        });

                        // Concise Tip Badge
                        col2.spawn(Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(6.0),
                            padding: UiRect::axes(Val::Px(8.0), Val::Px(6.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        }).with_children(|tip| {
                            tip.spawn((
                                ImageNode::new(pixel_assets.icons.warning.clone()),
                                Node { width: Val::Px(16.0), height: Val::Px(16.0), ..default() },
                            ));
                            tip.spawn((
                                Text::new("과소비(보온 실패)는 [D] 축소로, 과밀(분봉열)은 [F] 증소로 해결하세요!"),
                                kfont(11.5),
                                TextColor(Color::srgb(0.8, 0.9, 0.85)),
                            ));
                        });
                    });
                });

            // Action Panel (Bottom Buttons with Icons)
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceAround,
                        padding: UiRect::axes(Val::Px(6.0), Val::Px(6.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.1, 0.14, 0.12, 0.95)),
                    BorderColor(Color::srgb(0.9, 0.75, 0.2)),
                ))
                .with_children(|actions| {
                    let mut make_btn = |icon: Handle<Image>, key: &str, desc: &str, col: Color| {
                        actions.spawn((
                            Node {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(5.0),
                                padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                                border: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.15, 0.2, 0.18, 0.9)),
                            BorderColor(col),
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                ImageNode::new(icon),
                                Node { width: Val::Px(18.0), height: Val::Px(18.0), ..default() },
                            ));
                            btn.spawn((
                                Text::new(format!("{}\n{}", key, desc)),
                                kfont(11.0),
                                TextColor(col),
                            ));
                        });
                    };

                    make_btn(pixel_assets.icons.frame.clone(), "[F] 증소", "소비 +1", Color::srgb(0.4, 0.9, 0.5));
                    make_btn(pixel_assets.icons.frame.clone(), "[D] 축소", "소비 -1", Color::srgb(0.9, 0.6, 0.4));
                    make_btn(pixel_assets.icons.crown.clone(), "[R] 왕대제거", "분봉예방", Color::srgb(1.0, 0.75, 0.3));
                    make_btn(pixel_assets.icons.feeder.clone(), "[S] 사양액", "설탕시럽", Color::srgb(1.0, 0.85, 0.4));
                    make_btn(pixel_assets.icons.patty.clone(), "[P] 화분떡", "산란가속", Color::srgb(1.0, 0.6, 0.8));
                    make_btn(pixel_assets.icons.honey.clone(), "[H] 채밀", "벌집수확", Color::srgb(1.0, 0.9, 0.4));
                    make_btn(pixel_assets.icons.bee.clone(), "[1-8] 벌투입", "여왕/수벌", Color::srgb(0.6, 0.8, 1.0));
                });
        });
}

pub fn update_inspection_modal(
    selected_apiary: Res<SelectedApiary>,
    apiaries: Query<&Apiary>,
    mut texts: Query<(
        &mut Text,
        Option<&InspectionQueenText>,
        Option<&InspectionColonyText>,
        Option<&InspectionFeverText>,
        Option<&InspectionFeederText>,
        Option<&InspectionFloraText>,
        Option<&InspectionCombsText>,
    )>,
) {
    let Some(apiary_ent) = selected_apiary.0 else { return; };
    let Ok(apiary) = apiaries.get(apiary_ent) else { return; };

    for (mut text, q_opt, c_opt, fv_opt, fd_opt, fl_opt, cb_opt) in texts.iter_mut() {
        if q_opt.is_some() {
            if let Some(genome) = &apiary.queen_genome {
                let pct = (apiary.life_timer / apiary.max_life * 100.0).clamp(0.0, 100.0);
                **text = format!(
                    "여왕: [{}] (교배: [{}]) | 수명: {:.0}% | 산란: +15/초",
                    genome.primary_species,
                    genome.secondary_species,
                    pct,
                );
            } else if let Some(q_slot) = &apiary.queen_slot {
                **text = format!("여왕실: {} (수벌 교배 대기 중)", q_slot.item.display_name());
            } else {
                **text = "여왕실: 비어있음 (핫바[1-8]에서 여왕벌 투입)".to_string();
            }
        } else if c_opt.is_some() {
            let cap = apiary.frame_capacity();
            let status = if apiary.is_overcrowded() {
                format!("[경고: 과밀! +{}마리 -> F 증소 필요]", apiary.bee_population.saturating_sub(cap))
            } else if apiary.is_overframed() {
                format!("[경고: 과소비! 보온 -{:.0}% -> D 축소 필요]", (1.0 - apiary.thermal_efficiency()) * 100.0)
            } else {
                format!("[정상 착봉: 보온 100% (밀도 {:.0}마리/장)]", apiary.bees_per_frame())
            };

            **text = format!(
                "일벌: {}마리 (한도 {}마리) | 소비장: {}/{}장\n착봉 밀도: {:.0}마리/장 | 보온 효율: {:.0}%\n상태: {}",
                apiary.bee_population,
                cap,
                apiary.frame_count,
                apiary.max_frames,
                apiary.bees_per_frame(),
                apiary.thermal_efficiency() * 100.0,
                status,
            );
        } else if fv_opt.is_some() {
            let fever_pct = apiary.swarm_fever.clamp(0.0, 100.0);
            let q_cell_status = if apiary.queen_cells > 0 {
                format!("[경고: 왕대 {}개 축조! R키로 즉시 제거]", apiary.queen_cells)
            } else {
                "0개 (안전)".to_string()
            };

            **text = format!(
                "분봉열: {:.0}% (100% 도달 시 분봉) | 자연 왕대: {}\n* 왕대를 방치하면 여왕벌과 일벌 절반이 분봉 탈출합니다.",
                fever_pct,
                q_cell_status,
            );
        } else if fd_opt.is_some() {
            let patty_status = if apiary.has_pollen_patty {
                "공급 중 (산란 가속)"
            } else {
                "미공급 ([C] 제작)"
            };

            let mode_desc = if apiary.feeder_syrup > 5.0 {
                "사양꿀 모드 (설탕 사양벌꿀 생산 중)"
            } else if apiary.nearby_flowers > 0 {
                "천연꿀 모드 (주변 꽃밭 넥타 채집 중)"
            } else {
                "단식 정체! (사양액 급여 필요)"
            };

            **text = format!(
                "사양기(설탕시럽): {:.0}% [S 급여] | 화분떡: {} [P 급여]\n현재 생산: {}",
                apiary.feeder_syrup,
                patty_status,
                mode_desc,
            );
        } else if fl_opt.is_some() {
            let mult = apiary.honey_speed_multiplier();
            let eval = if apiary.nearby_flowers >= 8 {
                "최적 밀원 (채밀 배율 최대)"
            } else if apiary.nearby_flowers >= 4 {
                "양호 밀원"
            } else if apiary.nearby_flowers > 0 {
                "부족 (꽃씨를 더 심으세요)"
            } else if apiary.feeder_syrup > 0.0 {
                "사양액으로 보충 중"
            } else {
                "무밀기 (채밀 75% 감소)"
            };

            **text = format!(
                "주변 꽃밭: {}송이 (반경 220m) | 채밀 속도: x{:.2}\n밀원 상태: {}",
                apiary.nearby_flowers,
                mult,
                eval,
            );
        } else if cb_opt.is_some() {
            if apiary.output_combs.is_empty() {
                **text = "저장된 벌집/소비: 없음 (일벌들이 채밀 중...)".to_string();
            } else {
                let mut list = Vec::new();
                for s in &apiary.output_combs {
                    list.push(format!("{} {}개", s.item.display_name(), s.count));
                }
                **text = format!(
                    "저장 벌집 (총 {}종): {}\n-> [H] 키를 눌러 즉시 수확하세요!",
                    apiary.output_combs.len(),
                    list.join(", ")
                );
            }
        }
    }
}

