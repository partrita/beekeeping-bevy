use bevy::prelude::*;
use crate::player::{Player, PlayerInventory};
use crate::items::{ItemType, ItemStack, FlowerColor};
use crate::world::{HarvestableTree, HarvestableRock, HarvestableFlora, WildBeeHive, FerryBoat, RepopulationShrine, WifeEungNpc, CurrentIsland, IslandEntity, spawn_island};
use crate::apiary::{Apiary, SolitaryBeeHotel};
use crate::butterflies::{Butterfly, ActiveButterflyBuffs};
use crate::bees::BiomeType;
use crate::ui::{ActiveUI, DiscoveredSpecies, GameNotification, SelectedApiary};
use crate::crafting::CraftingRegistry;
use crate::market::MarketEconomy;
use crate::minigames::{CentrifugeMinigame, MicroscopeMinigame};
use crate::conservation::ConservationTracker;

pub fn player_hotbar_selection(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<&mut Player>,
) {
    let key_slots = [
        (KeyCode::Digit1, 0),
        (KeyCode::Digit2, 1),
        (KeyCode::Digit3, 2),
        (KeyCode::Digit4, 3),
        (KeyCode::Digit5, 4),
        (KeyCode::Digit6, 5),
        (KeyCode::Digit7, 6),
        (KeyCode::Digit8, 7),
    ];

    for mut player in player_q.iter_mut() {
        for (key, slot) in key_slots {
            if keyboard.just_pressed(key) {
                player.selected_slot = slot;
            }
        }
    }
}

pub fn handle_harvesting(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    player_q: Query<&Transform, With<Player>>,
    mut inventory: ResMut<PlayerInventory>,
    mut notification: ResMut<GameNotification>,
    active_ui: Res<State<ActiveUI>>,
    mut trees: Query<(Entity, &mut HarvestableTree, &Transform)>,
    mut rocks: Query<(Entity, &mut HarvestableRock, &Transform)>,
    mut floras: Query<(Entity, &HarvestableFlora, &Transform)>,
) {
    if *active_ui.get() != ActiveUI::None || !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }

    let Ok(player_tf) = player_q.get_single() else { return; };
    let player_pos = player_tf.translation.truncate();
    let interact_dist = 60.0;

    // Flora
    for (entity, flora, flora_tf) in floras.iter_mut() {
        if player_pos.distance(flora_tf.translation.truncate()) < interact_dist {
            inventory.add_item(ItemType::Wildflower(flora.flower_color), 1);
            commands.entity(entity).despawn();
            notification.send(format!("[채집] 신선한 {} 채취 완료!", flora.flower_color.name()));
            return;
        }
    }

    // Trees
    for (entity, mut tree, tree_tf) in trees.iter_mut() {
        if player_pos.distance(tree_tf.translation.truncate()) < interact_dist {
            if tree.wood_remaining > 0 {
                tree.wood_remaining -= 1;
                inventory.add_item(ItemType::Wood, 2);
                if tree.wood_remaining == 0 {
                    commands.entity(entity).despawn();
                    notification.send("[벌목] 나무를 베었습니다 (+2 목재). 묘목이 다시 자라납니다!");
                } else {
                    notification.send(format!("[벌목 진행] 나무 벌목 중... (+2 목재, 남은 횟수: {})", tree.wood_remaining));
                }
                return;
            }
        }
    }

    // Rocks
    for (entity, mut rock, rock_tf) in rocks.iter_mut() {
        if player_pos.distance(rock_tf.translation.truncate()) < interact_dist {
            if rock.stone_remaining > 0 {
                rock.stone_remaining -= 1;
                inventory.add_item(ItemType::Stone, 2);
                if rock.stone_remaining % 2 == 0 {
                    inventory.add_item(ItemType::Clay, 1);
                }
                if rock.stone_remaining == 0 {
                    commands.entity(entity).despawn();
                    notification.send("[채광] 바위를 채굴했습니다 (+2 석재, +1 점토)!");
                } else {
                    notification.send(format!("[채광 진행] 바위 채굴 중... (+2 석재, 남은 횟수: {})", rock.stone_remaining));
                }
                return;
            }
        }
    }
}

