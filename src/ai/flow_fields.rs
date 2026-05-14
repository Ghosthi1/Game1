use std::collections::HashMap;
use bevy::prelude::*;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
/// What the target of the flow field will be
pub enum FlowLayer {
    Colonist
}

/// A precomputed directional map guiding entities toward a goal across the tile grid.
pub struct FlowField{
    /// The Width of the FlowField
    width: u32,
    /// The Height of the FlowField
    height: u32,
    /// The direction the tile is pointing towards the goal: None if tile can't reach the goal
    directions: Vec<Option<(i8, i8)>>
}
impl FlowField{
    /// Creates a new flow field. Needs width and height, and it creates an empty vec for directions
    pub fn new_flow_field(width: u32, height: u32) -> FlowField{ FlowField{ width, height, directions: vec![None; (width * height) as usize]}}
    /// Gets the direction a tile is pointing
    pub fn direction_at(&self, x: u32, y: u32) -> Option<(i8, i8)>{
        // If x,y is out of bounds
        if x >= self.width || y >= self.height {None}
        else {
            let index = (x + y * self.width) as usize;
            // lookup the index in the vec
            self.directions[index]
        }
    }
}

/// Holds all the layers of the FlowFields
#[derive(Resource)]
pub struct FlowFields{
    pub layers: HashMap<FlowLayer, FlowField>
}
