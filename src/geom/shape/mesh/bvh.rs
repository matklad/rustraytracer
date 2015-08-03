use std::f64;
use std::iter::{IntoIterator};
use std::cmp::min;

use geom::{Axis};
use geom::ray::Ray;
use geom::shape::{Intersection, Shape};
use geom::shape::bound_box::{BoundBox, Bound};

pub trait BoundedShape: Shape + Bound {}
impl<T: Shape + Bound> BoundedShape for T {}


enum Node<T: BoundedShape> {
    Leaf {shape: T},
    Interior {
        children: [Box<Node<T>>; 2],
        // axis: Axis,
        bound: BoundBox,
    }
}


impl<T: BoundedShape> Node<T> {
    fn interior(l: Box<Node<T>>, r: Box<Node<T>>, axis: Axis) -> Node<T> {
        let bound = l.bound().union(&r.bound());
        Node::Interior {
            children: [l, r],
            // axis: axis,
            bound: bound
        }
    }

    fn bound(&self) -> BoundBox {
        match self {
            &Node::Leaf {ref shape} => shape.bound(),
            &Node::Interior { bound, ..} => bound
        }
    }


    fn build(shapes: Vec<T>) -> Node<T> {
        assert!(shapes.len() > 0);
        if shapes.len() == 1 {
            Node::Leaf {shape: shapes.into_iter().next().unwrap()}
        } else {
            let (left, right, axis) = Node::partition(shapes);
            Node::interior(
                Box::new(Node::build(left)),
                Box::new(Node::build(right)),
                axis)
        }
    }

    fn partition(mut shapes: Vec<T>) -> (Vec<T>, Vec<T>, Axis) {

        let axis = shapes.iter().map(|s| s.bound().center())
            .collect::<BoundBox>()
            .longext_axis();

        let key = |s: &T| s.bound().center()[axis];
        shapes.sort_by(|a, b| key(a).partial_cmp(&key(b)).unwrap());
        let mid = shapes.len() / 2;
        let mut l = Vec::new();
        let mut r = Vec::new();
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
        Bvh {
            root: Node::<T>::build(triangles)
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
                &Node::Leaf {ref shape} => if let Some(i) = shape.intersect(ray) {
                    match result {
                        None => {result = Some(i)},
                        Some(j) => {result = Some(min(i, j))}
                    };
                    t_bound = t_bound.min(i.t);
                },
                &Node::Interior {ref children, ..} => {
                    todo.push(&children[0]);
                    todo.push(&children[1]);
                }
            }
        }

        result
    }
}
