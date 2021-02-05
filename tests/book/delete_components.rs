use shipyard::{AllStoragesViewMut, Delete, EntitiesViewMut, ViewMut, World};

#[test]
#[rustfmt::skip]
fn world_one() {
// ANCHOR: world_one
let mut world = World::new();

let id = world.add_entity((0u32,));

world.delete_component::<(u32,)>(id);
// ANCHOR_END: world_one
}

#[test]
#[rustfmt::skip]
fn world_multiple() {
// ANCHOR: world_multiple
let mut world = World::new();

let id = world.add_entity((0u32, 1usize));

world.delete_component::<(u32, usize)>(id);
// ANCHOR_END: world_multiple
}

#[test]
#[rustfmt::skip]
fn world_all() {
// ANCHOR: world_all
let mut world = World::new();

let id = world.add_entity((0u32, 1usize));

world.strip(id);
// ANCHOR_END: world_all
}

#[test]
#[rustfmt::skip]
fn view_one() {
// ANCHOR: view_one
let world = World::new();

let (mut entities, mut u32s) = world.borrow::<(EntitiesViewMut, ViewMut<u32>)>().unwrap();

let id = entities.add_entity(&mut u32s, 0);

u32s.delete(id);
// ANCHOR_END: view_one
}

#[test]
#[rustfmt::skip]
fn view_multiple() {
// ANCHOR: view_multiple
let world = World::new();

let (mut entities, mut u32s, mut usizes) = world
    .borrow::<(EntitiesViewMut, ViewMut<u32>, ViewMut<usize>)>()
    .unwrap();

let id = entities.add_entity((&mut u32s, &mut usizes), (0, 1));

(&mut u32s, &mut usizes).delete(id);
// ANCHOR_END: view_multiple
}

#[test]
#[rustfmt::skip]
fn view_all() {
// ANCHOR: view_all
let world = World::new();

let mut all_storages = world.borrow::<AllStoragesViewMut>().unwrap();

let id = all_storages.add_entity((0u32, 1usize));

all_storages.strip(id);
// ANCHOR_END: view_all
}