pub fn handle_wild_interactions(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    player_q: Query<&Transform, With<Player>>,
    mut inventory: ResMut<PlayerInventory>,
    mut notification: ResMut<GameNotification>,
    mut discovered: ResMut<DiscoveredSpecies>,
    mut buffs: ResMut<ActiveButterflyBuffs>,
    mut conservation: ResMut<ConservationTracker>,
    mut next_ui: ResMut<NextState<ActiveUI>>,
    active_ui: Res<State<ActiveUI>>,
    mut hives: Query<(Entity, &mut WildBeeHive, &Transform)>,
    butterflies: Query<(Entity, &Butterfly, &Transform)>,
    ferry: Query<&Transform, With<FerryBoat>>,
    mut shrines: Query<(&mut RepopulationShrine, &Transform)>,
    wife_eung: Query<&Transform, With<WifeEungNpc>>,
) {
    if *active_ui.get() != ActiveUI::None || !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }

    let Ok(player_tf) = player_q.get_single() else { return; };
    let player_pos = player_tf.translation.truncate();
    let interact_dist = 60.0;

    // 0. Check 와이프응 (Wife-Eung)
    for npc_tf in wife_eung.iter() {
        if player_pos.distance(npc_tf.translation.truncate()) < 65.0 {
            let dialogues = [
                "와이프응: \"프응아! 오늘도 벌 보느라 수고 많았어~ 따뜻한 보리차 한잔 마시고 해!\"",
                "와이프응: \"기장 바닷바람이 시원하네! 오늘 채취한 꿀은 꿀단지에 예쁘게 포장해둘게~\"",
                "와이프응: \"말벌 나타나면 절대 맨손으로 잡지 말고 훈연기 꼭 챙겨! 다치면 큰일 나!\"",
                "와이프응: \"오늘도 기장 벌들이 건강하게 날아다니네. 당신 덕분에 기장 군도가 되살아나고 있어!\"",
            ];
            let msg = dialogues[rand::random::<usize>() % dialogues.len()];
            inventory.add_item(ItemType::GlassJar, 1);
            notification.send(format!("{} (선물: 유리병 +1)", msg));
            return;
        }
    }

    // Ferry
    for ferry_tf in ferry.iter() {
        if player_pos.distance(ferry_tf.translation.truncate()) < 80.0 {
            next_ui.set(ActiveUI::Ferry);
            return;
        }
    }

    // Repopulation Shrine
    for (mut shrine, shrine_tf) in shrines.iter_mut() {
        if player_pos.distance(shrine_tf.translation.truncate()) < interact_dist {
            let mut repopulated_any = false;
            for i in 0..3 {
                if !shrine.current_repopulated[i] {
                    let req_sp = shrine.target_species[i];
                    let queen_item = ItemType::QueenBee(req_sp.to_string());
                    let drone_item = ItemType::DroneBee(req_sp.to_string());

                    if inventory.remove_item(&queen_item, 1) || inventory.remove_item(&drone_item, 2) {
                        shrine.current_repopulated[i] = true;
                        conservation.release_species(shrine.biome, req_sp);
                        notification.send(format!("[방사 완료] {}을(를) 야생으로 방사했습니다! 생태계 건강도 상승!", req_sp));
                        repopulated_any = true;
                        break;
                    }
                }
            }

            if !repopulated_any {
                notification.send(format!(
                    "[제단] 필요 꿀벌: [{}, {}, {}]. 방사할 여왕벌이나 수벌을 가져오세요!",
                    shrine.target_species[0], shrine.target_species[1], shrine.target_species[2]
                ));
            }
            return;
        }
    }

    // Wild Bee Hives
    for (entity, mut hive, hive_tf) in hives.iter_mut() {
        if player_pos.distance(hive_tf.translation.truncate()) < interact_dist {
            if !hive.harvested {
                hive.harvested = true;
                let sp = hive.species_id.clone();
                inventory.add_item(ItemType::QueenBee(sp.clone()), 1);
                inventory.add_item(ItemType::DroneBee(sp.clone()), 2);
                inventory.add_item(ItemType::Honeycomb(sp.clone()), 2);
                discovered.discover(&sp);
                commands.entity(entity).despawn();
                notification.send(format!("[야생 벌통 포획] 훈연기로 진정 후 여왕벌과 수벌 획득 ({})!", sp));
                return;
            }
        }
    }

    // Butterflies
    for (entity, bfly, bfly_tf) in butterflies.iter() {
        if player_pos.distance(bfly_tf.translation.truncate()) < 50.0 {
            let name = bfly.species_name.to_string();
            inventory.add_item(ItemType::CapturedButterfly(name.clone()), 1);
            if !buffs.collected_species.contains(&name) {
                buffs.collected_species.push(name.clone());
                buffs.recalculate();
            }
            commands.entity(entity).despawn();
            notification.send(format!("[나비 포획] {} 나비를 포획했습니다! 오라 버프 활성화!", name));
            return;
        }
    }
}

