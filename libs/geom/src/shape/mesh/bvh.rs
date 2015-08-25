use std::f64;
use std::iter::{IntoIterator};
use std::cmp::min;

use Axis;
use ray::Ray;
use shape::{Intersection, Shape};
use shape::bound_box::{BoundBox, Bound};

pub trait BoundedShape: Shape + Bound + Clone {}
impl<T: Shape + Bound + Clone> BoundedShape for T {}


enum Node<T: BoundedShape> {
    Leaf {shape: T, bound: BoundBox },
    Interior {
        children: [Box<Node<T>>; 2],
        axis: Axis,
        bound: BoundBox,
    }
}


impl<T: BoundedShape> Node<T> {
    fn interior(l: Box<Node<T>>, r: Box<Node<T>>, axis: Axis) -> Node<T> {
        let bound = l.bound().union(&r.bound());
        Node::Interior {
            children: [l, r],
            axis: axis,
            bound: bound
        }
    }

    fn bound(&self) -> BoundBox {
        match self {
            &Node::Leaf {bound, ..} => bound,
            &Node::Interior { bound, ..} => bound
        }
    }


    fn build(shapes: Vec<(&T, BoundBox)>) -> Node<T> {
        assert!(shapes.len() > 0);
        if shapes.len() == 1 {
            let (shape, bound) = shapes.into_iter().next().unwrap();
            Node::Leaf {shape: (*shape).clone(), bound: bound }
        } else {
            let (left, right, axis) = Node::partition(shapes);
            Node::interior(
                Box::new(Node::build(left)),
                Box::new(Node::build(right)),
                axis)
        }
    }

    fn partition(mut shapes: Vec<(&T, BoundBox)>)
                 -> (Vec<(&T, BoundBox)>, Vec<(&T, BoundBox)>, Axis) {

        let axis = shapes.iter().map(|&(_, ref bound)| bound.center())
            .collect::<BoundBox>()
            .longext_axis();

        let key = |&(_, ref bound): &(&T, BoundBox)| bound.center()[axis];
        shapes.sort_by(|a, b| key(a).partial_cmp(&key(b)).unwrap());
        // TODO: use SAH
        let mid = shapes.len() / 2;
        let mut l = Vec::with_capacity(mid + 1);
        let mut r = Vec::with_capacity(mid + 1);
        for (i, item) in shapes.into_iter().enumerate() {
            if i < mid {
                l.push(item);
            } else {
                r.push(item);
            }
        }
        (l, r, axis)
    }
}

pub struct Bvh<T: BoundedShape> {
    root: Node<T>
}

impl<T: BoundedShape> Bvh<T>  {
    pub fn new(triangles: Vec<T>) -> Bvh<T> {
        let with_bounds = triangles.iter()
            .map(|t| (t, t.bound())).collect();
        let root = Node::<T>::build(with_bounds);
        Bvh {
            root: root
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut todo = Vec::with_capacity(64);
        let mut result = None;
        let mut t_bound = f64::INFINITY;

        todo.push(&self.root);
        while let Some(node) = todo.pop() {
            if !node.bound().is_intersected(ray, t_bound) {
                continue;
            }

            match node {
                &Node::Leaf {ref shape, ..} => if let Some(i) = shape.intersect(ray) {
                    let new_result = match result {
                        None => i,
                        Some(j) => min(i, j)
                    };
                    t_bound = t_bound.min(new_result.t);
                    result = Some(new_result);
                },
                &Node::Interior {ref children, axis, ..} => {
                    if ray.direction[axis] < 0.0 {
                        todo.push(&children[0]);
                        todo.push(&children[1]);
                    } else {
                        todo.push(&children[1]);
                        todo.push(&children[0]);
                    }
                }
            }
        }

        result
    }
}
