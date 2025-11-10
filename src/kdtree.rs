use std::collections::BinaryHeap;

use num_traits::{Float, One, Zero};
use thiserror::Error;

use crate::heap_element::HeapElement;

#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct KdTree<A, T, U: AsRef<[A]>> {
    // node
    left: Option<Box<KdTree<A, T, U>>>,
    right: Option<Box<KdTree<A, T, U>>>,
    // common
    dimensions: usize,
    capacity: usize,
    size: usize,
    min_bounds: Box<[A]>,
    max_bounds: Box<[A]>,
    // stem
    split_value: Option<A>,
    split_dimension: Option<usize>,
    // leaf
    points: Option<Vec<U>>,
    bucket: Option<Vec<T>>,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    #[error("wrong dimension")]
    WrongDimension,
    #[error("non-finite coordinate")]
    NonFiniteCoordinate,
    #[error("zero capacity")]
    ZeroCapacity,
}

impl<A: Float + Zero + One, T, U: AsRef<[A]>> KdTree<A, T, U> {
    // ============================================================================
    // === STRUCTURE ===
    // ============================================================================
    /// Create a new KD tree, specifying the dimension size of each point
    pub fn new(dims: usize) -> Self {
        KdTree::with_capacity(dims, 2_usize.pow(4))
    }

    /// Create a new KD tree, specifying the dimension size of each point and the capacity of leaf nodes
    pub fn with_capacity(dimensions: usize, capacity: usize) -> Self {
        let min_bounds = vec![A::max_value(); dimensions];
        let max_bounds = vec![A::min_value(); dimensions];
        KdTree {
            left: None,
            right: None,
            dimensions,
            capacity,
            size: 0,
            min_bounds: min_bounds.into_boxed_slice(),
            max_bounds: max_bounds.into_boxed_slice(),
            split_value: None,
            split_dimension: None,
            points: Some(vec![]),
            bucket: Some(vec![]),
        }
    }

    pub fn add(&mut self, point: U, data: T) -> Result<(), ErrorKind> {
        if self.capacity == 0 {
            return Err(ErrorKind::ZeroCapacity);
        }
        self.check_point(point.as_ref())?;
        self.add_unchecked(point, data)
    }

    fn add_unchecked(&mut self, point: U, data: T) -> Result<(), ErrorKind> {
        if self.is_leaf() {
            self.add_to_bucket(point, data);
            return Ok(());
        }
        self.extend(point.as_ref());
        self.size += 1;
        let next = if self.belongs_in_left(point.as_ref()) {
            self.left.as_mut()
        } else {
            self.right.as_mut()
        };
        next.unwrap().add_unchecked(point, data)
    }

