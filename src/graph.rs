use crate::edge::Edge;
use crate::index::Index;
use crate::vertex::{Direction, Vertex};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

/// A basic error enum with different potential error types and a tuple
/// variant for one-off and less predicatble error types
#[derive(Debug)]
pub enum GraphError {
    WouldCycle,
    NonExistentSource,
    NonExistentReference,
    NonExistentVertex,
    NoEdges,
    Other(String),
}

#[derive(Debug)]
pub enum GraphOk<Ix: Index + Debug> {
    Ok,
    VecRes(Vec<Ix>),
}

/// Custom Type representing a Result specific to the graph
pub type GraphResult<Ix> = Result<GraphOk<Ix>, GraphError>;

/// The core DAG graph structure, contains a hashmap of vertices
/// with the key being the vertex's index, and the value being the
/// vertex itself, and a vector of all the edges in the graph.
///
/// Example
///
/// ```
/// use bulldag::graph::BullDag;
///
/// let graph: BullDag<usize, &str> = BullDag::new();
/// println!("{:?}", graph);
/// assert!(graph.len() == 0);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BullDag<T: Clone + Debug, Ix: Index + Debug> {
    roots: HashSet<Ix>,
    leaves: HashSet<Ix>,
    vertices: HashMap<Ix, Vertex<T, Ix>>,
    edges: HashSet<Edge<Ix>>,
}

impl<T, Ix> Default for BullDag<T, Ix>
where
    T: Clone + Debug,
    Ix: Index + Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, Ix> BullDag<T, Ix>
