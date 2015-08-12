use std;
use std::collections::BinaryHeap;
use ::heap_element::HeapElement;
use ::util;

#[derive(Debug)]
pub struct KdTree<'a, T> {
    // node
    left: Option<*mut KdTree<'a, T>>,
    right: Option<*mut KdTree<'a, T>>,
    // common
    dimensions: usize,
    capacity: usize,
    size: usize,
    min_bounds: Box<[f64]>,
    max_bounds: Box<[f64]>,
    // stem
    split_value: Option<f64>,
    split_dimension: Option<usize>,
    // leaf
    points: Option<Vec<&'a [f64]>>,
    bucket: Option<Vec<T>>,
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    WrongDimension,
    NonFiniteCoordinate,
    ZeroCapacity
}

impl<'a, T> KdTree<'a, T> {
    pub fn new(dims: usize) -> KdTree<'a, T> {
        KdTree::new_with_capacity(dims, 2^4)
    }

    pub fn new_with_capacity(dimensions: usize, capacity: usize) -> KdTree<'a, T> {
        let min_bounds = vec![std::f64::NAN; dimensions];
        let max_bounds = vec![std::f64::NAN; dimensions];
        KdTree {
            left: None,
            right : None,
            dimensions: dimensions,
            capacity: capacity,
            size: 0,
            min_bounds: min_bounds.into_boxed_slice(),
            max_bounds: max_bounds.into_boxed_slice(),
            split_value : None,
            split_dimension : None,
            points: Some(vec![]),
            bucket: Some(vec![]),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn nearest<F> (&self, point: &[f64], num: usize, distance: &F) -> Result<Vec<(f64, &T)>, ErrorKind>
        where F: Fn(&[f64], &[f64]) -> f64 {
            match self.check_point(point) {
                Err(err) => return Err(err),
                Ok(_) => {}
            };
            let pending = &mut BinaryHeap::<HeapElement<&KdTree<T>>>::new();
            let mut evaluated = BinaryHeap::<HeapElement<&T>>::new();
            let num = std::cmp::min(num, self.size);
            pending.push(HeapElement {
                distance: 0f64,
                element: self
            });
            while pending.len() > 0 && num >=1 &&
                (evaluated.len() < num ||
                 ((pending.peek().unwrap().distance * -1f64) < evaluated.peek().unwrap().distance)) {
                self.nearest_step(point, num, distance, pending, &mut evaluated);
            }
            let mut result = vec![];
            loop {
                match evaluated.pop() {
                    None => {
                        result.reverse();
                        return Ok(result);
                    },
                    Some(e) => result.push((e.distance, e.element)),
                }
            }
        }

    fn nearest_step<'b, F> (&self, point: &[f64], num: usize, distance: &F,
                        pending: &mut BinaryHeap<HeapElement<&'b KdTree<T>>>,
                        evaluated: &mut BinaryHeap<HeapElement<&'b T>>)
        where F: Fn(&[f64], &[f64]) -> f64 {
            let curr = pending.pop();

            if curr.is_none() {
                return;
            }

            let mut curr = &*curr.unwrap().element;

            while !curr.is_leaf() {
                let candidate;
                if point[curr.split_dimension.unwrap()] > curr.split_value.unwrap() {
                    candidate = unsafe {&*curr.left.unwrap()};
                    curr = unsafe {&*curr.right.unwrap()};
                } else {
                    candidate = unsafe {&*curr.right.unwrap()};
                    curr = unsafe {&*curr.left.unwrap()};
                }
                let candidate_to_space= util::distance_to_space(point, &*curr.min_bounds, &*curr.max_bounds, distance);
                if evaluated.len() < num || candidate_to_space<= evaluated.peek().unwrap().distance {
                    pending.push(HeapElement {
                        distance: candidate_to_space* -1f64,
                        element: candidate
                    });
                }
            }

            for i in 0..curr.size {
                let p = curr.points.as_ref().unwrap()[i];
                let d = &curr.bucket.as_ref().unwrap()[i];
                let p_to_point = distance(p, point);
                if evaluated.len() < num {
                    evaluated.push(HeapElement {
                        distance: p_to_point,
                        element: d
                    });
                } else if p_to_point < evaluated.peek().unwrap().distance {
                    evaluated.pop();
                    evaluated.push(HeapElement {
                        distance: p_to_point,
                        element: d
                    });
                }
            }
        }

    pub fn add(&mut self, point: &'a [f64], data: T) -> Result<(), ErrorKind> {
        if self.capacity == 0 {
            return Err(ErrorKind::ZeroCapacity);
        }
        match self.check_point(point) {
            Err(err) => return Err(err),
            Ok(_) => {}
        };
        let mut curr = &mut *self;
        while !curr.is_leaf() {
            curr.extend(point);
            curr.size += 1;
            if point[curr.split_dimension.unwrap()] < curr.split_value.unwrap() {
                curr = unsafe {&mut *curr.left.unwrap()};
            } else {
                curr = unsafe {&mut *curr.right.unwrap()};
            }
        }
        curr.add_to_bucket(point, data);
        Ok(())
    }

    fn add_to_bucket(&mut self, point: &'a [f64], data: T) {
        self.extend(point);
        let mut points = self.points.take().unwrap();
        let mut bucket = self.bucket.take().unwrap();
        points.push(point);
        bucket.push(data);
        self.size += 1;
        if self.size > self.capacity {
            self.split(points, bucket);
        } else {
            self.points = Some(points);
            self.bucket = Some(bucket);
        }
    }

    fn split(&mut self, mut points: Vec<&'a [f64]>, mut bucket: Vec<T>) {
        let mut max = 0f64;
        for dim in 0..self.dimensions {
            let diff = self.max_bounds[dim] - self.min_bounds[dim];
            if !diff.is_nan() && diff > max {
                max = diff;
                self.split_dimension = Some(dim);
            }
        }
        match self.split_dimension {
            None => {
                self.points = Some(points);
                self.bucket = Some(bucket);
                return;
            },
            Some(dim) => {
                let min = self.min_bounds[dim];
                let max = self.max_bounds[dim];
                self.split_value = Some(min + (max - min) / 2f64);
            }
        };
        let mut left = Box::new(KdTree::<T>::new_with_capacity(self.dimensions, self.capacity));
        let mut right = Box::new(KdTree::<T>::new_with_capacity(self.dimensions, self.capacity));
        while points.len() > 0 {
            let point = points.swap_remove(0);
            let data = bucket.swap_remove(0);
            if point[self.split_dimension.unwrap()] < self.split_value.unwrap() {
                    left.add_to_bucket(point, data);
            } else {
                    right.add_to_bucket(point, data);
            }
        }
        self.left = Some(Box::into_raw(left));
        self.right = Some(Box::into_raw(right));
    }

    fn extend(&mut self, point: &[f64]) {
        for dim in 0..self.dimensions {
            if self.min_bounds[dim].is_nan() || self.min_bounds[dim] > point[dim] {
                self.min_bounds[dim] = point[dim];
            }
            if self.max_bounds[dim].is_nan() || self.max_bounds[dim] < point[dim] {
                self.max_bounds[dim] = point[dim];
            }
        }
    }

    fn is_leaf(&self) -> bool {
        self.bucket.is_some() && self.points.is_some() &&
            self.split_value.is_none() && self.split_dimension.is_none() &&
            self.left.is_none() && self.right.is_none()
    }

    fn check_point(&self, point: &[f64]) -> Result<(), ErrorKind> {
        if self.dimensions != point.len() {
            return Err(ErrorKind::WrongDimension);
        }
        for n in point {
            if !n.is_finite() {
                return Err(ErrorKind::NonFiniteCoordinate);
            }
        }
        Ok(())
    }
}
