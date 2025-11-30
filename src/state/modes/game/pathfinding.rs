use std::collections::{VecDeque, HashMap, HashSet};
use crate::map::map::Map;
use crate::map::position::MapPosition;

pub struct Pathfinding;

impl Pathfinding {
    pub fn find_path(map: &Map, start: MapPosition, goal: MapPosition) -> Option<Vec<MapPosition>> {
        if start == goal { return Some(vec![]); }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut came_from = HashMap::new();

        queue.push_back(start);
        visited.insert(start);

        let mut found = false;

        while let Some(current) = queue.pop_front() {
            if current == goal {
                found = true;
                break;
            }

            for neighbor in current.neighbors() {
                let is_walkable = map.get_tile(neighbor).map_or(false, |t| !t.is_solid());
                let is_occupied = map.get_tile(neighbor).map_or(false, |t| !t.is_occupied());
                if !(is_walkable || is_occupied){ continue; }

                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    came_from.insert(neighbor, current);
                    queue.push_back(neighbor);
                }
            }
        }

        if found {
            let mut path = Vec::new();
            let mut curr = goal;
            while curr != start {
                path.push(curr);
                curr = *came_from.get(&curr).unwrap(); 
            }
            path.reverse();
            return Some(path);
        }
        None
    }
}