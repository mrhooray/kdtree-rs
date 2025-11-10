use kdtree::KdTree;

#[test]
fn zero_capacity_tree_rejects_insertions() {
    let mut tree: KdTree<f64, i32, [f64; 2]> = KdTree::with_capacity(2, 0);
    assert!(tree.add([0.0, 0.0], 1).is_err());
    assert_eq!(tree.size(), 0);
}

#[test]
fn cloning_preserves_existing_data() {
    let mut original: KdTree<f64, i32, [f64; 2]> = KdTree::new(2);
    original.add([0.0, 0.0], 1).unwrap();
    original.add([1.0, 1.0], 2).unwrap();

    let mut cloned = original.clone();
    cloned.add([2.0, 2.0], 3).unwrap();

    assert_eq!(original.size(), 2);
    assert_eq!(cloned.size(), 3);
}

#[test]
fn splitting_handles_extreme_bounds() {
    // Values that previously exposed floating-point edge cases.
    let cases = [
        (0.47945351705599926, 0.479_453_517_055_999_3),
        (-0.479_453_517_055_999_3, -0.47945351705599926),
    ];

    for (low, high) in cases {
        let mut tree = KdTree::with_capacity(1, 2);
        tree.add([low], ()).unwrap();
        tree.add([high], ()).unwrap();
        tree.add([low], ()).unwrap();
        tree.add([high], ()).unwrap();
    }
}
