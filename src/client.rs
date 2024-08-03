use bitcoincore_rpc::{Auth, Client, RpcApi};
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::env;

/* Statische Initialisierung des RPC-Clients für den Bitcoin Core-Node.
    Verwendet Umgebungsvariablen, die aus einer `.env`-Datei geladen werden.
*/
lazy_static! {
    pub static ref RPC_CLIENT: Mutex<Client> = {
        dotenv().ok(); // Lädt die .env-Datei, falls vorhanden

        let rpc_url = env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL muss gesetzt sein");
        let rpc_user = env::var("BITCOIN_RPC_USER").expect("BITCOIN_RPC_USER muss gesetzt sein");
        let rpc_password = env::var("BITCOIN_RPC_PASSWORD").expect("BITCOIN_RPC_PASSWORD muss gesetzt sein");

        Mutex::new(Client::new(
            &rpc_url,
            Auth::UserPass(rpc_user, rpc_password),
        ).expect("Fehler beim Erstellen des Clients"))
    };
}

/*  Testet die Verbindung zum Bitcoin Core-Node.

    Gibt `Ok(())` zurück, wenn die Verbindung erfolgreich ist, ansonsten `Err(String)`
    mit einer Fehlermeldung.

    # Rückgabe

    * `Result<(), String>` - `Ok(())` bei Erfolg, `Err(String)` bei Fehler.
*/
pub fn test_node_connection() -> Result<(), String> {
    let client = RPC_CLIENT.lock().map_err(|_| "Fehler beim Sperren des Mutex".to_string())?;
    client.get_blockchain_info().map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Testet die Verbindung zum Bitcoin Core-Node.
    #[test]
    fn test_connection() {
        match test_node_connection() {
            Ok(_) => println!("Verbindung erfolgreich"),
            Err(e) => panic!("Verbindung fehlgeschlagen: {}", e),
        }
    }
}
