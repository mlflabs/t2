use bevy::{platform::collections::HashMap, prelude::*};
use std::collections::BinaryHeap;



#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Node {
    pub pos: GridPos,
    pub g_cost: u32, // Cost from start to this node
    pub h_cost: u32, // Heuristic cost from this node to end
    pub f_cost: u32, // g_cost + h_cost
}

impl Node {
    pub fn new(pos: GridPos, g_cost: u32, h_cost: u32) -> Self {
        Node {
            pos,
            g_cost,
            h_cost,
            f_cost: g_cost + h_cost,
        }
    }
}


impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare `f_cost` first. If equal, compare `h_cost` (tie-breaking, can improve performance)
        other.f_cost.cmp(&self.f_cost)
            .then_with(|| other.h_cost.cmp(&self.h_cost))
            .then_with(|| self.pos.x.cmp(&other.pos.x)) // Tie-break by position for consistent results
            .then_with(|| self.pos.y.cmp(&other.pos.y))
    }
}


// --- 3. Heuristic Function (Manhattan Distance) ---
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic(a: GridPos, b: GridPos) -> f32 {
    let min_dim = dx.min(dy);
    let max_dim = dx.max(dy);
    (min_dim * 1.4) + (max_dim - min_dim)
}




pub fn find_path(
    map: &GameMap,
    start: GridPos,
    end: GridPos,
) -> Option<Vec<GridPos>> {
    // Check if start or end are walls
    if map.get_tile(start).map_or(true, |t| t == TileType::Wall) ||
       map.get_tile(end).map_or(true, |t| t == TileType::Wall) {
        println!("Start or End position is a wall or invalid!");
        return None;
    }

    let mut open_set: BinaryHeap<Node> = BinaryHeap::new();
    let mut came_from: HashMap<GridPos, GridPos> = HashMap::new(); // To reconstruct path
    let mut g_costs: HashMap<GridPos, u32> = HashMap::new(); // Cost from start to current node

    // Initialize start node
    g_costs.insert(start, 0);
    open_set.push(Node::new(start, 0, heuristic(start, end)));

    while let Some(current_node) = open_set.pop() {
        let current_pos = current_node.pos;

        // If we reached the end
        if current_pos == end {
            return Some(reconstruct_path(came_from, end));
        }

        // Get the actual cost to reach the current node
        let current_g = *g_costs.get(&current_pos).unwrap_or(&u32::MAX);
        if current_g < current_node.g_cost { // Already found a better path
            continue;
        }

        // Explore neighbors
        for neighbor_pos in map.get_neighbors(current_pos) {
            let tile_cost = map.get_tile(neighbor_pos).unwrap().movement_cost() as u32; // Convert to u32
            let tentative_g_cost = current_g + tile_cost;

            // If we found a shorter path to this neighbor
            if tentative_g_cost < *g_costs.get(&neighbor_pos).unwrap_or(&u32::MAX) {
                came_from.insert(neighbor_pos, current_pos);
                g_costs.insert(neighbor_pos, tentative_g_cost);

                let h_cost = heuristic(neighbor_pos, end);
                open_set.push(Node::new(neighbor_pos, tentative_g_cost, h_cost));
            }
        }
    }

    // No path found
    None
}



// Helper function to reconstruct the path from the came_from map
fn reconstruct_path(came_from: HashMap<GridPos, GridPos>, mut current: GridPos) -> Vec<GridPos> {
    let mut path = Vec::new();
    while let Some(&parent) = came_from.get(&current) {
        path.push(current);
        current = parent;
    }
    path.reverse(); // Path is built backwards, so reverse it
    path
}































