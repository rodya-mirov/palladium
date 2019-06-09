use serde::Deserialize;

#[derive(Eq, PartialEq, Copy, Clone, Deserialize)]
pub struct MapGenerationParams {
    pub room_dimensions: RoomDimensions,
    pub map_dimensions: MapDimensions,
    pub max_retries: usize,
    pub seed: u64,
}

#[derive(Eq, PartialEq, Copy, Clone, Deserialize)]
pub struct RoomDimensions {
    pub room_min_width: usize,
    pub room_max_width: usize,
    pub room_min_height: usize,
    pub room_max_height: usize,
}

#[derive(Eq, PartialEq, Copy, Clone, Deserialize)]
pub struct MapDimensions {
    pub map_width: usize,
    pub map_height: usize,
}
