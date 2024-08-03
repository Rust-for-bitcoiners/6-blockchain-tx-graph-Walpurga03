mod client;
mod graph;
mod profile_transactions;
mod graphviz_export;

use client::test_node_connection;
use profile_transactions::build_transaction_graph;
use graphviz_export::export_to_dot;
use petgraph::graph::Graph as PetGraph;
use std::io::{self, Write};
use std::fs::File;
use bitcoincore_rpc::RpcApi;

/// Hauptfunktion, die das Programm startet.
fn main() {
    // Teste die Verbindung zum Bitcoin Core-Node
    if let Err(e) = test_node_connection() {
        eprintln!("Error: {}", e);
        return;
    }

    // Blocknummer eingeben
    let block_number = input_block_number();
    export_transactions(block_number).expect("Failed to export transactions");

    println!("Block {} transactions exported to transactions-{}.txt", block_number, block_number);

    // Hauptschleife für TXID-Eingaben
    loop {
        let txid = input_txid();

        if txid.to_lowercase() == "q" {
            println!("Exiting...");
            break;
        }

        match find_block_containing_txid(&txid, block_number) {
            Some(block) => {
                match build_transaction_graph(block) {
                    Ok(graph) => {
                        let petgraph: PetGraph<String, ()> = graph.convert_to_petgraph();
                        let filename = format!("block-{}-{}.dot", block, txid);
                        export_to_dot(&petgraph, &filename).expect("Failed to export graph");
                        println!("Graph exported to {}", filename);
                    },
                    Err(e) => println!("Fehler beim Aufbau des Transaktionsgraphen: {}", e),
                }
            },
            None => println!("TXID not found."),
        }
    }
}

/* Fragt den Benutzer nach der Blocknummer und gibt sie zurück.
    # Rückgabe
    * `u32` - Die eingegebene Blocknummer.
*/
fn input_block_number() -> u32 {
    let mut input = String::new();
    print!("Enter block number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

/* Fragt den Benutzer nach der TXID und gibt sie zurück.
    # Rückgabe
    * `String` - Die eingegebene TXID.
*/
fn input_txid() -> String {
    let mut input = String::new();
    print!("Enter TXID (or 'q' to quit): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/* Sucht den Block, der die angegebene TXID enthält.
    # Argumente
    * `txid` - Die zu suchende TXID.
    * `start_block` - Die Blocknummer, von der aus die Suche gestartet wird.
    # Rückgabe
    * `Option<u32>` - Die Blocknummer, die die TXID enthält, oder `None`, wenn die TXID nicht gefunden wurde.
*/
fn find_block_containing_txid(txid: &str, start_block: u32) -> Option<u32> {
    let client = client::RPC_CLIENT.lock().unwrap();

    for height in (0..=start_block).rev() {
        let block_hash = client.get_block_hash(height.into()).unwrap();
        let block = client.get_block(&block_hash).unwrap();

        for tx in block.txdata {
            if tx.txid().to_string() == txid {
                return Some(height);
            }
        }
    }

    None
}

/* Exportiert die Transaktionen eines Blocks in eine Datei.
    # Argumente
    * `block_number` - Die Blocknummer, deren Transaktionen exportiert werden sollen.
    # Rückgabe
    * `std::io::Result<()>` - Ergebnis der Dateioperation, `Ok(())` bei Erfolg, `Err` bei Fehler.
*/
fn export_transactions(block_number: u32) -> std::io::Result<()> {
    let client = client::RPC_CLIENT.lock().unwrap();
    let block_hash = client.get_block_hash(block_number.into()).unwrap();
    let block = client.get_block(&block_hash).unwrap();

    let filename = format!("transactions-{}.txt", block_number);
    let mut file = File::create(&filename)?;

    for tx in block.txdata {
        writeln!(file, "TXID: {}", tx.txid())?;
    }

    Ok(())
}
