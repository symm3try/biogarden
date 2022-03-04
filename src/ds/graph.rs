use hashbrown::{HashMap, HashSet};
use rand::{thread_rng, Rng};
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Clone, Debug, PartialEq)]
/// Graph operation error
pub enum GraphErr {
    /// There is no vertex with the given id in the graph
    NoSuchVertex,

    /// There is no such edge in the graph
    NoSuchEdge,

    /// Could not add an edge to the graph
    CannotAddEdge,

    /// The given weight is invalid
    InvalidWeight,

    /// The operation cannot be performed as it will
    /// create a cycle in the graph.
    CycleError,

    #[cfg(feature = "dot")]
    /// Could not render .dot file
    CouldNotRender,

    #[cfg(feature = "dot")]
    /// The name of the graph is invalid. Check [this](https://docs.rs/dot/0.1.1/dot/struct.Id.html#method.new)
    /// out for more information.
    InvalidGraphName,
}

#[derive(Clone, Debug)]
/// Edge internal struct
pub struct Edge<T> {
    start: u64,
    end: u64,
    pub data: T,
}

#[derive(Clone, Debug, Default)]
pub struct Node<T> {
    id: u64,
    incoming: Vec<u64>,
    outgoing: Vec<u64>,
    pub data: T,
}

#[derive(Clone, Debug, Default)]
/// Graph data-structure
pub struct Graph<N, E> {
    /// Mapping of vertex ids and vertex values
    nodes: HashMap<u64, Node<N>>,
    /// Mapping between edges and weights
    edges: HashMap<u64, Edge<E>>,
}

impl<N, E> Graph<N, E> {

    pub fn new() -> Graph<N, E> {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Graph<N, E> {
        let edges_capacity = if capacity < 100 {
            usize::pow(capacity, 2)
        } else {
            capacity
        };

        Graph {
            nodes: HashMap::with_capacity(capacity),
            edges: HashMap::with_capacity(edges_capacity),
        }
    }   

    pub fn add_node(&mut self, data: N) -> u64 {
        let mut rng = thread_rng();
        let nid = rng.gen_range(0..1000000000);
        self.nodes.insert(nid, Node{ id: nid, data: data, incoming: vec![], outgoing: vec![],});
        nid
    }   

    pub fn has_edge(&self, a: &u64, b: &u64) -> bool {
        match self.nodes.get(a) {
            Some(node) => node.outgoing.contains(b),
            None => false,
        }
    }    

    pub fn add_edge(&mut self, a: &u64, b: &u64, data: E) -> Result<(), GraphErr> {
        if self.has_edge(a, b) {
            return Ok(());
        }

        let id1 = if self.nodes.get(a).is_some() {
            *a
        } else {
            return Err(GraphErr::NoSuchVertex);
        };

        let id2 = if self.nodes.get(b).is_some() {
            *b
        } else {
            return Err(GraphErr::NoSuchVertex);
        };

        let mut rng = thread_rng();
        let eid = rng.gen_range(0..1000000000);
        self.edges.insert(eid, Edge{start: id1, end: id2, data: data});

        self.nodes.get_mut(&id1).unwrap().outgoing.push(eid);
        self.nodes.get_mut(&id2).unwrap().incoming.push(eid);

        Ok(())
    }

    pub fn edges(&self) -> impl Iterator<Item = (&Node<N>, &Node<N>)> {
        self.edges.iter().map(|(_, e)| (self.nodes.get(&e.start).unwrap(), self.nodes.get(&e.end).unwrap()))
    }

    // pub fn node(&self, id: u64) -> &N {
    //     self.nodes.get(id).unwrap()
    // }

    // pub fn in_neighbors(&self, id: &u64) -> impl Iterator<Item = &N> {
    //     match self.nodes.get(id) {
    //         Some(neighbors) => VertexIter(Box::new(neighbors.iter().map(AsRef::as_ref))),
    //         None => VertexIter(Box::new(iter::empty())),
    //     }
    // }

    // pub fn out_neighbors(&self, id: &VertexId) -> VertexIter<'_> {
    //     match self.outbound_table.get(id) {
    //         Some(iter) => VertexIter(Box::new(iter.iter().rev().map(AsRef::as_ref))),
    //         None => VertexIter(Box::new(iter::empty())),
    //     }
    // }


    // pub fn remove(&mut self, id: &VertexId) {
    //     self.vertices.remove(id);

    //     // Remove each inbound edge
    //     if let Some(inbounds) = self.inbound_table.remove(id) {
    //         for vertex in inbounds {
    //             self.remove_edge(&vertex, id);

    //             // Add to tips if inbound vertex doesn't
    //             // have other outbound vertices.
    //             if self.out_neighbors_count(&vertex) == 0 {
    //                 self.tips.insert(vertex);
    //             }
    //         }
    //     }

    //     // Remove each outbound edge
    //     if let Some(outbounds) = self.outbound_table.remove(id) {
    //         for vertex in outbounds {
    //             self.remove_edge(id, &vertex);

    //             // Add to roots if outbound vertex doesn't
    //             // have other inbound vertices.
    //             if self.in_neighbors_count(&vertex) == 0 {
    //                 self.roots.insert(vertex);
    //             }
    //         }
    //     }

    //     self.roots.remove(&id);
    //     self.tips.remove(&id);
    // }

    // pub fn remove_edge(&mut self, a: &VertexId, b: &VertexId) {
    //     if let Some(outbounds) = self.outbound_table.get_mut(a) {
    //         outbounds.retain(|v| v != b);
    //         if outbounds.is_empty() {
    //             self.outbound_table.remove(a);
    //         }
    //     }

    //     if let Some(inbounds) = self.inbound_table.get_mut(b) {
    //         inbounds.retain(|v| v != a);
    //         if inbounds.is_empty() {
    //             self.inbound_table.remove(b);
    //         }
    //     }

    //     // If outbound vertex doesn't have any more inbounds,
    //     // mark it as root.
    //     if self.in_neighbors_count(&b) == 0 {
    //         self.roots.insert(b.clone());
    //     }

    //     // Mark vertex as tip if it doesn't have any more outbounds.
    //     if self.out_neighbors_count(&a) == 0 {
    //         self.tips.insert(a.clone());
    //     }

    //     self.edges.remove(&Edge::new(*a, *b));
    // }
    
}
 

//     pub fn weight(&self, a: &VertexId, b: &VertexId) -> Option<f32> {
//         if !self.has_edge(a, b) {
//             return None;
//         }

//         if let Some(result) = self.edges.get(&Edge::new(*a, *b)) {
//             Some(*result)
//         } else {
//             None
//         }
//     }    


//     pub fn set_weight(
//         &mut self,
//         a: &VertexId,
//         b: &VertexId,
//         new_weight: f32,
//     ) -> Result<(), GraphErr> {
//         if !self.has_edge(a, b) {
//             return Err(GraphErr::NoSuchEdge);
//         }

//         if new_weight > 1.0 || new_weight < -1.0 {
//             return Err(GraphErr::InvalidWeight);
//         }

//         self.edges.insert(Edge::new(*a, *b), new_weight);

//         // Sort outbound vertices after setting a new weight
//         let mut outbounds = self.outbound_table.get(a).unwrap().clone();

//         self.sort_outbounds(a.clone(), &mut outbounds);

//         // Update outbounds
//         self.outbound_table.insert(a.clone(), outbounds);

//         Ok(())
//     }    

// }