# 🍯 HoneyBeaFather: Laid-back Beekeeping & Conservation Sim (Gijang Islands)

[한국어 안내 (README.ko.md)](README.ko.md)

> *"Leave your boring city job behind to return to your family home in Port Gijang and get back to your beekeeping roots in the Gijang Islands."*

**HoneyBeaFather** is a cozy, laid-back apiculture simulation game built in **Rust** with the **Bevy Engine (0.15)**. Inspired by cozy biology and apiculture classics, it uniquely combines resource gathering, Mendelian & fantasy bee genetics, interactive minigames, butterfly conservation, and island exploration across the lush **Gijang archipelago**.

> 💖 **Dedication**:  
> Dedicated with admiration to YouTuber **프응 (F-Eung TV)**, who shares the wonder, beauty, and craftsmanship of beekeeping and nature from Gijang!

---

## 🌟 Key Features

- 🍯 **Sugar-Fed Honey (사양꿀) & Feeder / Pollen Patty Mechanics**:
  - **Feeding Sugar Syrup (자극사양 & 월동사양)**: Keep bees nourished during nectar dearth periods, rains, or winter by pressing `[S]` inside the Hive Inspection screen to fill the internal hive feeder with sugar syrup!
  - **Producing Sugar-Fed Honey (사양벌꿀) & Transparent Labeling**: Fed hives produce **Sugar-Fed Honeycombs (사양 소비)** even with zero blooming flowers. Spin them in the Honey Centrifuge (`[M]`) to bottle budget-friendly **Sugar-Fed Honey Jars**! Sell them at the market or use them to brew Port Gijang's fizzy honey soda!
  - **High-Protein Pollen Patties (화분떡)**: Craft pollen cakes from sugar and wildflower pollen, then feed them with `[P]` inside inspection to supercharge brood rearing up to **+35 bees/sec**!
- 🐝 **Realistic Apiculture Mechanics (Colony Management, Frames, Swarming & Inspection)**:
  - **Colony Population & Adding Frames (소비장 증소)**: Worker bee population grows from 1,000 to over 15,000 bees through queen egg-laying. Each frame supports 1,500 bees; as the colony expands, you must install additional frames (`[F]` inside inspection) to prevent overcrowding.
  - **Overcrowding, Swarm Fever & Swarming (과밀 분봉열 & 분봉)**: An overcrowded hive accumulates Swarm Fever rapidly, prompting bees to construct **Queen Cells (자연 왕대)**. If Swarm Fever hits 100%, **half the colony and the queen swarm and fly away**!
  - **Hive Inspection & Pinching Queen Cells (내검 & 왕대 제거)**: Approach any Apiary and press `[E]` to open the **Hive Inspection Modal**. Inspect colony health, add frames (`[F]`), and pinch queen cells (`[R]`) to suppress swarm fever and harvest valuable **Royal Jelly** and Queen Cells.
  - **Surrounding Floral Density & Player Flower Planting (밀원 꽃밭)**: Nearby flower patches within foraging radius scale honey gathering speed from **0.25x up to 5.0x**! Select flower seeds (`FlowerSeed`) or wildflowers in your hotbar and press `[F]` anywhere in the world to plant custom flower fields around your hives.
- 🎨 **Pokemon-Style Cute Pixel Art Aesthetic**:
  - Nostalgic, heartwarming chibi pixel art inspired by Pokemon Gen 3 & 4 (GBA/NDS)!
  - Cute Beekeeper trainer with a straw hat and sparkling anime eyes, and lovely '와이프응' (Wife-Eung) NPC in her floral apron dress.
  - Plump, chubby bumblebees with translucent fluttering wings, vibrant jewel-toned butterflies, chubby 5-petal blossoms, and fluffy round trees.
- 👩‍🌾 **Meet '와이프응' (Wife-Eung) NPC in Port Gijang**:
  - Visit your loving companion in Port Gijang. Press `[E]` to talk and receive warm encouragement, refreshing barley tea, and bonus glass jars for your honey!
- 🔬 **Over 45 Bee Species & Deep Genetics**:
  - Full catalog of **46 unique bee species** spanning 5 tiers (Wild, First Hybrids, Refined Hybrids, Exotic, and Mythic Ancestral Relics).
  - Genotype alleles for Lifespan, Speed, Fertility, Diurnal/Nocturnal foraging, and Rain tolerance.
  - Cross-breeding mutations: combine parent species in Apiaries to discover brand-new hybrid species!
- 🪵 **Solitary Bees & Craftable Habitats**:
  - Discover solitary pollinators (Orchard Mason Bee, Leafcutter Bee, Blue Carpenter Bee, Glittering Orchid Bee).
  - Craft specialized **Solitary Bee Hotels** using Bamboo, Wood, and River Clay to collect pure Antiseptic Propolis!
- 🦋 **Catch & Collect Wild Butterflies**:
  - Fluttering butterflies with fluid sinusoidal wing wave animations across flower patches.
  - Catch them with your handcrafted Butterfly Net to activate permanent island aura buffs:
    - *Brimstone*: +25% Apiary production speed
    - *Peacock*: +20% Cross-breed mutation rate
    - *Rainbow Swallowtail*: +30% Produce market sell price
    - *Blue Morpho, Monarch, Moon Moth*: Flower regeneration and nighttime foraging!
- ⚙️ **Interactive Beekeeping Minigames**:
  - **Honey Centrifuge Extractor**: Pump the hand crank with `[Space]` and balance the RPM gauge inside the sweet spot to harvest pure honey jars and beeswax blocks!
  - **Genetics Microscope**: Fine-tune Focus and Magnification dials `[W/A/S/D]` to align chromosome markers and decode hidden recessive alleles!
