use rusqlite;
use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub trait UniversalStorage{
    fn create(&self) -> Result<()>;
    fn insert(&self, hash: String, proof: String) -> Result<()>;
    fn fetch(&self) -> Result<Vec<String>>;
    fn fetch_by_unique_id(&self, hash: &str) -> Result<Option<String>>;

}

#[derive(Clone)]
pub struct Storage {
    pub path: PathBuf,
}

impl UniversalStorage for Storage {
    fn create(&self) -> Result<()> {
        let conn = Connection::open(&self.path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS data (
                    id INTEGER PRIMARY KEY,
                    hash TEXT NOT NULL,
                    proof TEXT NOT NULL
                )",
            [],
        )?;
        Ok(())
    }
    fn insert(
        &self,
        hash: String,
        proof: String
    ) -> Result<()> {
        let conn = Connection::open(&self.path)?;
        conn.execute(
            "INSERT INTO data (hash, proof) VALUES (?1, ?2)",
            [&hash, &proof],
        )?;
        Ok(())
    }
    fn fetch(&self) -> Result<Vec<String>> {
        let conn: Connection = Connection::open(&self.path)?;
        let mut state: rusqlite::Statement<'_> = conn.prepare("SELECT hash, proof FROM data")?;
        let data_iter = state.query_map([], |row| {
            let proof: String = row.get(1)?;
            Ok(proof)
        })?;
        let mut result: Vec<String> = Vec::new();
        for data in data_iter {
            result.push(data?);
        }
        Ok(result)
    }
    fn fetch_by_unique_id(&self, hash: &str) -> Result<Option<String>> {
        let conn: Connection = Connection::open(&self.path)?;
        let mut stmt: rusqlite::Statement<'_> = conn
            .prepare("SELECT hash, proof FROM data WHERE hash = ?1 LIMIT 1")?;
        // Use query_row and check for QueryReturnedNoRows error
        match stmt.query_row([&hash], |row| {
            let proof: String = row.get(1)?;
            Ok(proof)
        }) {
            Ok(data) => Ok(Some(data)),
            Err(_) => Ok(None),
        }
    }
}

#[test]
fn create_noir_db() {
    // purge if exists
    // cargo test create -- /Users/chef/Desktop/exercise-project-6/database
    use std::fs;
    use std::env;

    let env_args: Vec<String> = env::args().collect();
    let path_to_db: PathBuf = PathBuf::from(&env_args[2]);
    match fs::remove_file(path_to_db.join("noir.db")) {
        Ok(_) => println!("File deleted successfully"),
        Err(e) => println!("Error deleting file: {}", e),
    }

    let noir = Storage {
        path: path_to_db.join("noir.db"),
    };
    let msg = format!("Failed to create db file at: {:?}!", &path_to_db.to_str());
    UniversalStorage::create(&noir).expect(&msg);
}

#[test]
fn create_circom_db(){
    use std::fs;
    use std::env;
    let env_args: Vec<String> = env::args().collect();
    let path_to_db: PathBuf = PathBuf::from(&env_args[2]);
    match fs::remove_file(path_to_db.join("circom.db")) {
        Ok(_) => println!("File deleted successfully"),
        Err(e) => println!("Error deleting file: {}", e),
    }

    let noir = Storage {
        path: path_to_db.join("circom.db"),
    };
    let msg = format!("Failed to create db file at: {:?}!", &path_to_db.to_str());
    UniversalStorage::create(&noir).expect(&msg);
}