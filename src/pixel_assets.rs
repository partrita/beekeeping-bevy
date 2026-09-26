use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::image::ImageSampler;

pub struct PixelCanvas {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl PixelCanvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0; (width * height * 4) as usize],
        }
    }

    #[inline]
    pub fn set_pixel(&mut self, x: i32, y: i32, rgba: [u8; 4]) {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return;
        }
        let idx = ((y as u32 * self.width + x as u32) * 4) as usize;
        self.data[idx] = rgba[0];
        self.data[idx + 1] = rgba[1];
        self.data[idx + 2] = rgba[2];
        self.data[idx + 3] = rgba[3];
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, rgba: [u8; 4]) {
        for dy in 0..h {
            for dx in 0..w {
                self.set_pixel(x + dx, y + dy, rgba);
            }
        }
    }

    pub fn fill_circle(&mut self, cx: i32, cy: i32, radius: i32, rgba: [u8; 4]) {
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if dx * dx + dy * dy <= radius * radius {
                    self.set_pixel(cx + dx, cy + dy, rgba);
                }
            }
        }
    }

    pub fn to_bevy_image(&self) -> Image {
        let mut img = Image::new(
            Extent3d {
                width: self.width,
                height: self.height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            self.data.clone(),
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::default(),
        );
        img.sampler = ImageSampler::nearest();
        img
    }
}

#[derive(Clone)]
pub struct PlayerSprites {
    pub down_idle: Handle<Image>,
    pub down_step1: Handle<Image>,
    pub down_step2: Handle<Image>,
    pub up_idle: Handle<Image>,
    pub up_step1: Handle<Image>,
    pub up_step2: Handle<Image>,
    pub side_idle: Handle<Image>,
    pub side_step1: Handle<Image>,
    pub side_step2: Handle<Image>,
}

#[derive(Clone)]
pub struct UiIcons {
    pub coin: Handle<Image>,
    pub leaf: Handle<Image>,
    pub island: Handle<Image>,
    pub honey: Handle<Image>,
    pub bee: Handle<Image>,
    pub crown: Handle<Image>,
    pub flower: Handle<Image>,
    pub frame: Handle<Image>,
    pub feeder: Handle<Image>,
    pub warning: Handle<Image>,
    pub patty: Handle<Image>,
    pub book: Handle<Image>,
    pub craft: Handle<Image>,
    pub gear: Handle<Image>,
    pub microscope: Handle<Image>,
    pub market: Handle<Image>,
    pub ferry: Handle<Image>,
    pub exit: Handle<Image>,
}

#[derive(Resource)]
pub struct GamePixelAssets {
    pub icons: UiIcons,
    pub player: Handle<Image>,
    pub player_sprites: PlayerSprites,
    pub wife_eung: Handle<Image>,
    pub bee: Handle<Image>,
    pub butterfly_brimstone: Handle<Image>,
    pub butterfly_peacock: Handle<Image>,
    pub butterfly_morpho: Handle<Image>,
    pub butterfly_monarch: Handle<Image>,
    pub butterfly_moon: Handle<Image>,
    pub butterfly_rainbow: Handle<Image>,
    pub flower_red: Handle<Image>,
    pub flower_yellow: Handle<Image>,
    pub flower_blue: Handle<Image>,
    pub flower_purple: Handle<Image>,
    pub flower_white: Handle<Image>,
    pub flower_gold: Handle<Image>,
    pub apiary: Handle<Image>,
    pub solitary_hotel: Handle<Image>,
    pub tree: Handle<Image>,
    pub rock: Handle<Image>,
    pub shrine: Handle<Image>,
    pub ferry_boat: Handle<Image>,
    pub wild_hive: Handle<Image>,
}

impl FromWorld for GamePixelAssets {
    fn from_world(world: &mut World) -> Self {
        let mut images = world.resource_mut::<Assets<Image>>();
        create_pixel_assets(&mut images)
    }
}

