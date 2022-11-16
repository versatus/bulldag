use crate::index::Index;
use std::fmt::Debug;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Edge<Ix: Index + Debug> {
    source: Ix,
    reference: Ix,
}

impl<Ix: Index + Debug> Edge<Ix> {
    pub fn new(source: Ix, reference: Ix) -> Edge<Ix> {
        Edge {
            source,
            reference
        }
    }

    pub fn get_reference(&self) -> Ix {
        self.reference.clone()
    }

    pub fn get_source(&self) -> Ix {
        self.source.clone()
    }
}

