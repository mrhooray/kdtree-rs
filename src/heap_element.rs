use num_traits::Float;
use std::cmp::Ordering;

pub struct HeapElement<A, T> {
    pub distance: A,
    pub element: T,
}

impl<A: Float, T> Ord for HeapElement<A, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl<A: Float, T> PartialOrd for HeapElement<A, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl<A: Float, T> PartialOrd<A> for HeapElement<A, T>
where
    HeapElement<A, T>: PartialEq<A>,
{
    fn partial_cmp(&self, other: &A) -> Option<Ordering> {
        self.distance.partial_cmp(other)
    }
}

impl<A: Float, T> Eq for HeapElement<A, T> {}

impl<A: Float, T> PartialEq for HeapElement<A, T> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<A: Float, T> PartialEq<A> for HeapElement<A, T> {
    fn eq(&self, other: &A) -> bool {
        self.distance == *other
    }
}

impl<A: Float, T> Into<(A, T)> for HeapElement<A, T> {
    fn into(self) -> (A, T) {
        (self.distance, self.element)
    }
}
