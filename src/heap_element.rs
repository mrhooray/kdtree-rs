use std::cmp::Ordering;

pub struct HeapElement<T> {
    pub distance: f64,
    pub element: T,
}

impl<T> Ord for HeapElement<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl<T> PartialOrd for HeapElement<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl<T> PartialOrd<f64> for HeapElement<T> {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        self.distance.partial_cmp(other)
    }
}

impl<T> Eq for HeapElement<T> {}

impl<T> PartialEq for HeapElement<T> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<T> PartialEq<f64> for HeapElement<T> {
    fn eq(&self, other: &f64) -> bool {
        self.distance == *other
    }
}

impl<T> Into<(f64, T)> for HeapElement<T> {
    fn into(self) -> (f64, T) {
        (self.distance, self.element)
    }
}
