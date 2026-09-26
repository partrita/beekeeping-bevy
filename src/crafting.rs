use bevy::prelude::Resource;
use crate::items::{ItemStack, ItemType, FlowerColor};

#[derive(Debug, Clone)]
pub struct CraftingRecipe {
    pub name: &'static str,
    pub description: &'static str,
    pub inputs: Vec<ItemStack>,
    pub output: ItemStack,
}

#[derive(Resource)]
pub struct CraftingRegistry {
    pub recipes: Vec<CraftingRecipe>,
}

impl CraftingRegistry {
    pub fn new() -> Self {
        let recipes = vec![
            CraftingRecipe {
                name: "Wooden Frame",
                description: "Basic frame for apiaries to collect comb.",
                inputs: vec![ItemStack::new(ItemType::Wood, 2)],
                output: ItemStack::new(ItemType::EmptyFrame, 1),
            },
            CraftingRecipe {
                name: "Waxed Frame",
                description: "Coated with sweet beeswax to accelerate comb gathering.",
                inputs: vec![
                    ItemStack::new(ItemType::EmptyFrame, 1),
                    ItemStack::new(ItemType::Beeswax, 1),
                ],
                output: ItemStack::new(ItemType::WaxedFrame, 1),
            },
            CraftingRecipe {
                name: "Standard Apiary",
                description: "A cozy wooden home for a queen, drones, and frames.",
                inputs: vec![
                    ItemStack::new(ItemType::Wood, 5),
                    ItemStack::new(ItemType::Stone, 2),
                ],
                output: ItemStack::new(ItemType::ApiaryBox, 1),
            },
            CraftingRecipe {
                name: "Solitary Bee Hotel",
                description: "Hollow reed sanctuary attracting solitary pollinators.",
                inputs: vec![
                    ItemStack::new(ItemType::Wood, 3),
                    ItemStack::new(ItemType::Clay, 2),
                ],
                output: ItemStack::new(ItemType::SolitaryBeeHotel, 1),
            },
            CraftingRecipe {
                name: "Butterfly Net",
                description: "Catch wild fluttering butterflies across the meadows.",
                inputs: vec![
                    ItemStack::new(ItemType::Wood, 3),
                    ItemStack::new(ItemType::Wildflower(FlowerColor::Yellow), 1),
                ],
                output: ItemStack::new(ItemType::ButterflyNet, 1),
            },
            CraftingRecipe {
                name: "Beekeeper Smoker",
                description: "Calms wild hives to safely collect queens and drones.",
                inputs: vec![
                    ItemStack::new(ItemType::Stone, 3),
                    ItemStack::new(ItemType::Wood, 2),
                ],
                output: ItemStack::new(ItemType::BeekeeperSmoker, 1),
            },
            CraftingRecipe {
                name: "Honey Centrifuge",
                description: "Extractor machine to spin honey out of raw comb.",
                inputs: vec![
                    ItemStack::new(ItemType::Wood, 6),
                    ItemStack::new(ItemType::Stone, 4),
                    ItemStack::new(ItemType::Beeswax, 1),
                ],
                output: ItemStack::new(ItemType::CentrifugeExtractor, 1),
            },
            CraftingRecipe {
                name: "HoneyBeaFatherla (Soda)",
                description: "Port Gijang's famous effervescent sparkling honey soda!",
                inputs: vec![
                    ItemStack::new(ItemType::HoneyJar("Wildflower".to_string()), 1),
                    ItemStack::new(ItemType::GlassJar, 1),
                ],
                output: ItemStack::new(ItemType::HoneyBeaFatherla, 2),
            },
            CraftingRecipe {
                name: "Aromatic Beeswax Candle",
                description: "Sweet honey scented candle sold for good coin.",
                inputs: vec![
                    ItemStack::new(ItemType::Beeswax, 2),
                    ItemStack::new(ItemType::Wood, 1),
                ],
                output: ItemStack::new(ItemType::BeeswaxCandle, 1),
            },
            CraftingRecipe {
                name: "Flower Seed Pack (x2)",
                description: "Extract seeds from fresh wildflowers to plant flower fields.",
                inputs: vec![
                    ItemStack::new(ItemType::Wildflower(FlowerColor::Yellow), 1),
                ],
                output: ItemStack::new(ItemType::FlowerSeed(FlowerColor::Yellow), 2),
            },
            CraftingRecipe {
                name: "Refined Royal Jelly",
                description: "Process natural queen cells from hive inspection into precious royal jelly.",
                inputs: vec![
                    ItemStack::new(ItemType::QueenCell, 1),
                    ItemStack::new(ItemType::GlassJar, 1),
                ],
                output: ItemStack::new(ItemType::RoyalJelly, 2),
            },
            CraftingRecipe {
                name: "Bee Sugar Feed Syrup (사양액)",
                description: "Dissolve sugar in glass jars to feed bees during dearth periods.",
                inputs: vec![
                    ItemStack::new(ItemType::Sugar, 2),
                    ItemStack::new(ItemType::GlassJar, 1),
                ],
                output: ItemStack::new(ItemType::SugarSyrup, 2),
            },
            CraftingRecipe {
                name: "Brood Pollen Patty (화분떡)",
                description: "Mix sugar and floral pollen to dramatically accelerate brood rearing.",
                inputs: vec![
                    ItemStack::new(ItemType::Sugar, 1),
                    ItemStack::new(ItemType::Wildflower(FlowerColor::Yellow), 1),
                ],
                output: ItemStack::new(ItemType::PollenPatty, 2),
            },
            CraftingRecipe {
                name: "HoneyBeaFatherla (Sugar Honey)",
                description: "Brew affordable sparkling honey soda using sugar-fed honey!",
                inputs: vec![
                    ItemStack::new(ItemType::SugarHoneyJar, 1),
                    ItemStack::new(ItemType::GlassJar, 1),
                ],
                output: ItemStack::new(ItemType::HoneyBeaFatherla, 2),
            },
        ];

        Self { recipes }
    }
}