pub fn handle_hive_tending(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_q: Query<(&Player, &Transform)>,
    mut inventory: ResMut<PlayerInventory>,
    mut notification: ResMut<GameNotification>,
    active_ui: Res<State<ActiveUI>>,
    mut next_ui: ResMut<NextState<ActiveUI>>,
    mut selected_apiary: ResMut<SelectedApiary>,
    apiaries: Query<(Entity, &Transform), With<Apiary>>,
    mut hotels: Query<(&mut SolitaryBeeHotel, &Transform)>,
) {
    if *active_ui.get() != ActiveUI::None || !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }

    let Ok((_player, player_tf)) = player_q.get_single() else { return; };
    let player_pos = player_tf.translation.truncate();
    let interact_dist = 60.0;

    // Apiaries -> Open Inspection Modal
    for (entity, api_tf) in apiaries.iter() {
        if player_pos.distance(api_tf.translation.truncate()) < interact_dist {
            selected_apiary.0 = Some(entity);
            next_ui.set(ActiveUI::ApiaryInspection);
            return;
        }
    }

    // Solitary Bee Hotels
    for (mut hotel, hotel_tf) in hotels.iter_mut() {
        if player_pos.distance(hotel_tf.translation.truncate()) < interact_dist {
            if hotel.propolis_stored > 0 {
                let count = hotel.propolis_stored;
                hotel.propolis_stored = 0;
                inventory.add_item(ItemType::Propolis, count);
                notification.send(format!("[채취 완료] 단독벌 호텔에서 프로폴리스 {}개 수확!", count));
            } else {
                notification.send("단독벌 호텔이 가동 중입니다. 주변 꽃들의 수분을 돕고 있습니다.");
            }
            return;
        }
    }
}

pub fn handle_flower_planting_and_placement(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    player_q: Query<(&Player, &Transform)>,
    mut inventory: ResMut<PlayerInventory>,
    mut notification: ResMut<GameNotification>,
    active_ui: Res<State<ActiveUI>>,
    pixel_assets: Res<GamePixelAssets>,
) {
    if *active_ui.get() != ActiveUI::None || !keyboard.just_pressed(KeyCode::KeyF) {
        return;
    }

    let Ok((player, player_tf)) = player_q.get_single() else { return; };
    let player_pos = player_tf.translation.truncate();

    // 1. Check if holding ApiaryBox in active hotbar slot
    if let Some(stack) = inventory.slots.get_mut(player.selected_slot).and_then(|s| s.as_mut()) {
        if stack.item == ItemType::ApiaryBox {
            stack.count -= 1;
            if stack.count == 0 {
                inventory.slots[player.selected_slot] = None;
            }

            commands.spawn((
                Sprite {
                    image: pixel_assets.apiary.clone(),
                    custom_size: Some(Vec2::new(48.0, 52.0)),
                    ..default()
                },
                Transform::from_xyz(player_pos.x, player_pos.y, 2.0),
                Apiary::default(),
                IslandEntity,
            ));

            notification.send("[벌통 설치 완료] 새로운 벌통을 배치했습니다! [E] 키로 벌통을 열어 내검 및 소비장을 관리하세요.");
            return;
        }
    }

    // 2. Check if holding FlowerSeed or Wildflower in active hotbar slot
    let mut planted_color = None;
    if let Some(stack) = inventory.slots.get_mut(player.selected_slot).and_then(|s| s.as_mut()) {
        match &stack.item {
            ItemType::FlowerSeed(col) => {
                planted_color = Some(*col);
                stack.count -= 1;
                if stack.count == 0 {
                    inventory.slots[player.selected_slot] = None;
                }
            }
            ItemType::Wildflower(col) => {
                planted_color = Some(*col);
                stack.count -= 1;
                if stack.count == 0 {
                    inventory.slots[player.selected_slot] = None;
                }
            }
            _ => {}
        }
    }

    // 3. If not held in active slot, search any slot for FlowerSeed
    if planted_color.is_none() {
        for slot in inventory.slots.iter_mut() {
            if let Some(stack) = slot {
                if let ItemType::FlowerSeed(col) = stack.item {
                    planted_color = Some(col);
                    stack.count -= 1;
                    if stack.count == 0 {
                        *slot = None;
                    }
                    break;
                }
            }
        }
    }

    if let Some(col) = planted_color {
        let f_img = match col {
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
                custom_size: Some(Vec2::new(24.0, 24.0)),
                ..default()
            },
            Transform::from_xyz(player_pos.x, player_pos.y, 1.0),
            HarvestableFlora { flower_color: col, growth_stage: 1.0 },
            IslandEntity,
        ));

        notification.send(format!(
            "[꽃 심기 완료] {} 밀원 꽃밭을 조성했습니다! (주변 벌통의 꿀 채집 속도 가속!)",
            col.name()
        ));
    } else {
        notification.send("[안내] 꽃을 심으려면 꽃씨(FlowerSeed)나 야생화(Wildflower)를 소지하고 [F]를 누르세요! (새 벌통 설치: ApiaryBox 들고 [F])");
    }
}