- 🧰 **Workbench Crafting**:
  - Build Apiaries, Frames, Waxed Frames, Solitary Hotels, Smokers, Nets, Centrifuges, and Candles.
  - Brew **HoneyBeaFatherla**: Port Gijang's famous effervescent sparkling honey soda!
- 🏪 **Market Economy & Port Stall**:
  - Sell honey jars, pure beeswax, HoneyBeaFatherla, mead, and candles for **Honey Coins**.
  - Purchase boat passage tickets to unlock remote archipelago islands!
- 🌿 **Island Repopulation & Conservation**:
  - Each island hosts an **Ancestral Repopulation Shrine**.
  - Release requested bee species back into the wild to restore island ecosystem health from 0% to 100%!
  - Decipher Grandfather's 6 lost journal entries and unlock the ritual to awaken the legendary **HoneyBeaFather Ancestral Queen**!
- ⛵ **Multiple Biomes to Explore (Gijang Archipelago)**:
  - **Port Gijang**: Grandfather's cottage, docks, market stall, 와이프응 (Wife-Eung), and starter apiaries.
  - **Emerald Meadows**: Rolling hills rich with clover, ruby blossoms, and solitary mason bees.
  - **Whisperwood Forest**: Towering ancient canopy, mossy cedar trunks, and bark bees.
  - **Sunken Shore**: Ocean breeze, tide pools, sandy dunes, and aquatic coastal bees.
  - **Mangrove Marsh**: Humid mist, water lilies, and orchid pollinators.
  - **Shimmering Peaks**: High alpine crags, crystalline frost, and cosmic nebula bees.

---

## 🎮 Controls & Shortcuts

| Key | Action |
|:---|:---|
| **W / A / S / D** or **Arrow Keys** | Move your Beekeeper character |
| **E** | **Interact / Hive Inspection**: Approach an Apiary and press `[E]` to open the **Hive Inspection Modal**; chop trees, mine rocks, pick flowers, catch butterflies, inspect shrines, and chat with 와이프응 (Wife-Eung) |
| **F** | **Plant Flower Field / Place Apiary**: Plant flowers using seeds (`FlowerSeed`) or wildflowers (`Wildflower`) from hotbar; or place down a new Apiary box |
| **1 – 8** | Select active hotbar slot |
| **B** | Open / Close **Beedex** (Species encyclopedia & breeding clues) |
| **C** | Open / Close **Crafting Workbench** (Press 1–9 to craft) |
| **M** | Open / Close **Honey Centrifuge Minigame** (Press Space to pump crank) |
| **G** | Open / Close **Genetics Microscope Minigame** (W/A/S/D to align lenses) |
| **P** | Open / Close **Port Market Stall** (Press S to quick sell, 1–7 to buy) |
| **J** | Open / Close **Grandfather's Apiculture Journal** |
| **Esc** | Close any open modal screen |

### 🔬 Hive Inspection Modal ([E]) Controls

| Key | Inspection Action |
|:---|:---|
| **F** | **Add Frame (증소)**: Installs an empty or waxed frame to increase capacity by +1,500 bees and reduce swarm fever |
| **R** | **Pinch Queen Cells (왕대 제거)**: Destroys natural queen cells to prevent swarming, heavily reduces swarm fever, and yields Royal Jelly! |
| **S** | **Feed Sugar Syrup (사양액 급여)**: Refills internal feeder to sustain bees during dearth periods and produce Sugar-Fed Honey (+50% syrup) |
| **P** | **Feed Pollen Patty (화분떡 급여)**: Supplies high-protein pollen cake to supercharge brood rearing rate (+35 bees/sec) |
| **H** | **Harvest Honeycombs (채밀)**: Collects all honeycombs (natural and sugar-fed) stored in the hive into your inventory |
| **1 – 8** | **House Queen / Drone**: Assigns queen or drone from hotbar slot into the apiary chambers |
| **Esc** | Exit inspection and return to the island |

---

## 🧬 Cross-Breeding Guide Preview

| Parent A | Parent B | Resulting Species | Tier |
|:---|:---|:---|:---|
| Forest Bee | Meadow Bee | **Verdant Bee** | Tier 2 |
| Meadow Bee | Coastal Bee | **Dewdrop Bee** | Tier 2 |
| Forest Bee | Swamp Bee | **Mossy Bee** | Tier 2 |
| Mountain Bee | Coastal Bee | **Frosty Bee** | Tier 2 |
| Mountain Bee | Rock Bee | **Stony Bee** | Tier 2 |
| Blossom Bee | Dewdrop Bee | **Honeycomb Bee** | Tier 3 |
| Blossom Bee | Honeycomb Bee | **Golden Bee** | Tier 3 |
| Frosty Bee | Stony Bee | **Crystal Bee** | Tier 3 |
| Crystal Bee | Dewdrop Bee | **Prisma Bee** | Tier 3 |
| Prisma Bee | Twilight Bee | **Nebula Bee** | Tier 4 |
| Amber Bee | Nebula Bee | **Ancient Bee** | Tier 5 (Mythic) |
| Radiant Bee | Ethereal Bee | **Celestial Bee** | Tier 5 (Mythic) |
| Golden Bee | Celestial Bee | **HoneyBeaFather Ancestral Queen** | Tier 5 (Legendary) |

---

## 🎨 Visuals & Typography

- **Google Noto Sans KR Font**: All in-game text (HUD, apiary inspection, Beedex, notices) renders natively with Noto Sans KR, providing crisp and complete Korean typography.
- **Pokemon Gen 3 Chibi Walk Cycle**: Full 4-directional 9-frame walk cycle with alternating foot strides and arm swings for Down, Up, Left, and Right movement.

---

## 🚀 Running the Game

To launch HoneyBeaFather:

```bash
cargo run
```

Or for optimized release performance:

```bash
cargo run --release
```
