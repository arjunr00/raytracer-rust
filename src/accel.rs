use super::geom::{ Bounded, BoundedHittable };
use super::math;
use super::vec::{ Coord, Point3 };

struct KDTree<'a> {
    node_index: usize,
    node_data: KDTreeData<'a>
}

pub struct KDTreeData<'a> {
    split_point: Point3,
    split_axis: Coord,
    children: (Option<usize>, Option<usize>),
    hittables: Option<Vec<&'a dyn BoundedHittable>>
}

pub struct KDTreeMemoryArena<'a> {
    nodes: Vec<KDTree<'a>>
}

impl<'a> KDTreeMemoryArena<'a> {
    const MAX_DEPTH: u32 = 10;
    const MAX_HITTABLE_COUNT: usize = 4;

    pub fn new() -> KDTreeMemoryArena<'a> {
        KDTreeMemoryArena { nodes: vec![] }
    }

    pub fn construct_tree(&mut self, hittables: &Vec<&'a dyn BoundedHittable>) {
        let local_hittables = hittables.to_vec();
        // TODO: choose better initial cut dimension based on extent
        self.construct_tree_helper(local_hittables, Coord::X, 0);
    }

    pub fn get_node(&self, index: usize) -> Option<&KDTreeData> {
        match self.nodes.get(index) {
            None => None,
            Some(node) => Some(&node.node_data)
        }
    }

    fn construct_tree_helper(&mut self, hittables: Vec<&'a dyn BoundedHittable>, coord: Coord, depth: u32)
        -> Option<usize>
    {
        if hittables.len() == 0 {
            return None;
        }

        let median_hittable = Self::median_hittable(&hittables, coord);
        if hittables.len() <= Self::MAX_HITTABLE_COUNT || depth > Self::MAX_DEPTH {
            return Some(self.insert(KDTreeData {
                split_point: median_hittable.bounding_box().center,
                split_axis: coord,
                children: (None, None),
                hittables: Some(hittables)
            }));
        }

        let mut hittables_left: Vec<&dyn BoundedHittable> = (&hittables).into_iter().filter(|obj| {
            math::f_leq(obj.bounding_box().center[coord], median_hittable.bounding_box().center[coord])
        }).map(|obj| *obj).collect();
        let mut hittables_right: Vec<&dyn BoundedHittable> = (&hittables).into_iter().filter(|obj| {
            obj.bounding_box().center[coord] > median_hittable.bounding_box().center[coord]
        }).map(|obj| *obj).collect();

        for obj in &hittables_left {
            if obj.bounding_box().intersects(hittables_right.bounding_box()) {
                hittables_right.push(*obj);
            }
        }

        for obj in &hittables_right {
            if obj.bounding_box().intersects(hittables_left.bounding_box()) {
                hittables_left.push(*obj);
            }
        }

        let left_child_id = self.construct_tree_helper(hittables_left, KDTreeMemoryArena::cycle_coord(coord), depth + 1);
        let right_child_id = self.construct_tree_helper(hittables_right, KDTreeMemoryArena::cycle_coord(coord), depth + 1);

        Some(self.insert(KDTreeData {
            split_point: median_hittable.bounding_box().center,
            split_axis: coord,
            children: (
                if let Some(node_id) = left_child_id { Some(node_id) } else { None },
                if let Some(node_id) = right_child_id { Some(node_id) } else { None }
            ),
            hittables: None
        }))
    }

    fn cycle_coord(coord: Coord) -> Coord {
        match coord {
            Coord::X => Coord::Y,
            Coord::Y => Coord::Z,
            Coord::Z => Coord::X
        }
    }

    fn median_hittable(hittables: &Vec<&'a dyn BoundedHittable>, coord: Coord) -> &'a dyn BoundedHittable {
        let mut local_hittables = hittables.to_vec();
        local_hittables.sort_by(|a, b| {
            a.bounding_box().center[coord].partial_cmp(&b.bounding_box().center[coord]).unwrap()
        });
        local_hittables[hittables.len() / 2]
    }

    fn insert(&mut self, node: KDTreeData<'a>) -> usize {
        let next_index = self.nodes.len();
        self.nodes.push(KDTree {
            node_index: next_index,
            node_data: node
        });
        next_index
    }
}
