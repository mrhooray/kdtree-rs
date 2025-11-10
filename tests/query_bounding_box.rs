use kdtree::KdTree;

#[test]
fn bounding_box_returns_all_points_in_range() {
    let mut tree = KdTree::with_capacity(2, 2);
    for i in 0..10 {
        for j in 0..10 {
            let id = i.to_string() + &j.to_string();
            tree.add([i as f64, j as f64], id).unwrap();
        }
    }

    let within: Vec<String> = tree
        .bounding_box(&[4.0, 4.0], &[6.0, 6.0])
        .unwrap()
        .iter()
        .cloned()
        .cloned()
        .collect();
    assert_eq!(within.len(), 9);
    assert!(within.contains(&String::from("44")));
    assert!(within.contains(&String::from("45")));
    assert!(within.contains(&String::from("46")));
    assert!(within.contains(&String::from("54")));
    assert!(within.contains(&String::from("55")));
    assert!(within.contains(&String::from("56")));
    assert!(within.contains(&String::from("64")));
    assert!(within.contains(&String::from("65")));
    assert!(within.contains(&String::from("66")));
}
