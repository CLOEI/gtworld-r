use gtitem_r::structs::ItemDatabase;
use std::io::{Cursor, Read};
use std::sync::Arc;

use byteorder::{LittleEndian, ReadBytesExt};

pub struct World {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub tile_count: u32,
    pub tiles: Vec<Tile>,
    pub dropped: Dropped,
    pub base_weather: u16,
    pub current_weather: u16,
    pub item_database: Arc<ItemDatabase>,
}

#[derive(Debug)]
pub struct Tile {
    pub foreground_item_id: u16,
    pub background_item_id: u16,
    pub parent_block_index: u16,
    pub flags: u16,
    pub tile_type: TileType,
}

#[derive(Debug)]
pub enum TileType {
    Basic,
    Door {
        text: String,
        unknown_1: u8,
    },
    Sign {
        text: String,
        unknown_1: u32,
    },
    Lock {
        settings: u8,
        owner_uid: u32,
        access_count: u32,
        access_uids: Vec<u32>,
        minimum_level: u8,
        unknown_1: [u8; 7],
    },
    Seed {
        time_passed: u32,
        item_on_tree: u8,
    },
    Mailbox {
        unknown_1: String,
        unknown_2: String,
        unknown_3: String,
        unknown_4: u8,
    },
    Bulletin {
        unknown_1: String,
        unknown_2: String,
        unknown_3: String,
        unknown_4: u8,
    },
    Dice {
        symbol: u8,
    },
    ChemicalSource {
        time_passed: u32,
    },
    AchievemntBlock {
        unknown_1: u32,
        tile_type: u8,
    },
    HearthMonitor {
        unknown_1: u32,
        player_name: String,
    },
    DonationBox {
        unknown_1: String,
        unknown_2: String,
        unknown_3: String,
        unknown_4: u8,
    },
    Mannequin {
        text: String,
        unknown_1: u8,
        clothing_1: u32,
        clothing_2: u16,
        clothing_3: u16,
        clothing_4: u16,
        clothing_5: u16,
        clothing_6: u16,
        clothing_7: u16,
        clothing_8: u16,
        clothing_9: u16,
        clothing_10: u16,
    },
    BunnyEgg {
        egg_placed: u32,
    },
    GamePack {
        team: u8,
    },
    GameGenerator {},
    XenoniteCrystal {
        unknown_1: u8,
        unknown_2: u32,
    },
    PhoneBooth {
        clothing_1: u16,
        clothing_2: u16,
        clothing_3: u16,
        clothing_4: u16,
        clothing_5: u16,
        clothing_6: u16,
        clothing_7: u16,
        clothing_8: u16,
        clothing_9: u16,
    },
    Crystal {
        unknown_1: String,
    },
    CrimeInProgress {
        unknown_1: String,
        unknown_2: u32,
        unknown_3: u8,
    },
    DisplayBlock {
        item_id: u32,
    },
    VendingMachine {
        item_id: u32,
        price: i32,
    },
    GivingTree {
        unknown_1: u16,
        unknown_2: u32,
    },
    CountryFlag {
        country: String,
    },
    WeatherMachine {
        item_id: u32,
    },
    DataBedrock {
        unknown_1: [u8; 21],
    },
}

#[derive(Debug)]
pub struct Dropped {
    pub items_count: u32,
    pub last_dropped_item_uid: u32,
    pub items: Vec<DroppedItem>,
}

#[derive(Debug)]
pub struct DroppedItem {
    pub id: u16,
    pub x: f32,
    pub y: f32,
    pub count: u8,
    pub flags: u8,
    pub uid: u32,
}

impl Tile {
    pub fn new(
        foreground_item_id: u16,
        background_item_id: u16,
        parent_block_index: u16,
        flags: u16,
    ) -> Tile {
        Tile {
            foreground_item_id,
            background_item_id,
            parent_block_index,
            flags,
            tile_type: TileType::Basic,
        }
    }
}

impl World {
    pub fn new(item_database: Arc<ItemDatabase>) -> World {
        World {
            name: "EXIT".to_string(),
            width: 0,
            height: 0,
            tile_count: 0,
            tiles: Vec::new(),
            dropped: Dropped {
                items_count: 0,
                last_dropped_item_uid: 0,
                items: Vec::new(),
            },
            base_weather: 0,
            current_weather: 0,
            item_database,
        }
    }

