use std::hash::{Hash, Hasher};


#[derive(Debug, Clone)]
pub(crate) struct Bond{
    pub(crate) i: usize,  // left_atom_index
    pub(crate) j: usize   // right_atom_index
}

impl PartialEq for Bond{

    /// Is this bond idential to another?
    fn eq(&self, other: &Self) -> bool {
        (self.i == other.i && self.j == other.j) || (self.i == other.j && self.j == other.i)
    }
}

impl Eq for Bond {}

impl Hash for Bond {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Vec::from([self.i, self.j]).sort().hash(state);
    }
}
