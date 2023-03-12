use crate::molecule::Molecule;

use crate::position::Position;
use crate::traits::IsVeryClose;


#[derive(Default, Debug, Clone)]
pub(crate) struct Atom{
    pub(crate) atomic_number: u16,
    pub(crate) position: Position
}

impl Atom {

    /// Construct an atom from a .xyz file line e.g "H  0.0  1.0  2.0"
    pub(crate) fn from_line(line: &str) -> Self{

        let mut atom = Atom::default();

        for (i, item) in line.split_whitespace().enumerate(){
            match i {
               0 => {
                   let index = ELEMENTS.iter().position(|&e| e == item).unwrap();
                   atom.atomic_number = (index as u16) + 1;
               }
               1 => {
                   atom.position.x = item.parse::<f32>().expect("Failed to x parse as float");
               }
               2 => {
                   atom.position.y = item.parse::<f32>().expect("Failed to y parse as float");
               }
               3 => {
                   atom.position.z = item.parse::<f32>().expect("Failed to z parse as float");
               }
               _ => {}  // Do nothing with extra columns
           }
        }
        atom
    }

    /// Covalent radius, used to determine if two atoms are bonded
    pub fn covalent_radius(&self) -> f32{

        let radius = COVALENT_RADII_PICOMETERS.get(self.index());
        match radius {
            None =>  2.0,
            Some(r) => r.clone() * PICOMETERS_TO_ANGSTROMS
        }
    }

    /// Obtain the maximal possible valence for this atom
    pub fn maximal_valence(&self) -> usize{

        match MAXIMAL_VALENCIES.get(self.index()) {
            None => 6,
            Some(v) => v.clone()
        }
    }

    /// Could this atom be bonded to another atom, based on the distance
    /// between them and their respective covalent radii?
    pub fn could_be_bonded_to(&self, atom: &Atom) -> bool{

        let tolerance: f32 = 1.3;  // Relative tolerance on whether a bond could be present

        let r = self.distance_to(&atom);
        let is_identical_atom = r < 1E-8;

        !is_identical_atom && r < tolerance * (self.covalent_radius() + atom.covalent_radius())
    }

    /// Calculate the distance to another atom, in Å
    pub fn distance_to(&self, other: &Atom) -> f32 {
        self.position.distance_to(&other.position)
    }

    /// Retrieve the atomic neighbours to this one in a molecule
    pub fn neighbours_in(&self, molecule: &Molecule) -> Vec<Neighbour>{

        let mut neighbours: Vec<Neighbour> = Vec::new();
        for (i, atom) in molecule.atoms.iter().enumerate(){
            if self.could_be_bonded_to(atom){
                let distance = self.distance_to(&atom);
                neighbours.push(Neighbour{distance, idx: i});
            }
        }
        neighbours
    }

    /// Get the color of this atom
    pub(crate) fn color(&self) -> RGB{
        match COLORS.get(self.index()) {
            None => DEFAULT_COLOR.clone(),
            Some(c) => c.clone()
        }
    }

    /// Index of this atom in the periodic table
    fn index(&self) -> usize {(self.atomic_number - 1) as usize}
}

impl PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        self.atomic_number == other.atomic_number && self.position.is_very_close_to(&other.position)
    }
}

pub(crate) struct Neighbour{
    pub(crate) distance: f32,
    pub(crate) idx: usize
}

#[derive(Clone)]
pub(crate) struct RGB(pub(crate) u8, pub(crate) u8, pub(crate) u8);

static ELEMENTS: [&'static str; 118] = [
    "H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne", "Na", "Mg",
    "Al", "Si", "P", "S", "Cl", "Ar", "K", "Ca", "Sc", "Ti", "V", "Cr",
    "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ga", "Ge", "As", "Se", "Br",
    "Kr", "Rb", "Sr", "Y", "Zr", "Nb", "Mo", "Tc", "Ru", "Rh", "Pd",
    "Ag", "Cd", "In", "Sn", "Sb", "Te", "I", "Xe", "Cs", "Ba", "La",
    "Ce", "Pr", "Nd", "Pm", "Sm", "Eu", "Gd", "Tb", "Dy", "Ho", "Er",
    "Tm", "Yb", "Lu", "Hf", "Ta", "W", "Re", "Os", "Ir", "Pt", "Au",
    "Hg", "Tl", "Pb", "Bi", "Po", "At", "Rn", "Fr", "Ra", "Ac", "Th",
    "Pa", "U", "Np", "Pu", "Am", "Cm", "Bk", "Cf", "Es", "Fm", "Md",
    "No", "Lr", "Rf", "Db", "Sg", "Bh", "Hs", "Mt", "Ds", "Rg", "Cn",
    "Nh", "Fl", "Mc", "Lv", "Ts", "Og"
];

static PICOMETERS_TO_ANGSTROMS: f32 = 0.01;

static COVALENT_RADII_PICOMETERS: [f32; 86] = [
    31.,                                                                                                   28.,
    128., 96.,                                                               84.,  76.,  71.,  66.,  57.,  58.,
    166., 141.,                                                             121., 111., 107., 105., 102., 106.,
    102., 203., 176., 170., 160., 153., 139., 161., 152., 150., 124., 132., 122., 122., 120., 119., 120., 116.,
    220., 195., 190., 175., 164., 154., 147., 146., 142., 139., 145., 144., 142., 139., 139., 138., 139., 140.,
    244., 215.,
    207., 204., 203., 201., 199., 198., 198., 196., 194., 192., 192., 189., 190., 187.,
    175., 187., 170., 162., 151., 144., 141., 136., 136., 132., 145., 146., 148., 140., 150., 150.
];

static MAXIMAL_VALENCIES: [usize; 38] = [
    1, 0, 1, 2, 3, 4, 5, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7, 7, 5, 4, 4, 6, 3, 4, 5, 6, 7, 2, 1, 2];

static DEFAULT_COLOR: RGB = RGB(252, 252, 252);

static COLORS: [RGB; 17] = [
    RGB(191, 191, 191),  // H
    RGB(208, 255, 255), // He
    RGB(217, 123, 255), // Li
    RGB(176, 255, 000), // Be
    RGB(255, 178, 179), // B
    RGB(102, 102, 102), // C
    RGB(012, 012, 255), // N
    RGB(255, 000, 000), // O
    RGB(112, 181, 255), // F
    RGB(166, 229, 248), // Ne
    RGB(183, 088, 251), // Na
    RGB(082, 255, 000), // Mg
    RGB(196, 165, 165), // Al
    RGB(121, 154, 153), // Si
    RGB(255, 119, 000), // P
    RGB(179, 179, 000), // S
    RGB(000, 244, 000), // Cl
];


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
    use crate::atom::Atom;
    use crate::position::Position;

    #[test]
    fn test_atom_init_from_line(){
        let expected_atom = Atom{atomic_number: 1, position: Position{x: 0.0, y: 1.0, z: 2.0}};
        assert_eq!(Atom::from_line("H  0.0  1.0  2.0"), expected_atom);
        assert_eq!(Atom::from_line("H  0.0000  1.00  2.0"), expected_atom);
        assert_eq!(Atom::from_line("H  0.0000     1.00 2.0"), expected_atom);
    }
}