    pub fn parse(&mut self, data: &[u8]) {
        let mut data = Cursor::new(data);
        // first 6 byte is unknown
        data.set_position(data.position() + 6);
        let str_len = data.read_u16::<LittleEndian>().unwrap();
        let mut name = vec![0; str_len as usize];
        data.read_exact(&mut name).unwrap();
        let width = data.read_u32::<LittleEndian>().unwrap();
        let height = data.read_u32::<LittleEndian>().unwrap();
        let tile_count = data.read_u32::<LittleEndian>().unwrap();
        data.set_position(data.position() + 5);
        self.name = String::from_utf8_lossy(&name).to_string();
        self.width = width;
        self.height = height;
        self.tile_count = tile_count;

        // tiles
        for _ in 0..tile_count {
            let mut tile = Tile::new(0, 0, 0, 0);
            tile.foreground_item_id = data.read_u16::<LittleEndian>().unwrap();
            tile.background_item_id = data.read_u16::<LittleEndian>().unwrap();
            tile.parent_block_index = data.read_u16::<LittleEndian>().unwrap();
            tile.flags = data.read_u16::<LittleEndian>().unwrap();

            if (tile.flags & 0x1) != 0 {
                let extra_tile_type = data.read_u8().unwrap();
                self.get_extra_tile_data(&mut tile, &mut data, extra_tile_type);
            }

            if (tile.flags & 0x2) != 0 {
                data.read_u16::<LittleEndian>().unwrap();
            }

            self.tiles.push(tile);
        }

        data.set_position(data.position() + 12); // it exist in the binary, i don't know what it is
        self.dropped.items_count = data.read_u32::<LittleEndian>().unwrap();
        self.dropped.last_dropped_item_uid = data.read_u32::<LittleEndian>().unwrap();
        for _ in 0..self.dropped.items_count {
            let id = data.read_u16::<LittleEndian>().unwrap();
            let x = data.read_f32::<LittleEndian>().unwrap();
            let y = data.read_f32::<LittleEndian>().unwrap();
            let count = data.read_u8().unwrap();
            let flags = data.read_u8().unwrap();
            let uid = data.read_u32::<LittleEndian>().unwrap();
            self.dropped.items.push(DroppedItem {
                id,
                x,
                y,
                count,
                flags,
                uid,
            });
        }

        self.base_weather = data.read_u16::<LittleEndian>().unwrap();
        data.read_u16::<LittleEndian>().unwrap(); // unknown
        self.current_weather = data.read_u16::<LittleEndian>().unwrap();
    }

