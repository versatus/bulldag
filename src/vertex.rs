use crate::edge::Edge;
use crate::index::Index;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::Debug;

pub type Edges<T, Ix> = Vec<(Vertex<T, Ix>, Vertex<T, Ix>)>;
pub enum Direction {
    Source,
    Reference,
}

/// The vertex is the "data" component of a graph, containing the
/// data the graph represents. Graphs, and DAGs in particular can be
/// used for a variety of different purposes, whether it is simply to
/// track causal dependencies, as a task scheduler, to track data
/// etc. Graphs consist of Vertices and Edges. Edges are the representation
/// of the connection between two vertices in a graph
/// Every vertex must have an edge. A vertex, within an edge can either
/// be a source or a reference. Vertices must also contain some "identifying"
/// information, in this implementation the `index` field is the the vertex's
/// identifying information, and within the graph is the key for O(1) lookups
/// and inserts.
///
/// Example
/// ```
/// use bulldag::vertex::Vertex;
/// let vertex: Vertex<usize, &str> = Vertex::new(5, "source");
/// println!("{:?}", vertex);
/// ```
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vertex<T, Ix>
where
    T: Clone + Debug,
    Ix: Index + Debug,
{
    data: T,
    sources: HashSet<Ix>,
    references: HashSet<Ix>,
    index: Ix,
}

impl<T, Ix> Vertex<T, Ix>
where
    T: Clone + Debug,
    Ix: Index + Debug,
{
    /// Creates a new Vertex, needs data and an index.
    ///
    /// Example
    /// ```
    /// use bulldag::vertex::Vertex;
    /// let vertex: Vertex<usize, &str> = Vertex::new(5, "source");
    /// println!("{:?}", vertex);
    /// ```
    pub fn new(data: T, index: Ix) -> Vertex<T, Ix> {
        Vertex {
            data,
            sources: HashSet::new(),
            references: HashSet::new(),
            index,
        }
    }

    /// Add a source to the vertex
    fn add_source(&mut self, source: Ix) {
        self.sources.insert(source);
    }

    /// Add a reference to the vertex
    fn add_reference(&mut self, reference: Ix) {
        self.references.insert(reference);
    }

    /// Add an edge (source or reference) to the vertex.
    /// Checks whether or not the edge source index matches
    /// the local index or if the edge reference index
    /// matches the local vertex index, and then adds the proper edge
    /// ```
    /// use bulldag::vertex::Vertex;
    /// use bulldag::edge::Edge;
    /// let mut vertex: Vertex<usize, &str> = Vertex::new(5, "source");
    /// let edge1: Edge<&str> = Edge::new("source", "reference");
    /// let edge2: Edge<&str> = Edge::new("sources_source", "source");
    /// vertex.add_edge(&edge1);
    /// println!("{:?}", vertex);
    /// assert!(vertex.n_references() == 1);
    /// vertex.add_edge(&edge2);
    /// println!("{:?}", vertex);
    /// assert!(vertex.n_sources() == 1);
    /// ```
    pub fn add_edge(&mut self, edge: &Edge<Ix>) {
        if edge.get_source() == self.index {
            self.add_reference(edge.get_reference());
        }

        if edge.get_reference() == self.index {
            self.add_source(edge.get_source());
        }
    }

    /// Get the data from the Vertex
    /// ```
    /// use bulldag::vertex::Vertex;
    /// let vertex: Vertex<usize, &str> = Vertex::new(5, "source");
    /// println!("{:?}", vertex.get_data());
    /// assert!(vertex.get_data() == 5usize);
    /// ```
    pub fn get_data(&self) -> T {
        self.data.clone()
    }

    /// Get the index from the Vertex
    /// ```
    /// use bulldag::vertex::Vertex;
    /// let vertex: Vertex<usize, &str> = Vertex::new(5, "source");
    /// println!("{:?}", vertex.get_index());
    /// assert!(vertex.get_index() == "source");
    /// ```
    pub fn get_index(&self) -> Ix {
        self.index.clone()
    }

    /// Get the sources for the current vertex
    /// ```
    /// use bulldag::vertex::Vertex;
    /// use bulldag::edge::Edge;
    /// let mut vertex: Vertex<usize, &str> = Vertex::new(5, "source");
    /// let edge: Edge<&str> = Edge::new("sources_source", "source");
    /// vertex.add_edge(&edge);
    /// println!("{:?}", vertex);
    /// assert!(vertex.get_sources().len() == 1);
    /// ```
    pub fn get_sources(&self) -> Vec<&Ix> {
        self.sources.iter().collect()
    }

    /// Get the references for the current vertex
    /// ```
    /// use bulldag::vertex::Vertex;
    /// use bulldag::edge::Edge;
    /// let mut vertex: Vertex<usize, &str> = Vertex::new(5, "source");
    /// let edge: Edge<&str> = Edge::new("source", "reference");
    /// vertex.add_edge(&edge);
    /// println!("{:?}", vertex);
    /// assert!(vertex.get_references().len() == 1);
    /// ```
    pub fn get_references(&self) -> Vec<&Ix> {
        self.references.iter().collect()
    }

    pub fn is_reference(&self, target: &Ix) -> bool {
        self.references.contains(target)
    }

    pub fn is_source(&self, target: &Ix) -> bool {
        self.sources.contains(target)
    }

    /// Get the number of source for the current vertex
    /// ```
    /// use bulldag::vertex::Vertex;
    /// use bulldag::edge::Edge;
    /// let mut vertex: Vertex<usize, &str> = Vertex::new(5, "source");
    /// let edge: Edge<&str> = Edge::new("sources_source", "source");
    /// vertex.add_edge(&edge);
    /// println!("{:?}", vertex);
    /// assert!(vertex.n_sources() == 1);
    /// ```
    pub fn n_sources(&self) -> usize {
        self.sources.len()
    }

    /// Get the number of references for the current vertex
    /// ```
    /// use bulldag::vertex::Vertex;
    /// use bulldag::edge::Edge;
    /// let mut vertex: Vertex<usize, &str> = Vertex::new(5, "source");
    /// let edge: Edge<&str> = Edge::new("sources_source", "source");
    /// vertex.add_edge(&edge);
    /// println!("{:?}", vertex);
    /// assert!(vertex.n_sources() == 1);
    /// ```
    pub fn n_references(&self) -> usize {
        self.references.len()
    }
}