pub fn create_pixel_assets(images: &mut Assets<Image>) -> GamePixelAssets {
    // 1. CHIBI POKEMON-STYLE BEEKEEPER TRAINER (16x22) - 9-FRAME 4-DIRECTIONAL WALK CYCLE
    let player_sprites = {
        let outline = [35, 25, 30, 255];
        let straw = [240, 195, 80, 255];
        let straw_band = [145, 90, 30, 255];
        let hair = [115, 65, 35, 255];
        let skin = [255, 218, 185, 255];
        let blush = [255, 140, 160, 255];
        let eye = [30, 30, 45, 255];
        let white = [255, 255, 255, 255];
        let smock = [245, 240, 230, 255];
        let smock_shadow = [210, 205, 195, 255];
        let belt = [110, 70, 35, 255];
        let buckle = [255, 215, 50, 255];
        let pants = [65, 105, 185, 255];
        let pants_shadow = [45, 75, 140, 255];
        let shoes = [80, 50, 35, 255];
        let shoe_sole = [50, 30, 20, 255];

        // --- DOWN (FRONT) VIEW BASE ---
        let make_down_base = || {
            let mut c = PixelCanvas::new(16, 22);
            // Straw Hat brim & crown
            c.fill_rect(2, 1, 12, 3, straw);
            c.fill_rect(4, 4, 8, 4, straw);
            c.fill_rect(4, 3, 8, 1, straw_band);

            // Chestnut hair
            c.fill_rect(3, 7, 2, 3, hair);
            c.fill_rect(11, 7, 2, 3, hair);
            c.set_pixel(5, 7, hair);
            c.set_pixel(10, 7, hair);

            // Face & Blushing cheeks
            c.fill_rect(4, 7, 8, 6, skin);
            c.set_pixel(4, 10, blush);
            c.set_pixel(11, 10, blush);

            // Cute Big Anime Eyes
            c.fill_rect(5, 8, 2, 3, eye);
            c.fill_rect(9, 8, 2, 3, eye);
            c.set_pixel(5, 8, white);
            c.set_pixel(9, 8, white);
            // Smile
            c.set_pixel(7, 11, [180, 80, 80, 255]);
            c.set_pixel(8, 11, [180, 80, 80, 255]);

            // Beekeeper Linen Smock
            c.fill_rect(4, 13, 8, 4, smock);
            // Belt & Buckle
            c.fill_rect(4, 17, 8, 1, belt);
            c.fill_rect(7, 17, 2, 1, buckle);

            // Outlines
            for x in 4..12 { c.set_pixel(x, 0, outline); }
            for y in 7..13 {
                c.set_pixel(3, y, outline);
                c.set_pixel(12, y, outline);
            }
            c
        };

        // Down Idle: legs together, arms at rest
        let down_idle = {
            let mut c = make_down_base();
            c.fill_rect(2, 13, 2, 4, smock);
            c.fill_rect(12, 13, 2, 4, smock);
            c.fill_rect(2, 17, 2, 1, skin);
            c.fill_rect(12, 17, 2, 1, skin);
            c.fill_rect(5, 18, 2, 2, pants);
            c.fill_rect(9, 18, 2, 2, pants);
            c.fill_rect(4, 20, 3, 2, shoes);
            c.fill_rect(9, 20, 3, 2, shoes);
            images.add(c.to_bevy_image())
        };

        // Down Step 1: left foot forward & down, right foot back, arms swinging
        let down_step1 = {
            let mut c = make_down_base();
            c.fill_rect(2, 12, 2, 4, smock); // Left arm swing up/back
            c.fill_rect(2, 16, 2, 1, skin);
            c.fill_rect(12, 14, 2, 4, smock); // Right arm swing down/forward
            c.fill_rect(12, 18, 2, 1, skin);
            // Left leg (stepping forward)
            c.fill_rect(4, 18, 3, 2, pants);
            c.fill_rect(3, 20, 4, 2, shoes);
            c.fill_rect(3, 21, 4, 1, shoe_sole);
            // Right leg (trailing back)
            c.fill_rect(10, 18, 2, 2, pants_shadow);
            c.fill_rect(10, 19, 2, 2, shoes);
            images.add(c.to_bevy_image())
        };

        // Down Step 2: right foot forward & down, left foot back, arms swinging
        let down_step2 = {
            let mut c = make_down_base();
            c.fill_rect(2, 14, 2, 4, smock); // Left arm swing down/forward
            c.fill_rect(2, 18, 2, 1, skin);
            c.fill_rect(12, 12, 2, 4, smock); // Right arm swing up/back
            c.fill_rect(12, 16, 2, 1, skin);
            // Left leg (trailing back)
            c.fill_rect(4, 18, 2, 2, pants_shadow);
            c.fill_rect(4, 19, 2, 2, shoes);
            // Right leg (stepping forward)
            c.fill_rect(9, 18, 3, 2, pants);
            c.fill_rect(9, 20, 4, 2, shoes);
            c.fill_rect(9, 21, 4, 1, shoe_sole);
            images.add(c.to_bevy_image())
        };

        // --- UP (BACK) VIEW BASE ---
        let make_up_base = || {
            let mut c = PixelCanvas::new(16, 22);
            // Straw Hat crown & brim from back
            c.fill_rect(2, 2, 12, 3, straw);
            c.fill_rect(4, 4, 8, 4, straw);
            c.fill_rect(4, 3, 8, 1, straw_band);

            // Chestnut hair peeking from under hat
            c.fill_rect(3, 7, 10, 5, hair);

            // Smock back
            c.fill_rect(4, 12, 8, 5, smock);
            c.fill_rect(6, 12, 4, 1, smock_shadow);
            // Belt
            c.fill_rect(4, 17, 8, 1, belt);

            // Outlines
            for x in 4..12 { c.set_pixel(x, 0, outline); }
            for y in 7..13 {
                c.set_pixel(3, y, outline);
                c.set_pixel(12, y, outline);
            }
            c
        };

        // Up Idle
        let up_idle = {
            let mut c = make_up_base();
            c.fill_rect(2, 13, 2, 4, smock);
            c.fill_rect(12, 13, 2, 4, smock);
            c.fill_rect(5, 18, 2, 2, pants);
            c.fill_rect(9, 18, 2, 2, pants);
            c.fill_rect(4, 20, 3, 2, shoes);
            c.fill_rect(9, 20, 3, 2, shoes);
            images.add(c.to_bevy_image())
        };

        // Up Step 1: left foot lifts up, right foot grounded
        let up_step1 = {
            let mut c = make_up_base();
            c.fill_rect(2, 12, 2, 4, smock); // Left arm up
            c.fill_rect(12, 14, 2, 4, smock); // Right arm down
            c.fill_rect(4, 17, 3, 3, pants);
            c.fill_rect(4, 19, 3, 2, shoes);
            c.fill_rect(4, 20, 3, 1, shoe_sole);
            c.fill_rect(9, 18, 2, 2, pants);
            c.fill_rect(9, 20, 3, 2, shoes);
            images.add(c.to_bevy_image())
        };

        // Up Step 2: right foot lifts up, left foot grounded
        let up_step2 = {
            let mut c = make_up_base();
            c.fill_rect(2, 14, 2, 4, smock); // Left arm down
            c.fill_rect(12, 12, 2, 4, smock); // Right arm up
            c.fill_rect(5, 18, 2, 2, pants);
            c.fill_rect(4, 20, 3, 2, shoes);
            c.fill_rect(9, 17, 3, 3, pants);
            c.fill_rect(9, 19, 3, 2, shoes);
            c.fill_rect(9, 20, 3, 1, shoe_sole);
            images.add(c.to_bevy_image())
        };

        // --- SIDE (PROFILE) VIEW BASE ---
        let make_side_base = || {
            let mut c = PixelCanvas::new(16, 22);
            // Straw hat profile with front visor/brim extending rightwards
            c.fill_rect(3, 2, 11, 2, straw);
            c.set_pixel(14, 3, straw);
            c.fill_rect(4, 4, 7, 4, straw);
            c.fill_rect(4, 3, 7, 1, straw_band);

            // Profile head
            c.fill_rect(3, 6, 4, 6, hair);
            c.fill_rect(7, 7, 6, 5, skin);
            c.fill_rect(10, 8, 2, 3, eye);
            c.set_pixel(10, 8, white);
            c.set_pixel(11, 10, blush);
            c.set_pixel(13, 9, skin); // nose tip

            // Smock torso
            c.fill_rect(5, 12, 7, 5, smock);
            c.fill_rect(5, 17, 7, 1, belt);

            // Outline
            for x in 4..11 { c.set_pixel(x, 0, outline); }
            c
        };

        // Side Idle: profile stance
        let side_idle = {
            let mut c = make_side_base();
            c.fill_rect(7, 13, 2, 4, smock);
            c.fill_rect(7, 17, 2, 1, skin);
            c.fill_rect(6, 18, 4, 2, pants);
            c.fill_rect(5, 20, 5, 2, shoes);
            images.add(c.to_bevy_image())
        };

        // Side Step 1: front leg strides forward right, back leg stretches left
        let side_step1 = {
            let mut c = make_side_base();
            c.fill_rect(5, 13, 2, 4, smock); // arm swing back
            c.fill_rect(5, 17, 2, 1, skin);
            c.fill_rect(8, 18, 3, 2, pants); // front leg
            c.fill_rect(9, 20, 4, 2, shoes);
            c.fill_rect(4, 18, 3, 2, pants_shadow); // back leg
            c.fill_rect(3, 19, 3, 2, shoes);
            images.add(c.to_bevy_image())
        };

        // Side Step 2: other leg strides forward, back leg pushes
        let side_step2 = {
            let mut c = make_side_base();
            c.fill_rect(9, 13, 2, 4, smock); // arm swing forward
            c.fill_rect(10, 16, 2, 1, skin);
            c.fill_rect(8, 17, 3, 3, pants); // front leg raised
            c.fill_rect(8, 20, 4, 2, shoes);
            c.fill_rect(4, 18, 3, 2, pants_shadow); // back leg
            c.fill_rect(2, 20, 3, 2, shoes);
            images.add(c.to_bevy_image())
        };

        PlayerSprites {
            down_idle,
            down_step1,
            down_step2,
            up_idle,
            up_step1,
            up_step2,
            side_idle,
            side_step1,
            side_step2,
        }
    };
    let player = player_sprites.down_idle.clone();

    // 2. CHIBI WIFE-EUNG NPC (16x22)
    let wife_eung = {
        let mut c = PixelCanvas::new(16, 22);
        let hair = [125, 70, 40, 255];
        let ribbon = [255, 95, 140, 255];
        let skin = [255, 220, 195, 255];
        let blush = [255, 135, 160, 255];
        let eye = [35, 30, 45, 255];
        let white = [255, 255, 255, 255];
        let apron_pink = [255, 165, 190, 255];
        let apron_lace = [255, 245, 250, 255];
        let honey_jar = [255, 205, 50, 255];
        let shoes = [120, 65, 45, 255];

        // Cute hair with side pigtails / waves
        c.fill_rect(4, 1, 8, 5, hair);
        c.fill_rect(2, 4, 3, 8, hair);
        c.fill_rect(11, 4, 3, 8, hair);

        // Cherry Blossom Hair Ribbon
        c.fill_rect(2, 2, 3, 3, ribbon);
        c.fill_rect(11, 2, 3, 3, ribbon);
        c.set_pixel(3, 3, [255, 230, 240, 255]);

        // Face & Rosy Blushing Cheeks
        c.fill_rect(5, 5, 6, 7, skin);
        c.set_pixel(4, 9, blush);
        c.set_pixel(11, 9, blush);

        // Big Anime Eyes with double catchlights
        c.fill_rect(5, 7, 2, 3, eye);
        c.fill_rect(9, 7, 2, 3, eye);
        c.set_pixel(5, 7, white);
        c.set_pixel(9, 7, white);
        c.set_pixel(6, 9, white);
        c.set_pixel(10, 9, white);

        // Cheerful Smile
        c.set_pixel(7, 10, [220, 80, 100, 255]);
        c.set_pixel(8, 10, [220, 80, 100, 255]);

        // Apron Dress
        c.fill_rect(4, 12, 8, 7, apron_pink);
        c.fill_rect(5, 12, 6, 2, apron_lace);
        c.fill_rect(4, 18, 8, 1, apron_lace);

        // Arms holding a mini Honey Jar
        c.fill_rect(3, 13, 2, 3, skin);
        c.fill_rect(11, 13, 2, 3, skin);
        c.fill_rect(7, 14, 2, 3, honey_jar);
        c.set_pixel(7, 13, [220, 140, 40, 255]); // Jar lid

        // Cute Shoes
        c.fill_rect(5, 19, 2, 2, shoes);
        c.fill_rect(9, 19, 2, 2, shoes);

        images.add(c.to_bevy_image())
    };

    // 3. CUTE ROUND CHUBBY BEE (14x12)
    let bee = {
        let mut c = PixelCanvas::new(14, 12);
        let gold = [255, 215, 35, 255];
        let brown = [55, 35, 25, 255];
        let wing = [220, 245, 255, 220];
        let eye = [25, 20, 25, 255];
        let blush = [255, 130, 160, 220];

        // Wings
        c.fill_rect(3, 0, 3, 3, wing);
        c.fill_rect(8, 0, 3, 3, wing);
        c.set_pixel(4, 1, [255, 255, 255, 255]);
        c.set_pixel(9, 1, [255, 255, 255, 255]);

        // Fluffy body stripes
        c.fill_rect(2, 3, 10, 7, gold);
        c.fill_rect(5, 3, 2, 7, brown);
        c.fill_rect(9, 3, 2, 7, brown);

        // Cute face
        c.fill_rect(2, 5, 2, 3, eye);
        c.set_pixel(2, 5, [255, 255, 255, 255]); // sparkle
        c.set_pixel(2, 9, blush); // rosy cheek

        // Antennae
        c.set_pixel(2, 2, brown);
        c.set_pixel(4, 2, brown);

        // Stinger nub
        c.set_pixel(12, 6, brown);

        images.add(c.to_bevy_image())
    };

    // 4. BUTTERFLIES (16x14)
    let mut make_butterfly = |primary: [u8; 4], accent: [u8; 4]| {
        let mut c = PixelCanvas::new(16, 14);
        let black = [30, 25, 30, 255];
        let white = [255, 255, 255, 255];

        // Body
        c.fill_rect(7, 3, 2, 8, black);
        c.set_pixel(6, 1, black);
        c.set_pixel(9, 1, black);

        // Left Wing
        c.fill_rect(1, 2, 6, 5, primary);
        c.fill_rect(2, 7, 5, 4, primary);
        c.fill_circle(3, 4, 1, accent);
        c.set_pixel(3, 4, white);

        // Right Wing
        c.fill_rect(9, 2, 6, 5, primary);
        c.fill_rect(9, 7, 5, 4, primary);
        c.fill_circle(12, 4, 1, accent);
        c.set_pixel(12, 4, white);

        // Wing outlines
        for x in 1..7 { c.set_pixel(x, 1, black); }
        for x in 9..15 { c.set_pixel(x, 1, black); }
        for y in 2..8 {
            c.set_pixel(0, y, black);
            c.set_pixel(15, y, black);
        }

        images.add(c.to_bevy_image())
    };

    let butterfly_brimstone = make_butterfly([255, 235, 55, 255], [255, 175, 20, 255]);
    let butterfly_peacock = make_butterfly([215, 45, 65, 255], [45, 120, 240, 255]);
    let butterfly_morpho = make_butterfly([45, 170, 255, 255], [180, 240, 255, 255]);
    let butterfly_monarch = make_butterfly([250, 125, 25, 255], [40, 35, 35, 255]);
    let butterfly_moon = make_butterfly([145, 245, 195, 255], [255, 255, 255, 255]);
    let butterfly_rainbow = make_butterfly([245, 95, 190, 255], [65, 230, 250, 255]);

    // 5. WILDFLOWERS (14x14)
    let mut make_flower = |petal: [u8; 4]| {
        let mut c = PixelCanvas::new(14, 14);
        let stem = [65, 155, 55, 255];
        let center = [255, 215, 35, 255];

        // Stem & leaves
        c.fill_rect(6, 7, 2, 6, stem);
        c.fill_rect(4, 9, 2, 2, stem);
        c.fill_rect(8, 9, 2, 2, stem);

        // 5 Oversized Chibi Petals
        c.fill_circle(6, 3, 2, petal);
        c.fill_circle(3, 5, 2, petal);
        c.fill_circle(9, 5, 2, petal);
        c.fill_circle(4, 8, 2, petal);
        c.fill_circle(8, 8, 2, petal);

        // Golden Center with white shine
        c.fill_circle(6, 6, 2, center);
        c.set_pixel(6, 5, [255, 255, 255, 255]);

        images.add(c.to_bevy_image())
    };

    let flower_red = make_flower([245, 55, 75, 255]);
    let flower_yellow = make_flower([255, 215, 45, 255]);
    let flower_blue = make_flower([65, 145, 245, 255]);
    let flower_purple = make_flower([185, 85, 230, 255]);
    let flower_white = make_flower([250, 250, 255, 255]);
    let flower_gold = make_flower([255, 185, 25, 255]);

    // 6. CUTE POKEMON APIARY HIVE (24x26)
    let apiary = {
        let mut c = PixelCanvas::new(24, 26);
        let wood = [225, 165, 85, 255];
        let wood_dark = [165, 105, 45, 255];
        let roof_shingle = [190, 85, 45, 255];
        let honey_gold = [255, 205, 35, 255];
        let outline = [50, 30, 20, 255];

        // Gabled Roof
        for y in 0..6 {
            let inset = 5 - y;
            c.fill_rect(inset, y, 24 - inset * 2, 1, roof_shingle);
        }
        c.fill_rect(0, 6, 24, 2, [155, 65, 30, 255]); // Roof trim

        // Main Hive Box Body
        c.fill_rect(3, 8, 18, 16, wood);
        c.fill_rect(3, 14, 18, 1, wood_dark); // Plank separation lines
        c.fill_rect(3, 19, 18, 1, wood_dark);

        // Entrance slot with dripping golden honey
        c.fill_rect(8, 21, 8, 3, outline);
        c.fill_rect(9, 20, 2, 2, honey_gold);
        c.set_pixel(10, 22, honey_gold);

        // Cute painted bee emblem on front
        c.fill_circle(12, 11, 2, honey_gold);
        c.set_pixel(12, 11, wood_dark);

        images.add(c.to_bevy_image())
    };

    // 7. SOLITARY BEE HOTEL (20x24)
    let solitary_hotel = {
        let mut c = PixelCanvas::new(20, 24);
        let wood = [185, 130, 75, 255];
        let moss = [85, 165, 70, 255];
        let bamboo = [225, 195, 125, 255];
        let hole = [55, 40, 30, 255];

        // Mossy roof canopy
        c.fill_rect(2, 1, 16, 4, wood);
        c.fill_circle(5, 1, 2, moss);
        c.fill_circle(10, 1, 2, moss);
        c.fill_circle(15, 1, 2, moss);

        // Frame
        c.fill_rect(2, 5, 16, 18, wood);

        // Packed bamboo tubes
        let tube_centers = [
            (5, 8), (10, 8), (15, 8),
            (7, 13), (12, 13),
            (5, 18), (10, 18), (15, 18),
        ];

        for &(tx, ty) in &tube_centers {
            c.fill_circle(tx, ty, 2, bamboo);
            c.set_pixel(tx, ty, hole);
        }

        // Cute little bee face peeking out of center tube!
        c.set_pixel(12, 13, [255, 215, 40, 255]); // bee face
        c.set_pixel(11, 12, [25, 20, 20, 255]);  // little antenna
        c.set_pixel(13, 12, [25, 20, 20, 255]);

        images.add(c.to_bevy_image())
    };

    // 8. LUSH POKEMON-STYLE TREE (32x40)
    let tree = {
        let mut c = PixelCanvas::new(32, 40);
        let trunk = [135, 85, 45, 255];
        let trunk_dark = [95, 55, 30, 255];
        let leaf_highlight = [125, 215, 80, 255];
        let leaf_mid = [65, 165, 60, 255];
        let leaf_shadow = [35, 105, 45, 255];

        // Trunk
        c.fill_rect(13, 24, 6, 14, trunk);
        c.fill_rect(13, 24, 2, 14, trunk_dark);
        c.fill_rect(11, 36, 10, 2, trunk_dark); // Root flare

        // 3 Layered Fluffy Foliage Puffs (Pokemon Gen 3 style)
        c.fill_circle(16, 14, 12, leaf_shadow);
        c.fill_circle(16, 12, 11, leaf_mid);
        c.fill_circle(16, 9, 8, leaf_highlight);

        c.fill_circle(9, 16, 7, leaf_mid);
        c.fill_circle(8, 14, 5, leaf_highlight);

        c.fill_circle(23, 16, 7, leaf_mid);
        c.fill_circle(24, 14, 5, leaf_highlight);

        images.add(c.to_bevy_image())
    };

    // 9. CUTE SMOOTH BOULDER ROCK (20x16)
    let rock = {
        let mut c = PixelCanvas::new(20, 16);
        let grey = [170, 180, 190, 255];
        let highlight = [215, 225, 235, 255];
        let shadow = [95, 105, 120, 255];

        c.fill_circle(10, 8, 7, shadow);
        c.fill_circle(9, 7, 6, grey);
        c.fill_circle(7, 5, 3, highlight);
        c.set_pixel(7, 4, [255, 255, 255, 255]); // Mineral sparkle!

        images.add(c.to_bevy_image())
    };

    // 10. ANCESTRAL REPOPULATION SHRINE (28x28)
    let shrine = {
        let mut c = PixelCanvas::new(28, 28);
        let stone = [180, 175, 195, 255];
        let stone_dark = [120, 115, 140, 255];
        let glow_gold = [255, 215, 55, 255];
        let glow_pink = [255, 135, 195, 255];

        // Pedestal tiers
        c.fill_rect(2, 22, 24, 5, stone_dark);
        c.fill_rect(4, 17, 20, 5, stone);
        c.fill_rect(6, 12, 16, 5, stone_dark);

        // Crystal Altar Blossom
        c.fill_circle(14, 8, 5, glow_gold);
        c.fill_circle(14, 8, 3, glow_pink);
        c.set_pixel(14, 8, [255, 255, 255, 255]); // Altar core gleam

        images.add(c.to_bevy_image())
    };

    // 11. GIJANG FERRY BOAT (36x24)
    let ferry_boat = {
        let mut c = PixelCanvas::new(36, 24);
        let hull_blue = [55, 125, 195, 255];
        let hull_white = [250, 250, 255, 255];
        let deck_wood = [210, 155, 95, 255];
        let canopy_red = [235, 65, 75, 255];
        let canopy_white = [255, 255, 255, 255];

        // Boat Hull
        c.fill_rect(4, 14, 28, 6, hull_blue);
        c.fill_rect(2, 12, 32, 2, hull_white);

        // Deck
        c.fill_rect(6, 10, 24, 2, deck_wood);

        // Cabin / Canopy
        c.fill_rect(10, 4, 16, 6, [240, 235, 225, 255]);
        for x in 0..16 {
            let col = if (x / 3) % 2 == 0 { canopy_red } else { canopy_white };
            c.set_pixel(10 + x, 3, col);
        }

        // Lifebuoy ring on hull
        c.fill_circle(18, 16, 3, [245, 65, 65, 255]);
        c.fill_circle(18, 16, 1, [255, 255, 255, 255]);

        images.add(c.to_bevy_image())
    };

    // 12. WILD BEE HIVE (20x22)
    let wild_hive = {
        let mut c = PixelCanvas::new(20, 22);
        let amber = [245, 175, 35, 255];
        let amber_dark = [185, 115, 20, 255];
        let comb = [255, 220, 60, 255];

        // Tiered oval wild honey comb nest
        c.fill_circle(10, 6, 5, amber);
        c.fill_circle(10, 11, 7, amber);
        c.fill_circle(10, 16, 5, amber_dark);
        c.fill_circle(10, 11, 3, comb);

        // Entrance hole
        c.fill_circle(10, 13, 2, [50, 30, 15, 255]);

        images.add(c.to_bevy_image())
    };

    // --- UI ICONS (16x16) ---
    let icons = {
        // 1. Coin: Gold coin with shine
        let coin = {
            let mut c = PixelCanvas::new(16, 16);
            let rim = [200, 140, 20, 255];
            let gold = [255, 215, 0, 255];
            let shine = [255, 245, 120, 255];
            let dark = [160, 100, 10, 255];
            c.fill_circle(8, 8, 6, rim);
            c.fill_circle(8, 8, 5, gold);
            c.fill_circle(6, 6, 2, shine);
            c.set_pixel(8, 6, dark);
            c.set_pixel(8, 7, dark);
            c.set_pixel(8, 8, dark);
            c.set_pixel(8, 9, dark);
            c.set_pixel(7, 7, dark);
            c.set_pixel(9, 7, dark);
            images.add(c.to_bevy_image())
        };

        // 2. Leaf: Emerald ecosystem leaf
        let leaf = {
            let mut c = PixelCanvas::new(16, 16);
            let dark = [20, 100, 40, 255];
            let green = [50, 185, 75, 255];
            let light = [130, 240, 140, 255];
            for dy in 3..13 {
                let width = if dy < 8 { dy - 1 } else { 14 - dy };
                for dx in (8 - width)..=(8 + width) {
                    c.set_pixel(dx, dy, green);
                }
            }
            for y in 4..14 { c.set_pixel(8, y, dark); }
            c.set_pixel(7, 14, dark);
            c.set_pixel(6, 15, dark);
            c.set_pixel(6, 5, light);
            c.set_pixel(7, 6, light);
            images.add(c.to_bevy_image())
        };

        // 3. Island: Palm island
        let island = {
            let mut c = PixelCanvas::new(16, 16);
            let sea = [40, 120, 220, 255];
            let sand = [235, 205, 120, 255];
            let trunk = [130, 80, 40, 255];
            let palm = [30, 160, 50, 255];
            c.fill_rect(2, 11, 12, 3, sand);
            c.fill_rect(4, 10, 8, 1, sand);
            c.fill_rect(1, 14, 14, 2, sea);
            c.set_pixel(8, 9, trunk);
            c.set_pixel(8, 8, trunk);
            c.set_pixel(9, 7, trunk);
            c.fill_rect(6, 4, 7, 2, palm);
            c.set_pixel(5, 5, palm);
            c.set_pixel(13, 5, palm);
            images.add(c.to_bevy_image())
        };

        // 4. Honey: Honey jar
        let honey = {
            let mut c = PixelCanvas::new(16, 16);
            let honey_col = [255, 175, 20, 255];
            let honey_light = [255, 225, 70, 255];
            let lid = [180, 70, 30, 255];
            c.fill_rect(5, 2, 6, 2, lid);
            c.fill_rect(4, 4, 8, 1, [230, 200, 160, 255]);
            c.fill_rect(4, 5, 8, 8, honey_col);
            c.fill_rect(3, 7, 10, 5, honey_col);
            c.fill_rect(5, 7, 2, 4, honey_light);
            c.fill_rect(3, 12, 10, 1, [220, 235, 245, 255]);
            images.add(c.to_bevy_image())
        };

        // 5. Bee: Mini bee
        let bee_ico = {
            let mut c = PixelCanvas::new(16, 16);
            let yellow = [255, 210, 40, 255];
            let black = [35, 30, 30, 255];
            let wing = [210, 235, 255, 220];
            c.fill_rect(5, 3, 3, 4, wing);
            c.fill_rect(8, 3, 3, 4, wing);
            c.fill_rect(4, 7, 8, 5, yellow);
            c.fill_rect(6, 7, 2, 5, black);
            c.fill_rect(10, 7, 2, 5, black);
            c.set_pixel(4, 8, black);
            images.add(c.to_bevy_image())
        };

        // 6. Crown: Queen crown
        let crown = {
            let mut c = PixelCanvas::new(16, 16);
            let gold = [255, 205, 30, 255];
            let jewel = [230, 40, 60, 255];
            let base = [200, 140, 20, 255];
            c.fill_rect(3, 11, 10, 2, base);
            c.fill_rect(3, 7, 2, 4, gold);
            c.fill_rect(7, 5, 2, 6, gold);
            c.fill_rect(11, 7, 2, 4, gold);
            c.set_pixel(7, 10, jewel);
            c.set_pixel(8, 10, jewel);
            images.add(c.to_bevy_image())
        };

        // 7. Flower: Petal blossom
        let flower = {
            let mut c = PixelCanvas::new(16, 16);
            let petal = [255, 110, 150, 255];
            let center = [255, 225, 50, 255];
            c.fill_circle(5, 5, 2, petal);
            c.fill_circle(11, 5, 2, petal);
            c.fill_circle(5, 11, 2, petal);
            c.fill_circle(11, 11, 2, petal);
            c.fill_circle(8, 8, 3, center);
            images.add(c.to_bevy_image())
        };

        // 8. Frame: Hive frame
        let frame = {
            let mut c = PixelCanvas::new(16, 16);
            let wood = [140, 90, 45, 255];
            let wax = [250, 205, 70, 255];
            let dark = [210, 160, 40, 255];
            c.fill_rect(2, 2, 12, 12, wood);
            c.fill_rect(4, 4, 8, 8, wax);
            c.set_pixel(6, 6, dark);
            c.set_pixel(9, 6, dark);
            c.set_pixel(6, 9, dark);
            c.set_pixel(9, 9, dark);
            images.add(c.to_bevy_image())
        };

        // 9. Feeder: Syrup bottle & drop
        let feeder = {
            let mut c = PixelCanvas::new(16, 16);
            let blue = [50, 160, 240, 255];
            let white = [240, 248, 255, 255];
            c.fill_rect(6, 2, 4, 2, [220, 80, 50, 255]);
            c.fill_rect(5, 4, 6, 9, white);
            c.fill_rect(5, 8, 6, 5, blue);
            c.set_pixel(7, 14, blue);
            c.set_pixel(8, 14, blue);
            images.add(c.to_bevy_image())
        };

        // 10. Warning: Alert exclamation
        let warning = {
            let mut c = PixelCanvas::new(16, 16);
            let yellow = [255, 190, 20, 255];
            let dark = [40, 30, 20, 255];
            for y in 2..14 {
                let half_w = (y - 2) * 6 / 11;
                for x in (8 - half_w)..=(8 + half_w) {
                    c.set_pixel(x, y, yellow);
                }
            }
            c.fill_rect(7, 5, 2, 4, dark);
            c.fill_rect(7, 11, 2, 2, dark);
            images.add(c.to_bevy_image())
        };

        // 11. Patty: Protein pollen patty
        let patty = {
            let mut c = PixelCanvas::new(16, 16);
            let tan = [170, 110, 50, 255];
            let pollen = [255, 220, 40, 255];
            c.fill_rect(3, 5, 10, 7, tan);
            c.fill_rect(4, 4, 8, 2, [215, 160, 80, 255]);
            c.set_pixel(5, 7, pollen);
            c.set_pixel(8, 6, pollen);
            c.set_pixel(10, 8, pollen);
            images.add(c.to_bevy_image())
        };

        // 12. Book: Beedex book
        let book = {
            let mut c = PixelCanvas::new(16, 16);
            let leather = [120, 60, 30, 255];
            let gold = [255, 205, 40, 255];
            c.fill_rect(3, 3, 10, 11, leather);
            c.fill_rect(12, 4, 1, 9, [245, 240, 225, 255]);
            c.fill_rect(6, 6, 4, 4, gold);
            images.add(c.to_bevy_image())
        };

        // 13. Craft: Hammer
        let craft = {
            let mut c = PixelCanvas::new(16, 16);
            let steel = [160, 175, 185, 255];
            let wood = [140, 85, 40, 255];
            for i in 0..7 {
                c.set_pixel(4 + i, 12 - i, wood);
            }
            c.fill_rect(9, 3, 5, 3, steel);
            images.add(c.to_bevy_image())
        };

        // 14. Gear: Centrifuge gear
        let gear = {
            let mut c = PixelCanvas::new(16, 16);
            let brass = [210, 155, 45, 255];
            c.fill_circle(8, 8, 5, brass);
            c.set_pixel(8, 2, brass);
            c.set_pixel(8, 13, brass);
            c.set_pixel(2, 8, brass);
            c.set_pixel(13, 8, brass);
            c.fill_circle(8, 8, 2, [0, 0, 0, 0]);
            images.add(c.to_bevy_image())
        };

        // 15. Microscope: Scope lens
        let microscope = {
            let mut c = PixelCanvas::new(16, 16);
            let metal = [140, 155, 170, 255];
            let glass = [100, 200, 240, 255];
            c.fill_rect(4, 12, 8, 2, metal);
            c.fill_rect(10, 6, 2, 7, metal);
            c.fill_rect(5, 4, 4, 5, metal);
            c.fill_rect(6, 9, 2, 1, glass);
            images.add(c.to_bevy_image())
        };

        // 16. Market: Shop awning
        let market = {
            let mut c = PixelCanvas::new(16, 16);
            let red = [220, 50, 50, 255];
            let white = [245, 245, 245, 255];
            c.fill_rect(2, 3, 12, 3, red);
            c.fill_rect(4, 3, 2, 3, white);
            c.fill_rect(8, 3, 2, 3, white);
            c.fill_rect(12, 3, 2, 3, white);
            c.fill_rect(3, 7, 10, 6, [190, 140, 80, 255]);
            images.add(c.to_bevy_image())
        };

        // 17. Ferry: Sailboat
        let ferry = {
            let mut c = PixelCanvas::new(16, 16);
            let sail = [245, 245, 250, 255];
            let wood = [135, 75, 35, 255];
            for y in 3..10 {
                c.fill_rect(8, y, y - 2, 1, sail);
            }
            c.fill_rect(7, 2, 1, 9, [90, 50, 25, 255]);
            c.fill_rect(3, 11, 10, 2, wood);
            c.fill_rect(1, 14, 14, 1, [50, 130, 230, 255]);
            images.add(c.to_bevy_image())
        };

        // 18. Exit: Power / Exit icon
        let exit = {
            let mut c = PixelCanvas::new(16, 16);
            let red = [225, 45, 55, 255];
            let white = [255, 255, 255, 255];
            c.fill_circle(8, 8, 6, red);
            c.fill_rect(7, 3, 2, 5, white);
            c.fill_circle(8, 8, 4, white);
            c.fill_circle(8, 8, 2, red);
            c.fill_rect(7, 3, 2, 5, white);
            images.add(c.to_bevy_image())
        };

        UiIcons {
            coin,
            leaf,
            island,
            honey,
            bee: bee_ico,
            crown,
            flower,
            frame,
            feeder,
            warning,
            patty,
            book,
            craft,
            gear,
            microscope,
            market,
            ferry,
            exit,
        }
    };

    GamePixelAssets {
        icons,
        player,
        player_sprites,
        wife_eung,
        bee,
        butterfly_brimstone,
        butterfly_peacock,
        butterfly_morpho,
        butterfly_monarch,
        butterfly_moon,
        butterfly_rainbow,
        flower_red,
        flower_yellow,
        flower_blue,
        flower_purple,
        flower_white,
        flower_gold,
        apiary,
        solitary_hotel,
        tree,
        rock,
        shrine,
        ferry_boat,
        wild_hive,
    }
}
