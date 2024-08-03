use bitcoincore_rpc::RpcApi;
use crate::graph::Graph;

/* Baut einen Transaktionsgraphen aus einem angegebenen Block auf.
    # Argumente
    * `block_number` - Die Blocknummer, aus der der Graph erstellt werden soll.
    # Rückgabe
    * `Result<Graph, String>` - Der aus dem angegebenen Block erstellte Transaktionsgraph
    oder ein Fehlerstring bei einem Fehler.
*/
pub fn build_transaction_graph(block_number: u32) -> Result<Graph, String> {
    let client = crate::client::RPC_CLIENT.lock().map_err(|_| "Fehler beim Sperren des Mutex".to_string())?;

    let mut graph = Graph::new();

    let block_hash = client.get_block_hash(block_number.into()).map_err(|e| e.to_string())?;
    let block = client.get_block(&block_hash).map_err(|e| e.to_string())?;

    for tx in block.txdata {
        let txid = tx.txid().to_string();
        graph.add_node(txid.clone());

        for input in tx.input {
            // Überprüfen, ob die Transaktion eine Coinbase-Transaktion ist
            let prev_txid = if input.previous_output.txid.to_string() == "0000000000000000000000000000000000000000000000000000000000000000" {
                format!("Coinbase-tx -> block {}", block_number)
            } else {
                input.previous_output.txid.to_string()
            };
            graph.add_node(prev_txid.clone());
            graph.add_edge(prev_txid, txid.clone());
        }
    }

    Ok(graph)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_transaction_graph() {
        let block_number = 0; // Beispiel-Blocknummer, anpassen für tatsächlichen Test
        match build_transaction_graph(block_number) {
            Ok(graph) => {
                assert!(!graph.nodes().is_empty());
                assert!(!graph.edges().is_empty());
            }
            Err(e) => panic!("Fehler beim Aufbau des Transaktionsgraphen: {}", e),
        }
    }
}
