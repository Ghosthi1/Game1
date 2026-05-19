use bevy::prelude::*;

use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::{Query, Res};
use crate::map::Map;
use crate::ai::{FlowFields, FlowLayer, FlowField};
use crate::character::GridPosition;

pub struct AiPlugin;
impl Plugin for AiPlugin {
    fn build(&self, app: &mut App ) {
        app.add_systems(Update, rebuild_colonist_flow_field);
    }
}

fn rebuild_colonist_flow_field(mut flow_fields: ResMut<FlowFields>, map: Res<Map>, query: Query<&GridPosition, Changed<GridPosition>>) {
    if query.is_empty() { return } // no colonist moved

    let mut positions: Vec<(u32, u32)> = vec![];

    for grid_pos in query.iter() {
        positions.push(grid_pos.0);
    }

    if let Some(field) = flow_fields.layers.get_mut(&FlowLayer::Colonists){
        field.build_flow_fields(&map, &positions);
    }
}