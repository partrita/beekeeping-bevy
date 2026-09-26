use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemType {
    // Raw resources
    Wood,
    Stone,
    Clay,
    Bamboo,
    Wildflower(FlowerColor),
    FlowerSeed(FlowerColor), // Plantable floral seed
    GlassJar,

    // Bee equipment & structures
    EmptyFrame,
    WaxedFrame,
    ApiaryBox,
    SolitaryBeeHotel,
    CentrifugeExtractor,
    FermenterStation,
    ButterflyNet,
    BeekeeperSmoker,

    // Bee items
    QueenBee(String),    // Species key
    PrincessBee(String), // Species key
    DroneBee(String),    // Species key
    Honeycomb(String),   // Species key (yields specific honey)
    QueenCell,           // Harvested queen cell (왕대) from inspection

    // Beekeeping products
    HoneyJar(String),    // Honey variety
    Beeswax,
    RoyalJelly,
    Propolis,
    HoneyBeaFatherla,    // Signature sparkling honey beverage!
    InfusedMead,
    BeeswaxCandle,
    // Feeding & Sugar honey (사양꿀 콘텐츠)
    Sugar,
    SugarSyrup,
    PollenPatty,
    SugarHoneycomb,
    SugarHoneyJar,

    // Butterflies
    CapturedButterfly(String), // Butterfly species

    // Island exploration
    FerryTicket(String), // Island destination
    AncientRelicFragment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlowerColor {
    Red,
    Blue,
    Yellow,
    Purple,
    White,
    Golden,
}

impl FlowerColor {
    pub fn name(&self) -> &'static str {
        match self {
            FlowerColor::Red => "Ruby Blossom",
            FlowerColor::Blue => "Azure Bluebell",
            FlowerColor::Yellow => "Sunny Marigold",
            FlowerColor::Purple => "Velvet Lavender",
            FlowerColor::White => "Dewdrop Lily",
            FlowerColor::Golden => "Sunburst Orchid",
        }
    }
}

impl ItemType {
    pub fn display_name(&self) -> String {
        match self {
            ItemType::Wood => "Lumber".to_string(),
            ItemType::Stone => "Smooth Stone".to_string(),
            ItemType::Clay => "River Clay".to_string(),
            ItemType::Bamboo => "Wild Bamboo".to_string(),
            ItemType::Wildflower(col) => col.name().to_string(),
            ItemType::FlowerSeed(col) => format!("{} Seed Pack (심기 가능)", col.name()),
            ItemType::GlassJar => "Empty Glass Jar".to_string(),
            ItemType::EmptyFrame => "Empty Wooden Frame".to_string(),
            ItemType::WaxedFrame => "Waxed Honey Frame".to_string(),
            ItemType::ApiaryBox => "Standard Apiary".to_string(),
            ItemType::SolitaryBeeHotel => "Solitary Bee Hotel".to_string(),
            ItemType::CentrifugeExtractor => "Honey Centrifuge".to_string(),
            ItemType::FermenterStation => "Fermenter & Bottler".to_string(),
            ItemType::ButterflyNet => "Handwoven Butterfly Net".to_string(),
            ItemType::BeekeeperSmoker => "Brass Bee Smoker".to_string(),
            ItemType::QueenBee(sp) => format!("Queen Bee ({})", sp),
            ItemType::PrincessBee(sp) => format!("Princess Bee ({})", sp),
            ItemType::DroneBee(sp) => format!("Drone Bee ({})", sp),
            ItemType::Honeycomb(sp) => format!("Honeycomb ({})", sp),
            ItemType::QueenCell => "Royal Queen Cell (자연 왕대)".to_string(),
            ItemType::HoneyJar(flavor) => format!("{} Honey Jar", flavor),
            ItemType::Beeswax => "Pure Beeswax".to_string(),
            ItemType::RoyalJelly => "Royal Jelly Pot".to_string(),
            ItemType::Propolis => "Antiseptic Propolis".to_string(),
            ItemType::HoneyBeaFatherla => "HoneyBeaFatherla (Sparkling Honey Soda)".to_string(),
            ItemType::InfusedMead => "Aged Honey Mead".to_string(),
            ItemType::BeeswaxCandle => "Aromatic Beeswax Candle".to_string(),
            ItemType::Sugar => "Refined Sugar Sack (원당 설탕)".to_string(),
            ItemType::SugarSyrup => "Sugar Feed Syrup (사양액)".to_string(),
            ItemType::PollenPatty => "Protein Pollen Cake (화분떡)".to_string(),
            ItemType::SugarHoneycomb => "Sugar-Fed Honeycomb (사양 소비)".to_string(),
            ItemType::SugarHoneyJar => "Sugar-Fed Honey Jar (사양벌꿀 단지)".to_string(),
            ItemType::CapturedButterfly(name) => format!("{} Butterfly", name),
            ItemType::FerryTicket(dest) => format!("Ferry Passage: {}", dest),
            ItemType::AncientRelicFragment => "Ancient Apiculture Tablet".to_string(),
        }
    }

    pub fn base_sell_value(&self) -> u32 {
        match self {
            ItemType::Wood => 2,
            ItemType::Stone => 2,
            ItemType::Clay => 3,
            ItemType::Bamboo => 4,
            ItemType::Wildflower(_) => 5,
            ItemType::FlowerSeed(_) => 4,
            ItemType::GlassJar => 4,
            ItemType::EmptyFrame => 8,
            ItemType::WaxedFrame => 15,
            ItemType::ApiaryBox => 45,
            ItemType::SolitaryBeeHotel => 50,
            ItemType::CentrifugeExtractor => 90,
            ItemType::FermenterStation => 120,
            ItemType::ButterflyNet => 25,
            ItemType::BeekeeperSmoker => 35,
            ItemType::QueenBee(_) => 30,
            ItemType::PrincessBee(_) => 20,
            ItemType::DroneBee(_) => 10,
            ItemType::Honeycomb(_) => 15,
            ItemType::QueenCell => 40, // High value royal product
            ItemType::HoneyJar(_) => 35,
            ItemType::Beeswax => 18,
            ItemType::RoyalJelly => 60,
            ItemType::Propolis => 25,
            ItemType::HoneyBeaFatherla => 85, // Signature luxury beverage!
            ItemType::InfusedMead => 70,
            ItemType::BeeswaxCandle => 30,
            ItemType::Sugar => 3,
            ItemType::SugarSyrup => 6,
            ItemType::PollenPatty => 14,
            ItemType::SugarHoneycomb => 8,
            ItemType::SugarHoneyJar => 18, // Budget honey price (transparently labeled)
            ItemType::CapturedButterfly(_) => 20,
            ItemType::FerryTicket(_) => 0,
            ItemType::AncientRelicFragment => 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemStack {
    pub item: ItemType,
    pub count: u32,
}

impl ItemStack {
    pub fn new(item: ItemType, count: u32) -> Self {
        Self { item, count }
    }
}