    fn get_extra_tile_data(&self, tile: &mut Tile, data: &mut Cursor<&[u8]>, item_type: u8) {
        match item_type {
            1 => {
                let str_len = data.read_u16::<LittleEndian>().unwrap();
                let mut text = vec![0; str_len as usize];
                data.read_exact(&mut text).unwrap();
                let text = String::from_utf8_lossy(&text).to_string();
                let unknown_1 = data.read_u8().unwrap();

                tile.tile_type = TileType::Door { text, unknown_1 };
            }
            2 => {
                let str_len = data.read_u16::<LittleEndian>().unwrap();
                let mut text = vec![0; str_len as usize];
                data.read_exact(&mut text).unwrap();
                let text = String::from_utf8_lossy(&text).to_string();
                let unknown_1 = data.read_u32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::Sign { text, unknown_1 };
            }
            3 => {
                let settings = data.read_u8().unwrap();
                let owner_uid = data.read_u32::<LittleEndian>().unwrap();
                let access_count = data.read_u32::<LittleEndian>().unwrap();
                let mut access_uids = Vec::new();
                for _ in 0..access_count {
                    access_uids.push(data.read_u32::<LittleEndian>().unwrap());
                }
                let minimum_level = data.read_u8().unwrap();
                let mut unknown_1 = [0; 7];
                data.read_exact(&mut unknown_1).unwrap();

                if tile.foreground_item_id == 5814 {
                    let _ = data.read_exact([0; 16].as_mut());
                }

                tile.tile_type = TileType::Lock {
                    settings,
                    owner_uid,
                    access_count,
                    access_uids,
                    minimum_level,
                    unknown_1,
                };
            }
            4 => {
                let time_passed = data.read_u32::<LittleEndian>().unwrap();
                let item_on_tree = data.read_u8().unwrap();

                tile.tile_type = TileType::Seed {
                    time_passed,
                    item_on_tree,
                };
            }
            6 => {
                let unknown_1_len = data.read_u16::<LittleEndian>().unwrap();
                let mut unknown_1 = vec![0; unknown_1_len as usize];
                data.read_exact(&mut unknown_1).unwrap();

                let unknown_2_len = data.read_u16::<LittleEndian>().unwrap();
                let mut unknown_2 = vec![0; unknown_2_len as usize];
                data.read_exact(&mut unknown_2).unwrap();

                let unknown_3_len = data.read_u16::<LittleEndian>().unwrap();
                let mut unknown_3 = vec![0; unknown_3_len as usize];
                data.read_exact(&mut unknown_3).unwrap();

                let unknown_4 = data.read_u8().unwrap();

                tile.tile_type = TileType::Mailbox {
                    unknown_1: String::from_utf8_lossy(&unknown_1).to_string(),
                    unknown_2: String::from_utf8_lossy(&unknown_2).to_string(),
                    unknown_3: String::from_utf8_lossy(&unknown_3).to_string(),
                    unknown_4,
                };
            }
            7 => {
                let unknown_1_len = data.read_u16::<LittleEndian>().unwrap();
                let mut unknown_1 = vec![0; unknown_1_len as usize];
                data.read_exact(&mut unknown_1).unwrap();

                let unknown_2_len = data.read_u16::<LittleEndian>().unwrap();
                let mut unknown_2 = vec![0; unknown_2_len as usize];
                data.read_exact(&mut unknown_2).unwrap();

                let unknown_3_len = data.read_u16::<LittleEndian>().unwrap();
                let mut unknown_3 = vec![0; unknown_3_len as usize];
                data.read_exact(&mut unknown_3).unwrap();

                let unknown_4 = data.read_u8().unwrap();

                tile.tile_type = TileType::Bulletin {
                    unknown_1: String::from_utf8_lossy(&unknown_1).to_string(),
                    unknown_2: String::from_utf8_lossy(&unknown_2).to_string(),
                    unknown_3: String::from_utf8_lossy(&unknown_3).to_string(),
                    unknown_4,
                };
            }
            8 => {
                let symbol = data.read_u8().unwrap();

                tile.tile_type = TileType::Dice { symbol };
            }
            9 => {
                let time_passed = data.read_u32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::ChemicalSource { time_passed };
            }
            10 => {
                let unknown_1 = data.read_u32::<LittleEndian>().unwrap();
                let tile_type = data.read_u8().unwrap();

                tile.tile_type = TileType::AchievemntBlock {
                    unknown_1,
                    tile_type,
                };
            }
            11 => {
                let unknown_1 = data.read_u32::<LittleEndian>().unwrap();
                let str_len = data.read_u16::<LittleEndian>().unwrap();
                let mut player_name = vec![0; str_len as usize];
                data.read_exact(&mut player_name).unwrap();
                let player_name = String::from_utf8_lossy(&player_name).to_string();

                tile.tile_type = TileType::HearthMonitor {
                    unknown_1,
                    player_name,
                };
            }
            12 => {
                let unknown_1_len = data.read_u16::<LittleEndian>().unwrap();
                let mut unknown_1 = vec![0; unknown_1_len as usize];
                data.read_exact(&mut unknown_1).unwrap();

                let unknown_2_len = data.read_u16::<LittleEndian>().unwrap();
                let mut unknown_2 = vec![0; unknown_2_len as usize];
                data.read_exact(&mut unknown_2).unwrap();

                let unknown_3_len = data.read_u16::<LittleEndian>().unwrap();
                let mut unknown_3 = vec![0; unknown_3_len as usize];
                data.read_exact(&mut unknown_3).unwrap();

                let unknown_4 = data.read_u8().unwrap();

                tile.tile_type = TileType::DonationBox {
                    unknown_1: String::from_utf8_lossy(&unknown_1).to_string(),
                    unknown_2: String::from_utf8_lossy(&unknown_2).to_string(),
                    unknown_3: String::from_utf8_lossy(&unknown_3).to_string(),
                    unknown_4,
                };
            }
            14 => {
                let str_len = data.read_u16::<LittleEndian>().unwrap();
                let mut text = vec![0; str_len as usize];
                data.read_exact(&mut text).unwrap();
                let text = String::from_utf8_lossy(&text).to_string();
                let unknown_1 = data.read_u8().unwrap();
                let clothing_1 = data.read_u32::<LittleEndian>().unwrap();
                let clothing_2 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_3 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_4 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_5 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_6 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_7 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_8 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_9 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_10 = data.read_u16::<LittleEndian>().unwrap();

                tile.tile_type = TileType::Mannequin {
                    text,
                    unknown_1,
                    clothing_1,
                    clothing_2,
                    clothing_3,
                    clothing_4,
                    clothing_5,
                    clothing_6,
                    clothing_7,
                    clothing_8,
                    clothing_9,
                    clothing_10,
                };
            }
            15 => {
                let egg_placed = data.read_u32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::BunnyEgg { egg_placed };
            }
            16 => {
                let team = data.read_u8().unwrap();

                tile.tile_type = TileType::GamePack { team };
            }
            17 => {
                tile.tile_type = TileType::GameGenerator {};
            }
            18 => {
                let unknown_1 = data.read_u8().unwrap();
                let unknown_2 = data.read_u32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::XenoniteCrystal {
                    unknown_1,
                    unknown_2,
                };
            }
            19 => {
                let clothing_1 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_2 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_3 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_4 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_5 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_6 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_7 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_8 = data.read_u16::<LittleEndian>().unwrap();
                let clothing_9 = data.read_u16::<LittleEndian>().unwrap();

                tile.tile_type = TileType::PhoneBooth {
                    clothing_1,
                    clothing_2,
                    clothing_3,
                    clothing_4,
                    clothing_5,
                    clothing_6,
                    clothing_7,
                    clothing_8,
                    clothing_9,
                };
            }
            20 => {
                let str_len = data.read_u16::<LittleEndian>().unwrap();
                let mut unknown_1 = vec![0; str_len as usize];
                data.read_exact(&mut unknown_1).unwrap();
                let unknown_1 = String::from_utf8_lossy(&unknown_1).to_string();

                tile.tile_type = TileType::Crystal { unknown_1 };
            }
            21 => {
                let str_len = data.read_u16::<LittleEndian>().unwrap();
                let mut unknown_1 = vec![0; str_len as usize];
                data.read_exact(&mut unknown_1).unwrap();
                let unknown_1 = String::from_utf8_lossy(&unknown_1).to_string();
                let unknown_2 = data.read_u32::<LittleEndian>().unwrap();
                let unknown_3 = data.read_u8().unwrap();

                tile.tile_type = TileType::CrimeInProgress {
                    unknown_1,
                    unknown_2,
                    unknown_3,
                };
            }
            23 => {
                let item_id = data.read_u32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::DisplayBlock { item_id };
            }
            24 => {
                let item_id = data.read_u32::<LittleEndian>().unwrap();
                let price = data.read_i32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::VendingMachine { item_id, price };
            }
            28 => {
                let unknown_1 = data.read_u16::<LittleEndian>().unwrap();
                let unknown_2 = data.read_u32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::GivingTree {
                    unknown_1,
                    unknown_2,
                };
            }
            33 => {
                let country_len = data.read_u16::<LittleEndian>().unwrap();
                let mut country = vec![0; country_len as usize];
                data.read_exact(&mut country).unwrap();
                let country = String::from_utf8_lossy(&country).to_string();

                tile.tile_type = TileType::CountryFlag { country };
            }
            40 => {
                let item_id = data.read_u32::<LittleEndian>().unwrap();

                tile.tile_type = TileType::WeatherMachine { item_id };
            }
            42 => {
                let mut unknown_1 = [0; 21];
                data.read_exact(&mut unknown_1).unwrap();

                tile.tile_type = TileType::DataBedrock { unknown_1 };
            }
            _ => {
                tile.tile_type = TileType::Basic;
            }
        };
    }
}

