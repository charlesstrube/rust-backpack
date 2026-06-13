# Backpack — Exercices d'extension

Suite du mini-exo inventaire. Couvre les concepts vus depuis la dernière fois.
Lance `cargo test <nom_du_test>` pour itérer sur un test à la fois.

Les phases sont ordonnées pour minimiser les conflits (refactos d'abord, ajouts ensuite).
Tu peux dévier l'ordre, mais **la Phase 4 doit passer avant les Phases 12 et 13** (elles dépendent de `Eq + Hash` sur `Rarity` et de `Clone` sur `Item`).

---

## Phase 1 — Encapsulation, accesseurs, setters
Concepts : encapsulation, accessor methods, setters, visibility, string slices, vec resizing

- [x] Rendre privés les champs de `Backpack` et `Item`
- [x] Getters sur `Item` : `name()`, `kind()`, `rarity()`, `weight()`
- [x] `Backpack::items()` retournant `&[Item]`
- [x] `Backpack::max_weight()` getter
- [x] `Backpack::set_max_weight(&mut self, value: u32) -> Result<(), InventoryError>` qui refuse une valeur inférieure au poids total actuel
- [x] `Backpack::reserve(&mut self, additional: usize)` qui expose le redimensionnement du Vec

Tests :
- `item_getters_return_field_values`
- `backpack_items_getter_exposes_slice`
- `backpack_set_max_weight_accepts_increase`
- `backpack_set_max_weight_rejects_below_current_load`
- `reserve_increases_capacity_without_changing_len`

⚠️ Adapter les tests existants qui lisent les champs directement.

---

## Phase 1.5 — Manual `From<E1> for E2` (préparation \)
Concepts : From trait, error propagation, `?` operator, error wrapping

Cette mini-phase prépare le terrain pour que `Item::new` (Phase 2) puisse utiliser l'opérateur `?` au lieu d'un `panic!`. Elle introduit le pattern `?` + `From` qui revient partout en Rust.

Contexte : tu as déjà un type d'erreur côté newtype (`ItemNameError::NameIsEmpty`) côté `ItemName::try_from`. Il faut pouvoir le **convertir** vers `InventoryError` pour que `?` fonctionne dans `Item::new`.

- [x] Ajouter le variant `InvalidName` à `InventoryError` (les autres variants `InvalidWeight`, `InvalidMaxWeight` viendront en Phase 2)
- [x] Implémenter `impl From<ItemNameError> for InventoryError` à la main — mappe `NameIsEmpty` → `InvalidName`
- [x] Vérifier mentalement que `let name: ItemName = some_string.try_into()?;` compile désormais dans un contexte où la fonction retourne `Result<_, InventoryError>`

Tests :
- `inventory_error_from_item_name_error_maps_to_invalid_name`
- `question_mark_propagates_item_name_error_into_inventory_error` (petite fonction de démo qui retourne `Result<ItemName, InventoryError>` et utilise `?`)

**À retenir** : ce `impl From` manuel sera remplacé en Phase 5 par un `#[from]` généré par `thiserror`. Le code appelant (`?`) ne changera pas — c'est tout l'intérêt du pattern.

**À éviter** : `.map_err(|_| InventoryError::InvalidName)?`. Ça compile, mais ça **jette** la cause sous-jacente, ce qui rend `Error::source()` (Phase 5) inutile pour ce variant. Utilise `From` + `?` à la place.

---

## Phase 2 — Constructeurs validés et `panic!`
Concepts : validation in constructor, error enums, panics

- [x] `Item::new(name: String, kind: ItemKind, rarity: Rarity, weight: u32) -> Result<Item, InventoryError>` qui rejette nom vide et poids zéro
- [x] `Backpack::new` retourne désormais `Result<Backpack, InventoryError>` (rejette `max_weight == 0`)
- [x] Nouveaux variants d'erreur : `InvalidName`, `InvalidWeight`, `InvalidMaxWeight`
- [ ] Ajouter un point dans le code où `panic!` (ou `unreachable!`) est justifié — invariant interne qui ne peut pas être violé depuis l'extérieur

Tests :
- `item_new_returns_ok_for_valid_input`
- `item_new_rejects_empty_name`
- `item_new_rejects_zero_weight`
- `backpack_new_rejects_zero_max_weight`

⚠️ Les anciens tests doivent passer par `Item::new(...).unwrap()` et `Backpack::new(...).unwrap()`.

---

## Phase 2.1 — Test back-door (`#[cfg(test)]`)
Concepts : conditional compilation, test-only API, invariants vs testabilité

Contexte : `add_item` garantit `total_weight ≤ max_weight`, et `max_weight: u32`. Donc par construction, la somme des poids ne peut jamais déborder un `u32` via l'API publique. Les méthodes de la Phase 3 (`total_weight_saturating`, `total_weight_checked`) deviennent intestables sans une porte dérobée.

- [ ] Ajouter un helper sur `Backpack` qui pousse un `Item` sans vérifier la capacité, gated derrière `#[cfg(test)]`
- [ ] L'invariant normal d'`add_item` reste intact en prod (le helper n'existe pas hors tests)

Pas de test dédié — ce helper est l'**outil** qui rend les tests de Phase 3 (`*_caps_at_u32_max`, `*_returns_none_on_overflow`) exécutables.

**À retenir** : `#[cfg(test)]` permet d'exposer une surface de test sans polluer l'API publique. C'est légitime quand on veut tester une propriété qui contredit une invariante normale. À distinguer d'un constructeur "unsafe" public, qui lui exposerait le contournement à tout appelant.

---

## Phase 3 — Arithmétique sécurisée, casting, factorielle
Concepts : overflow/underflow, saturating arithmetic, as casting, factorial, while/for

- [x] `Backpack::total_weight_saturating(&self) -> u32` avec `saturating_add`
- [x] `Backpack::total_weight_checked(&self) -> Option<u32>` avec `checked_add`
- [x] `Backpack::average_weight(&self) -> f64` (utilise un cast `as f64`)
- [x] Helper libre `fn factorial(n: u32) -> u128` (boucle `for` ou `while`)
- [x] `Backpack::slot_combinations(&self, slots: u32) -> u128` qui appelle `factorial`

Tests :
- `total_weight_saturating_caps_at_u32_max`
- `total_weight_checked_returns_none_on_overflow`
- `average_weight_returns_zero_for_empty_backpack`
- `average_weight_computes_mean`
- `factorial_of_five_is_one_hundred_twenty`
- `slot_combinations_uses_factorial`

---

## Phase 4 — Macros `derive` et règle de l'orphelin
Concepts : derive macros, orphan rule, Copy, Clone, Eq, Hash

- [x] Remplacer tous les `impl Debug`, `impl PartialEq`, `impl Clone`, `impl Copy` manuels par `#[derive(...)]` sur `Rarity`, `ItemKind`, `Item`, `InventoryError`
- [x] Ajouter `Eq` et `Hash` au derive de `Rarity` (requis Phase 12)
- [x] Ajouter `Clone` au derive de `Item` (requis Phases 9, 13)
- [x] Conserver `impl Display for ItemKind` manuel
- [x] Ajouter un commentaire en anglais dans le code qui explique pourquoi `impl Display for Vec<Item>` est interdit (orphan rule)

Test :
- `rarity_works_as_hashmap_key` — petit smoke test qui insère deux `Rarity` dans un `HashMap`

---

## Phase 5 — `thiserror`, trait `Error`, chaîne `source`
Concepts : packages, dependencies, thiserror, Error trait, error source

- [x] `cargo add thiserror`
- [x] Réécrire `InventoryError` avec `#[derive(thiserror::Error, Debug)]` et un `#[error("...")]` par variante
- [x] Ajouter un variant `Parse(#[from] ParseRarityError)` (le type sera créé en Phase 6) pour que `?` convertisse automatiquement
- [x] `InventoryError::source()` doit renvoyer la cause interne pour le variant `Parse`

Tests :
- `inventory_error_implements_std_error`
- `inventory_error_display_shows_human_message`
- `inventory_error_source_returns_inner_for_parse_variant`

---

## Phase 6 — `From` et `TryFrom`
Concepts : From trait, TryFrom trait

- [x] `impl TryFrom<&str> for Rarity` avec son type d'erreur dédié `ParseRarityError`
- [x] `impl TryFrom<&str> for ItemKind` qui accepte `"weapon:50"`, `"potion:25"`, `"armor:30"`
- [x] Nouvelle struct publique `ItemSummary { name: String, value: u32 }`
- [x] `impl From<&Item> for ItemSummary` (la `value` dépend de la variante de `ItemKind`)

Tests :
- `rarity_try_from_parses_known_variants`
- `rarity_try_from_rejects_unknown_value`
- `item_kind_try_from_parses_weapon`
- `item_kind_try_from_rejects_garbage`
- `item_summary_from_item_copies_name_and_value`

---

## Phase 7 — Newtype, `Deref`, `Sized`
Concepts : Deref trait, Sized trait (`?Sized`), string slices, orphan rule

- [ ] Newtype `pub struct ItemName(String)`
- [ ] `impl From<&str> for ItemName` et `impl From<String> for ItemName`
- [ ] `impl Deref for ItemName` avec `Target = str` (coercion auto vers `&str`)
- [ ] Remplacer le champ `name: String` de `Item` par `name: ItemName`
- [ ] Helper libre `pub fn length_of<T: ?Sized + AsRef<str>>(x: &T) -> usize`

Tests :
- `item_name_from_str_wraps_owned_string`
- `item_name_derefs_to_str_methods`
- `length_of_accepts_str_string_and_item_name`

---

## Phase 8 — `Drop` et trace mémoire
Concepts : Destructors (drop), stack, heap, references in memory

- [ ] `impl Drop for Backpack` qui incrémente un compteur global `static DROPPED: AtomicU32`
- [ ] Helper public `pub fn dropped_count() -> u32`
- [ ] Dans `main.rs`, créer puis dropper un `Backpack` dans un scope et imprimer le compteur — un commentaire en anglais explique ce qui vit sur la stack vs le heap

Test :
- `dropping_backpack_increments_drop_counter`

---

## Phase 9 — Slices, lifetimes, `impl Trait`
Concepts : slices, mutable slices, lifetimes, impl Trait (return + param)

- [ ] `Backpack::as_slice(&self) -> &[Item]`
- [ ] `Backpack::as_mut_slice(&mut self) -> &mut [Item]`
- [ ] `Backpack::bulk_add(&mut self, items: &[Item]) -> Result<(), InventoryError>` (Item doit être `Clone`, cf. Phase 4)
- [ ] `Backpack::find_all<'a>(&'a self, query: &str) -> impl Iterator<Item = &'a Item> + 'a` avec lifetime explicite
- [ ] `Backpack::heaviest(&self, n: usize) -> Vec<&Item>` (tri décroissant, top N)

Tests :
- `as_slice_returns_all_items`
- `as_mut_slice_allows_sort_in_place`
- `bulk_add_accepts_slice`
- `find_all_returns_filtered_iterator`
- `heaviest_returns_top_n_in_descending_order`

---

## Phase 10 — `Index` et `IndexMut`
Concepts : Index trait, IndexMut trait, panics

- [ ] `impl Index<usize> for Backpack` (`bag[0]` → `&Item`)
- [ ] `impl Index<&str> for Backpack` (`bag["Sword"]` → `&Item`, panique si absent)
- [ ] `impl IndexMut<usize> for Backpack`

Tests :
- `index_by_position_returns_item`
- `index_by_name_returns_item`
- `index_by_unknown_name_panics` (à annoter `#[should_panic]`)
- `index_mut_allows_field_update`

---

## Phase 11 — Surcharge d'opérateurs et ordre
Concepts : operator overloading, PartialOrd, Ord

- [ ] `impl PartialOrd for Rarity` + `impl Ord for Rarity` (Common < Rare < Epic < Legendary)
- [ ] `impl std::ops::Add for Backpack` : `a + b` produit un nouveau `Backpack` (max_weight additionnés, items concaténés)

Tests :
- `rarity_orders_common_below_legendary`
- `rarity_sort_ascending_puts_common_first`
- `backpack_add_merges_max_weight_and_items`

---

## Phase 12 — `HashMap` et `BTreeMap`
Concepts : hashmap, btreemap, combinators

- [ ] `Backpack::group_by_rarity(&self) -> HashMap<Rarity, Vec<&Item>>`
- [ ] `Backpack::weights_by_value(&self) -> BTreeMap<u32, Vec<&Item>>` (clé = poids, ordre croissant garanti par BTreeMap)
- [ ] `Backpack::most_common_rarity(&self) -> Option<Rarity>` (chaîne de combinators sur la HashMap)

Tests :
- `group_by_rarity_buckets_correctly`
- `weights_by_value_orders_keys_ascending`
- `most_common_rarity_returns_top_bucket`
- `most_common_rarity_returns_none_for_empty`

---

## Phase 13 — Generics et trait bounds
Concepts : trait bounds, generics, associated vs generic types, arrays

- [ ] `Backpack::add_many<I: IntoIterator<Item = Item>>(&mut self, src: I) -> Result<(), InventoryError>` (accepte `Vec`, array, iterator chain)
- [ ] Un commentaire en anglais dans la signature explique pourquoi `IntoIterator::Item` est un type **associé** alors que `I` est un type **générique**

Tests :
- `add_many_accepts_vec`
- `add_many_accepts_fixed_size_array`
- `add_many_accepts_iterator_chain`

---

## Phase 14 — Combinators avancés et `let-else`
Concepts : combinators, let-else, if-let, branching

- [ ] `Backpack::total_damage(&self) -> u32` via `filter_map` + `sum`
- [ ] `Item::parse_compact(s: &str) -> Result<Item, InventoryError>` qui parse `"Sword|weapon:50|epic|5"` en utilisant `let ... else { return Err(...) }` au moins deux fois (sur le split et sur les conversions)

Tests :
- `total_damage_sums_only_weapon_damage`
- `total_damage_is_zero_without_weapons`
- `parse_compact_round_trips_a_sword`
- `parse_compact_rejects_malformed_input`

---

## Notes conceptuelles

- **Stack / heap** : le `Backpack` lui-même vit sur la stack ; son `Vec<Item>` alloue son buffer sur le heap. Quand le `Backpack` est dropé (Phase 8), le `Vec` libère son buffer.
- **References in memory** : `&Item` est un pointeur sur la stack qui pointe vers la donnée stockée par le `Vec` sur le heap. Les lifetimes (Phase 9) garantissent que la référence ne survit pas à la donnée.
- **Associated vs generic types** : dans `IntoIterator`, `Item` est un type **associé** (un seul type possible par implémentation pour un `Self` donné). Dans `add_many<I: IntoIterator>` (Phase 13), `I` est un **paramètre générique** (un nouveau type à chaque appel).
- **Orphan rule** : tu peux `impl` un trait étranger sur un type local (Phase 4/7 : `Deref` sur `ItemName`), ou un trait local sur un type étranger, mais jamais les deux étrangers ensemble (d'où l'intérêt du newtype).
