use bevy::prelude::*;
use crate::colony::{Colony, HoneyJars};
use crate::meadow::{DayNight, Weather};

// Single-screen shell UI: thin top bar, tutorial notice, bottom hints,
// slim side panel. Watch-first; H harvests, Space finds queen.

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    World,
    #[default]
    Inspection,
}

#[derive(Component)]
pub struct PanelRoot;

#[derive(Component)]
pub struct HudRoot;

#[derive(Component)]
pub struct HudJars;
#[derive(Component)]
pub struct HudBees;
#[derive(Component)]
pub struct HudDay;
#[derive(Component)]
pub struct NoticeText;

#[derive(Component)]
pub struct PanelQueen;
#[derive(Component)]
pub struct PanelColony;
#[derive(Component)]
pub struct PanelStores;

#[derive(Resource, Default)]
pub struct Notice {
    pub message: String,
    pub timer: f32,
}

impl Notice {
    pub fn send(&mut self, msg: impl Into<String>) {
        self.message = msg.into();
        self.timer = 4.0;
    }

    pub fn tick(&mut self, dt: f32) {
        if self.timer > 0.0 {
            self.timer -= dt;
            if self.timer <= 0.0 {
                self.message.clear();
            }
        }
    }
}

fn kfont(fonts: &crate::GameFonts, size: f32) -> TextFont {
    TextFont {
        font: fonts.korean.clone(),
        font_size: size,
        ..default()
    }
}

pub fn setup_hud(mut commands: Commands, fonts: Res<crate::GameFonts>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                row_gap: Val::Px(8.0),
                padding: UiRect::all(Val::Px(12.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 50.0),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(48.0),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::axes(Val::Px(16.0), Val::Px(6.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.1, 0.14, 0.12, 0.92)),
                    BorderColor(Color::srgb(0.85, 0.65, 0.2)),
                    HudRoot,
                ))
                .with_children(|top| {
                    top.spawn((
                        Text::new("🍯 꿀벌 내검 일기"),
                        kfont(&fonts, 17.0),
                        TextColor(Color::srgb(0.95, 0.9, 0.7)),
                    ));
                    top.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(16.0),
                        ..default()
                    })
                    .with_children(|stats| {
                        stats.spawn((
                            Text::new("꿀단지 0"),
                            kfont(&fonts, 15.0),
                            TextColor(Color::srgb(1.0, 0.88, 0.3)),
                            HudJars,
                        ));
                        stats.spawn((
                            Text::new("일벌 0"),
                            kfont(&fonts, 15.0),
                            TextColor(Color::srgb(0.85, 0.92, 0.85)),
                            HudBees,
                        ));
                        stats.spawn((
                            Text::new("☀️ 낮"),
                            kfont(&fonts, 15.0),
                            TextColor(Color::srgb(0.7, 0.9, 1.0)),
                            HudDay,
                        ));
                    });
                });

            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                })
                .with_children(|banner| {
                    banner.spawn((
                        Text::new("가만히 보세요 — 여왕이 알을 낳고 일벌이 꿀·화분을 물어옵니다"),
                        kfont(&fonts, 15.0),
                        TextColor(Color::srgb(0.95, 0.95, 0.8)),
                        NoticeText,
                    ));
                });

            // Spacer pushes the hints to the bottom; notice stays at top.
            parent.spawn(Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                ..default()
            });

            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    HudRoot,
                ))
                .with_children(|hints| {
                    hints.spawn((
                        Text::new(
                            "[H] 채밀 · [Space] 여왕 찾기 · 클릭 따라가기 · 우드래그 이동 · 휠 줌 · [Tab] HUD숨기기 · [M] 음소거 · [ESC] 나가기",
                        ),
                        kfont(&fonts, 13.0),
                        TextColor(Color::srgb(0.8, 0.85, 0.8)),
                    ));
                });
        });
}

