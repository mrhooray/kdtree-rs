use num_traits::Float;
use std::cmp::Ordering;

pub struct HeapElement<A, T> {
    pub distance: A,
    pub element: T,
}

impl<A: Float, T> Ord for HeapElement<A, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.partial_cmp(&other.distance).unwrap_or(Ordering::Equal)
    }
}

impl<A: Float, T> PartialOrd for HeapElement<A, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<A: Float, T> Eq for HeapElement<A, T> {}

impl<A: Float, T> PartialEq for HeapElement<A, T> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<A: Float, T> From<HeapElement<A, T>> for (A, T) {
    fn from(e: HeapElement<A, T>) -> Self {
        (e.distance, e.element)
    }
}
