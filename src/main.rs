// ============================================================================
// TODO (FR) - Ordre d'implémentation suggéré (du plus simple au plus dur)
// ============================================================================
//
// Phase 1 - Trait impls (formate les valeurs, compare l'égalité)
//   1.  Debug + PartialEq + Copy + Clone pour Rarity     (src/item.rs)
//   2.  Debug + PartialEq pour InventoryError            (src/error.rs)
//   3.  Debug + PartialEq pour ItemKind                  (src/item.rs)
//   4.  Debug + PartialEq pour Item                      (src/item.rs)
//
// Phase 2 - Méthodes de Backpack et Item
//   5.  Backpack::new                                    (src/backpack.rs)
//   6.  Backpack::total_weight
//   7.  Backpack::add_item
//   8.  Backpack::count_by_rarity
//   9.  Backpack::remove_item
//   10. Item::describe (trait Describable)               (src/item.rs)
//   11. Backpack::strongest_weapon                       (src/backpack.rs)
//   12. Backpack::total_value
//
// Phase 3 - Itérateurs
//   13. Backpack::find_by_name                           (src/backpack.rs)
//   14. Backpack::weapons
//   15. impl IntoIterator for &Backpack
//   16. impl IntoIterator for Backpack
//
// Lance `cargo test` pour voir tous les tests échouer, puis vise un test à la fois :
//   cargo test new_backpack_is_empty
// ============================================================================

mod backpack;
mod error;
mod item;

fn main() {
    println!("Inventory exercise - run `cargo test` to validate your implementation.");
}
