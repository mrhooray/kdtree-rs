use std::cmp::Ordering;

pub struct HeapElement<T> {
    pub distance: f64,
    pub element: T
}

impl<T> Ord for HeapElement<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.distance < other.distance {
            return Ordering::Less;
        } else if self.distance > other.distance {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    }
}

impl<T> PartialOrd for HeapElement<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Eq for HeapElement<T> {
}

impl<T> PartialEq for HeapElement<T> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}
