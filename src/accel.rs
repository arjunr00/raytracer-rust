use std::fmt::Debug;

use super::geom::{ AxisAlignedBoundingBox, Bounded, BoundedHittable, Hit, Hittable };
use super::math;
use super::vec::{ Coord, Ray };

#[derive(Debug)]
struct KDTreeNode<'a> {
    node_index: usize,
    node_data: KDTreeNodeData<'a>
}

#[derive(Debug)]
pub struct KDTreeNodeData<'a> {
    bounding_box: AxisAlignedBoundingBox,
    children: (Option<usize>, Option<usize>),
    hittables: Option<Vec<&'a dyn BoundedHittable>>
}

#[derive(Debug)]
pub struct KDTreeMemoryArena<'a> {
    nodes: Vec<KDTreeNode<'a>>
}

impl<'a> KDTreeMemoryArena<'a> {
    const MAX_DEPTH: u32 = 10;
    const MAX_HITTABLE_COUNT: usize = 4;

    pub fn new(hittables: &Vec<&'a dyn BoundedHittable>) -> KDTreeMemoryArena<'a> {
        let mut arena = KDTreeMemoryArena { nodes: vec![] };
        arena.construct_tree(hittables);
        arena
    }

    pub fn construct_tree(&mut self, hittables: &Vec<&'a dyn BoundedHittable>) {
        let local_hittables = hittables.to_vec();
        // TODO: choose better initial cut dimension based on extent
        self.construct_tree_helper(local_hittables, Coord::Z, 0);
    }

    pub fn get_node(&self, index: usize) -> Option<&KDTreeNodeData> {
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

        let (median_hittable, median_index) = Self::median_hittable(&hittables, coord);
        let median_bounding_box = median_hittable.bounding_box();
        let split_point = median_bounding_box.center;

        if hittables.len() <= Self::MAX_HITTABLE_COUNT || depth > Self::MAX_DEPTH {
            return Some(self.insert(KDTreeNodeData {
                bounding_box: hittables.bounding_box(),
                children: (None, None),
                hittables: Some(hittables)
            }));
        }

        let mut local_hittables = hittables.to_vec();
        local_hittables.remove(median_index);

        let mut hittables_left: Vec<&dyn BoundedHittable> = (&local_hittables).into_iter().filter(|obj| {
            math::f_leq(obj.bounding_box().center[coord], split_point[coord])
        }).map(|obj| *obj).collect();
        let mut hittables_right: Vec<&dyn BoundedHittable> = (&local_hittables).into_iter().filter(|obj| {
            obj.bounding_box().center[coord] > split_point[coord]
        }).map(|obj| *obj).collect();

        for obj in &hittables_left {
            if obj.bounding_box().box_intersects(hittables_right.bounding_box()) {
                hittables_right.push(*obj);
            }
        }

        for obj in &hittables_right {
            if obj.bounding_box().box_intersects(hittables_left.bounding_box()) {
                hittables_left.push(*obj);
            }
        }

        hittables_left.push(median_hittable);

        let left_child_id = self.construct_tree_helper(hittables_left, KDTreeMemoryArena::cycle_coord(coord), depth + 1);
        let right_child_id = self.construct_tree_helper(hittables_right, KDTreeMemoryArena::cycle_coord(coord), depth + 1);

        Some(self.insert(KDTreeNodeData {
            bounding_box: hittables.bounding_box(),
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

    fn median_hittable(hittables: &Vec<&'a dyn BoundedHittable>, coord: Coord) -> (&'a dyn BoundedHittable, usize)
    {
        let mut local_hittables = hittables.to_vec();
        local_hittables.sort_by(|a, b| {
            a.bounding_box().center[coord].partial_cmp(&b.bounding_box().center[coord]).unwrap()
        });
        (local_hittables[hittables.len() / 2], hittables.len() / 2)
    }

    fn insert(&mut self, node: KDTreeNodeData<'a>) -> usize {
        let next_index = self.nodes.len();
        self.nodes.push(KDTreeNode {
            node_index: next_index,
            node_data: node
        });
        next_index
    }

    fn node_is_hit(&self, root_index: usize, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let node = self.nodes.get(root_index)?;
        if !node.node_data.bounding_box.ray_intersects(ray, t_min, t_max) {
            return None;
        }

        if let Some(hittables) = &node.node_data.hittables {
            let mut hit = None;
            let mut closest_t = t_max;

            for obj in hittables {
                if let Some(obj_hit) = obj.is_hit(ray, t_min, closest_t) {
                    closest_t = obj_hit.t;
                    hit = Some(obj_hit);
                }
            }

            return hit;
        }

        let mut hit = None;
        if let Some(left_index) = node.node_data.children.0 {
            hit = self.node_is_hit(left_index, ray, t_min, t_max);
        }

        if hit.is_some() {
            return hit;
        }

        if let Some(right_index) = node.node_data.children.1 {
            hit = self.node_is_hit(right_index, ray, t_min, t_max);
        }

        hit
    }
}

impl Hittable for KDTreeMemoryArena<'_> {
    fn is_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        self.node_is_hit(self.nodes.len() - 1, ray, t_min, t_max)
    }

    fn surface_area(&self) -> f64 {
        todo!();
    }
}