    fn add_to_bucket(&mut self, point: U, data: T) {
        self.extend(point.as_ref());
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

    fn split(&mut self, mut points: Vec<U>, mut bucket: Vec<T>) {
        let mut max = A::zero();
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
            }
            Some(dim) => {
                let min = self.min_bounds[dim];
                let max = self.max_bounds[dim];
                self.split_value = Some(min + (max - min) / A::from(2.0).unwrap());
            }
        };
        let mut left = Box::new(KdTree::with_capacity(self.dimensions, self.capacity));
        let mut right = Box::new(KdTree::with_capacity(self.dimensions, self.capacity));
        while !points.is_empty() {
            let point = points.swap_remove(0);
            let data = bucket.swap_remove(0);
            if self.belongs_in_left(point.as_ref()) {
                left.add_to_bucket(point, data);
            } else {
                right.add_to_bucket(point, data);
            }
        }
        self.left = Some(left);
        self.right = Some(right);
    }

    pub fn remove(&mut self, point: &U, data: &T) -> Result<usize, ErrorKind>
    where
        T: std::cmp::PartialEq,
        U: std::cmp::PartialEq,
    {
        let mut removed = 0;
        self.check_point(point.as_ref())?;
        if let (Some(mut points), Some(mut bucket)) = (self.points.take(), self.bucket.take()) {
            while let Some(p_index) = points
                .iter()
                .zip(bucket.iter())
                .position(|(p, d)| p == point && d == data)
            {
                points.remove(p_index);
                bucket.remove(p_index);
                removed += 1;
                self.size -= 1;
            }
            self.points = Some(points);
            self.bucket = Some(bucket);
        } else {
            if let Some(right) = self.right.as_mut() {
                let right_removed = right.remove(point, data)?;
                if right_removed > 0 {
                    self.size -= right_removed;
                    removed += right_removed;
                }
            }
            if let Some(left) = self.left.as_mut() {
                let left_removed = left.remove(point, data)?;
                if left_removed > 0 {
                    self.size -= left_removed;
                    removed += left_removed;
                }
            }
        }
        Ok(removed)
    }

    pub fn size(&self) -> usize {
        self.size
    }

    // ============================================================================
    // === NEAREST QUERIES ===
    // ============================================================================
    pub fn nearest<F>(&self, point: &[A], num: usize, distance: &F) -> Result<Vec<(A, &T)>, ErrorKind>
    where
        F: Fn(&[A], &[A]) -> A,
    {
        self.nearest_within_radius_internal(point, num, A::max_value(), distance)
    }

    pub fn nearest_within_radius<F>(
        &self,
        point: &[A],
        num: usize,
        radius: Option<A>,
        distance: &F,
    ) -> Result<Vec<(A, &T)>, ErrorKind>
    where
        F: Fn(&[A], &[A]) -> A,
    {
        let radius = radius.unwrap_or_else(A::max_value);
        self.nearest_within_radius_internal(point, num, radius, distance)
    }

    pub fn iter_nearest<'a, F>(
        &'a self,
        point: &'a [A],
        distance: &'a F,
    ) -> Result<NearestIter<'a, A, T, U, F>, ErrorKind>
    where
        F: Fn(&[A], &[A]) -> A,
    {
        self.iter_nearest_within_radius(point, None, distance)
            .map(|inner| NearestIter { inner })
    }

    pub fn iter_nearest_within_radius<'a, F>(
        &'a self,
        point: &'a [A],
        radius: Option<A>,
        distance: &'a F,
    ) -> Result<NearestWithinRadiusIter<'a, A, T, U, F>, ErrorKind>
    where
        F: Fn(&[A], &[A]) -> A,
    {
        self.check_point(point)?;
        let mut pending = BinaryHeap::new();
        let evaluated = BinaryHeap::<HeapElement<A, &T>>::new();
        pending.push(HeapElement {
            distance: A::zero(),
            element: self,
        });
        Ok(NearestWithinRadiusIter {
            point,
            pending,
            evaluated,
            distance,
            radius: radius.unwrap_or_else(A::max_value),
        })
    }

    pub fn iter_nearest_mut<'a, F>(
        &'a mut self,
        point: &'a [A],
        distance: &'a F,
    ) -> Result<NearestIterMut<'a, A, T, U, F>, ErrorKind>
    where
        F: Fn(&[A], &[A]) -> A,
    {
        let radius_iter = self.iter_nearest_within_radius_mut(point, None, distance)?;
        Ok(NearestIterMut { inner: radius_iter })
    }

    pub fn iter_nearest_within_radius_mut<'a, F>(
        &'a mut self,
        point: &'a [A],
        radius: Option<A>,
        distance: &'a F,
    ) -> Result<NearestWithinRadiusIterMut<'a, A, T, U, F>, ErrorKind>
    where
        F: Fn(&[A], &[A]) -> A,
    {
        self.check_point(point)?;
        let mut pending = BinaryHeap::new();
        let evaluated = BinaryHeap::<HeapElement<A, &mut T>>::new();
        pending.push(HeapElement {
            distance: A::zero(),
            element: self,
        });
        Ok(NearestWithinRadiusIterMut {
            point,
            pending,
            evaluated,
            distance,
            radius: radius.unwrap_or_else(A::max_value),
        })
    }

    // ============================================================================
    // === NEAREST HELPERS ===
    // ============================================================================
    fn nearest_within_radius_internal<F>(
        &self,
        point: &[A],
        num: usize,
        radius: A,
        distance: &F,
    ) -> Result<Vec<(A, &T)>, ErrorKind>
    where
        F: Fn(&[A], &[A]) -> A,
    {
        self.check_point(point)?;
        let num = std::cmp::min(num, self.size);
        if num == 0 {
            return Ok(vec![]);
        }
        let mut pending = BinaryHeap::new();
        let mut evaluated = BinaryHeap::<HeapElement<A, &T>>::new();
        pending.push(HeapElement {
            distance: A::zero(),
            element: self,
        });
        while !pending.is_empty()
            && (-pending.peek().unwrap().distance <= radius)
            && (evaluated.len() < num || (-pending.peek().unwrap().distance <= evaluated.peek().unwrap().distance))
        {
            self.nearest_step(point, num, radius, distance, &mut pending, &mut evaluated);
        }
        Ok(evaluated
            .into_sorted_vec()
            .into_iter()
            .take(num)
            .map(Into::into)
            .collect())
    }

    fn nearest_step<'b, F>(
        &self,
        point: &[A],
        num: usize,
        max_dist: A,
        distance: &F,
        pending: &mut BinaryHeap<HeapElement<A, &'b Self>>,
        evaluated: &mut BinaryHeap<HeapElement<A, &'b T>>,
    ) where
        F: Fn(&[A], &[A]) -> A,
    {
        let mut curr = pending.pop().unwrap().element;
        debug_assert!(evaluated.len() <= num);
        let evaluated_dist = if evaluated.len() == num {
            max_dist.min(evaluated.peek().unwrap().distance)
        } else {
            max_dist
        };

        while !curr.is_leaf() {
            let candidate;
            if curr.belongs_in_left(point) {
                candidate = curr.right.as_ref().unwrap();
                curr = curr.left.as_ref().unwrap();
            } else {
                candidate = curr.left.as_ref().unwrap();
                curr = curr.right.as_ref().unwrap();
            }
            let candidate_to_space =
                Self::distance_to_space(point, &candidate.min_bounds, &candidate.max_bounds, distance);
            if candidate_to_space <= evaluated_dist {
                pending.push(HeapElement {
                    distance: candidate_to_space * -A::one(),
                    element: &**candidate,
                });
            }
        }

        let points = curr.points.as_ref().unwrap().iter();
        let bucket = curr.bucket.as_ref().unwrap().iter();
        let iter = points.zip(bucket).map(|(p, d)| HeapElement {
            distance: distance(point, p.as_ref()),
            element: d,
        });
        for element in iter {
            if element.distance <= max_dist {
                if evaluated.len() < num {
                    evaluated.push(element);
                } else if element < *evaluated.peek().unwrap() {
                    evaluated.pop();
                    evaluated.push(element);
                }
            }
        }
    }

    fn distance_to_space<F, V>(p1: &[V], min_bounds: &[V], max_bounds: &[V], distance: &F) -> V
    where
        F: Fn(&[V], &[V]) -> V,
        V: Float,
    {
        let mut p2 = vec![V::nan(); p1.len()];
        for i in 0..p1.len() {
            if p1[i] > max_bounds[i] {
                p2[i] = max_bounds[i];
            } else if p1[i] < min_bounds[i] {
                p2[i] = min_bounds[i];
            } else {
                p2[i] = p1[i];
            }
        }
        distance(p1, &p2[..])
    }

    // ============================================================================
    // === WITHIN QUERIES ===
    // ============================================================================
    pub fn within<F>(&self, point: &[A], radius: A, distance: &F) -> Result<Vec<(A, &T)>, ErrorKind>
    where
        F: Fn(&[A], &[A]) -> A,
    {
        self.check_point(point)?;
        if self.size == 0 {
            return Ok(vec![]);
        }
        let evaluated = self.evaluated_heap(point, radius, distance);
        Ok(evaluated.into_iter().map(Into::into).collect())
    }

    pub fn within_count<F>(&self, point: &[A], radius: A, distance: &F) -> Result<usize, ErrorKind>
    where
        F: Fn(&[A], &[A]) -> A,
    {
        self.check_point(point)?;
        if self.size == 0 {
            return Ok(0);
        }
        let evaluated = self.evaluated_heap(point, radius, distance);
        Ok(evaluated.len())
    }

    // ============================================================================
    // === BOUNDING BOX ===
    // ============================================================================

    pub fn bounding_box(&self, min_bounds: &[A], max_bounds: &[A]) -> Result<Vec<&T>, ErrorKind> {
        self.check_point(min_bounds)?;
        self.check_point(max_bounds)?;
        if self.size == 0 {
            return Ok(vec![]);
        }
        let mut pending = vec![];
        let mut evaluated = vec![];
        pending.push(self);
        while let Some(curr) = pending.pop() {
            if curr.is_leaf() {
                let points = curr.points.as_ref().unwrap().iter();
                let bucket = curr.bucket.as_ref().unwrap().iter();
                for (p, b) in points.zip(bucket) {
                    if Self::in_bounding_box(p.as_ref(), min_bounds, max_bounds) {
                        evaluated.push(b);
                    }
                }
            } else {
                if curr.belongs_in_left(min_bounds) {
                    pending.push(curr.left.as_ref().unwrap());
                }
                if !curr.belongs_in_left(max_bounds) {
                    pending.push(curr.right.as_ref().unwrap());
                }
            }
        }

        Ok(evaluated)
    }

    fn in_bounding_box<V>(p: &[V], min_bounds: &[V], max_bounds: &[V]) -> bool
    where
        V: Float,
    {
        for ((l, h), v) in min_bounds.iter().zip(max_bounds.iter()).zip(p) {
            if v < l || v > h {
                return false;
            }
        }
        true
    }

    // ============================================================================
    // === SHARED TRAVERSAL UTILITIES ===
    // ============================================================================
    #[inline(always)]
    fn evaluated_heap<F>(&self, point: &[A], radius: A, distance: &F) -> BinaryHeap<HeapElement<A, &T>>
    where
        F: Fn(&[A], &[A]) -> A,
    {
        let mut pending = BinaryHeap::new();
        let mut evaluated = BinaryHeap::<HeapElement<A, &T>>::new();
        pending.push(HeapElement {
            distance: A::zero(),
            element: self,
        });
        while !pending.is_empty() && (-pending.peek().unwrap().distance <= radius) {
            self.nearest_step(point, self.size, radius, distance, &mut pending, &mut evaluated);
        }
        evaluated
    }

    fn belongs_in_left(&self, point: &[A]) -> bool {
        if self.min_bounds[self.split_dimension.unwrap()] == self.split_value.unwrap() {
            point[self.split_dimension.unwrap()] <= self.split_value.unwrap()
        } else {
            point[self.split_dimension.unwrap()] < self.split_value.unwrap()
        }
    }

    fn extend(&mut self, point: &[A]) {
        let min = self.min_bounds.iter_mut();
        let max = self.max_bounds.iter_mut();
        for ((l, h), v) in min.zip(max).zip(point.iter()) {
            if v < l {
                *l = *v
            }
            if v > h {
                *h = *v
            }
        }
    }

    fn is_leaf(&self) -> bool {
        self.bucket.is_some()
            && self.points.is_some()
            && self.split_value.is_none()
            && self.split_dimension.is_none()
            && self.left.is_none()
            && self.right.is_none()
    }

    fn check_point(&self, point: &[A]) -> Result<(), ErrorKind> {
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

// ============================================================================
// === NEAREST ITERATOR TYPES ===
// ============================================================================

pub struct NearestIter<'a, A: Float, T, U: AsRef<[A]>, F: Fn(&[A], &[A]) -> A> {
    inner: NearestWithinRadiusIter<'a, A, T, U, F>,
}

