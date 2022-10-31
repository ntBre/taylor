// could I use const generics for these for m and n?
pub use checks::*;
pub mod checks;

/// a taylor series expansion of f(x1, x2, ... n) of order m-1
#[derive(Clone, Debug)]
pub struct Taylor {
    pub forces: Vec<Vec<usize>>,
}

impl Taylor {
    /// returns the directly-derived Cartesian product row, where index is the
    /// desired row index, n is the truncation order and m is the number of
    /// variables in the Taylor series expansion. This corresponds to Algorithm
    /// 3 from Thackston18 with the meanings of n and m reversed to actually
    /// work
    fn row(index: usize, n: usize, m: usize) -> Vec<usize> {
        let mut index = index;
        let mut ret = Vec::new();
        for i in (0..n).rev() {
            let ni = m.pow(i as u32);
            let di = index / ni;
            ret.push(di);
            index -= di * ni;
        }
        ret
    }

    /// takes an invalid row of the Cartesian product, the number of variables
    /// n, and the truncation order m and returns the index of the next valid
    /// row. This corresponds to Algorithm 4 in Thackston18
    fn next_row(row: Vec<usize>, n: usize, m: usize) -> usize {
        let mut row = row;
        for i in (0..n).rev() {
            if row[i] > 0 {
                row[i] = 0;
                if i > 0 {
                    row[i - 1] += 1;
                }
                break;
            }
        }
        let mut index = 0;
        let lr = row.len();
        for i in (0..n).rev() {
            index += row[lr - i - 1] * m.pow(i as u32);
        }
        index
    }

    pub fn new(
        m: usize,
        n: usize,
        modchecks: Option<Checks>,
        eqchecks: Option<Checks>,
    ) -> Self {
        let last_index = m.pow(n as u32);
        let mut forces = Vec::new();
        let mut i = 0;
        while i < last_index {
            let row = Self::row(i, n, m);
            let s: usize = row.iter().sum();
            if s < m {
                let mc = if let Some(checks) = &modchecks {
                    checks.mod_check(&row)
                } else {
                    true
                };
                let ec = if let Some(checks) = &eqchecks {
                    checks.eq_check(&row)
                } else {
                    true
                };
                if (modchecks.is_none() && !ec)
                    || (eqchecks.is_none() && !mc)
                    || (!ec && !mc)
                {
                    i += 1;
                    continue;
                }
                forces.push(row);
                i += 1;
            } else {
                i = Self::next_row(row, n, m);
            }
        }
        Self { forces }
    }

    /// CartProd returns the Cartesian product of the elements in prods.
    /// Implementation adapted from
    /// https://docs.python.org/3/library/itertools.html#itertools.product
    fn cart_prod(pools: Vec<Vec<isize>>) -> Vec<Vec<isize>> {
        let mut result = vec![vec![]];
        for pool in pools {
            let mut tmp = Vec::new();
            for x in result {
                for y in &pool {
                    let l = x.len() + 1;
                    let mut a = x.clone();
                    a.resize(l, 0);
                    a[l - 1] = *y;
                    tmp.push(a);
                }
            }
            result = tmp;
        }
        result
    }

    /// return the displacements associated with the expansion described by
    /// `self`
    pub fn disps(&self) -> Vec<Vec<isize>> {
        let mut disps = Vec::new();
        for row in &self.forces {
            let mut indices = Vec::new();
            let mut values = Vec::new();
            for (i, digit) in row.iter().enumerate() {
                if *digit != 0 {
                    indices.push(i);
                    values.push(digit);
                }
            }
            if values.is_empty() {
                disps.push(row.iter().map(|u| *u as isize).collect());
                continue;
            }
            let mut prods = Vec::new();
            for digit in values {
                let digit = *digit as isize;
                let mut tmp = Vec::new();
                for j in (-digit..=digit).step_by(2) {
                    tmp.push(j);
                }
                prods.push(tmp);
            }
            let new_rows = Self::cart_prod(prods);
            for nrow in new_rows {
                let mut r: Vec<_> = row.iter().map(|u| *u as isize).collect();
                for (i, index) in indices.iter().enumerate() {
                    r[*index] = nrow[i];
                }
                disps.push(r);
            }
        }
        // sort -u on disps
        disps.sort();
        disps.dedup();
        disps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_vec_isize(filename: &str) -> Vec<Vec<isize>> {
        let mut ret = Vec::new();
        let contents = std::fs::read_to_string(filename).unwrap();
        let lines = contents.lines();
        for line in lines {
            ret.push(line.split(",").map(|s| s.parse().unwrap()).collect());
        }
        ret
    }

    fn load_vec_usize(filename: &str) -> Vec<Vec<usize>> {
        let mut ret = Vec::new();
        let contents = std::fs::read_to_string(filename).unwrap();
        let lines = contents.lines();
        for line in lines {
            ret.push(line.split(",").map(|s| s.parse().unwrap()).collect());
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
        let mut want = load_vec_isize("testfiles/dispu.h2o.txt");
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
        let mut want = load_vec_isize("testfiles/dispu.c3h2.mod.txt");
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
        let mut want = load_vec_isize("testfiles/dispu.h2o.mod.txt");
        want.sort();
        assert_eq!(got, want);
    }
}
