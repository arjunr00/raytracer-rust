use std::cmp::Ordering;

use crate::geom::hit::{
    AxisAlignedBoundingBox,
    HittableRefs
};
use crate::math;
use crate::vec::Coord;

#[derive(Debug)]
struct BVHNode {
    bounding_box: AxisAlignedBoundingBox,
    children_indices: Option<(usize, usize)>,
    split_axis: Option<Coord>
}

#[derive(Debug)]
pub struct BVH {
    nodes: Vec<BVHNode>,
    objects: HittableRefs
}

impl BVH {
    pub fn new(objects: HittableRefs) -> BVH {
        let mut bvh = BVH {
            objects, nodes: vec![]
        };

        bvh.build(0, bvh.objects.len());

        bvh
    }

    pub fn objects(&self) -> &HittableRefs {
        &self.objects
    }

    fn build(&mut self, start: usize, end: usize) -> usize {
        let objs = &self.objects;
        let bounds = AxisAlignedBoundingBox::union_from_objs(&objs);

        if end - start == 1 {
            self.add_leaf_node(bounds)
        } else {
            let centroids_iter: Vec<_>
                = objs.iter().map(|obj| obj.centroid()).collect();
            let centroids_bounds = AxisAlignedBoundingBox::union_from_points(&centroids_iter);

            let split_axis = centroids_bounds.largest_extent_axis();
            let mut mid = (start + end) / 2;

            if math::f_eq(centroids_bounds.volume(), 0.0) {
                self.add_leaf_node(bounds)
            } else {
                if start - end <= 4 {
                    let mid_obj = objs[mid].clone();
                    let mut partitions = objs.iter().partition::<Vec<_>, _>(|a| {
                        a.centroid()[split_axis] < mid_obj.centroid()[split_axis]
                    });
                    partitions.0.append(&mut partitions.1);
                    self.objects = partitions.0.iter().map(|arc| (*arc).clone()).collect::<Vec<_>>();
                } else {
                    // Use surface area heuristic to partition
                    const NUM_BUCKETS: usize = 12;

                    let mut buckets = vec![(0, AxisAlignedBoundingBox::empty()); 12];
                    for i in start..end {
                        let mut b = NUM_BUCKETS *
                            centroids_bounds.point_offset(&objs[i].centroid())[split_axis] as usize;
                        if b == NUM_BUCKETS {
                            b = NUM_BUCKETS - 1
                        }

                        buckets[b].0 += 1;
                        buckets[b].1 = AxisAlignedBoundingBox::union(vec![
                            &buckets[b].1,
                            &objs[i].bounding_box()
                        ]);
                    }

                    let mut bucket_costs = [0.0; NUM_BUCKETS - 1];
                    for (i, cost) in &mut bucket_costs.iter_mut().enumerate() {
                        let bucket_counts: Vec<_> = buckets.iter().map(|bucket| bucket.0).collect();
                        let bucket_bounds: Vec<_> = buckets.iter().map(|bucket| &bucket.1).collect();

                        let bucket_bounds = (
                            AxisAlignedBoundingBox::union(bucket_bounds[..i+1].to_vec()),
                            AxisAlignedBoundingBox::union(bucket_bounds[i+1..].to_vec())
                        );
                        let obj_counts: (usize, usize) = (
                            bucket_counts[..i+1].to_vec().iter().sum(),
                            bucket_counts[i+1..].to_vec().iter().sum()
                        );

                        *cost = 0.125
                            + (obj_counts.0 as f64 * bucket_bounds.0.surface_area()
                                + obj_counts.1 as f64 * bucket_bounds.1.surface_area())
                            / bounds.surface_area()
                    }

                    let min_cost = bucket_costs.iter().enumerate().min_by(
                        |a, b| a.1.partial_cmp(&b.1).unwrap_or_else(|| Ordering::Equal)
                    ).unwrap_or_else(|| (0, &0.0));

                    let leaf_cost = (start - end) as f64;
                    if min_cost.1 < &leaf_cost {
                        let mut partitions = objs.iter().partition::<Vec<_>, _>(|a| {
                            let mut b = NUM_BUCKETS *
                                centroids_bounds.point_offset(&a.centroid())[split_axis] as usize;
                            if b == NUM_BUCKETS {
                                b = NUM_BUCKETS - 1;
                            }

                            b <= min_cost.0
                        });

                        mid = partitions.0.len();
                        partitions.0.append(&mut partitions.1);
                        self.objects = partitions.0.iter().map(|arc| (*arc).clone()).collect::<Vec<_>>();
                    } else {
                        return self.add_leaf_node(bounds);
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

    fn add_leaf_node(&mut self, bounding_box: AxisAlignedBoundingBox)
        -> usize
    {
        self.nodes.push(BVHNode {
            bounding_box,
            split_axis: None, children_indices: None
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
            bounding_box: bounds
        });

        self.nodes.len() - 1
    }
}
