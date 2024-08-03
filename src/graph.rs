use serde::{Serialize, Serializer, Deserialize, Deserializer};
use petgraph::graph::{Graph as PetGraph};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

/// Repräsentiert einen Knoten im Transaktionsgraphen.
#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    id: String,
}

/// Repräsentiert eine Kante zwischen zwei Knoten im Transaktionsgraphen.
#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    from: String,
    to: String,
}

/// Wrapper-Typ für Rc<RefCell<Node>>.
#[derive(Debug)]
pub struct RcRefCellNode(pub Rc<RefCell<Node>>);

impl Serialize for RcRefCellNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.borrow().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RcRefCellNode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let node = Node::deserialize(deserializer)?;
        Ok(RcRefCellNode(Rc::new(RefCell::new(node))))
    }
}

/// Wrapper-Typ für Rc<RefCell<Edge>>.
#[derive(Debug)]
pub struct RcRefCellEdge(pub Rc<RefCell<Edge>>);

impl Serialize for RcRefCellEdge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.borrow().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RcRefCellEdge {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let edge = Edge::deserialize(deserializer)?;
        Ok(RcRefCellEdge(Rc::new(RefCell::new(edge))))
    }
}

/// Repräsentiert einen Transaktionsgraphen.
#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    nodes: HashMap<String, RcRefCellNode>,
    edges: Vec<RcRefCellEdge>,
}

impl Graph {
    /// Erstellt einen neuen, leeren Graphen.
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    /* Fügt dem Graphen einen Knoten hinzu.
       # Argumente
       * `id` - Eine Zeichenkette, die die ID des Knotens hält.
    */ 
    pub fn add_node(&mut self, id: String) {
        let node = RcRefCellNode(Rc::new(RefCell::new(Node { id: id.clone() })));
        self.nodes.insert(id, node);
    }

    /* Fügt dem Graphen eine Kante hinzu.
        # Argumente
        * `from` - Eine Zeichenkette, die die ID des Quellknotens hält.
        * `to` - Eine Zeichenkette, die die ID des Zielknotens hält.
    */ 
    pub fn add_edge(&mut self, from: String, to: String) {
        let edge = RcRefCellEdge(Rc::new(RefCell::new(Edge { from, to })));
        self.edges.push(edge);
    }

    /// Konvertiert den benutzerdefinierten Graphen in einen `petgraph` Graphen zur Visualisierung.
    pub fn convert_to_petgraph(&self) -> PetGraph<String, ()> {
        let mut petgraph = PetGraph::new();
        let mut nodes = HashMap::new();

        for (id, node) in &self.nodes {
            let index = petgraph.add_node(node.0.borrow().id.clone());
            nodes.insert(id.clone(), index);
        }

        for edge in &self.edges {
            let edge = edge.0.borrow();
            let from_index = nodes[&edge.from];
            let to_index = nodes[&edge.to];
            petgraph.add_edge(from_index, to_index, ());
        }

        petgraph
    }

    /// Gibt eine Referenz auf die Knoten zurück.
    pub fn nodes(&self) -> &HashMap<String, RcRefCellNode> {
        &self.nodes
    }

    /// Gibt eine Referenz auf die Kanten zurück.
    pub fn edges(&self) -> &Vec<RcRefCellEdge> {
        &self.edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node() {
        let mut graph = Graph::new();
        graph.add_node("node1".to_string());
        assert!(graph.nodes.contains_key("node1"));
    }

    #[test]
    fn test_add_edge() {
        let mut graph = Graph::new();
        graph.add_node("node1".to_string());
        graph.add_node("node2".to_string());
        graph.add_edge("node1".to_string(), "node2".to_string());
        assert_eq!(graph.edges.len(), 1);
    }

    #[test]
    fn test_convert_to_petgraph() {
        let mut graph = Graph::new();
        graph.add_node("node1".to_string());
        graph.add_node("node2".to_string());
        graph.add_edge("node1".to_string(), "node2".to_string());
        let petgraph = graph.convert_to_petgraph();
        assert_eq!(petgraph.node_count(), 2);
        assert_eq!(petgraph.edge_count(), 1);
    }
}
