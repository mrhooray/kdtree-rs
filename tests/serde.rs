#[cfg(all(test, feature = "serialize"))]
mod serialization_tests {
    use kdtree::KdTree;

    #[test]
    fn empty_tree() {
        let kdtree = KdTree::<f64, String, [f64; 2]>::new(2);

        let serialized = serde_json::to_string(&kdtree).unwrap();
        let deserialized_tree: KdTree<f64, String, [f64; 2]> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized_tree.size(), 0);
    }

    #[test]
    fn tree_with_one_element() {
        let mut kdtree = KdTree::<f64, String, [f64; 2]>::new(2);
        kdtree.add([0.0, 0.0], "test point".to_string()).unwrap();

        let serialized = serde_json::to_string(&kdtree).unwrap();
        let deserialized_tree: KdTree<f64, String, [f64; 2]> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized_tree.size(), 1);
    }
}