impl<'a, A: Float + Zero + One, T, U: AsRef<[A]>, F> Iterator for NearestIter<'a, A, T, U, F>
where
    F: Fn(&[A], &[A]) -> A,
{
    type Item = (A, &'a T);
    fn next(&mut self) -> Option<(A, &'a T)> {
        self.inner.next()
    }
}

pub struct NearestIterMut<'a, A: Float, T, U: AsRef<[A]>, F: Fn(&[A], &[A]) -> A> {
    inner: NearestWithinRadiusIterMut<'a, A, T, U, F>,
}

impl<'a, A: Float + Zero + One, T, U: AsRef<[A]>, F> Iterator for NearestIterMut<'a, A, T, U, F>
where
    F: Fn(&[A], &[A]) -> A,
{
    type Item = (A, &'a mut T);
    fn next(&mut self) -> Option<(A, &'a mut T)> {
        self.inner.next()
    }
}

pub struct NearestWithinRadiusIter<'a, A: Float, T, U: AsRef<[A]>, F: Fn(&[A], &[A]) -> A> {
    point: &'a [A],
    pending: BinaryHeap<HeapElement<A, &'a KdTree<A, T, U>>>,
    evaluated: BinaryHeap<HeapElement<A, &'a T>>,
    distance: &'a F,
    radius: A,
}

