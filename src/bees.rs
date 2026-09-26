use bevy::prelude::Resource;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BeeTier {
    Tier1Wild,
    Tier2Hybrid,
    Tier3Refined,
    Tier4Exotic,
    Tier5Mythic,
    Solitary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BiomeType {
    PortHoneyBeaFather,
    EmeraldMeadows,
    WhisperwoodForest,
    SunkenShore,
    MangroveMarsh,
    ShimmeringPeaks,
    Any,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeeSpecies {
    pub id: String,
    pub common_name: String,
    pub latin_name: String,
    pub tier: BeeTier,
    pub preferred_biome: BiomeType,
    pub description: String,
    pub base_lifespan: f32, // In seconds for full queen cycle (e.g., 25-45s in sim time)
    pub base_speed: f32,    // Comb production interval (seconds per comb)
    pub fertility: u32,     // Drones spawned (1-4)
    pub diurnal: bool,      // Works during day
    pub nocturnal: bool,    // Works during night
    pub honey_type: String, // Type of honey jar produced
    pub is_solitary: bool,  // Lives in solitary hotels instead of apiary
    pub color_primary: [f32; 3],
    pub color_accent: [f32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeeGenome {
    pub primary_species: String,
    pub secondary_species: String,
    pub lifespan_trait: i32, // -1: short, 0: normal, +1: long
    pub speed_trait: i32,    // -1: slow, 0: normal, +1: fast
    pub fertility_trait: u32,// 1..=4
    pub is_nocturnal: bool,
    pub rain_tolerant: bool,
}

impl BeeGenome {
    pub fn pure(species_id: &str) -> Self {
        Self {
            primary_species: species_id.to_string(),
            secondary_species: species_id.to_string(),
            lifespan_trait: 0,
            speed_trait: 0,
            fertility_trait: 2,
            is_nocturnal: false,
            rain_tolerant: false,
        }
    }
}

#[derive(Resource)]
pub struct BeeCatalog {
    pub species: HashMap<String, BeeSpecies>,
    pub mutation_recipes: Vec<MutationRecipe>,
}

#[derive(Debug, Clone)]
pub struct MutationRecipe {
    pub parent_a: &'static str,
    pub parent_b: &'static str,
    pub result: &'static str,
    pub chance: f32, // e.g. 0.35 (35%)
}

impl BeeCatalog {
    pub fn new() -> Self {
        let mut species = HashMap::new();

        macro_rules! add_bee {
            ($id:expr, $name:expr, $latin:expr, $tier:expr, $biome:expr, $desc:expr, $life:expr, $speed:expr, $fert:expr, $diurnal:expr, $nocturnal:expr, $honey:expr, $solitary:expr, $c1:expr, $c2:expr) => {
                species.insert(
                    $id.to_string(),
                    BeeSpecies {
                        id: $id.to_string(),
                        common_name: $name.to_string(),
                        latin_name: $latin.to_string(),
                        tier: $tier,
                        preferred_biome: $biome,
                        description: $desc.to_string(),
                        base_lifespan: $life,
                        base_speed: $speed,
                        fertility: $fert,
                        diurnal: $diurnal,
                        nocturnal: $nocturnal,
                        honey_type: $honey.to_string(),
                        is_solitary: $solitary,
                        color_primary: $c1,
                        color_accent: $c2,
                    },
                );
            };
        }

        // TIER 1 - WILD SPECIES (Base Foundations)
        add_bee!("common", "Common Bee", "Apis Communis", BeeTier::Tier1Wild, BiomeType::PortHoneyBeaFather, "The trusty honeybee of Port Gijang. Gentle and dependable.", 25.0, 5.0, 2, true, false, "Wildflower", false, [0.85, 0.65, 0.15], [0.2, 0.15, 0.05]);
        add_bee!("meadow", "Meadow Bee", "Apis Prata", BeeTier::Tier1Wild, BiomeType::EmeraldMeadows, "Loves open grasslands and rolling hills covered in clover.", 28.0, 4.5, 2, true, false, "Clover", false, [0.95, 0.8, 0.2], [0.3, 0.6, 0.2]);
        add_bee!("forest", "Forest Bee", "Apis Sylvestris", BeeTier::Tier1Wild, BiomeType::WhisperwoodForest, "Nests in hollow cedar trunks deep within the ancient canopy.", 30.0, 5.5, 2, true, false, "Oak Bark", false, [0.55, 0.4, 0.2], [0.15, 0.35, 0.1]);
        add_bee!("coastal", "Coastal Bee", "Apis Maritima", BeeTier::Tier1Wild, BiomeType::SunkenShore, "Thrives near salty sea breezes and sandy dunes.", 26.0, 5.0, 2, true, false, "Salty Nectar", false, [0.3, 0.7, 0.85], [0.9, 0.8, 0.4]);
        add_bee!("swamp", "Swamp Bee", "Apis Paludis", BeeTier::Tier1Wild, BiomeType::MangroveMarsh, "Hardy bee resistant to moisture and humid mangrove breezes.", 32.0, 6.0, 2, true, true, "Mangrove", false, [0.35, 0.5, 0.3], [0.2, 0.25, 0.15]);
        add_bee!("mountain", "Mountain Bee", "Apis Montis", BeeTier::Tier1Wild, BiomeType::ShimmeringPeaks, "Endures brisk alpine temperatures with thick insulating fuzz.", 34.0, 6.5, 1, true, false, "Alpine Frost", false, [0.75, 0.85, 0.95], [0.3, 0.4, 0.6]);
        add_bee!("rock", "Rock Bee", "Apis Saxum", BeeTier::Tier1Wild, BiomeType::ShimmeringPeaks, "Builds sturdy comb inside granite fissures.", 35.0, 7.0, 2, true, false, "Mineral Amber", false, [0.5, 0.5, 0.55], [0.8, 0.4, 0.1]);

        // SOLITARY BEES (Live in craftable hotels, promote flora & produce rare solitary propolis)
        add_bee!("solitary_mason", "Orchard Mason Bee", "Osmia Lignaria", BeeTier::Solitary, BiomeType::EmeraldMeadows, "A gentle solitary pollinator nesting in hollow reeds.", 40.0, 8.0, 1, true, false, "Mason Balm", true, [0.2, 0.4, 0.7], [0.1, 0.1, 0.2]);
        add_bee!("solitary_leafcutter", "Leafcutter Bee", "Megachile Rotundata", BeeTier::Solitary, BiomeType::WhisperwoodForest, "Cuts neat circular leaf fragments to build nursery cells.", 40.0, 8.0, 1, true, false, "Leaf Resin", true, [0.3, 0.6, 0.3], [0.1, 0.2, 0.1]);
        add_bee!("solitary_carpenter", "Blue Carpenter Bee", "Xylocopa Caerulea", BeeTier::Solitary, BiomeType::PortHoneyBeaFather, "Large, shimmering iridescent blue solitary bee with deep hum.", 45.0, 9.0, 1, true, false, "Wood Honey", true, [0.1, 0.2, 0.9], [0.0, 0.8, 0.9]);
        add_bee!("solitary_orchid", "Glittering Orchid Bee", "Euglossa Bidentata", BeeTier::Solitary, BiomeType::MangroveMarsh, "Brilliant metallic green solitary bee collecting floral fragrances.", 42.0, 8.5, 1, true, false, "Fragrant Essence", true, [0.0, 0.85, 0.45], [0.9, 0.95, 0.2]);

        // TIER 2 - FIRST HYBRIDS (10 Species)
        add_bee!("verdant", "Verdant Bee", "Apis Vridis", BeeTier::Tier2Hybrid, BiomeType::WhisperwoodForest, "Cross of Meadow and Forest. Radiates revitalizing energy.", 28.0, 4.2, 2, true, false, "Sprout Nectar", false, [0.4, 0.8, 0.3], [0.85, 0.95, 0.3]);
        add_bee!("dewdrop", "Dewdrop Bee", "Apis Rorida", BeeTier::Tier2Hybrid, BiomeType::SunkenShore, "Cross of Meadow and Coastal. Gathers morning condensation.", 26.0, 4.0, 2, true, false, "Dewdrop", false, [0.5, 0.85, 0.95], [0.2, 0.4, 0.7]);
        add_bee!("mossy", "Mossy Bee", "Apis Muscosus", BeeTier::Tier2Hybrid, BiomeType::WhisperwoodForest, "Cross of Forest and Swamp. Covered in soft green moss velvet.", 32.0, 5.0, 2, true, true, "Peat Honey", false, [0.35, 0.6, 0.25], [0.15, 0.3, 0.1]);
        add_bee!("sandy", "Sandy Bee", "Apis Arenosa", BeeTier::Tier2Hybrid, BiomeType::SunkenShore, "Cross of Coastal and Meadow. Thrives in warm coastal sands.", 27.0, 4.8, 2, true, false, "Golden Sand", false, [0.9, 0.8, 0.5], [0.7, 0.5, 0.2]);
        add_bee!("frosty", "Frosty Bee", "Apis Glacialis", BeeTier::Tier2Hybrid, BiomeType::ShimmeringPeaks, "Cross of Mountain and Coastal. Leaves crystalline frost on frames.", 30.0, 5.2, 2, true, false, "Chill Glaze", false, [0.85, 0.95, 1.0], [0.4, 0.6, 0.8]);
        add_bee!("stony", "Stony Bee", "Apis Lithos", BeeTier::Tier2Hybrid, BiomeType::ShimmeringPeaks, "Cross of Mountain and Rock. Hardened chitin protects it from mountain gales.", 34.0, 6.0, 2, true, false, "Granite Paste", false, [0.6, 0.6, 0.65], [0.3, 0.3, 0.35]);
        add_bee!("blossom", "Blossom Bee", "Apis Floralis", BeeTier::Tier2Hybrid, BiomeType::EmeraldMeadows, "Cross of Meadow and Verdant. Has an uncanny affinity for flowers.", 25.0, 3.8, 3, true, false, "Petal Blossom", false, [0.95, 0.45, 0.65], [1.0, 0.8, 0.9]);
        add_bee!("bark", "Bark Bee", "Apis Cortex", BeeTier::Tier2Hybrid, BiomeType::WhisperwoodForest, "Cross of Forest and Rock. Its comb has the texture of aged cedar bark.", 31.0, 5.4, 2, true, false, "Resin Treacle", false, [0.45, 0.3, 0.15], [0.65, 0.45, 0.2]);
        add_bee!("reed", "Reed Bee", "Apis Arundo", BeeTier::Tier2Hybrid, BiomeType::MangroveMarsh, "Cross of Coastal and Swamp. Weaves slender comb among marsh reeds.", 29.0, 4.9, 2, true, true, "River Dew", false, [0.55, 0.7, 0.45], [0.3, 0.4, 0.2]);
        add_bee!("mud", "Mud Bee", "Apis Lutum", BeeTier::Tier2Hybrid, BiomeType::MangroveMarsh, "Cross of Swamp and Rock. Mixes mineral clay into rich propolis.", 33.0, 5.8, 2, true, true, "Terracotta Honey", false, [0.5, 0.35, 0.25], [0.25, 0.15, 0.1]);

        // TIER 3 - REFINED HYBRIDS (10 Species)
        add_bee!("honeycomb", "Honeycomb Bee", "Apis Favus", BeeTier::Tier3Refined, BiomeType::EmeraldMeadows, "Blossom + Dewdrop. High yield bee producing golden combs twice as fast.", 24.0, 3.2, 3, true, false, "Liquid Gold", false, [1.0, 0.75, 0.1], [0.95, 0.5, 0.0]);
        add_bee!("amber", "Amber Bee", "Apis Electrum", BeeTier::Tier3Refined, BiomeType::WhisperwoodForest, "Bark + Mossy. Drops fossilized sweet amber alongside honey.", 30.0, 4.6, 2, true, false, "Fossil Amber", false, [0.9, 0.55, 0.1], [0.6, 0.2, 0.05]);
        add_bee!("golden", "Golden Bee", "Apis Aurum", BeeTier::Tier3Refined, BiomeType::EmeraldMeadows, "Blossom + Honeycomb. Shimmers with pure golden luster.", 26.0, 3.5, 3, true, false, "Royal Gold", false, [1.0, 0.85, 0.2], [1.0, 0.95, 0.6]);
        add_bee!("twilight", "Twilight Bee", "Apis Crepusculum", BeeTier::Tier3Refined, BiomeType::WhisperwoodForest, "Verdant + Frosty. Forages strictly during dawn and dusk.", 28.0, 4.0, 2, false, true, "Twilight Nectar", false, [0.45, 0.25, 0.65], [0.75, 0.45, 0.85]);
        add_bee!("dawn", "Dawn Bee", "Apis Aurora", BeeTier::Tier3Refined, BiomeType::SunkenShore, "Dewdrop + Blossom. Awakens with the very first morning light.", 25.0, 3.6, 3, true, false, "Morning Gleam", false, [0.95, 0.6, 0.4], [1.0, 0.85, 0.5]);
        add_bee!("coral", "Coral Bee", "Apis Corallium", BeeTier::Tier3Refined, BiomeType::SunkenShore, "Dewdrop + Coastal. Builds honeycomb resembling vibrant sea reefs.", 27.0, 4.2, 2, true, false, "Tide Pool Honey", false, [0.95, 0.4, 0.45], [0.3, 0.8, 0.9]);
        add_bee!("mist", "Mist Bee", "Apis Nebula", BeeTier::Tier3Refined, BiomeType::MangroveMarsh, "Dewdrop + Swamp. Shrouded in a calming vapor that calms other bees.", 30.0, 4.5, 2, true, true, "Vapor Honey", false, [0.7, 0.8, 0.85], [0.4, 0.6, 0.7]);
        add_bee!("flora", "Flora Bee", "Apis Botanica", BeeTier::Tier3Refined, BiomeType::EmeraldMeadows, "Blossom + Meadow. Supercharges flower growth around its hive.", 24.0, 3.4, 3, true, false, "Floral Elixir", false, [0.85, 0.3, 0.6], [0.95, 0.8, 0.3]);
        add_bee!("crystal", "Crystal Bee", "Apis Crystallum", BeeTier::Tier3Refined, BiomeType::ShimmeringPeaks, "Frosty + Stony. Produces crystallized sugar shards in lieu of wax.", 32.0, 4.8, 2, true, false, "Sugar Crystal", false, [0.8, 0.95, 1.0], [0.6, 0.8, 0.95]);
        add_bee!("prisma", "Prisma Bee", "Apis Iris", BeeTier::Tier3Refined, BiomeType::ShimmeringPeaks, "Crystal + Dewdrop. Refracts sunlight into a dazzling rainbow aura.", 28.0, 3.9, 2, true, false, "Prismatic Nectar", false, [0.9, 0.3, 0.8], [0.2, 0.9, 0.8]);

        // TIER 4 - EXOTIC & RARE (10 Species)
        add_bee!("nebula", "Nebula Bee", "Apis Astri", BeeTier::Tier4Exotic, BiomeType::ShimmeringPeaks, "Prisma + Twilight. Cosmic dusted wings resembling distant galaxies.", 30.0, 3.8, 2, false, true, "Starlight Honey", false, [0.3, 0.1, 0.55], [0.85, 0.3, 0.9]);
        add_bee!("solar", "Solar Bee", "Apis Helios", BeeTier::Tier4Exotic, BiomeType::EmeraldMeadows, "Dawn + Golden. Absorbs solar rays to heat its hive to optimal temperature.", 26.0, 3.0, 3, true, false, "Solar Flare Honey", false, [1.0, 0.6, 0.0], [1.0, 0.9, 0.2]);
        add_bee!("lunar", "Lunar Bee", "Apis Selene", BeeTier::Tier4Exotic, BiomeType::WhisperwoodForest, "Twilight + Mist. Forages under full moon, producing glowing luminescent honey.", 28.0, 3.6, 2, false, true, "Moonlit Honey", false, [0.65, 0.75, 0.95], [0.9, 0.95, 1.0]);
        add_bee!("volcanic", "Volcanic Bee", "Apis Vulcani", BeeTier::Tier4Exotic, BiomeType::ShimmeringPeaks, "Stony + Mud. Breathes gentle warmth that keeps hives active in freezing snow.", 32.0, 4.4, 2, true, true, "Obsidian Honey", false, [0.8, 0.25, 0.1], [0.2, 0.1, 0.1]);
        add_bee!("magma", "Magma Bee", "Apis Magmatis", BeeTier::Tier4Exotic, BiomeType::ShimmeringPeaks, "Volcanic + Amber. Boiling sweet magma honey prized by royal confectioners.", 30.0, 4.0, 2, true, false, "Lava Honey", false, [0.95, 0.4, 0.05], [0.5, 0.1, 0.0]);
        add_bee!("ocean", "Oceanic Bee", "Apis Oceanus", BeeTier::Tier4Exotic, BiomeType::SunkenShore, "Coral + Reed. Capable of diving over shallow tidepools for aquatic pollen.", 27.0, 3.7, 2, true, false, "Brine Honey", false, [0.1, 0.5, 0.7], [0.4, 0.9, 0.9]);
        add_bee!("abyssal", "Abyssal Bee", "Apis Profundus", BeeTier::Tier4Exotic, BiomeType::SunkenShore, "Ocean + Swamp. Deep black and bioluminescent cyan stripes.", 33.0, 4.6, 2, false, true, "Abyssal Jelly", false, [0.05, 0.1, 0.2], [0.0, 0.8, 0.7]);
        add_bee!("radiant", "Radiant Bee", "Apis Radiata", BeeTier::Tier4Exotic, BiomeType::EmeraldMeadows, "Solar + Prisma. Emits bright beams of golden light while humming.", 25.0, 2.9, 3, true, false, "Radiant Ambrosia", false, [1.0, 0.9, 0.4], [1.0, 1.0, 0.9]);
        add_bee!("glacial", "Glacial Bee", "Apis Borealis", BeeTier::Tier4Exotic, BiomeType::ShimmeringPeaks, "Frosty + Crystal. Comb is composed of perpetual mountain glacier ice.", 31.0, 4.2, 2, true, false, "Glacial Syrup", false, [0.7, 0.85, 1.0], [0.2, 0.5, 0.8]);
        add_bee!("emerald", "Emerald Bee", "Apis Smaragdina", BeeTier::Tier4Exotic, BiomeType::EmeraldMeadows, "Verdant + Prisma. Wings glisten like polished emerald gemstones.", 26.0, 3.2, 3, true, false, "Emerald Dew", false, [0.1, 0.8, 0.4], [0.4, 1.0, 0.6]);

        // TIER 5 - MYTHIC & ANCESTRAL CONSERVATION RELICS (6 Species)
        add_bee!("ancient", "Ancient Bee", "Apis Antiqua", BeeTier::Tier5Mythic, BiomeType::WhisperwoodForest, "Amber + Nebula. Rediscovered from prehistoric resin. Holds island memories.", 35.0, 3.5, 2, true, true, "Primordial Mead", false, [0.75, 0.5, 0.2], [0.95, 0.8, 0.3]);
        add_bee!("ethereal", "Ethereal Bee", "Apis Aetheria", BeeTier::Tier5Mythic, BiomeType::ShimmeringPeaks, "Nebula + Lunar. Floats effortlessly as if unburdened by gravity.", 32.0, 3.0, 2, true, true, "Spirit Nectar", false, [0.8, 0.7, 0.95], [0.95, 0.9, 1.0]);
        add_bee!("celestial", "Celestial Bee", "Apis Caelestis", BeeTier::Tier5Mythic, BiomeType::EmeraldMeadows, "Radiant + Ethereal. A descendant of the starry night heavens.", 30.0, 2.6, 3, true, true, "Astral Honey", false, [0.9, 0.85, 0.4], [0.4, 0.7, 1.0]);
        add_bee!("chrono", "Chrono Bee", "Apis Chronos", BeeTier::Tier5Mythic, BiomeType::SunkenShore, "Ancient + Radiant. Whispers of the past and future hum from its hive.", 34.0, 2.8, 2, true, true, "Temporal Honey", false, [0.65, 0.55, 0.85], [1.0, 0.8, 0.2]);
        add_bee!("nirvana", "Nirvana Bee", "Apis Nirvana", BeeTier::Tier5Mythic, BiomeType::WhisperwoodForest, "Ethereal + Celestial. Brings total harmony and peace to all surrounding nature.", 32.0, 2.5, 3, true, true, "Elixir of Peace", false, [0.95, 0.95, 0.9], [0.7, 0.85, 0.7]);
        add_bee!("honeybeafather", "HoneyBeaFather Ancestral Queen", "Apis Progenitor", BeeTier::Tier5Mythic, BiomeType::PortHoneyBeaFather, "The legendary founding matriarch of Port Gijang! Fully restores the Gijang archipelago.", 40.0, 2.0, 4, true, true, "HoneyBeaFatherla Ambrosia", false, [1.0, 0.8, 0.15], [0.9, 0.2, 0.4]);

        let mutation_recipes = vec![
            // Tier 1 -> Tier 2
            MutationRecipe { parent_a: "forest", parent_b: "meadow", result: "verdant", chance: 0.40 },
            MutationRecipe { parent_a: "meadow", parent_b: "coastal", result: "dewdrop", chance: 0.40 },
            MutationRecipe { parent_a: "forest", parent_b: "swamp", result: "mossy", chance: 0.40 },
            MutationRecipe { parent_a: "coastal", parent_b: "meadow", result: "sandy", chance: 0.35 },
            MutationRecipe { parent_a: "mountain", parent_b: "coastal", result: "frosty", chance: 0.40 },
            MutationRecipe { parent_a: "mountain", parent_b: "rock", result: "stony", chance: 0.40 },
            MutationRecipe { parent_a: "meadow", parent_b: "verdant", result: "blossom", chance: 0.35 },
            MutationRecipe { parent_a: "forest", parent_b: "rock", result: "bark", chance: 0.35 },
            MutationRecipe { parent_a: "coastal", parent_b: "swamp", result: "reed", chance: 0.35 },
            MutationRecipe { parent_a: "swamp", parent_b: "rock", result: "mud", chance: 0.35 },

            // Tier 2 -> Tier 3
            MutationRecipe { parent_a: "blossom", parent_b: "dewdrop", result: "honeycomb", chance: 0.30 },
            MutationRecipe { parent_a: "bark", parent_b: "mossy", result: "amber", chance: 0.30 },
            MutationRecipe { parent_a: "blossom", parent_b: "honeycomb", result: "golden", chance: 0.25 },
            MutationRecipe { parent_a: "verdant", parent_b: "frosty", result: "twilight", chance: 0.30 },
            MutationRecipe { parent_a: "dewdrop", parent_b: "blossom", result: "dawn", chance: 0.30 },
            MutationRecipe { parent_a: "dewdrop", parent_b: "coastal", result: "coral", chance: 0.30 },
            MutationRecipe { parent_a: "dewdrop", parent_b: "swamp", result: "mist", chance: 0.30 },
            MutationRecipe { parent_a: "blossom", parent_b: "meadow", result: "flora", chance: 0.30 },
            MutationRecipe { parent_a: "frosty", parent_b: "stony", result: "crystal", chance: 0.28 },
            MutationRecipe { parent_a: "crystal", parent_b: "dewdrop", result: "prisma", chance: 0.25 },

            // Tier 3 -> Tier 4
            MutationRecipe { parent_a: "prisma", parent_b: "twilight", result: "nebula", chance: 0.22 },
            MutationRecipe { parent_a: "dawn", parent_b: "golden", result: "solar", chance: 0.22 },
            MutationRecipe { parent_a: "twilight", parent_b: "mist", result: "lunar", chance: 0.22 },
            MutationRecipe { parent_a: "stony", parent_b: "mud", result: "volcanic", chance: 0.25 },
            MutationRecipe { parent_a: "volcanic", parent_b: "amber", result: "magma", chance: 0.20 },
            MutationRecipe { parent_a: "coral", parent_b: "reed", result: "ocean", chance: 0.22 },
            MutationRecipe { parent_a: "ocean", parent_b: "swamp", result: "abyssal", chance: 0.20 },
            MutationRecipe { parent_a: "solar", parent_b: "prisma", result: "radiant", chance: 0.18 },
            MutationRecipe { parent_a: "frosty", parent_b: "crystal", result: "glacial", chance: 0.20 },
            MutationRecipe { parent_a: "verdant", parent_b: "prisma", result: "emerald", chance: 0.22 },

            // Tier 4 -> Tier 5 (Mythic)
            MutationRecipe { parent_a: "amber", parent_b: "nebula", result: "ancient", chance: 0.15 },
            MutationRecipe { parent_a: "nebula", parent_b: "lunar", result: "ethereal", chance: 0.15 },
            MutationRecipe { parent_a: "radiant", parent_b: "ethereal", result: "celestial", chance: 0.12 },
            MutationRecipe { parent_a: "ancient", parent_b: "radiant", result: "chrono", chance: 0.12 },
            MutationRecipe { parent_a: "ethereal", parent_b: "celestial", result: "nirvana", chance: 0.10 },
            MutationRecipe { parent_a: "golden", parent_b: "celestial", result: "honeybeafather", chance: 0.08 },
        ];

        Self {
            species,
            mutation_recipes,
        }
    }

    pub fn get(&self, id: &str) -> Option<&BeeSpecies> {
        self.species.get(id)
    }

    /// Check if two parents mutate into a new species
    pub fn try_mutate(&self, parent_a: &str, parent_b: &str, bonus_mutation_rate: f32) -> Option<&'static str> {
        let mut rng = rand::thread_rng();
        for recipe in &self.mutation_recipes {
            let matches = (recipe.parent_a == parent_a && recipe.parent_b == parent_b)
                || (recipe.parent_a == parent_b && recipe.parent_b == parent_a);

            if matches {
                let effective_chance = (recipe.chance + bonus_mutation_rate).min(0.95);
                if rng.gen::<f32>() < effective_chance {
                    return Some(recipe.result);
                }
            }
        }
        None
    }
}
