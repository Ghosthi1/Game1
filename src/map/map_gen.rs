use super:: map::Map;

pub fn generate_map(width: u32, height: u32) -> Map {
    Map::new(width, height)
}