impl<'a, A: Float + Zero + One, T, U: AsRef<[A]>, F> Iterator for NearestWithinRadiusIter<'a, A, T, U, F>
where
    F: Fn(&[A], &[A]) -> A,
{
    type Item = (A, &'a T);
    fn next(&mut self) -> Option<(A, &'a T)> {
        let distance = self.distance;
        let point = self.point;
        let radius_limit = self.radius;
        while !self.pending.is_empty()
            && (-self.pending.peek().unwrap().distance <= radius_limit)
            && (self.evaluated.peek().map_or(A::max_value(), |x| -x.distance) >= -self.pending.peek().unwrap().distance)
        {
            let mut curr = self.pending.pop().unwrap().element;
            while !curr.is_leaf() {
                let candidate;
                if curr.belongs_in_left(point) {
                    candidate = curr.right.as_ref().unwrap();
                    curr = curr.left.as_ref().unwrap();
                } else {
                    candidate = curr.left.as_ref().unwrap();
                    curr = curr.right.as_ref().unwrap();
                }
                let candidate_distance =
                    KdTree::<A, T, U>::distance_to_space(point, &candidate.min_bounds, &candidate.max_bounds, distance);
                if candidate_distance <= radius_limit {
                    self.pending.push(HeapElement {
                        distance: -candidate_distance,
                        element: &**candidate,
                    });
                }
            }
            let points = curr.points.as_ref().unwrap().iter();
            let bucket = curr.bucket.as_ref().unwrap().iter();
            self.evaluated.extend(points.zip(bucket).filter_map(|(p, d)| {
                let dist = distance(point, p.as_ref());
                if dist <= radius_limit {
                    Some(HeapElement {
                        distance: -dist,
                        element: d,
                    })
                } else {
                    None
                }
            }));
        }
        self.evaluated.pop().map(|x| (-x.distance, x.element))
    }
}

pub struct NearestWithinRadiusIterMut<'a, A: Float, T, U: AsRef<[A]>, F: Fn(&[A], &[A]) -> A> {
    point: &'a [A],
    pending: BinaryHeap<HeapElement<A, &'a mut KdTree<A, T, U>>>,
    evaluated: BinaryHeap<HeapElement<A, &'a mut T>>,
    distance: &'a F,
    radius: A,
}

