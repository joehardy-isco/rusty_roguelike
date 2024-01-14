use crate::prelude::*;

//constants may only include other constants (including function constants)
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize; //usize is the preferred bit size for your CPU (e.g. x64->i64; x86->i32)
                                                                  //for a 80x50 map, NUM_TILES = 4000
                                                                  //Entities (such as players, monsters, chest, other objs) are overlaid on top of the map

#[derive(Copy, Clone, PartialEq)]
//Copy copies variables instead of moving them (?)
//Clone creates a deep copy that doesn't affect the original
//PartialEq adds code that allows you to compare TileType values with == (e.g. Wall==Floor)
pub enum TileType {
    Wall,
    Floor,
} //since this is a pub enum and main.rs imports it as a pub from map under prelude, any part of program can access this

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    }

                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                }
            }
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType.Floor
    }
}