pub fn handle_apiary_inspection_inputs(
    keyboard: Res<ButtonInput<KeyCode>>,
    active_ui: Res<State<ActiveUI>>,
    selected_apiary: Res<SelectedApiary>,
    mut apiaries: Query<&mut Apiary>,
    mut inventory: ResMut<PlayerInventory>,
    mut notification: ResMut<GameNotification>,
) {
    if *active_ui.get() != ActiveUI::ApiaryInspection {
        return;
    }

    let Some(apiary_ent) = selected_apiary.0 else { return; };
    let Ok(mut apiary) = apiaries.get_mut(apiary_ent) else { return; };

    // [F] 증소 (소비장 추가)
    if keyboard.just_pressed(KeyCode::KeyF) {
        if apiary.frame_count >= apiary.max_frames {
            notification.send("[알림] 벌통에 이미 최대 소비장(8장)이 꽉 찼습니다! (최대 수용량 도달)");
        } else {
            if inventory.remove_item(&ItemType::WaxedFrame, 1) {
                apiary.add_frame();
                notification.send(format!(
                    "[증소 완료] 소초광(Waxed Frame)을 넣었습니다! (소비장: {}/{}장, 수용한도: {}마리, 분봉열 감소)",
                    apiary.frame_count, apiary.max_frames, apiary.frame_capacity()
                ));
            } else if inventory.remove_item(&ItemType::EmptyFrame, 1) {
                apiary.add_frame();
                notification.send(format!(
                    "[증소 완료] 빈 소비장(Empty Frame)을 넣었습니다! (소비장: {}/{}장, 수용한도: {}마리, 분봉열 감소)",
                    apiary.frame_count, apiary.max_frames, apiary.frame_capacity()
                ));
            } else {
                notification.send("[오류] 인벤토리에 소비장(Empty Frame 또는 Waxed Frame)이 없습니다! 제작대[C]에서 제작하세요.");
            }
        }
    }

    // [D] 축소 (소비장 빼기)
    if keyboard.just_pressed(KeyCode::KeyD) {
        if apiary.remove_frame() {
            inventory.add_item(ItemType::EmptyFrame, 1);
            notification.send(format!(
                "[축소 완료] 빈 소비장을 회수했습니다! (소비장: {}/{}장, 착봉 밀도: {:.0}마리/장, 보온성 및 산란 집중도 향상)",
                apiary.frame_count, apiary.max_frames, apiary.bees_per_frame()
            ));
        } else {
            notification.send("[경고] 벌통에는 최소 1장의 소비장이 유지되어야 합니다! (더 이상 축소 불가)");
        }
    }

    // [R] 내검 & 왕대 제거
    if keyboard.just_pressed(KeyCode::KeyR) {
        let removed = apiary.remove_queen_cells();
        if removed > 0 {
            inventory.add_item(ItemType::QueenCell, removed);
            inventory.add_item(ItemType::RoyalJelly, removed);
            notification.send(format!(
                "[내검 완료] 왕대 {}개를 제거하여 분봉열을 크게 낮췄습니다! (로열젤리 +{}, 왕대 +{})",
                removed, removed, removed
            ));
        } else {
            apiary.swarm_fever = (apiary.swarm_fever - 15.0).max(0.0);
            notification.send("[내검 완료] 벌통 내부를 점검했습니다. 아직 왕대가 없어 분봉열이 15% 진정되었습니다.");
        }
    }

    // [S] 사양액/설탕 급여 (Feed Sugar Syrup)
    if keyboard.just_pressed(KeyCode::KeyS) {
        if inventory.remove_item(&ItemType::SugarSyrup, 1) {
            apiary.feed_sugar(50.0);
            notification.send(format!(
                "[사양 완료] 설탕 시럽을 사양기에 채웠습니다! (사양기 잔량: {:.0}%, 무밀기 사양꿀 생산 및 군세 보존)",
                apiary.feeder_syrup
            ));
        } else if inventory.remove_item(&ItemType::Sugar, 1) {
            apiary.feed_sugar(25.0);
            notification.send(format!(
                "[사양 완료] 설탕을 공급했습니다! (사양기 잔량: {:.0}%)",
                apiary.feeder_syrup
            ));
        } else {
            notification.send("[오류] 사양액(SugarSyrup)이나 설탕(Sugar)이 인벤토리에 없습니다! 작업대[C]나 시장[P]에서 준비하세요.");
        }
    }

    // [P] 화분떡 급여 (Feed Pollen Patty)
    if keyboard.just_pressed(KeyCode::KeyP) {
        if inventory.remove_item(&ItemType::PollenPatty, 1) {
            apiary.feed_pollen();
            notification.send("[급여 완료] 고단백 화분떡을 소비장 위에 올려주었습니다! (산란 및 일벌 육아 속도 폭증: +35마리/초)");
        } else {
            notification.send("[오류] 화분떡(PollenPatty)이 인벤토리에 없습니다! 작업대[C]에서 설탕과 야생화로 제작하세요.");
        }
    }

    // [H] 꿀 소비 채취
    if keyboard.just_pressed(KeyCode::KeyH) {
        if !apiary.output_combs.is_empty() {
            let mut total = 0;
            for stack in apiary.output_combs.drain(..) {
                total += stack.count;
                inventory.add_item(stack.item, stack.count);
            }
            notification.send(format!(
                "[채밀 완료] 숙성된 꿀 소비 {}개를 수확했습니다! 채밀기[M]로 꿀을 추출하세요!",
                total
            ));
        } else {
            notification.send("저장된 꿀 소비가 없습니다. 꿀벌들이 꽃밭에서 넥타를 모으고 있습니다.");
        }
    }

    // [1-8] 핫바 번호로 여왕벌 / 공주벌 / 수벌 넣기
    let key_slots = [
        (KeyCode::Digit1, 0),
        (KeyCode::Digit2, 1),
        (KeyCode::Digit3, 2),
        (KeyCode::Digit4, 3),
        (KeyCode::Digit5, 4),
        (KeyCode::Digit6, 5),
        (KeyCode::Digit7, 6),
        (KeyCode::Digit8, 7),
    ];

    for (key, slot) in key_slots {
        if keyboard.just_pressed(key) {
            if let Some(stack) = inventory.slots.get_mut(slot).and_then(|s| s.as_mut()) {
                if matches!(stack.item, ItemType::QueenBee(_) | ItemType::PrincessBee(_)) {
                    if apiary.queen_slot.is_none() {
                        let item = stack.item.clone();
                        stack.count -= 1;
                        if stack.count == 0 {
                            inventory.slots[slot] = None;
                        }
                        apiary.queen_slot = Some(ItemStack::new(item, 1));
                        notification.send("[배치 완료] 여왕실에 꿀벌을 배치했습니다!");
                    } else {
                        notification.send("여왕실에 이미 여왕벌이 있습니다.");
                    }
                } else if matches!(stack.item, ItemType::DroneBee(_)) {
                    if apiary.drone_slot.is_none() {
                        let item = stack.item.clone();
                        stack.count -= 1;
                        if stack.count == 0 {
                            inventory.slots[slot] = None;
                        }
                        apiary.drone_slot = Some(ItemStack::new(item, 1));
                        notification.send("[배치 완료] 수벌방에 수벌을 배치했습니다!");
                    } else {
                        notification.send("수벌방에 이미 수벌이 있습니다.");
                    }
                }
            }
        }
    }
}

