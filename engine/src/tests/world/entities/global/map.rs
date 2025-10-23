use crate::world::{
    entities::global::{SurfaceType, Tile, map::Map},
    geometry::TilePos,
};

#[rustfmt::skip]
fn add_tile(map: &mut Map, line: i32, column: i32) {
    map.add_tile(
        Tile::new(
            TilePos::new(line, column),
            SurfaceType::Ground
        )
    ).unwrap();
}

#[test]
pub fn Map__width__new_map__is_0() {
    assert_eq!(Map::new().width(), 0);
}

#[test]
#[rustfmt::skip]
pub fn Map__width__add_one_tile__is_1() {
    let mut map = Map::new();
    add_tile(&mut map, 0, 0);
    assert_eq!(map.width(), 1);
}

#[test]
#[rustfmt::skip]
pub fn Map__width__add_two_tiles_on_same_column__is_1() {
    let mut map = Map::new();
    add_tile(&mut map, 0, 0);
    add_tile(&mut map, 1, 0);
    assert_eq!(map.width(), 1);
}

#[test]
#[rustfmt::skip]
pub fn Map__width__add_two_tiles_on_different_column__equals_distance_between() {
    let mut map = Map::new();
    add_tile(&mut map, 0, -1);
    add_tile(&mut map, 1, 2);
    assert_eq!(map.width(), 4);
}

#[test]
#[rustfmt::skip]
pub fn Map__width__add_three_tiles_last_between_first_two__is_correct() {
    let mut map = Map::new();
    add_tile(&mut map, 0, -1);
    add_tile(&mut map, 1, 3);
    add_tile(&mut map, 2, 1);
    assert_eq!(map.width(), 5);
}

#[test]
#[rustfmt::skip]
pub fn Map__width__add_three_tiles_last_greater_first_two__is_correct() {
    let mut map = Map::new();
    add_tile(&mut map, 0, -1);
    add_tile(&mut map, 1, 3);
    add_tile(&mut map, 2, 6);
    assert_eq!(map.width(), 8);
}

#[test]
#[rustfmt::skip]
pub fn Map__width__add_three_tiles_last_less_first_two__is_correct() {
    let mut map = Map::new();
    add_tile(&mut map, 0, -1);
    add_tile(&mut map, 1, 3);
    add_tile(&mut map, 2, -3);
    assert_eq!(map.width(), 7);
}

#[test]
pub fn Map__height__new_map__is_0() {
    assert_eq!(Map::new().height(), 0);
}

#[test]
#[rustfmt::skip]
pub fn Map__height__add_one_tile__is_1() {
    let mut map = Map::new();
    add_tile(&mut map, 0, 0);
    assert_eq!(map.height(), 1);
}

#[test]
#[rustfmt::skip]
pub fn Map__height__add_two_tiles_on_same_column__is_1() {
    let mut map = Map::new();
    add_tile(&mut map, 0, 0);
    add_tile(&mut map, 0, 1);
    assert_eq!(map.height(), 1);
}

#[test]
#[rustfmt::skip]
pub fn Map__height__add_two_tiles_on_different_column__equals_distance_between() {
    let mut map = Map::new();
    add_tile(&mut map, -1, 0);
    add_tile(&mut map, 2, 1);
    assert_eq!(map.height(), 4);
}

#[test]
#[rustfmt::skip]
pub fn Map__height__add_three_tiles_last_between_first_two__is_correct() {
    let mut map = Map::new();
    add_tile(&mut map, -1, 0);
    add_tile(&mut map, 3, 1);
    add_tile(&mut map, 1, 2);
    assert_eq!(map.height(), 5);
}

#[test]
#[rustfmt::skip]
pub fn Map__height__add_three_tiles_last_greater_first_two__is_correct() {
    let mut map = Map::new();
    add_tile(&mut map, -1, 0);
    add_tile(&mut map, 3, 1);
    add_tile(&mut map, 6, 2);
    assert_eq!(map.height(), 8);
}

#[test]
#[rustfmt::skip]
pub fn Map__height__add_three_tiles_last_less_first_two__is_correct() {
    let mut map = Map::new();
    add_tile(&mut map, -1, 0);
    add_tile(&mut map, 3, 1);
    add_tile(&mut map, -3, 2);
    assert_eq!(map.height(), 7);
}
