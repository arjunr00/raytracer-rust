use std::cmp::Ordering;

use crate::geom::hit::{
    AxisAlignedBoundingBox,
    Hit,
    Hittable,
    HittableRefs
};
use crate::math;
use crate::vec::{ Coord, Ray };

#[derive(Debug)]
struct BVHNode {
    bounding_box: AxisAlignedBoundingBox,
    object_indices: Option<(usize, usize)>,
    children_indices: Option<(usize, usize)>,
    split_axis: Option<Coord>
}

#[derive(Debug)]
pub struct BVH {
    nodes: Vec<BVHNode>,
    root: usize,
    objects: HittableRefs
}

impl BVH {
    pub fn new(objects: HittableRefs) -> BVH {
        let mut bvh = BVH {
            objects, root: 0, nodes: vec![]
        };

        bvh.root = bvh.build(0, bvh.objects.len());

        bvh
    }

    pub fn objects(&self) -> &HittableRefs {
        &self.objects
    }

    fn build(&mut self, start: usize, end: usize) -> usize {
        let objs = self.objects[start..end].to_vec();
        let num_objs = end - start;
        let bounds = AxisAlignedBoundingBox::union_from_objs(&objs);

        if num_objs <= 4 {
            self.add_leaf_node((start, end), bounds)
        } else {
            let centroids_iter: Vec<_>
                = objs.iter().map(|obj| obj.centroid()).collect();
            let centroids_bounds = AxisAlignedBoundingBox::union_from_points(&centroids_iter);

            let split_axis = centroids_bounds.largest_extent_axis();
            let mut mid = (start + end) / 2;

            if math::f_eq(centroids_bounds.volume(), 0.0) {
                self.add_leaf_node((start, end), bounds)
            } else {
                if num_objs <= 8 {
                    let mid_obj = objs[mid - start].clone();
                    let mut partitions = objs.iter().partition::<Vec<_>, _>(|a| {
                        a.centroid()[split_axis] < mid_obj.centroid()[split_axis]
                    });
                    partitions.0.append(&mut partitions.1);
                    // self.objects = partitions.0.iter().map(|arc| (*arc).clone()).collect::<Vec<_>>();
                } else {
                    // Use surface area heuristic to partition
                    const NUM_BUCKETS: usize = 12;

                    let mut buckets = vec![(0, AxisAlignedBoundingBox::empty()); 12];
                    for i in 0..num_objs {
                        let mut b = (NUM_BUCKETS as f64 *
                            centroids_bounds.point_offset(&objs[i].centroid())[split_axis]) as usize;
                        if b == NUM_BUCKETS {
                            b = NUM_BUCKETS - 1
                        }

                        buckets[b].0 += 1;
                        buckets[b].1 = AxisAlignedBoundingBox::union(vec![
                            &buckets[b].1,
                            &objs[i].bounding_box()
                        ]);

                    }

                    let mut bucket_costs = vec![];
                    for i in 0..NUM_BUCKETS-1{
                        let bucket_counts: Vec<_> = buckets.iter().map(|bucket| bucket.0).collect();
                        let bucket_bounds: Vec<_> = buckets.iter().map(|bucket| &bucket.1).collect();

                        let partition_bucket_bounds = (
                            AxisAlignedBoundingBox::union(bucket_bounds[..i+1].to_vec()),
                            AxisAlignedBoundingBox::union(bucket_bounds[i+1..].to_vec())
                        );
                        let obj_counts: (usize, usize) = (
                            bucket_counts[..i+1].to_vec().iter().sum(),
                            bucket_counts[i+1..].to_vec().iter().sum()
                        );

                        bucket_costs.push(0.125
                            + (obj_counts.0 as f64 * partition_bucket_bounds.0.surface_area()
                                + obj_counts.1 as f64 * partition_bucket_bounds.1.surface_area())
                            / bounds.surface_area());
                    }

                    let min_cost = bucket_costs.iter().enumerate().min_by(
                        |a, b| a.1.partial_cmp(&b.1).unwrap_or_else(|| Ordering::Equal)
                    ).unwrap_or_else(|| (0, &0.0));

                    let leaf_cost = num_objs as f64;
                    if min_cost.1 < &leaf_cost {
                        let mut partitions = objs.iter().partition::<Vec<_>, _>(|a| {
                            let mut b = (NUM_BUCKETS as f64 *
                                centroids_bounds.point_offset(&a.centroid())[split_axis]) as usize;
                            if b == NUM_BUCKETS {
                                b = NUM_BUCKETS - 1;
                            }

                            b <= min_cost.0
                        });

                        mid = start + partitions.0.len();
                        partitions.0.append(&mut partitions.1);
                        // self.objects = partitions.0.iter().map(|arc| (*arc).clone()).collect::<Vec<_>>();
                    } else {
                        return self.add_leaf_node((start, end), bounds);
                    }
                }

                let left_child_index = self.build(start, mid);
                let right_child_index = self.build(mid, end);
                self.add_interior_node(
                    split_axis, (left_child_index, right_child_index)
                )
            }
        }
    }

    fn add_leaf_node(&mut self, object_indices: (usize, usize), bounding_box: AxisAlignedBoundingBox)
        -> usize
    {
        self.nodes.push(BVHNode {
            bounding_box,
            split_axis: None,
            object_indices: Some(object_indices),
            children_indices: None
        });

        self.nodes.len() - 1
    }

    fn add_interior_node(&mut self, split_axis: Coord, children_indices: (usize, usize))
        -> usize
    {
        let left_child = self.nodes.get(children_indices.0);
        let right_child = self.nodes.get(children_indices.1);
        let mut bounds = vec![];

        if let Some(left_child) = left_child {
            bounds.push(&left_child.bounding_box);
        }

        if let Some(right_child) = right_child {
            bounds.push(&right_child.bounding_box);
        }

        let bounds = AxisAlignedBoundingBox::union(bounds);

        self.nodes.push(BVHNode {
            split_axis: Some(split_axis),
            children_indices: Some(children_indices),
            object_indices: None,
            bounding_box: bounds
        });

        self.nodes.len() - 1
    }

    fn get_node(&self, index: usize) -> Option<&BVHNode> {
        self.nodes.get(index)
    }
}

impl Hittable for BVH {
    fn is_hit(&self, ray: &Ray, t_min: f64, mut t_max: f64) -> Option<Hit> {
        let root = &self.nodes[self.root];

        let mut hit = None;
        let mut node_stack = vec![root];
        while let Some(node) = node_stack.pop() {
            if let Some((node_t_min, node_t_max))
                = node.bounding_box.ray_intersects(ray, t_min, t_max)
            {
                if t_max < node_t_min || node_t_max < t_min {
                    continue;
                }

                match node.children_indices {
                    Some(children) => {
                        // Interior node
                        if let Some(right_child) = self.get_node(children.1) {
                            node_stack.push(right_child);
                        }
                        if let Some(left_child) = self.get_node(children.0) {
                            node_stack.push(left_child);
                        }
                    },
                    None => {
                        // Leaf node
                        if let Some((obj_start, obj_end)) = node.object_indices {
                            let mut closest_t = t_max;
                            for i in obj_start..obj_end {
                                if let Some(obj_hit) = self.objects[i].is_hit(ray, t_min, closest_t) {
                                    closest_t = obj_hit.t;
                                    hit = Some(obj_hit);
                                }
                            }
                            t_max = closest_t;
                        }
                    }
                }
            }
        }

        hit
    }
}
