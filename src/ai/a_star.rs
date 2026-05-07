use std::cmp::{max, Ordering};
use std::collections::{BinaryHeap, HashSet};
use bevy::platform::collections::HashMap;
use crate::map::Map;

/// A position being evaluated during the search, tracking costs ro reach it and estimating the remaining distance to the goal.
#[derive(Eq, PartialEq)]
struct Node {
    pos: (u32, u32),
    /// How much it actually cost to get here
    g: u32,
    /// How much it's estimated to cost to get to the goal from here
    h: u32,
}
impl Node {
    /// The total estimated cost of a path
    fn f(&self) -> u32 {
        self.g + self.h
    }
}
// Order the binary heap in lowest to highest
impl Ord for Node {
    fn cmp(&self, other: &Self) ->  std::cmp::Ordering {
        self.f().cmp(&other.f()).reverse()
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Estimates the minimum possible distance between two grid positions, assuming no walls.
fn heuristic(a: (u32, u32), b: (u32, u32)) -> u32 {
    max( a.0.abs_diff(b.0),  a.1.abs_diff(b.1))
}

pub fn find_path(map: &Map, start:(u32, u32), goal:(u32, u32)) -> Option<Vec<(u32, u32)>> {
    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();
    let mut came_from: HashMap<(u32,u32), (u32,u32)> = HashMap::new();

    open_set.push(Node{pos: start, g:0, h: heuristic(start, goal)});

    while let Some(node) = open_set.pop() {
        // If node pos is the goal get the path taken
        if node.pos == goal{
            let mut path = vec![goal];
            let mut current = goal;

            // Gets the path by getting the tile before from the goal to start
            while let Some(&parent) = came_from.get(&current) {
                current = parent;
                path.push(parent);
            }
            path.reverse();
            return Some(path)
        }
        if closed_set.contains(&node.pos) {
            continue
        }

        closed_set.insert(node.pos);



    }

    None
}