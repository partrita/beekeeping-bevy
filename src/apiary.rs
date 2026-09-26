use bevy::prelude::*;
use crate::bees::{BeeCatalog, BeeGenome};
use crate::items::{ItemType, ItemStack};
use crate::world::HarvestableFlora;
use crate::ui::GameNotification;

#[derive(Component)]
pub struct Apiary {
    pub id: u32,
    pub queen_slot: Option<ItemStack>,
    pub drone_slot: Option<ItemStack>,
    pub frame_count: u32,       // Installed frames (1..=8)
    pub max_frames: u32,        // Max 8 frames
    pub bee_population: u32,    // Colony population (1,000..=12,000+)
    pub queen_cells: u32,       // Natural queen cells formed during swarm fever (0..=5)
    pub swarm_fever: f32,       // 0.0 to 100.0%
    pub swarmed: bool,          // Swarm event trigger
    pub nearby_flowers: u32,    // Flowers within foraging radius
    pub feeder_syrup: f32,       // Feeder sugar syrup level (0.0 to 100.0%)
    pub has_pollen_patty: bool,  // High protein pollen cake active for brood rearing
    pub output_combs: Vec<ItemStack>, // Produced honeycombs
    pub queen_genome: Option<BeeGenome>,
    pub life_timer: f32,
    pub max_life: f32,
    pub production_timer: f32,
    pub production_interval: f32,
    pub is_active: bool,
}

impl Default for Apiary {
    fn default() -> Self {
        Self {
            id: 1,
            queen_slot: None,
            drone_slot: None,
            frame_count: 2, // Starts with 2 frames
            max_frames: 8,
            bee_population: 1800,
            queen_cells: 0,
            swarm_fever: 0.0,
            swarmed: false,
            nearby_flowers: 3,
            feeder_syrup: 0.0,
            has_pollen_patty: false,
            output_combs: Vec::new(),
            queen_genome: None,
            life_timer: 0.0,
            max_life: 35.0,
            production_timer: 0.0,
            production_interval: 6.0,
            is_active: false,
        }
    }
}

impl Apiary {
    pub fn frame_capacity(&self) -> u32 {
        self.frame_count * 1500
    }

    pub fn bees_per_frame(&self) -> f32 {
        if self.frame_count == 0 {
            0.0
        } else {
            self.bee_population as f32 / self.frame_count as f32
        }
    }

    pub fn is_overcrowded(&self) -> bool {
        self.bee_population > self.frame_capacity()
    }

    // Overframed (과소비 / 착봉 불량 / 보온 실패): too many frames for small population
    pub fn is_overframed(&self) -> bool {
        self.frame_count > 1 && self.bees_per_frame() < 900.0
    }

    // Thermal & clustering efficiency (착봉 및 보온 효율):
    // If bees_per_frame >= 900: 1.0 (100% optimal warmth & cluster)
    // If bees_per_frame < 900: drops down to 0.25 (chilled brood, scattered nectar)
    pub fn thermal_efficiency(&self) -> f32 {
        if self.frame_count <= 1 {
            1.0
        } else {
            let density = self.bees_per_frame();
            if density >= 900.0 {
                1.0
            } else {
                (density / 900.0).clamp(0.25, 1.0)
            }
        }
    }

    pub fn add_frame(&mut self) -> bool {
        if self.frame_count < self.max_frames {
            self.frame_count += 1;
            self.swarm_fever = (self.swarm_fever - 25.0).max(0.0);
            true
        } else {
            false
        }
    }

    // Remove frame (축소): pulling out excess frame to tighten cluster and restore warmth
    pub fn remove_frame(&mut self) -> bool {
        if self.frame_count > 1 {
            self.frame_count -= 1;
            true
        } else {
            false
        }
    }

    pub fn remove_queen_cells(&mut self) -> u32 {
        let count = self.queen_cells;
        self.queen_cells = 0;
        self.swarm_fever = (self.swarm_fever - 45.0).max(0.0);
        count
    }

    pub fn feed_sugar(&mut self, amount: f32) {
        self.feeder_syrup = (self.feeder_syrup + amount).min(100.0);
    }

    pub fn feed_pollen(&mut self) {
        self.has_pollen_patty = true;
    }

