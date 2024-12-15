use std::{cmp::Reverse, collections::BinaryHeap, fmt::Debug};

use geometry::aabb::Aabb;
use glam::Vec3;

use crate::{
    Bvh, BvhIter, Node,
    utils::{GetAabb, NodeOrd},
};

impl<T: Debug> Bvh<T> {
    /// Returns the closest element to the target and the distance squared to it.
    pub fn get_closest(&self, target: Vec3, get_aabb: impl Fn(&T) -> Aabb) -> Option<(&T, f64)> {
        let mut min_dist2 = f64::INFINITY;
        let mut min_node = None;

        let on = self.root();

        let on = match on {
            Node::Internal(internal) => internal,
            Node::Leaf(leaf) => {
                return leaf
                    .iter()
                    .map(|elem| {
                        let aabb = get_aabb(elem);
                        let dist2 = aabb.dist2(target);
                        (elem, dist2)
                    })
                    .min_by_key(|(_, dist)| dist.to_bits());
            }
        };

        // let mut stack: SmallVec<&BvhNode, 64> = SmallVec::new();
        let mut heap: BinaryHeap<_> = std::iter::once(on)
            .map(|node| Reverse(NodeOrd { node, dist2: 0.0 }))
            .collect();

        while let Some(on) = heap.pop() {
            let on = on.0.node;
            let dist2 = on.aabb.dist2(target);

            if dist2 > min_dist2 {
                break;
            }

            for child in on.children(self) {
                match child {
                    Node::Internal(internal) => {
                        heap.push(Reverse(NodeOrd {
                            node: internal,
                            dist2: internal.aabb.dist2(target),
                        }));
                    }
                    Node::Leaf(leaf) => {
                        let (elem, dist2) = leaf
                            .iter()
                            .map(|elem| {
                                let aabb = get_aabb(elem);
                                let dist2 = aabb.dist2(target);
                                (elem, dist2)
                            })
                            .min_by_key(|(_, dist)| dist.to_bits())
                            .unwrap();

                        if dist2 < min_dist2 {
                            min_dist2 = dist2;
                            min_node = Some(elem);
                        }
                    }
                }
            }
        }

        min_node.map(|elem| (elem, min_dist2))
    }

    pub fn get_collisions<'a>(
        &'a self,
        target: Aabb,
        get_aabb: impl GetAabb<T> + 'a,
    ) -> impl Iterator<Item = &'a T> + 'a {
        BvhIter::consume(self, target, get_aabb)
    }
}