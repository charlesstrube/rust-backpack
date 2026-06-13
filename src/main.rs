// See EXERCICES.md for the current TODO list.

use crate::backpack::Backpack;

mod backpack;
mod error;
mod item;
mod item_summary;
mod rarity;
mod utils;

fn main() {
    let backpack = Backpack::new(50);
    drop(backpack);
    println!("Inventory exercise - run `cargo test` to validate your implementation.");
}