    pub fn honey_speed_multiplier(&self) -> f32 {
        let base = if self.nearby_flowers == 0 {
            if self.feeder_syrup > 0.0 {
                1.25 // Sugar syrup guarantees steady production even with 0 flowers!
            } else {
                0.25 // Barely any nectar
            }
        } else {
            0.5 + (self.nearby_flowers as f32 * 0.25)
        };

        let with_feed = if self.feeder_syrup > 0.0 {
            base + 0.5
        } else {
            base
        };

        // If over-framed (과소비), bees cannot maintain warmth and nectar is scattered thinly, reducing yield!
        with_feed * self.thermal_efficiency()
    }
}

#[derive(Component)]
pub struct SolitaryBeeHotel {
    pub bee_species: Option<String>,
    pub occupancy: u32, // 0 to 5 cells
    pub max_occupancy: u32,
    pub propolis_stored: u32,
    pub timer: f32,
}

impl Default for SolitaryBeeHotel {
    fn default() -> Self {
        Self {
            bee_species: None,
            occupancy: 0,
            max_occupancy: 5,
            propolis_stored: 0,
            timer: 0.0,
        }
    }
}

pub fn update_apiaries(
    time: Res<Time>,
    catalog: Res<BeeCatalog>,
    mut notification: ResMut<GameNotification>,
    floras: Query<&Transform, With<HarvestableFlora>>,
    mut apiaries: Query<(&mut Apiary, &Transform)>,
) {
    let delta = time.delta_secs();

    for (mut apiary, apiary_tf) in apiaries.iter_mut() {
        let api_pos = apiary_tf.translation.truncate();

        // 1. Calculate nearby flowers within foraging radius (220 units)
        let mut flower_count = 0;
        for flora_tf in floras.iter() {
            if api_pos.distance(flora_tf.translation.truncate()) <= 220.0 {
                flower_count += 1;
            }
        }
        apiary.nearby_flowers = flower_count;

        // 2. Mating (Princess + Drone -> Queen)
        if apiary.queen_genome.is_none() {
            let has_princess = apiary.queen_slot.as_ref().map_or(false, |s| matches!(s.item, ItemType::PrincessBee(_)));
            let has_drone = apiary.drone_slot.as_ref().map_or(false, |s| matches!(s.item, ItemType::DroneBee(_)));

            if has_princess && has_drone {
                let p_sp = match &apiary.queen_slot.as_ref().unwrap().item {
                    ItemType::PrincessBee(sp) => sp.clone(),
                    _ => unreachable!(),
                };
                let d_sp = match &apiary.drone_slot.as_ref().unwrap().item {
                    ItemType::DroneBee(sp) => sp.clone(),
                    _ => unreachable!(),
                };

                if let Some(drone_stack) = apiary.drone_slot.as_mut() {
                    drone_stack.count -= 1;
                    if drone_stack.count == 0 {
                        apiary.drone_slot = None;
                    }
                }

                apiary.queen_slot = Some(ItemStack::new(ItemType::QueenBee(p_sp.clone()), 1));

                let mutated_species = catalog.try_mutate(&p_sp, &d_sp, 0.0);
                let active_species = mutated_species.map(|s| s.to_string()).unwrap_or(p_sp.clone());

                let species_info = catalog.get(&active_species).or_else(|| catalog.get(&p_sp));
                let base_lifespan = species_info.map(|s| s.base_lifespan).unwrap_or(35.0);
                let base_speed = species_info.map(|s| s.base_speed).unwrap_or(6.0);

                apiary.queen_genome = Some(BeeGenome {
                    primary_species: active_species.clone(),
                    secondary_species: d_sp,
                    lifespan_trait: 0,
                    speed_trait: 0,
                    fertility_trait: species_info.map(|s| s.fertility).unwrap_or(2),
                    is_nocturnal: species_info.map(|s| s.nocturnal).unwrap_or(false),
                    rain_tolerant: false,
                });

                apiary.life_timer = base_lifespan;
                apiary.max_life = base_lifespan;
                apiary.production_interval = base_speed;
                apiary.production_timer = 0.0;
                apiary.is_active = true;
                apiary.bee_population = 2000;
                apiary.queen_cells = 0;
                apiary.swarm_fever = 0.0;
            }
        }

        // 3. Active Queen & Colony Lifecycle
        if apiary.is_active {
            if let Some(genome) = apiary.queen_genome.clone() {
                // Population growth from queen egg-laying (+15 bees/sec base, +25 if syrup fed, +35 if pollen patty given)
                let base_growth = if apiary.has_pollen_patty && apiary.feeder_syrup > 0.0 {
                    35.0
                } else if apiary.feeder_syrup > 0.0 {
                    25.0
                } else {
                    15.0
                };
                // Chilled brood penalty from over-framing (과소비 / 보온 실패 시 산란 및 육아 속도 급감)
                let thermal = apiary.thermal_efficiency();
                let actual_growth = base_growth * thermal;
                apiary.bee_population = (apiary.bee_population + (actual_growth * delta) as u32).min(15_000);

                // Consumption of sugar syrup over time
                if apiary.feeder_syrup > 0.0 {
                    apiary.feeder_syrup = (apiary.feeder_syrup - delta * 1.5).max(0.0);
                }

                // Population capacity vs installed frames
                let capacity = apiary.frame_capacity();
                if apiary.bee_population > capacity {
                    // Overcrowded! Swarm fever rises rapidly
                    let overflow_ratio = (apiary.bee_population - capacity) as f32 / 1000.0;
                    apiary.swarm_fever = (apiary.swarm_fever + delta * 2.5 * (1.0 + overflow_ratio)).min(100.0);

                    // Build queen cells when swarm fever rises
                    if apiary.swarm_fever > 30.0 && apiary.queen_cells == 0 {
                        apiary.queen_cells = 1;
                    }
                    if apiary.swarm_fever > 65.0 && apiary.queen_cells == 1 {
                        apiary.queen_cells = 2;
                    }
                    if apiary.swarm_fever > 90.0 && apiary.queen_cells == 2 {
                        apiary.queen_cells = 3;
                    }

                    // SWARMING EVENT!
                    if apiary.swarm_fever >= 100.0 && apiary.queen_cells > 0 {
                        apiary.bee_population /= 2; // Half the colony leaves
                        apiary.queen_cells = 0;
                        apiary.swarm_fever = 0.0;
                        notification.send("[경고: 분봉 발생] 벌통 과밀로 여왕벌과 일벌 절반이 분봉 탈출! 증소나 왕대 제거가 필요합니다!");
                    }
                } else {
                    // Sufficient frames: swarm fever naturally dissipates
                    apiary.swarm_fever = (apiary.swarm_fever - delta * 1.5).max(0.0);
                }

                // Honeycomb production (scaled by nearby floral density + feeder syrup)
                if apiary.frame_count > 0 {
                    let rate = apiary.honey_speed_multiplier();
                    apiary.production_timer += delta * rate;

                    if apiary.production_timer >= apiary.production_interval {
                        apiary.production_timer = 0.0;

                        // Check whether this batch is Sugar Honeycomb (사양 소비) or Natural Honeycomb (천연 꽃꿀)
                        let is_sugar_comb = apiary.feeder_syrup > 5.0 && (apiary.nearby_flowers == 0 || rand::random::<f32>() < 0.65);
                        let product_item = if is_sugar_comb {
                            ItemType::SugarHoneycomb
                        } else {
                            ItemType::Honeycomb(genome.primary_species.clone())
                        };

                        let mut added = false;
                        for out in apiary.output_combs.iter_mut() {
                            if out.item == product_item {
                                out.count += 1;
                                added = true;
                                break;
                            }
                        }
                        if !added && apiary.output_combs.len() < 5 {
                            apiary.output_combs.push(ItemStack::new(product_item, 1));
                        }
                    }
                }

                // Queen aging
                apiary.life_timer -= delta;
                if apiary.life_timer <= 0.0 {
                    apiary.is_active = false;
                    apiary.queen_slot = None;

                    let offspring_sp = genome.primary_species.clone();
                    apiary.queen_slot = Some(ItemStack::new(ItemType::PrincessBee(offspring_sp.clone()), 1));

                    let drone_count = genome.fertility_trait;
                    if let Some(d_stack) = apiary.drone_slot.as_mut() {
                        if let ItemType::DroneBee(sp) = &d_stack.item {
                            if sp == &offspring_sp {
                                d_stack.count += drone_count;
                            }
                        }
                    } else {
                        apiary.drone_slot = Some(ItemStack::new(ItemType::DroneBee(offspring_sp), drone_count));
                    }

                    apiary.queen_genome = None;
                }
            }
        }
    }
}

pub fn update_solitary_hotels(
    time: Res<Time>,
    mut hotels: Query<&mut SolitaryBeeHotel>,
) {
    let delta = time.delta_secs();
    for mut hotel in hotels.iter_mut() {
        hotel.timer += delta;
        if hotel.timer >= 12.0 {
            hotel.timer = 0.0;
            if hotel.occupancy < hotel.max_occupancy {
                hotel.occupancy += 1;
                hotel.propolis_stored += 1;
            }
        }
    }
}