where
    T: Clone + Debug,
    Ix: Index + Debug,
{
    /// Creates a new BullDag
    ///
    /// Example:
    /// ```
    /// use bulldag::graph::BullDag;
    ///
    /// let mut graph: BullDag<usize, &str> = BullDag::new();
    /// println!("{:?}", graph);
    /// assert!(graph.len() == 0);
    /// ```
    pub fn new() -> BullDag<T, Ix> {
        BullDag {
            roots: HashSet::new(),
            leaves: HashSet::new(),
            vertices: HashMap::new(),
            edges: HashSet::new(),
        }
    }

    /// Adds a root to the roots set, roots are vertices with no sources
    fn add_root(&mut self, index: Ix) {
        self.roots.insert(index);
    }

    /// Cleans the roots set when a vertex adds an edge where the vertex
    /// is a reference.
    fn clean_root(&mut self, index: Ix) {
        if self.roots.contains(&index) {
            if let Some(vtx) = self.vertices.get(&index) {
                if !vtx.get_sources().is_empty() {
                    self.roots.remove(&index);
                }
            }
        }
    }

    /// Get the root set
    pub fn get_roots(&self) -> HashSet<Ix> {
        self.roots.clone()
    }

    pub fn n_roots(&self) -> usize {
        self.roots.len()
    }

    /// Adds a leaf to the leaves set, leaves are vertices with no references
    fn add_leaf(&mut self, index: Ix) {
        self.leaves.insert(index);
    }

    /// Cleans the leaves set when a vertex adds an edge where the vertex is
    /// as source.
    fn clean_leaf(&mut self, index: Ix) {
        if self.leaves.contains(&index) {
            if let Some(vtx) = self.vertices.get(&index) {
                if !vtx.get_references().is_empty() {
                    self.leaves.remove(&index);
                }
            }
        }
    }

    /// Get the leaf set
    pub fn get_leaves(&self) -> HashSet<Ix> {
        self.leaves.clone()
    }

    pub fn n_leaves(&self) -> usize {
        self.leaves.len()
    }

    /// Adds an edge to the graph, and to the vertices
    ///
    /// Example:
    /// ```
    /// use bulldag::graph::BullDag;
    /// use bulldag::vertex::Vertex;
    /// use bulldag::edge::Edge;
    ///
    /// let mut graph: BullDag<usize, &str> = BullDag::new();
    /// let mut v1: Vertex<usize, &str> = Vertex::new(5, "source");
    /// let mut v2: Vertex<usize, &str> = Vertex::new(4, "reference");
    /// graph.add_edge(&(&v1, &v2));
    /// println!("{:?}", graph);
    /// assert!(graph.n_edges() == 1);
    /// ```
    pub fn add_edge(&mut self, edge: &(&Vertex<T, Ix>, &Vertex<T, Ix>)) {
        let mut source = edge.0.clone();
        let mut reference = edge.1.clone();
        let e: Edge<Ix> = edge.into();

        source.add_edge(&e);
        reference.add_edge(&e);

        if self.check_cycles(edge).is_ok() {
            // Check if the vertex already exists, if so, get a mutable reference
            // to it, so that you can add this new edge to its `references` store
            // since we are adding a reference to this vertex, check if it was
            // previously a `leaf`, i.e. a node with no references.
            // if it was, remove it as it no longer is a `leaf`, it now contains
            // a reference.
            //
            // If the vertex does not already exist, add the edge and add the
            // vertex, the `add_vertex` method will handle the rest.
            if let Some(vtx) = self.get_vertex(source.get_index()) {
                let mut updated_vtx = vtx.clone();
                updated_vtx.add_edge(&e);
                self.add_vertex(&updated_vtx);
                self.clean_leaf(updated_vtx.get_index());
            } else {
                self.add_vertex(&source);
            }

            // Check if the vertex already exists, if so, get a mutable reference
            // to it, so that you can add this new edge to its `sources` store
            // since we are adding a source to this vertex, check if it was
            // previously a `root`, i.e. a node with no sources.
            // if it was, remove it as it is no longer a `root`, it now contains
            // a source.
            //
            // If the vertex does not already exist, add the edge and add the
            // vertex, the `add_vertex` method will handle the rest.
            if let Some(vtx) = self.get_vertex(reference.get_index()) {
                let mut updated_vtx = vtx.clone();
                updated_vtx.add_edge(&e);
                self.add_vertex(&updated_vtx);
                self.clean_root(updated_vtx.get_index());
            } else {
                self.add_vertex(&reference);
            }

            self.edges.insert(e.clone());
        }
    }

    /// Batch add edges (and vertices)
    ///
    /// Example:
    ///
    /// ```
    /// use bulldag::graph::BullDag;
    /// use bulldag::vertex::Vertex;
    /// use bulldag::edge::Edge;
    ///
    /// let mut graph: BullDag<usize, &str> = BullDag::new();
    /// let mut v1: Vertex<usize, &str> = Vertex::new(5, "source");
    /// let mut v2: Vertex<usize, &str> = Vertex::new(4, "reference_1");
    /// let mut v3: Vertex<usize, &str> = Vertex::new(3, "reference_2");
    ///
    /// let edges = vec![(&v1, &v2), (&v1, &v3)];
    /// graph.extend_from_edges(&edges);
    /// assert!(graph.len() == 3);
    /// assert!(graph.n_roots() == 1);
    /// assert!(graph.n_leaves() == 2);
    /// ```
    #[allow(clippy::type_complexity)]
    pub fn extend_from_edges(&mut self, edges: &[(&Vertex<T, Ix>, &Vertex<T, Ix>)]) {
        edges.iter().for_each(|e| {
            let source = e.0.get_index();
            let reference = e.1.get_index();
            if let Some(src) = self.get_vertex(source) {
                let edge: Edge<Ix> = e.into();
                let mut src = src.clone();
                src.add_edge(&edge);
            } else {
                let edge: Edge<Ix> = e.into();
                let mut src = e.0.clone();
                src.add_edge(&edge);
            }

            if let Some(r) = self.get_vertex(reference) {
                let edge: Edge<Ix> = e.into();
                let mut r = r.clone();
                r.add_edge(&edge);
            } else {
                let edge = e.into();
                let mut r = e.1.clone();
                r.add_edge(&edge);
            }
            self.add_edge(e);
        });
    }

    /// Adds a single vertex to the graph
    pub fn add_vertex(&mut self, vertex: &Vertex<T, Ix>) {
        if vertex.get_sources().is_empty() {
            self.add_root(vertex.get_index());
        }

        if vertex.get_references().is_empty() {
            self.add_leaf(vertex.get_index());
        }

        self.vertices.insert(vertex.get_index(), vertex.clone());
    }

    /// Gets the vertex at key `target`
    pub fn get_vertex(&self, target: Ix) -> Option<&Vertex<T, Ix>> {
        self.vertices.get(&target)
    }

    pub fn get_vertex_mut(&mut self, target: Ix) -> Option<&mut Vertex<T, Ix>> {
        self.vertices.get_mut(&target)
    }

    pub fn add_vertices(&mut self, vertices: &[Vertex<T, Ix>]) {
        vertices.iter().for_each(|v| {
            self.add_vertex(v);
        });
    }

    /// Returns the number of vertices in the graph as usize
    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of edges in the graph as a usize
    pub fn n_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn trace(&self, target: &Vertex<T, Ix>, direction: Direction) -> Vec<Ix> {
        let mut stack = vec![];
        match direction {
            Direction::Source => {
                self.get_sources(target, &mut stack);
            }
            Direction::Reference => {
                self.get_references(target, &mut stack);
            }
        }

        stack
    }

    fn get_sources(&self, target: &Vertex<T, Ix>, stack: &mut Vec<Ix>) {
        let mut edges = self.edges.clone();
        edges.retain(|e| e.get_reference() == target.get_index());
        let sources: Vec<Ix> = edges.iter().map(|e| e.get_source()).collect();

        if !sources.is_empty() {
            for source in sources {
                if let Some(vtx) = self.get_vertex(source.clone()) {
                    self.get_sources(vtx, stack);
                }
            }
        }

        if !stack.contains(&target.get_index()) {
            stack.push(target.get_index());
        }
    }

    fn get_references(&self, target: &Vertex<T, Ix>, stack: &mut Vec<Ix>) {
        let mut edges = self.edges.clone();
        edges.retain(|e| e.get_source() == target.get_index());
        let references: Vec<Ix> = edges.iter().map(|e| e.get_reference()).collect();

        if !references.is_empty() {
            for reference in references {
                if let Some(vtx) = self.get_vertex(reference.clone()) {
                    self.get_references(vtx, stack);
                }
            }
        }

        if !stack.contains(&target.get_index()) {
            stack.push(target.get_index());
        }
    }

    fn auto_source_cycle(&self) -> bool {
        self.n_roots() == 0 && !self.is_empty()
    }

    fn auto_ref_cycle(&self) -> bool {
        self.n_leaves() == 0 && !self.is_empty()
    }

    /// Checks whether the given edge would cause a cycle
    fn check_cycles(&self, edge: &(&Vertex<T, Ix>, &Vertex<T, Ix>)) -> GraphResult<Ix> {
        if self.auto_source_cycle() || self.auto_ref_cycle() {
            return Err(GraphError::WouldCycle);
        }

        let source_trace = self.trace(edge.0, Direction::Source);
        if source_trace.contains(&edge.1.get_index()) {
            return Err(GraphError::WouldCycle);
        }

        let ref_trace = self.trace(edge.1, Direction::Reference);
        if ref_trace.contains(&edge.0.get_index()) {
            return Err(GraphError::WouldCycle);
        }

        Ok(GraphOk::Ok)
    }

    #[cfg(test)]
    pub(crate) fn topological_sort(&self) -> GraphResult<Ix> {
        let roots = self.get_roots();
        let leaves = self.get_leaves();

        if roots.is_empty() {
            return Err(GraphError::WouldCycle);
        }

        if leaves.is_empty() {
            return Err(GraphError::WouldCycle);
        }

        let mut stack: Vec<Ix> = vec![];
        let mut visited: Vec<Ix> = vec![];

        for root in roots {
            if let Some(vtx) = self.get_vertex(root.clone()) {
                self.dfs(vtx, &mut stack, &mut visited)?;
            }
        }

        stack.reverse();

        Ok(GraphOk::VecRes(stack))
    }

    #[cfg(test)]
    fn dfs(
        &self,
        vertex: &Vertex<T, Ix>,
        stack: &mut Vec<Ix>,
        visited: &mut Vec<Ix>,
    ) -> GraphResult<Ix> {
        let references = vertex.get_references();
        if !references.is_empty() {
            for r in references {
                if let Some(vtx) = self.get_vertex(r.clone()) {
                    self.dfs(vtx, stack, visited)?;
                }
            }
        }

        if !stack.contains(&vertex.get_index()) {
            stack.push(vertex.get_index().clone());
        }

        Ok(GraphOk::Ok)
    }
}
