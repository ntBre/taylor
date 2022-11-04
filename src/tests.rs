use super::*;

fn load_vec_isize(filename: &str) -> Vec<Vec<isize>> {
    let mut ret = Vec::new();
    let contents = std::fs::read_to_string(filename).unwrap();
    let lines = contents.lines();
    for line in lines {
        ret.push(line.split(',').map(|s| s.parse().unwrap()).collect());
    }
    ret
}

fn load_vec_usize(filename: &str) -> Vec<Vec<usize>> {
    let mut ret = Vec::new();
    let contents = std::fs::read_to_string(filename).unwrap();
    let lines = contents.lines();
    for line in lines {
        ret.push(line.split(',').map(|s| s.parse().unwrap()).collect());
    }
    ret
}

#[test]
fn test_forces() {
    let got = Taylor::new(5, 3, None, None).forces;
    #[rustfmt::skip]
	let want = vec![
	    vec![0, 0, 0], vec![0, 0, 1], vec![0, 0, 2],
	    vec![0, 0, 3], vec![0, 0, 4], vec![0, 1, 0],
	    vec![0, 1, 1], vec![0, 1, 2], vec![0, 1, 3],
	    vec![0, 2, 0], vec![0, 2, 1], vec![0, 2, 2],
	    vec![0, 3, 0], vec![0, 3, 1], vec![0, 4, 0],
	    vec![1, 0, 0], vec![1, 0, 1], vec![1, 0, 2],
	    vec![1, 0, 3], vec![1, 1, 0], vec![1, 1, 1],
	    vec![1, 1, 2], vec![1, 2, 0], vec![1, 2, 1],
	    vec![1, 3, 0], vec![2, 0, 0], vec![2, 0, 1],
	    vec![2, 0, 2], vec![2, 1, 0], vec![2, 1, 1],
	    vec![2, 2, 0], vec![3, 0, 0], vec![3, 0, 1],
	    vec![3, 1, 0], vec![4, 0, 0]];
    assert_eq!(got, want);
}

#[test]
fn test_forces_with_checks() {
    let got = Taylor::new(
        5,
        9,
        Some(Checks([[5, 7], [8, 8], [9, 9]])),
        Some(Checks([[5, 7], [8, 8], [9, 9]])),
    );
    let want = load_vec_usize("testfiles/force.txt");
    assert_eq!(got.forces, want);
}

#[test]
fn test_forces_with_zero_checks() {
    let got = Taylor::new(
        5,
        3,
        Some(Checks([[3, 3], [0, 0], [0, 0]])),
        Some(Checks([[3, 3], [0, 0], [0, 0]])),
    );
    #[rustfmt::skip]
    let want = vec![
        vec![0, 0, 0], vec![0, 0, 2], vec![0, 0, 4],
        vec![0, 1, 0], vec![0, 1, 2], vec![0, 2, 0],
        vec![0, 2, 2], vec![0, 3, 0], vec![0, 4, 0],
        vec![1, 0, 0], vec![1, 0, 2], vec![1, 1, 0],
        vec![1, 1, 2], vec![1, 2, 0], vec![1, 3, 0],
        vec![2, 0, 0], vec![2, 0, 2], vec![2, 1, 0],
        vec![2, 2, 0], vec![3, 0, 0], vec![3, 1, 0],
        vec![4, 0, 0],
    ];
    assert_eq!(got.forces, want);
}

#[test]
fn test_disps() {
    let got = Taylor::new(5, 3, None, None).disps();
    let mut want = Disps(load_vec_isize("testfiles/dispu.h2o.txt"));
    // the order doesn't matter, so let rust sort both
    want.sort();
    assert_eq!(got, want);
}

#[test]
fn test_disps_with_checks() {
    let got = Taylor::new(
        5,
        9,
        Some(Checks([[5, 7], [8, 8], [9, 9]])),
        Some(Checks([[5, 7], [8, 8], [9, 9]])),
    )
    .disps();
    let mut want = Disps(load_vec_isize("testfiles/dispu.c3h2.mod.txt"));
    want.sort();
    assert_eq!(got, want);
}

#[test]
fn test_disps_with_zero_checks() {
    let got = Taylor::new(
        5,
        3,
        Some(Checks([[3, 3], [0, 0], [0, 0]])),
        Some(Checks([[3, 3], [0, 0], [0, 0]])),
    )
    .disps();
    let mut want = Disps(load_vec_isize("testfiles/dispu.h2o.mod.txt"));
    want.sort();
    assert_eq!(got, want);
}
