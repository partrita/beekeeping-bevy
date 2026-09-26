use bevy::prelude::*;
use crate::bees::BiomeType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IslandLoreEntry {
    pub title: &'static str,
    pub biome: BiomeType,
    pub content: &'static str,
    pub unlocked: bool,
}

#[derive(Resource)]
pub struct ConservationTracker {
    pub island_health: HashMap<BiomeType, f32>, // 0.0 to 100.0%
    pub repopulated_species: Vec<String>,
    pub lore_entries: Vec<IslandLoreEntry>,
    pub ancestral_queen_unlocked: bool,
}

impl Default for ConservationTracker {
    fn default() -> Self {
        let mut island_health = HashMap::new();
        island_health.insert(BiomeType::PortHoneyBeaFather, 25.0);
        island_health.insert(BiomeType::EmeraldMeadows, 0.0);
        island_health.insert(BiomeType::WhisperwoodForest, 0.0);
        island_health.insert(BiomeType::SunkenShore, 0.0);
        island_health.insert(BiomeType::MangroveMarsh, 0.0);
        island_health.insert(BiomeType::ShimmeringPeaks, 0.0);

        let lore_entries = vec![
            IslandLoreEntry {
                title: "Grandfather's First Log - The Return",
                biome: BiomeType::PortHoneyBeaFather,
                content: "If you are reading this, grandchild, you've left the noise of the concrete city behind. Port Gijang was once the jewel of solitary and social bees alike. Take my smoker, tend to the old hives, and watch how cross-pollination breathes soul back into this harbor.",
                unlocked: true,
            },
            IslandLoreEntry {
                title: "Meadow Secrets - The Verdant Bloom",
                biome: BiomeType::EmeraldMeadows,
                content: "When Meadow Bees and Forest Bees share a hive, the green sap of the ancient canopy fuses with field nectar to create the Verdant Bee. Releasing them back into the wild will spark new wildflower varieties across Gijang.",
                unlocked: false,
            },
            IslandLoreEntry {
                title: "Whisperwood Annals - Solitary Guardians",
                biome: BiomeType::WhisperwoodForest,
                content: "Do not neglect the solitary ones! The Orchard Mason and Leafcutter bees don't produce royal jelly, but their gentle craftsmanship creates propolis of incomparable purity.",
                unlocked: false,
            },
            IslandLoreEntry {
                title: "Tides of Sunken Shore - Salty Pearls",
                biome: BiomeType::SunkenShore,
                content: "The Coastal Bees taught me patience. In the salt spray, they craft Dewdrop honeycombs that quench the thirst of any weary traveler.",
                unlocked: false,
            },
            IslandLoreEntry {
                title: "Mangrove Mysteries - The Fog Song",
                biome: BiomeType::MangroveMarsh,
                content: "Through the humid mist, the Mist Bee hums at a pitch that calms wild swarms without needing smoke. Release them, and the swamps will bloom with silver lilies.",
                unlocked: false,
            },
            IslandLoreEntry {
                title: "Peak Prophecy - The HoneyBeaFather Ancestral Queen",
                biome: BiomeType::ShimmeringPeaks,
                content: "At the pinnacle of the Shimmering Peaks, when the Golden Queen of the plains meets the Celestial Queen of the starry heavens, the legendary HoneyBeaFather Ancestral Queen shall return, bringing everlasting harmony to the Gijang archipelago!",
                unlocked: false,
            },
        ];

        Self {
            island_health,
            repopulated_species: Vec::new(),
            lore_entries,
            ancestral_queen_unlocked: false,
        }
    }
}

impl ConservationTracker {
    pub fn release_species(&mut self, biome: BiomeType, species_id: &str) {
        if !self.repopulated_species.contains(&species_id.to_string()) {
            self.repopulated_species.push(species_id.to_string());
        }

        let current = self.island_health.entry(biome).or_insert(0.0);
        *current = (*current + 33.4).min(100.0);

        if *current >= 99.0 {
            // Unlock lore for this biome
            for entry in &mut self.lore_entries {
                if entry.biome == biome {
                    entry.unlocked = true;
                }
            }
        }

        // Check if all islands are restored
        let total_health: f32 = self.island_health.values().sum();
        if total_health >= 590.0 {
            self.ancestral_queen_unlocked = true;
        }
    }
}