pub fn update_hud(
    time: Res<Time>,
    jars: Res<HoneyJars>,
    day: Res<DayNight>,
    weather: Res<Weather>,
    hidden: Res<crate::HudHidden>,
    muted: Res<crate::Muted>,
    offline: Res<crate::OfflineSummary>,
    mut notice: ResMut<Notice>,
    hive: Query<&Colony>,
    mut q_jars: Query<&mut Text, (With<HudJars>, Without<HudBees>, Without<HudDay>, Without<NoticeText>)>,
    mut q_bees: Query<&mut Text, (With<HudBees>, Without<HudJars>, Without<HudDay>, Without<NoticeText>)>,
    mut q_day: Query<&mut Text, (With<HudDay>, Without<HudJars>, Without<HudBees>, Without<NoticeText>)>,
    mut q_notice: Query<&mut Text, (With<NoticeText>, Without<HudJars>, Without<HudBees>, Without<HudDay>)>,
    mut huds: Query<&mut Visibility, With<HudRoot>>,
) {
    notice.tick(time.delta_secs());
    let vis = if hidden.0 {
        Visibility::Hidden
    } else {
        Visibility::Visible
    };
    for mut v in huds.iter_mut() {
        *v = vis;
    }
    for mut t in q_jars.iter_mut() {
        t.0 = if muted.0 {
            format!("🍯 꿀단지 {} (음소거)", jars.0)
        } else {
            format!("🍯 꿀단지 {}", jars.0)
        };
    }
    let bees = hive.iter().next().map(|c| c.workers as u32).unwrap_or(0);
    for mut t in q_bees.iter_mut() {
        t.0 = format!("🐝 일벌 {}", bees);
    }
    for mut t in q_day.iter_mut() {
        t.0 = day.label(&weather);
    }
    for mut t in q_notice.iter_mut() {
        if !notice.message.is_empty() {
            t.0 = notice.message.clone();
        } else if offline.timer > 0.0 && !offline.text.is_empty() {
            t.0 = offline.text.clone();
        } else {
            t.0 = "가만히 보세요 — 여왕이 알을 낳고 일벌이 꿀·화분을 물어옵니다".to_string();
        }
    }
}

/// Slim side panel: auto stats, nothing to press but H.
pub fn spawn_panel(
    mut commands: Commands,
    fonts: Res<crate::GameFonts>,
    pixel_assets: Res<crate::pixel_assets::GamePixelAssets>,
) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(1.5),
                top: Val::Percent(4.0),
                width: Val::Px(320.0),
                height: Val::Percent(92.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                row_gap: Val::Px(8.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.08, 0.12, 0.1, 0.93)),
            BorderColor(Color::srgb(0.9, 0.75, 0.2)),
            PanelRoot,
        ))
        .with_children(|panel| {
            panel.spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(6.0),
                ..default()
            }).with_children(|t| {
                t.spawn((
                    ImageNode::new(pixel_assets.icons.bee.clone()),
                    Node { width: Val::Px(20.0), height: Val::Px(20.0), ..default() },
                ));
                t.spawn((
                    Text::new("벌통 안 엿보기"),
                    kfont(&fonts, 18.0),
                    TextColor(Color::srgb(1.0, 0.85, 0.2)),
                ));
            });
            let mut card = |icon: Handle<Image>, marker: PanelKind| {
                panel
                    .spawn(Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(6.0),
                        padding: UiRect::all(Val::Px(6.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    })
                    .with_children(|row| {
                        row.spawn((
                            ImageNode::new(icon),
                            Node { width: Val::Px(16.0), height: Val::Px(16.0), ..default() },
                        ));
                        match marker {
                            PanelKind::Queen => {
                                row.spawn((
                                    Text::new("여왕벌..."),
                                    kfont(&fonts, 12.0),
                                    TextColor(Color::WHITE),
                                    PanelQueen,
                                ));
                            }
                            PanelKind::Colony => {
                                row.spawn((
                                    Text::new("군체..."),
                                    kfont(&fonts, 12.0),
                                    TextColor(Color::srgb(0.85, 0.92, 0.85)),
                                    PanelColony,
                                ));
                            }
                            PanelKind::Stores => {
                                row.spawn((
                                    Text::new("저장고..."),
                                    kfont(&fonts, 12.0),
                                    TextColor(Color::srgb(1.0, 0.9, 0.6)),
                                    PanelStores,
                                ));
                            }
                        }
                    });
            };
            card(pixel_assets.icons.crown.clone(), PanelKind::Queen);
            card(pixel_assets.icons.bee.clone(), PanelKind::Colony);
            card(pixel_assets.icons.honey.clone(), PanelKind::Stores);
            panel.spawn((
                Text::new("[H] 꿀 채밀하기 · [Space] 여왕 찾기"),
                kfont(&fonts, 12.0),
                TextColor(Color::srgb(0.8, 0.85, 0.8)),
            ));
        });
}

