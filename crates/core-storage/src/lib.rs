use redb::{Database, ReadableTable, TableDefinition};
use std::path::Path;
use thiserror::Error;

const VAULT_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("vault_data");

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    DbError(#[from] redb::Error),
    #[error("Storage error: {0}")]
    Custom(String),
}

pub struct MicoreStorage {
    db: Database,
}

impl MicoreStorage {
    pub fn open_or_create<P: AsRef<Path>>(path: P) -> Result<Self, StorageError> {
        let db = Database::create(path).map_err(|e| StorageError::Custom(e.to_string()))?;
        Ok(Self { db })
    }

    pub fn set_bytes(&self, key: &str, data: &[u8]) -> Result<(), StorageError> {
        let write_txn = self.db.begin_write().map_err(|e| StorageError::Custom(e.to_string()))?;
        {
            let mut table = write_txn.open_table(VAULT_TABLE).map_err(|e| StorageError::Custom(e.to_string()))?;
            table.insert(key, data).map_err(|e| StorageError::Custom(e.to_string()))?;
        }
        write_txn.commit().map_err(|e| StorageError::Custom(e.to_string()))?;
        Ok(())
    }

    pub fn get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        let read_txn = self.db.begin_read().map_err(|e| StorageError::Custom(e.to_string()))?;
        let table = read_txn.open_table(VAULT_TABLE).map_err(|e| StorageError::Custom(e.to_string()))?;
        let result = table.get(key).map_err(|e| StorageError::Custom(e.to_string()))?;
        Ok(result.map(|access| access.value().to_vec()))
    }
}