/// Convert a tuple of two [`Vertex`]s into an [`Edge`].
/// Source is the first item, reference the second item.
impl<T, Ix> From<(Vertex<T, Ix>, Vertex<T, Ix>)> for Edge<Ix>
where
    T: Clone + Debug,
    Ix: Index + Debug,
{
    fn from(item: (Vertex<T, Ix>, Vertex<T, Ix>)) -> Self {
        let source: Ix = item.0.get_index();
        let reference: Ix = item.1.get_index();
        Edge::new(source, reference)
    }
}

/// Convert a tuple of two [`Vertex`]s into an [`Edge`].
/// Source is the first item, reference the second item.
impl<T, Ix> From<&(&Vertex<T, Ix>, &Vertex<T, Ix>)> for Edge<Ix>
where
    T: Clone + Debug,
    Ix: Index + Debug,
{
    fn from(item: &(&Vertex<T, Ix>, &Vertex<T, Ix>)) -> Self {
        let source: Ix = item.0.get_index();
        let reference: Ix = item.1.get_index();
        Edge::new(source, reference)
    }
}

/// Convert a tuple of two [`Vertex`]s into an [`Edge`].
/// Source is the first item, reference the second item.
impl<T, Ix> From<(&mut Vertex<T, Ix>, &mut Vertex<T, Ix>)> for Edge<Ix>
where
    T: Clone + Debug,
    Ix: Index + Debug,
{
    fn from(item: (&mut Vertex<T, Ix>, &mut Vertex<T, Ix>)) -> Self {
        let source: Ix = item.0.get_index();
        let reference: Ix = item.1.get_index();
        Edge::new(source, reference)
    }
}

impl<T, Ix> From<Vertex<T, Ix>> for (Ix, Vertex<T, Ix>)
where
    T: Clone + Debug,
    Ix: Index + Debug,
{
    fn from(item: Vertex<T, Ix>) -> Self {
        (item.get_index().clone(), item.clone())
    }
}