enum PanelKind {
    Queen,
    Colony,
    Stores,
}

pub fn update_panel(
    hive: Query<&Colony>,
    jars: Res<HoneyJars>,
    day: Res<DayNight>,
    mut q: Query<(
        &mut Text,
        Option<&PanelQueen>,
        Option<&PanelColony>,
        Option<&PanelStores>,
    )>,
) {
    let Ok(col) = hive.get_single() else {
        return;
    };
    for (mut text, qn, co, st) in q.iter_mut() {
        if qn.is_some() {
            **text = format!(
                "여왕: 알을 낳는 중 (초당 {:.1}알)\n수명 걱정 없음 · {}",
                col.lay_display.max(0.0),
                if day.is_night() {
                    "밤이라 쉬엄쉬엄"
                } else {
                    "낮이라 한창 산란"
                },
            );
        } else if co.is_some() {
            // Q33=B: soft one-line toast, no alarm.
            let hunger = if col.starving {
                "\n[화분 부족] 꽃이 필 때까지 조금만 기다려요"
            } else {
                ""
            };
            **text = format!(
                "일벌 {:.0} (채집 {:.0} · 집안 {:.0})\n수벌 {:.0} · 알 {:.0} · 유충 {:.0} · 번데기 {:.0}{}\n빈집 {:.0}칸 · 누적 우화 {}마리",
                col.workers,
                col.foragers(),
                col.nurses(),
                col.drones,
                col.eggs,
                col.larvae,
                col.pupae,
                hunger,
                col.empty_cells(),
                col.total_emerged,
            );
        } else if st.is_some() {
            **text = format!(
                "꿀 {:.0} · 화분 {:.0}\n선반의 꿀단지 {}개 ([H]를 누르면 꿀 10 → 1단지)",
                col.honey, col.pollen, jars.0,
            );
        }
    }
}

pub fn cleanup_panel(mut commands: Commands, roots: Query<Entity, With<PanelRoot>>) {
    for e in roots.iter() {
        commands.entity(e).despawn_recursive();
    }
}

/// Harvest: H anywhere on the single screen presses jars (Q5=A, Q22=A).
pub fn harvest_honey(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut hive: Query<&mut Colony>,
    mut jars: ResMut<HoneyJars>,
    mut notice: ResMut<Notice>,
) {
    if !keyboard.just_pressed(KeyCode::KeyH) {
        return;
    }
    if let Ok(mut col) = hive.get_single_mut() {
        let n = col.harvest();
        if n > 0 {
            jars.0 += n;
            notice.send(format!("[채밀] 향긋한 꿀 {}단지! 수고한 일벌들에게 감사", n));
        } else {
            notice.send("아직 짤 꿀이 모자라요. 일벌들이 열심히 나르고 있어요");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notice_lifecycle() {
        let mut n = Notice::default();
        n.send("hi");
        assert_eq!(n.message, "hi");
        n.tick(5.0);
        assert!(n.message.is_empty());
    }
}
