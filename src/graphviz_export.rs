use petgraph::dot::{Dot, Config};
use petgraph::graph::Graph;
use std::fs::File;
use std::io::{self, Write};

/* Exportiert einen Graphen in das DOT-Format zur Visualisierung mit Graphviz.
    # Argumente
    * `graph` - Der zu exportierende Graph.
    * `filename` - Der Name der Ausgabedatei.
    # Rückgabe
    * `io::Result<()>` - Ergebnis der Dateioperation, `Ok(())` bei Erfolg, `Err` bei Fehler.
*/
pub fn export_to_dot(graph: &Graph<String, ()>, filename: &str) -> io::Result<()> {
    let dot = Dot::with_config(graph, &[Config::EdgeNoLabel]);
    let mut file = File::create(filename)?;
    write!(file, "{:?}", dot)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::Graph;

    #[test]
    fn test_export_to_dot() {
        let mut graph = Graph::<String, ()>::new();
        let a = graph.add_node("A".to_string());
        let b = graph.add_node("B".to_string());
        graph.add_edge(a, b, ());
        
        let result = export_to_dot(&graph, "test_output.dot");
        assert!(result.is_ok());
    }
}