use crate::pixel_assets::GamePixelAssets;

pub fn handle_modal_inputs(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    active_ui: Res<State<ActiveUI>>,
    mut next_ui: ResMut<NextState<ActiveUI>>,
    crafting: Res<CraftingRegistry>,
    mut inventory: ResMut<PlayerInventory>,
    mut economy: ResMut<MarketEconomy>,
    mut centrifuge: ResMut<CentrifugeMinigame>,
    mut microscope: ResMut<MicroscopeMinigame>,
    mut notification: ResMut<GameNotification>,
    mut discovered: ResMut<DiscoveredSpecies>,
    buffs: Res<ActiveButterflyBuffs>,
    mut current_island: ResMut<CurrentIsland>,
    island_entities: Query<Entity, With<IslandEntity>>,
    pixel_assets: Res<GamePixelAssets>,
) {
    match *active_ui.get() {
        ActiveUI::Crafting => {
            let key_indices = [
                (KeyCode::Digit1, 0),
                (KeyCode::Digit2, 1),
                (KeyCode::Digit3, 2),
                (KeyCode::Digit4, 3),
                (KeyCode::Digit5, 4),
                (KeyCode::Digit6, 5),
                (KeyCode::Digit7, 6),
                (KeyCode::Digit8, 7),
                (KeyCode::Digit9, 8),
            ];

            for (key, idx) in key_indices {
                if keyboard.just_pressed(key) {
                    if let Some(recipe) = crafting.recipes.get(idx) {
                        let can_craft = recipe.inputs.iter().all(|inp| inventory.count_item(&inp.item) >= inp.count);
                        if can_craft {
                            for inp in &recipe.inputs {
                                inventory.remove_item(&inp.item, inp.count);
                            }
                            inventory.add_item(recipe.output.item.clone(), recipe.output.count);
                            notification.send(format!("[제작 완료] {} {}개 제작 완료!", recipe.output.item.display_name(), recipe.output.count));
                        } else {
                            notification.send("제작 재료가 부족합니다!");
                        }
                    }
                }
            }
        }
        ActiveUI::Centrifuge => {
            if keyboard.just_pressed(KeyCode::Space) {
                centrifuge.pump();
            }

            if centrifuge.success {
                // If player has SugarHoneycomb in inventory, extract into SugarHoneyJar!
                if inventory.remove_item(&ItemType::SugarHoneycomb, 1) {
                    inventory.add_item(ItemType::SugarHoneyJar, 2);
                    inventory.add_item(ItemType::Beeswax, 1);
                    notification.send("[사양벌꿀 추출 완료] 사양 소비 원심분리 완료! 사양벌꿀 2단지와 밀랍 1개 획득 (사양꿀 라벨 부착)");
                } else {
                    // Try to consume any floral honeycomb
                    let mut consumed_floral = false;
                    for slot in inventory.slots.iter_mut() {
                        if let Some(stack) = slot {
                            if matches!(stack.item, ItemType::Honeycomb(_)) {
                                stack.count -= 1;
                                if stack.count == 0 {
                                    *slot = None;
                                }
                                consumed_floral = true;
                                break;
                            }
                        }
                    }

                    if consumed_floral {
                        inventory.add_item(ItemType::HoneyJar("Wildflower".to_string()), 2);
                        inventory.add_item(ItemType::Beeswax, 2);
                        notification.send("[천연 꽃꿀 추출 성공] 향긋한 천연 꽃꿀 2단지와 순수 밀랍 2개를 추출했습니다!");
                    } else {
                        inventory.add_item(ItemType::HoneyJar("Wildflower".to_string()), 1);
                        inventory.add_item(ItemType::Beeswax, 1);
                        notification.send("[채밀 완료] 꿀 1단지와 밀랍 1개를 추출했습니다!");
                    }
                }
                centrifuge.success = false;
                centrifuge.current_rpm = 0.0;
                centrifuge.sweet_spot_timer = 0.0;
                next_ui.set(ActiveUI::None);
            }
        }
        ActiveUI::Microscope => {
            if keyboard.pressed(KeyCode::KeyA) {
                microscope.focus_dial = (microscope.focus_dial - 0.5).max(0.0);
            }
            if keyboard.pressed(KeyCode::KeyD) {
                microscope.focus_dial = (microscope.focus_dial + 0.5).min(100.0);
            }
            if keyboard.pressed(KeyCode::KeyW) {
                microscope.magnification_dial = (microscope.magnification_dial + 0.5).min(100.0);
            }
            if keyboard.pressed(KeyCode::KeyS) {
                microscope.magnification_dial = (microscope.magnification_dial - 0.5).max(0.0);
            }

            if microscope.check_alignment() && !microscope.analyzed {
                microscope.analyzed = true;
                discovered.discover("blossom");
                discovered.discover("honeycomb");
                notification.send("[현미경 분석] 꿀벌 유전자 대립형질 분석 완료! 신규 형질 발견!");
            }
        }
        ActiveUI::Market => {
            if keyboard.just_pressed(KeyCode::KeyS) {
                let mut earned = 0;
                let sellables = [
                    ItemType::HoneyJar("Wildflower".to_string()),
                    ItemType::HoneyJar("Clover".to_string()),
                    ItemType::SugarHoneyJar,
                    ItemType::SugarHoneycomb,
                    ItemType::RoyalJelly,
                    ItemType::QueenCell,
                    ItemType::Beeswax,
                    ItemType::BeeswaxCandle,
                    ItemType::HoneyBeaFatherla,
                    ItemType::Propolis,
                    ItemType::InfusedMead,
                ];

                for item in &sellables {
                    let count = inventory.count_item(item);
                    if count > 0 {
                        inventory.remove_item(item, count);
                        earned += economy.sell_item(item, count, buffs.sell_multiplier);
                    }
                }

                if earned > 0 {
                    notification.send(format!("[판매 완료] 양봉 생산물을 판매하여 {} 허니 코인을 획득했습니다!", earned));
                } else {
                    notification.send("인벤토리에 판매 가능한 양봉 생산물이 없습니다!");
                }
            }

            let buy_keys = [
                (KeyCode::Digit1, 0),
                (KeyCode::Digit2, 1),
                (KeyCode::Digit3, 2),
                (KeyCode::Digit4, 3),
                (KeyCode::Digit5, 4),
                (KeyCode::Digit6, 5),
                (KeyCode::Digit7, 6),
                (KeyCode::Digit8, 7),
                (KeyCode::Digit9, 8),
            ];

            for (key, idx) in buy_keys {
                if keyboard.just_pressed(key) {
                    if let Some(entry) = economy.shop_items.get(idx) {
                        let price = entry.price;
                        let item = entry.item.clone();
                        let name = entry.name;
                        if economy.player_coins >= price {
                            economy.player_coins -= price;
                            inventory.add_item(item, 1);
                            notification.send(format!("[구매 완료] {} 구매 완료!", name));
                        } else {
                            notification.send("허니 코인이 부족합니다!");
                        }
                    }
                }
            }
        }
        ActiveUI::Ferry => {
            let island_keys = [
                (KeyCode::Digit1, BiomeType::PortHoneyBeaFather, "Port Gijang (Home)"),
                (KeyCode::Digit2, BiomeType::EmeraldMeadows, "Emerald Meadows"),
                (KeyCode::Digit3, BiomeType::WhisperwoodForest, "Whisperwood Forest"),
                (KeyCode::Digit4, BiomeType::SunkenShore, "Sunken Shore"),
                (KeyCode::Digit5, BiomeType::MangroveMarsh, "Mangrove Marsh"),
                (KeyCode::Digit6, BiomeType::ShimmeringPeaks, "Shimmering Peaks"),
            ];

            for (key, biome, name) in island_keys {
                if keyboard.just_pressed(key) {
                    if current_island.biome != biome {
                        current_island.biome = biome;
                        current_island.island_name = name.to_string();

                        for e in island_entities.iter() {
                            commands.entity(e).despawn();
                        }

                        spawn_island(&mut commands, biome, &pixel_assets);
                        notification.send(format!("[항해 완료] {}에 도착했습니다!", name));
                        next_ui.set(ActiveUI::None);
                        return;
                    }
                }
            }
        }
        _ => {}
    }
}
