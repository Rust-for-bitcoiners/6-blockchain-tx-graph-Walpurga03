# Assignment 6

## Implementing Graph in Rust

First implement the graph.rs file, make sure you pass all the tests.
Write your own tests for complicated cases.
Try to finish this by next Monday without fail.

## Build transaction graph for profiling

Download the blocks from [start_height, end_height] inclusive. Build the graph
as explained in the `build_transaction_graph` function of profile_transactions.rs file.
Write some tests using Regtest node to verify your implementation works correctly.
Use [bitcoind](https://crates.io/crates/bitcoind) crate to start the bitcoin crate programmatically
in regtest mode.
This test might be involved and will require patience and lots of thoughts from your end.
You can take your time for this.

Also learning to test like this is a valuable skill that you will require to build production ready
bitcoin applications in Rust.


#### Deutsch

## Implementierung eines Graphen in Rust

Zuerst implementiere die Datei `graph.rs` und stelle sicher, dass du alle Tests bestehst.
Schreibe deine eigenen Tests für komplizierte Fälle.
Versuche, dies bis nächsten Montag ohne Verzögerung abzuschließen.

## Erstellen eines Transaktionsgraphen zur Profilierung

Lade die Blöcke von `[start_height, end_height]` einschließlich herunter. Baue den Graphen
wie im `build_transaction_graph` Funktion der Datei `profile_transactions.rs` erklärt.
Schreibe einige Tests mit einem Regtest-Knoten, um sicherzustellen, dass deine Implementierung korrekt funktioniert.
Verwende das [bitcoind](https://crates.io/crates/bitcoind) crate, um das Bitcoin-Programm programmgesteuert
im Regtest-Modus zu starten.
Dieser Test könnte aufwendig sein und erfordert Geduld und viel Nachdenken von deiner Seite.
Du kannst dir dafür Zeit nehmen.

Das Erlernen solcher Tests ist auch eine wertvolle Fähigkeit, die du benötigst, um produktionsreife
Bitcoin-Anwendungen in Rust zu entwickeln.

Teile mir bitte mit, welche Tools du für diese Übung verwenden möchtest, damit ich dir weiterhelfen kann.