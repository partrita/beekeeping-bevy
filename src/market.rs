use bevy::prelude::*;
use crate::items::ItemType;

#[derive(Resource)]
pub struct MarketEconomy {
    pub player_coins: u32,
    pub shop_items: Vec<ShopEntry>,
}

#[derive(Debug, Clone)]
pub struct ShopEntry {
    pub name: &'static str,
    pub description: &'static str,
    pub price: u32,
    pub item: ItemType,
    pub purchased: bool,
    pub is_repeatable: bool,
}

impl Default for MarketEconomy {
    fn default() -> Self {
        let shop_items = vec![
            ShopEntry {
                name: "Empty Glass Jars (Pack of 5)",
                description: "Essential for bottling extracted honey & beverages.",
                price: 15,
                item: ItemType::GlassJar,
                purchased: false,
                is_repeatable: true,
            },
            ShopEntry {
                name: "Ready-to-use Frame Pack",
                description: "3 pre-built wooden frames for your apiaries.",
                price: 20,
                item: ItemType::EmptyFrame,
                purchased: false,
                is_repeatable: true,
            },
            ShopEntry {
                name: "Wildflower Seed Packet (x3)",
                description: "Plant flowers near hives to dramatically boost honey nectar gathering!",
                price: 10,
                item: ItemType::FlowerSeed(crate::items::FlowerColor::Yellow),
                purchased: false,
                is_repeatable: true,
            },
            ShopEntry {
                name: "White Sugar Sack (설탕 포대)",
                description: "Refined sugar to feed bees or craft syrup and pollen patties.",
                price: 6,
                item: ItemType::Sugar,
                purchased: false,
                is_repeatable: true,
            },
            ShopEntry {
                name: "Prepared Sugar Feed Syrup (사양액)",
                description: "Ready-to-use liquid feed for hives to produce sugar-fed honey.",
                price: 12,
                item: ItemType::SugarSyrup,
                purchased: false,
                is_repeatable: true,
            },
            ShopEntry {
                name: "Ferry Ticket: Emerald Meadows",
                description: "Unlocks boat travel to the flower-strewn Emerald Meadows.",
                price: 40,
                item: ItemType::FerryTicket("Emerald Meadows".to_string()),
                purchased: false,
                is_repeatable: false,
            },
            ShopEntry {
                name: "Ferry Ticket: Whisperwood Forest",
                description: "Unlocks boat travel to the ancient Whisperwood Forest.",
                price: 80,
                item: ItemType::FerryTicket("Whisperwood Forest".to_string()),
                purchased: false,
                is_repeatable: false,
            },
            ShopEntry {
                name: "Ferry Ticket: Sunken Shore",
                description: "Unlocks boat travel to the salty tidepool dunes.",
                price: 120,
                item: ItemType::FerryTicket("Sunken Shore".to_string()),
                purchased: false,
                is_repeatable: false,
            },
            ShopEntry {
                name: "Ferry Ticket: Mangrove Marsh",
                description: "Unlocks boat travel to the humid Mangrove wetlands.",
                price: 160,
                item: ItemType::FerryTicket("Mangrove Marsh".to_string()),
                purchased: false,
                is_repeatable: false,
            },
            ShopEntry {
                name: "Ferry Ticket: Shimmering Peaks",
                description: "Unlocks boat travel to the high alpine crystalline summit.",
                price: 220,
                item: ItemType::FerryTicket("Shimmering Peaks".to_string()),
                purchased: false,
                is_repeatable: false,
            },
        ];

        Self {
            player_coins: 50, // Starting money
            shop_items,
        }
    }
}

impl MarketEconomy {
    pub fn sell_item(&mut self, item: &ItemType, count: u32, sell_multiplier: f32) -> u32 {
        let base = item.base_sell_value();
        let total = ((base as f32 * count as f32 * sell_multiplier).round()) as u32;
        self.player_coins += total;
        total
    }
}