#[test]
fn test_render_world() {
    use gtitem_r::load_from_file;
    use image::{ImageBuffer, Rgba};
    use std::fs::File;

    let item_database = Arc::new(load_from_file("items.dat").unwrap());
    let mut world = World::new(item_database);
    // get byte from world.dat file
    let mut file = File::open("world.dat").unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    world.parse(&data);

    let item_pixel_size = 32;
    let img_width = world.width * item_pixel_size;
    let img_height = world.height * item_pixel_size;
    let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(img_width as u32, img_height as u32);

    for x in 0..world.width {
        for y in 0..world.height {
            // get current tile
            let tile = &world.tiles[(y * world.width + x) as usize];
            let item = world
                .item_database
                .get_item(&(tile.foreground_item_id as u32))
                .unwrap();
            let mut color = Rgba([0, 0, 0, 255]);
            if item.name == "Blank" {
                color = Rgba([0, 0, 255, 255]);
            } else if !item.name.contains("Seed") {
                color = Rgba([139, 69, 19, 255])
            }
            if item.name.contains("Seed") {
                color = Rgba([0, 255, 0, 255])
            }
            if item.name.contains("Door") {
                color = Rgba([255, 255, 255, 255])
            }
            if item.name.contains("Rock") {
                color = Rgba([128, 128, 128, 255])
            }
            if item.name.contains("Lava") {
                color = Rgba([255, 0, 0, 255])
            }
            if item.name.contains("Bedrock") {
                color = Rgba([0, 0, 0, 255])
            }
            if item.name.contains("Lock") {
                // yellow
                color = Rgba([255, 255, 0, 255])
            }
            if item.name.contains("Sign") {
                // orange
                color = Rgba([255, 165, 0, 255])
            }

            for px in 0..item_pixel_size {
                for py in 0..item_pixel_size {
                    let pixel_x = (x * item_pixel_size + px) as u32;
                    let pixel_y = (y * item_pixel_size + py) as u32;
                    img.put_pixel(pixel_x, pixel_y, color);
                }
            }
        }
    }

    img.save("output.png").unwrap();
}
