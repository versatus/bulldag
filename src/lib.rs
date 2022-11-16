pub mod graph;
pub mod node;
pub mod edge;
pub mod vertex;
pub mod index;

#[cfg(test)]
mod tests {
    #![allow(dead_code)]
    use crate::graph::BullDag;
    use crate::vertex::Vertex;
    use crate::graph::GraphOk;

    #[test]
    fn create_new_dag() {
        let graph: BullDag<usize, &str> = BullDag::new();
        assert!(graph.len() == 0);
    }

    #[test]
    fn test_add_cyclic_edge_fails() {

        let mut graph: BullDag<usize, &str> = BullDag::new();
        let v1: Vertex<usize, &str> = Vertex::new(5, "source");
        let v2: Vertex<usize, &str> = Vertex::new(4, "reference");
        let v3: Vertex<usize, &str> = Vertex::new(3, "ultimate_source");
        let v4: Vertex<usize, &str> = Vertex::new(2, "ref_reference"); 
        let v5: Vertex<usize, &str> = Vertex::new(1, "new_reference");
        let v6: Vertex<usize, &str> = Vertex::new(0, "cycle_ref");
        let v7: Vertex<usize, &str> = Vertex::new(6, "cycle_source");
        let edges = vec![
            (&v1, &v2), 
            (&v3, &v1), 
            (&v2, &v4), 
            (&v3, &v4),
            (&v4, &v5),
            (&v5, &v6),
            (&v6, &v7),
            (&v6, &v1),
        ];

        graph.extend_from_edges(edges);
        assert!(graph.n_edges() == 7);
    }
    
    #[test]
    fn test_add_acyclic_edge_works() {
        let mut graph: BullDag<usize, &str> = BullDag::new();
        let v1: Vertex<usize, &str> = Vertex::new(5, "source");
        let v2: Vertex<usize, &str> = Vertex::new(4, "reference");
        let v3: Vertex<usize, &str> = Vertex::new(3, "ultimate_source");
        let v4: Vertex<usize, &str> = Vertex::new(2, "ref_reference"); 
        let v5: Vertex<usize, &str> = Vertex::new(1, "new_reference");
        let edges = vec![
            (&v1, &v2), 
            (&v3, &v1), 
            (&v3, &v2), 
            (&v2, &v4),
            (&v2, &v5),
            (&v1, &v5)
        ];

        graph.extend_from_edges(edges);

        assert!(graph.n_edges() == 6);
    }

    #[test]
    fn test_adding_edges_auto_adds_vertices() {
        let mut graph: BullDag<usize, &str> = BullDag::new();
        let v1: Vertex<usize, &str> = Vertex::new(5, "source");
        let v2: Vertex<usize, &str> = Vertex::new(4, "reference");
        let v3: Vertex<usize, &str> = Vertex::new(3, "ultimate_source");
        let v4: Vertex<usize, &str> = Vertex::new(2, "ref_reference"); 
        let v5: Vertex<usize, &str> = Vertex::new(1, "new_reference");
        let edges = vec![
            (&v1, &v2), 
            (&v3, &v1), 
            (&v3, &v2), 
            (&v2, &v4),
            (&v2, &v5),
            (&v1, &v5)
        ];

        graph.extend_from_edges(edges);

        assert!(graph.len() == 5);
    }

    #[test]
    fn test_get_vertex_references() {
        let mut graph: BullDag<usize, &str> = BullDag::new();
        let v1: Vertex<usize, &str> = Vertex::new(5, "source");
        let v2: Vertex<usize, &str> = Vertex::new(4, "reference");
        let v3: Vertex<usize, &str> = Vertex::new(3, "ultimate_source");
        let v4: Vertex<usize, &str> = Vertex::new(2, "ref_reference"); 
        let v5: Vertex<usize, &str> = Vertex::new(1, "new_reference");
        let edges = vec![
            (&v1, &v2), 
            (&v3, &v1), 
            (&v3, &v2), 
            (&v2, &v4),
            (&v2, &v5),
            (&v1, &v5)
        ];

        graph.extend_from_edges(edges);

        let target = graph.get_vertex("source"); 
        if target.is_some() {
            assert!(target.unwrap().is_reference(&v2.get_index()));
            assert!(target.unwrap().is_reference(&v5.get_index()));
        } else {
            panic!("Vertex not found");
        }
    }

    #[test]
    fn test_get_vertex_source() {
        let mut graph: BullDag<usize, &str> = BullDag::new();
        let v1: Vertex<usize, &str> = Vertex::new(5, "source");
        let v2: Vertex<usize, &str> = Vertex::new(4, "reference");
        let v3: Vertex<usize, &str> = Vertex::new(3, "ultimate_source");
        let v4: Vertex<usize, &str> = Vertex::new(2, "ref_reference"); 
        let v5: Vertex<usize, &str> = Vertex::new(1, "new_reference");
        let edges = vec![
            (&v1, &v2), 
            (&v3, &v1), 
            (&v3, &v2), 
            (&v2, &v4),
            (&v2, &v5),
            (&v1, &v5)
        ];

        graph.extend_from_edges(edges);

        let target = graph.get_vertex("source"); 
        if target.is_some() {
            assert!(target.unwrap().is_source(&v3.get_index()));
        } else {
            panic!("Vertex not found");
        }
    }

    #[ignore]
    #[test]
    fn test_get_vertex_dfs() {

    }

    #[test]
    fn test_get_topological_order() {
        let mut graph: BullDag<usize, &str> = BullDag::new();
        let v1: Vertex<usize, &str> = Vertex::new(5, "source");
        let v2: Vertex<usize, &str> = Vertex::new(4, "reference");
        let v3: Vertex<usize, &str> = Vertex::new(3, "ultimate_source");
        let v4: Vertex<usize, &str> = Vertex::new(2, "ref_reference"); 
        let v5: Vertex<usize, &str> = Vertex::new(1, "new_reference");
        let edges = vec![
            (&v1, &v2), 
            (&v3, &v1), 
            (&v3, &v2), 
            (&v2, &v4),
            (&v2, &v5),
            (&v1, &v5)
        ];

        graph.extend_from_edges(edges);

        let opt_1 = vec![
            "ultimate_source", 
            "source", 
            "reference", 
            "new_reference", 
            "ref_reference"
        ];

        let opt_2 = vec![
            "ultimate_source", 
            "source", 
            "reference", 
            "ref_reference",
            "new_reference" 
        ];

        if let Ok(GraphOk::VecRes(v)) = graph.topological_sort() {

            assert!(
                (v == opt_1 || 
                 v == opt_2)
            );
        }
    }
}
