use intder::Intder;
use symm::{Atom, Molecule};
use taylor::Taylor;

// both of these can probably be clap flags
const SYMM_EPS: f64 = 1e-6;
const STEP_SIZE: f64 = 0.005;

// this is pieced together from parts of pbqff, but it's not clear how to reuse
// any of the parts
fn main() -> std::io::Result<()> {
    let infile = if let Some(infile) = std::env::args().nth(1) {
        infile
    } else {
        eprintln!("usage: taylor INFILE");
        return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
    };
    let mut intder = Intder::load_file(&infile);
    let pairs = intder.geom.0.iter().zip(&intder.atoms);
    let mut atoms = Vec::new();
    for (g, a) in pairs {
        atoms.push(Atom::new_from_label(&a.label, g[0], g[1], g[2]));
    }
    let mol = {
        let mut mol = Molecule::new(atoms);
        mol.normalize();
        mol
    };
    let pg = mol.point_group_approx(SYMM_EPS);

    println!("Normalized Geometry:\n{:20.12}", mol);
    println!("Point Group = {}", pg);

    let nsic = intder.symmetry_internals.len();
    // generate a displacement for each SIC
    let mut disps = Vec::new();
    for i in 0..nsic {
        let mut disp = vec![0.0; nsic];
        disp[i] = STEP_SIZE;
        disps.push(disp);
    }
    intder.disps = disps;
    intder.geom = mol.clone().into();
    let disps = intder.convert_disps();

    let atomic_numbers = mol.atomic_numbers();
    let mut irreps = Vec::new();
    for (i, disp) in disps.iter().enumerate() {
        let disp = disp.as_slice();
        let m = Molecule::from_slices(atomic_numbers.clone(), disp);
        let irrep = match m.irrep_approx(&pg, SYMM_EPS) {
            Ok(rep) => rep,
            Err(e) => panic!("failed on coord {} with {}", i, e.msg()),
        };
        irreps.push((i, irrep));
    }
    // sort by irrep symmetry
    irreps.sort_by_key(|k| k.1);

    let just_irreps: Vec<_> = irreps.iter().map(|s| s.1).collect();

    let mut new_sics = Vec::new();
    for irrep in &irreps {
        new_sics.push(intder.symmetry_internals[irrep.0].clone());
    }
    intder.symmetry_internals = new_sics;

    println!("\nSymmetry Internal Coordinates:");
    intder.print_sics(&mut std::io::stdout(), &just_irreps);

    // generate checks
    let checks = Taylor::make_checks(irreps, &pg);
    // run taylor.py to get fcs and disps
    let taylor = Taylor::new(5, nsic, checks.0, checks.1);
    let taylor_disps = taylor.disps();

    intder.disps = taylor_disps.to_intder(STEP_SIZE);

    let mut f = std::fs::File::create("intder.in")?;
    use std::io::Write;
    writeln!(f, "{}", intder)?;

    let anpass = taylor.to_anpass(
        &taylor_disps,
        &vec![0.0; taylor_disps.len()],
        STEP_SIZE,
    );
    let mut f = std::fs::File::create("anpass.in")?;
    writeln!(f, "{}", anpass)?;

    Ok(())
}
