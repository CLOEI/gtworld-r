<br/>
<div align="center">
<h3 align="center">GTworld-r</h3>
<p align="center">
World tile serialization
</p>
</div>

## About The Project

This is a Growtopia world tile serialization library. This library also depends on an [items.dat parser](https://github.com/CLOEI/gtitem-r), so make sure to install it as well to be able to use this library.

## Usage

```rust
use gtworld_r::World;
use gtitem_r::load_from_file;

fn main() {
  let item_database = load_from_file("items.dat").unwrap();
  let world = World::new().parse(world_data, &item_database);
  println!("{:?}", world)
}
```

## Property

### World

- name: String
- width: u32
- height: u32
- tile_count: u32
- tiles: Vec\<Tile>
- dropped: Dropped
- base_weather: u16
- current_weather: u16
- item_database: Arc\<ItemDatabase>

### Tile

- foreground_item_id: u16
- background_item_id: u16
- parent_block_index: u16
- flags: u16
- tile_type: TileType

### TileType

- Basic
- Door
  - text: String
  - unknown_1: u8
- Sign
  - text: String
  - unknown_1: u32
- Lock
  - settings: u8
  - owner_uid: u32
  - access_count: u32
  - access_uids: Vec<u32>
  - minimum_level: u8
  - unknown_1: [u8; 7]
- Seed
  - time_passed: u32
  - item_on_tree: u8
- Mailbox
  - unknown_1: String
  - unknown_2: String
  - unknown_3: String
  - unknown_4: u8
- Bulletin
  - unknown_1: String
  - unknown_2: String
  - unknown_3: String
  - unknown_4: u8
- Dice
  - symbol: u8
- ChemicalSource
  - time_passed: u32
- AchievementBlock
  - unknown_1: u32
  - tile_type: u8
- HeartMonitor
  - unknown_1: u32
  - player_name: String
- DonationBox
  - unknown_1: String
  - unknown_2: String
  - unknown_3: String
  - unknown_4: u8
- Mannequin
  - text: String
  - unknown_1: u8
  - clothing_1: u32
  - clothing_2: u16
  - clothing_3: u16
  - clothing_4: u16
  - clothing_5: u16
  - clothing_6: u16
  - clothing_7: u16
  - clothing_8: u16
  - clothing_9: u16
  - clothing_10: u16
- BunnyEgg
  - egg_placed: u32
- GamePack
  - team: u8
- GameGenerator
- XenoniteCrystal
  - unknown_1: u8
  - unknown_2: u32
- PhoneBooth
  - clothing_1: u16
  - clothing_2: u16
  - clothing_3: u16
  - clothing_4: u16
  - clothing_5: u16
  - clothing_6: u16
  - clothing_7: u16
  - clothing_8: u16
  - clothing_9: u16
- Crystal
  - unknown_1: String
- CrimeInProgress
  - unknown_1: String
  - unknown_2: u32
  - unknown_3: u8
- DisplayBlock
  - item_id: u32
- VendingMachine
  - item_id: u32
  - price: i32
- GivingTree
  - unknown_1: u16
  - unknown_2: u32
- CountryFlag
  - country: String
- WeatherMachine
  - item_id: u32
- DataBedrock
  - unknown_1: [u8; 21]

## Contribution

Contributions to improve this library are highly appreciated. If you have any ideas, bug fixes, or new features to suggest, please feel free to open an issue or submit a pull request on the [GitHub repository](https://github.com/cloei/gtworld-r). Your contributions will help make this library even better for the Growtopia community.

Thank you for your support!

### Credit

[Badewen](https://github.com/badewen/GrowDocs/) - For growtopia packet documentation.