impl<'a, A: Float + Zero + One, T, U: AsRef<[A]>, F> Iterator for NearestWithinRadiusIterMut<'a, A, T, U, F>
where
    F: Fn(&[A], &[A]) -> A,
{
    type Item = (A, &'a mut T);
    fn next(&mut self) -> Option<(A, &'a mut T)> {
        let distance = self.distance;
        let point = self.point;
        let radius_limit = self.radius;
        while !self.pending.is_empty()
            && (-self.pending.peek().unwrap().distance <= radius_limit)
            && (self.evaluated.peek().map_or(A::max_value(), |x| -x.distance) >= -self.pending.peek().unwrap().distance)
        {
            let mut curr = &mut *self.pending.pop().unwrap().element;
            while !curr.is_leaf() {
                let candidate;
                if curr.belongs_in_left(point) {
                    candidate = curr.right.as_mut().unwrap();
                    curr = curr.left.as_mut().unwrap();
                } else {
                    candidate = curr.left.as_mut().unwrap();
                    curr = curr.right.as_mut().unwrap();
                }
                let candidate_distance =
                    KdTree::<A, T, U>::distance_to_space(point, &candidate.min_bounds, &candidate.max_bounds, distance);
                if candidate_distance <= radius_limit {
                    self.pending.push(HeapElement {
                        distance: -candidate_distance,
                        element: &mut **candidate,
                    });
                }
            }
            let points = curr.points.as_ref().unwrap().iter();
            let bucket = curr.bucket.as_mut().unwrap().iter_mut();
            self.evaluated.extend(points.zip(bucket).filter_map(|(p, d)| {
                let dist = distance(point, p.as_ref());
                if dist <= radius_limit {
                    Some(HeapElement {
                        distance: -dist,
                        element: d,
                    })
                } else {
                    None
                }
            }));
        }
        self.evaluated.pop().map(|x| (-x.distance, x.element))
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;
    use super::KdTree;

    fn random_point() -> ([f64; 2], i32) {
        rand::random::<([f64; 2], i32)>()
    }

    #[test]
    fn it_holds_onto_capacity_before_splitting() {
        let mut tree: KdTree<f64, i32, [f64; 2]> = KdTree::new(2);
        let capacity = 2_usize.pow(4);
        for _ in 0..capacity {
            let (pos, data) = random_point();
            tree.add(pos, data).unwrap();
        }
        assert_eq!(tree.size(), capacity);
        assert!(tree.left.is_none() && tree.right.is_none());
        {
            let (pos, data) = random_point();
            tree.add(pos, data).unwrap();
        }
        assert_eq!(tree.size(), capacity + 1);
        assert!(tree.left.is_some() && tree.right.is_some());
    }

    #[test]
    fn test_normal_distance_to_space() {
        use crate::distance::squared_euclidean;
        let dis =
            KdTree::<f64, i32, [f64; 2]>::distance_to_space(&[0.0, 0.0], &[1.0, 1.0], &[2.0, 2.0], &squared_euclidean);
        assert_eq!(dis, 2.0);
    }

    #[test]
    fn test_distance_outside_inf() {
        use crate::distance::squared_euclidean;
        let dis = KdTree::<f64, i32, [f64; 2]>::distance_to_space(
            &[0.0, 0.0],
            &[1.0, 1.0],
            &[f64::INFINITY, f64::INFINITY],
            &squared_euclidean,
        );
        assert_eq!(dis, 2.0);
    }

    #[test]
    fn test_distance_inside_inf() {
        use crate::distance::squared_euclidean;
        let dis = KdTree::<f64, i32, [f64; 2]>::distance_to_space(
            &[2.0, 2.0],
            &[f64::NEG_INFINITY, f64::NEG_INFINITY],
            &[f64::INFINITY, f64::INFINITY],
            &squared_euclidean,
        );
        assert_eq!(dis, 0.0);
    }

    #[test]
    fn test_distance_inside_normal() {
        use crate::distance::squared_euclidean;
        let dis =
            KdTree::<f64, i32, [f64; 2]>::distance_to_space(&[2.0, 2.0], &[0.0, 0.0], &[3.0, 3.0], &squared_euclidean);
        assert_eq!(dis, 0.0);
    }

    #[test]
    fn distance_to_half_space() {
        use crate::distance::squared_euclidean;
        let dis = KdTree::<f64, i32, [f64; 2]>::distance_to_space(
            &[-2.0, 0.0],
            &[0.0, f64::NEG_INFINITY],
            &[f64::INFINITY, f64::INFINITY],
            &squared_euclidean,
        );
        assert_eq!(dis, 4.0);
    }

    #[test]
    fn test_in_bounding_box_via_bounding_box() {
        let mut tree: KdTree<f64, i32, [f64; 2]> = KdTree::new(2);
        tree.add([1.0, 1.0], 1).unwrap();
        tree.add([0.5, 0.5], 2).unwrap();
        tree.add([2.0, 2.0], 3).unwrap();

        // Test bounding box that includes first two points
        let result = tree.bounding_box(&[0.0, 0.0], &[1.5, 1.5]).unwrap();
        assert_eq!(result.len(), 2);

        // Test bounding box that includes only one point
        let result = tree.bounding_box(&[1.0, 1.0], &[1.0, 1.0]).unwrap();
        assert_eq!(result.len(), 1);

        // Test bounding box that includes no points
        let result = tree.bounding_box(&[2.5, 2.5], &[3.0, 3.0]).unwrap();
        assert_eq!(result.len(), 0);
    }
}
