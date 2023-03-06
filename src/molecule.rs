use std::collections::HashSet;
use crate::atom::Atom;
use crate::bond::Bond;


#[derive(Default, PartialEq, Debug)]
pub(crate) struct Molecule{
    pub(crate) atoms: Vec<Atom>,
    pub(crate) bonds: HashSet<Bond>
}

impl Molecule {

    /// Construct a molecule form a set of xyz file lines
    pub(crate) fn from_xyz_file(file_lines: &str) -> Self{

        let file_lines_stripped = file_lines.replace("\r", "");
        let lines = file_lines_stripped.split("\n");
        let mut molecule = Molecule::default();
        let mut n_atoms: usize = 0;

        for (i, line) in lines.enumerate() {
            if i == 0 {
                n_atoms = line.parse::<usize>()
                    .expect("Failed to parse the first line as an integer");
                continue;
            }
            else if i == 1 { // Skip the title line
                continue;
            }
            else if line.trim().is_empty(){ // Skip blank lines
                continue;
            }
            molecule.atoms.push(Atom::from_line(line));
        }

        if n_atoms != molecule.atoms.len() {
            panic!("Number or atoms declared not equal to that provided")
        }

        molecule.set_bonds();
        molecule
    }

    /// Set the bonds between atoms in this molecule based on distance and possible valiences
    fn set_bonds(&mut self){

        for (i, atom_i) in self.atoms.iter().enumerate(){
            let mut neighbours = atom_i.neighbours_in(self);
            neighbours.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

            for j in 0..atom_i.maximal_valence(){
                match neighbours.get(j) {
                    None => {}, // Do nothing
                    Some(n) => {self.bonds.insert(Bond{i, j: n.idx});}
                }
            }

        }
    }
}


/*
   /$$                           /$$
  | $$                          | $$
 /$$$$$$    /$$$$$$   /$$$$$$$ /$$$$$$   /$$$$$$$
|_  $$_/   /$$__  $$ /$$_____/|_  $$_/  /$$_____/
  | $$    | $$$$$$$$|  $$$$$$   | $$   |  $$$$$$
  | $$ /$$| $$_____/ \____  $$  | $$ /$$\____  $$
  |  $$$$/|  $$$$$$$ /$$$$$$$/  |  $$$$//$$$$$$$/
   \___/   \_______/|_______/    \___/ |_______/
 */

#[cfg(test)]
mod tests{
    use std::collections::HashSet;
    use crate::molecule::Molecule;
    use crate::atom::Atom;
    use crate::position::Position;

    #[test]
    fn test_atom_init_from_line(){
        let expected_molecule = Molecule{
            atoms: vec![
                Atom{atomic_number: 1, position: Position{x: 0.0, y: 1.0, z: 2.0}},
                Atom{atomic_number: 6, position: Position{x: 5.0, y: 1.1, z: 0.0}},
            ],
            bonds: HashSet::new()
        };
        assert_eq!(
            Molecule::from_xyz_file("2\n\nH  0.0  1.0  2.0\nC 5.0  1.1 0.0\n\n"),
            expected_molecule
        );
    }

    #[test]
    fn test_dos_and_unix_line_returns_are_parsed(){
        assert_eq!(
            Molecule::from_xyz_file("1\n\nH  0.0  1.0  2.0"),
            Molecule::from_xyz_file("1\r\n\r\nH  0.0  1.0  2.0"),
        );
    }
}
