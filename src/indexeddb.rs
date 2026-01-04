use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Errors that can occur in IndexedDB operations
#[derive(Debug, Clone, PartialEq)]
pub enum IDBError {
    NotFoundError(String),
    InvalidStateError(String),
    ConstraintError(String),
    VersionError(String),
    DataError(String),
    TransactionInactiveError,
    ReadOnlyError,
    AbortError,
    UnknownError(String),
}

impl std::fmt::Display for IDBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IDBError::NotFoundError(msg) => write!(f, "NotFoundError: {}", msg),
            IDBError::InvalidStateError(msg) => write!(f, "InvalidStateError: {}", msg),
            IDBError::ConstraintError(msg) => write!(f, "ConstraintError: {}", msg),
            IDBError::VersionError(msg) => write!(f, "VersionError: {}", msg),
            IDBError::DataError(msg) => write!(f, "DataError: {}", msg),
            IDBError::TransactionInactiveError => write!(f, "TransactionInactiveError"),
            IDBError::ReadOnlyError => write!(f, "ReadOnlyError"),
            IDBError::AbortError => write!(f, "AbortError"),
            IDBError::UnknownError(msg) => write!(f, "UnknownError: {}", msg),
        }
    }
}

impl std::error::Error for IDBError {}

/// Key for IndexedDB records
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IDBKey {
    Number(i64),
    String(String),
    Date(i64), // Unix timestamp
    Array(Vec<IDBKey>),
}

impl IDBKey {
    pub fn compare(&self, other: &IDBKey) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        match (self, other) {
            (IDBKey::Number(a), IDBKey::Number(b)) => a.cmp(b),
            (IDBKey::String(a), IDBKey::String(b)) => a.cmp(b),
            (IDBKey::Date(a), IDBKey::Date(b)) => a.cmp(b),
            (IDBKey::Array(a), IDBKey::Array(b)) => {
                // Compare element by element
                for (a_elem, b_elem) in a.iter().zip(b.iter()) {
                    let cmp = a_elem.compare(b_elem);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
                a.len().cmp(&b.len())
            }
            _ => Ordering::Equal,
        }
    }
}

/// Key range for queries
#[derive(Debug, Clone)]
pub struct IDBKeyRange {
    pub lower: Option<IDBKey>,
    pub upper: Option<IDBKey>,
    pub lower_open: bool,
    pub upper_open: bool,
}

impl IDBKeyRange {
    pub fn only(key: IDBKey) -> Self {
        Self {
            lower: Some(key.clone()),
            upper: Some(key),
            lower_open: false,
            upper_open: false,
        }
    }

    pub fn lower_bound(key: IDBKey, open: bool) -> Self {
        Self {
            lower: Some(key),
            upper: None,
            lower_open: open,
            upper_open: false,
        }
    }

    pub fn upper_bound(key: IDBKey, open: bool) -> Self {
        Self {
            lower: None,
            upper: Some(key),
            lower_open: false,
            upper_open: open,
        }
    }

    pub fn bound(lower: IDBKey, upper: IDBKey, lower_open: bool, upper_open: bool) -> Self {
        Self {
            lower: Some(lower),
            upper: Some(upper),
            lower_open,
            upper_open,
        }
    }

    pub fn includes(&self, key: &IDBKey) -> bool {
        if let Some(ref lower) = self.lower {
            let cmp = key.compare(lower);
            if cmp == std::cmp::Ordering::Less || (self.lower_open && cmp == std::cmp::Ordering::Equal) {
                return false;
            }
        }
        if let Some(ref upper) = self.upper {
            let cmp = key.compare(upper);
            if cmp == std::cmp::Ordering::Greater || (self.upper_open && cmp == std::cmp::Ordering::Equal) {
                return false;
            }
        }
        true
    }
}

/// Cursor direction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IDBCursorDirection {
    Next,
    NextUnique,
    Prev,
    PrevUnique,
}

/// Transaction mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IDBTransactionMode {
    ReadOnly,
    ReadWrite,
    VersionChange,
}

/// Index parameters
#[derive(Debug, Clone)]
pub struct IDBIndexParameters {
    pub unique: bool,
    pub multi_entry: bool,
}

impl Default for IDBIndexParameters {
    fn default() -> Self {
        Self {
            unique: false,
            multi_entry: false,
        }
    }
}

/// Object store parameters
#[derive(Debug, Clone)]
pub struct IDBObjectStoreParameters {
    pub key_path: Option<String>,
    pub auto_increment: bool,
}

impl Default for IDBObjectStoreParameters {
    fn default() -> Self {
        Self {
            key_path: None,
            auto_increment: false,
        }
    }
}

/// Index data
#[derive(Debug, Clone, Serialize, Deserialize)]
struct IndexData {
    name: String,
    key_path: String,
    unique: bool,
    multi_entry: bool,
    /// Map from index key to primary keys
    entries: HashMap<IDBKey, Vec<IDBKey>>,
}

/// Object store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDBObjectStore {
    name: String,
    key_path: Option<String>,
    auto_increment: bool,
    auto_increment_counter: i64,
    /// Primary key -> value mapping
    records: HashMap<IDBKey, JsonValue>,
    /// Index name -> index data
    indexes: HashMap<String, IndexData>,
}

impl IDBObjectStore {
    pub fn new(name: String, params: IDBObjectStoreParameters) -> Self {
        Self {
            name,
            key_path: params.key_path,
            auto_increment: params.auto_increment,
            auto_increment_counter: 0,
            records: HashMap::new(),
            indexes: HashMap::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn key_path(&self) -> Option<&str> {
        self.key_path.as_deref()
    }

    pub fn auto_increment(&self) -> bool {
        self.auto_increment
    }

    pub fn index_names(&self) -> Vec<String> {
        self.indexes.keys().cloned().collect()
    }

    pub fn add(&mut self, value: JsonValue, key: Option<IDBKey>) -> Result<IDBKey, IDBError> {
        let final_key = self.resolve_key(key, &value)?;
        
        if self.records.contains_key(&final_key) {
            return Err(IDBError::ConstraintError("Key already exists".to_string()));
        }
        
        self.records.insert(final_key.clone(), value.clone());
        self.update_indexes(&final_key, &value)?;
        
        Ok(final_key)
    }

    pub fn put(&mut self, value: JsonValue, key: Option<IDBKey>) -> Result<IDBKey, IDBError> {
        let final_key = self.resolve_key(key, &value)?;
        
        // Remove from indexes if updating
        if self.records.contains_key(&final_key) {
            self.remove_from_indexes(&final_key);
        }
        
        self.records.insert(final_key.clone(), value.clone());
        self.update_indexes(&final_key, &value)?;
        
        Ok(final_key)
    }

    pub fn get(&self, key: &IDBKey) -> Option<&JsonValue> {
        self.records.get(key)
    }

    pub fn delete(&mut self, key: &IDBKey) -> Result<(), IDBError> {
        if self.records.remove(key).is_some() {
            self.remove_from_indexes(key);
            Ok(())
        } else {
            Err(IDBError::NotFoundError("Key not found".to_string()))
        }
    }

    pub fn clear(&mut self) -> Result<(), IDBError> {
        self.records.clear();
        for index in self.indexes.values_mut() {
            index.entries.clear();
        }
        Ok(())
    }

    pub fn count(&self) -> usize {
        self.records.len()
    }

    pub fn get_all(&self, range: Option<&IDBKeyRange>, count: Option<usize>) -> Vec<(IDBKey, JsonValue)> {
        let mut results: Vec<_> = self.records.iter()
            .filter(|(key, _)| {
                range.map_or(true, |r| r.includes(key))
            })
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        
        results.sort_by(|a, b| a.0.compare(&b.0));
        
        if let Some(max_count) = count {
            results.truncate(max_count);
        }
        
        results
    }

    pub fn get_all_keys(&self, range: Option<&IDBKeyRange>, count: Option<usize>) -> Vec<IDBKey> {
        let mut keys: Vec<_> = self.records.keys()
            .filter(|key| range.map_or(true, |r| r.includes(key)))
            .cloned()
            .collect();
        
        keys.sort_by(|a, b| a.compare(b));
        
        if let Some(max_count) = count {
            keys.truncate(max_count);
        }
        
        keys
    }

    pub fn create_index(&mut self, name: String, key_path: String, params: IDBIndexParameters) -> Result<(), IDBError> {
        if self.indexes.contains_key(&name) {
            return Err(IDBError::ConstraintError("Index already exists".to_string()));
        }
        
        let mut index = IndexData {
            name: name.clone(),
            key_path,
            unique: params.unique,
            multi_entry: params.multi_entry,
            entries: HashMap::new(),
        };
        
        // Build index from existing records
        for (primary_key, value) in &self.records {
            if let Some(index_key) = self.extract_index_key(value, &index.key_path) {
                index.entries.entry(index_key)
                    .or_insert_with(Vec::new)
                    .push(primary_key.clone());
            }
        }
        
        self.indexes.insert(name, index);
        Ok(())
    }

    pub fn delete_index(&mut self, name: &str) -> Result<(), IDBError> {
        self.indexes.remove(name)
            .ok_or_else(|| IDBError::NotFoundError("Index not found".to_string()))?;
        Ok(())
    }

    pub fn index(&self, name: &str) -> Result<&IndexData, IDBError> {
        self.indexes.get(name)
            .ok_or_else(|| IDBError::NotFoundError("Index not found".to_string()))
    }

    fn resolve_key(&mut self, key: Option<IDBKey>, _value: &JsonValue) -> Result<IDBKey, IDBError> {
        if let Some(k) = key {
            Ok(k)
        } else if self.auto_increment {
            self.auto_increment_counter += 1;
            Ok(IDBKey::Number(self.auto_increment_counter))
        } else if self.key_path.is_some() {
            // In real implementation, extract from value using key_path
            Err(IDBError::DataError("Key path not implemented".to_string()))
        } else {
            Err(IDBError::DataError("No key provided".to_string()))
        }
    }

    fn update_indexes(&mut self, key: &IDBKey, value: &JsonValue) -> Result<(), IDBError> {
        // Collect index updates to avoid borrowing issues
        let updates: Vec<_> = self.indexes.iter()
            .filter_map(|(name, index)| {
                self.extract_index_key(value, &index.key_path)
                    .map(|index_key| (name.clone(), index_key))
            })
            .collect();
        
        // Apply updates
        for (name, index_key) in updates {
            if let Some(index) = self.indexes.get_mut(&name) {
                index.entries.entry(index_key)
                    .or_insert_with(Vec::new)
                    .push(key.clone());
            }
        }
        Ok(())
    }

    fn remove_from_indexes(&mut self, key: &IDBKey) {
        for index in self.indexes.values_mut() {
            for primary_keys in index.entries.values_mut() {
                primary_keys.retain(|k| k != key);
            }
        }
    }

    fn extract_index_key(&self, value: &JsonValue, key_path: &str) -> Option<IDBKey> {
        // Simplified: extract from top-level property
        if let JsonValue::Object(obj) = value {
            if let Some(JsonValue::Number(n)) = obj.get(key_path) {
                return n.as_i64().map(IDBKey::Number);
            } else if let Some(JsonValue::String(s)) = obj.get(key_path) {
                return Some(IDBKey::String(s.clone()));
            }
        }
        None
    }
}

/// Cursor for iterating records
#[derive(Debug)]
pub struct IDBCursor {
    keys: Vec<IDBKey>,
    current_index: usize,
    direction: IDBCursorDirection,
}

impl IDBCursor {
    pub fn new(mut keys: Vec<IDBKey>, direction: IDBCursorDirection) -> Self {
        keys.sort_by(|a, b| a.compare(b));
        
        match direction {
            IDBCursorDirection::Prev | IDBCursorDirection::PrevUnique => {
                keys.reverse();
            }
            _ => {}
        }
        
        Self {
            keys,
            current_index: 0,
            direction,
        }
    }

    pub fn key(&self) -> Option<&IDBKey> {
        self.keys.get(self.current_index)
    }

    pub fn advance(&mut self, count: usize) {
        self.current_index += count;
    }

    pub fn continue_cursor(&mut self) {
        self.current_index += 1;
    }

    pub fn has_value(&self) -> bool {
        self.current_index < self.keys.len()
    }
}

/// Transaction
pub struct IDBTransaction {
    mode: IDBTransactionMode,
    store_names: Vec<String>,
    active: bool,
    aborted: bool,
}

impl IDBTransaction {
    pub fn new(store_names: Vec<String>, mode: IDBTransactionMode) -> Self {
        Self {
            mode,
            store_names,
            active: true,
            aborted: false,
        }
    }

    pub fn mode(&self) -> IDBTransactionMode {
        self.mode
    }

    pub fn object_store_names(&self) -> &[String] {
        &self.store_names
    }

    pub fn is_active(&self) -> bool {
        self.active && !self.aborted
    }

    pub fn abort(&mut self) {
        self.aborted = true;
        self.active = false;
    }

    pub fn commit(&mut self) {
        self.active = false;
    }
}

/// Database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDBDatabase {
    name: String,
    version: u64,
    object_stores: HashMap<String, IDBObjectStore>,
}

impl IDBDatabase {
    pub fn new(name: String, version: u64) -> Self {
        Self {
            name,
            version,
            object_stores: HashMap::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn object_store_names(&self) -> Vec<String> {
        self.object_stores.keys().cloned().collect()
    }

    pub fn create_object_store(&mut self, name: String, params: IDBObjectStoreParameters) -> Result<(), IDBError> {
        if self.object_stores.contains_key(&name) {
            return Err(IDBError::ConstraintError("Object store already exists".to_string()));
        }
        
        let store = IDBObjectStore::new(name.clone(), params);
        self.object_stores.insert(name, store);
        Ok(())
    }

    pub fn delete_object_store(&mut self, name: &str) -> Result<(), IDBError> {
        self.object_stores.remove(name)
            .ok_or_else(|| IDBError::NotFoundError("Object store not found".to_string()))?;
        Ok(())
    }

    pub fn object_store(&self, name: &str) -> Result<&IDBObjectStore, IDBError> {
        self.object_stores.get(name)
            .ok_or_else(|| IDBError::NotFoundError("Object store not found".to_string()))
    }

    pub fn object_store_mut(&mut self, name: &str) -> Result<&mut IDBObjectStore, IDBError> {
        self.object_stores.get_mut(name)
            .ok_or_else(|| IDBError::NotFoundError("Object store not found".to_string()))
    }

    pub fn transaction(&self, store_names: Vec<String>, mode: IDBTransactionMode) -> Result<IDBTransaction, IDBError> {
        // Verify all stores exist
        for name in &store_names {
            if !self.object_stores.contains_key(name) {
                return Err(IDBError::NotFoundError(format!("Store '{}' not found", name)));
            }
        }
        
        Ok(IDBTransaction::new(store_names, mode))
    }
}

/// Factory for creating/opening databases
pub struct IDBFactory {
    databases: Arc<Mutex<HashMap<String, IDBDatabase>>>,
}

impl IDBFactory {
    pub fn new() -> Self {
        Self {
            databases: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn open(&self, name: &str, version: u64) -> Result<IDBDatabase, IDBError> {
        let mut databases = self.databases.lock().unwrap();
        
        if let Some(db) = databases.get(name) {
            if db.version > version {
                return Err(IDBError::VersionError("Requested version is lower than current".to_string()));
            }
            if db.version == version {
                return Ok(db.clone());
            }
        }
        
        // Create or upgrade database
        let db = IDBDatabase::new(name.to_string(), version);
        databases.insert(name.to_string(), db.clone());
        Ok(db)
    }

    pub fn delete_database(&self, name: &str) -> Result<(), IDBError> {
        let mut databases = self.databases.lock().unwrap();
        databases.remove(name)
            .ok_or_else(|| IDBError::NotFoundError("Database not found".to_string()))?;
        Ok(())
    }

    pub fn databases(&self) -> Vec<String> {
        let databases = self.databases.lock().unwrap();
        databases.keys().cloned().collect()
    }
}

impl Default for IDBFactory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idb_key_compare() {
        let key1 = IDBKey::Number(1);
        let key2 = IDBKey::Number(2);
        assert_eq!(key1.compare(&key2), std::cmp::Ordering::Less);
        assert_eq!(key2.compare(&key1), std::cmp::Ordering::Greater);
        assert_eq!(key1.compare(&key1), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_idb_key_range() {
        let range = IDBKeyRange::bound(
            IDBKey::Number(1),
            IDBKey::Number(10),
            false,
            false,
        );
        
        assert!(range.includes(&IDBKey::Number(1)));
        assert!(range.includes(&IDBKey::Number(5)));
        assert!(range.includes(&IDBKey::Number(10)));
        assert!(!range.includes(&IDBKey::Number(0)));
        assert!(!range.includes(&IDBKey::Number(11)));
    }

    #[test]
    fn test_factory_create_database() {
        let factory = IDBFactory::new();
        let db = factory.open("testdb", 1).unwrap();
        assert_eq!(db.name(), "testdb");
        assert_eq!(db.version(), 1);
    }

    #[test]
    fn test_factory_delete_database() {
        let factory = IDBFactory::new();
        factory.open("testdb", 1).unwrap();
        assert!(factory.delete_database("testdb").is_ok());
        assert_eq!(factory.databases().len(), 0);
    }

    #[test]
    fn test_create_object_store() {
        let mut db = IDBDatabase::new("testdb".to_string(), 1);
        let params = IDBObjectStoreParameters {
            key_path: None,
            auto_increment: true,
        };
        assert!(db.create_object_store("store1".to_string(), params).is_ok());
        assert_eq!(db.object_store_names().len(), 1);
    }

    #[test]
    fn test_object_store_add() {
        let params = IDBObjectStoreParameters {
            key_path: None,
            auto_increment: true,
        };
        let mut store = IDBObjectStore::new("test".to_string(), params);
        
        let value = serde_json::json!({"name": "test"});
        let key = store.add(value, None).unwrap();
        assert_eq!(key, IDBKey::Number(1));
        assert_eq!(store.count(), 1);
    }

    #[test]
    fn test_object_store_put() {
        let params = IDBObjectStoreParameters {
            key_path: None,
            auto_increment: false,
        };
        let mut store = IDBObjectStore::new("test".to_string(), params);
        
        let value = serde_json::json!({"name": "test"});
        let key = IDBKey::String("key1".to_string());
        store.put(value.clone(), Some(key.clone())).unwrap();
        
        assert_eq!(store.get(&key).unwrap(), &value);
    }

    #[test]
    fn test_object_store_delete() {
        let params = IDBObjectStoreParameters {
            key_path: None,
            auto_increment: false,
        };
        let mut store = IDBObjectStore::new("test".to_string(), params);
        
        let key = IDBKey::String("key1".to_string());
        store.put(serde_json::json!({"test": 1}), Some(key.clone())).unwrap();
        
        assert!(store.delete(&key).is_ok());
        assert_eq!(store.count(), 0);
    }

    #[test]
    fn test_object_store_clear() {
        let params = IDBObjectStoreParameters {
            key_path: None,
            auto_increment: true,
        };
        let mut store = IDBObjectStore::new("test".to_string(), params);
        
        store.add(serde_json::json!({"a": 1}), None).unwrap();
        store.add(serde_json::json!({"b": 2}), None).unwrap();
        assert_eq!(store.count(), 2);
        
        store.clear().unwrap();
        assert_eq!(store.count(), 0);
    }

    #[test]
    fn test_object_store_get_all() {
        let params = IDBObjectStoreParameters {
            key_path: None,
            auto_increment: false,
        };
        let mut store = IDBObjectStore::new("test".to_string(), params);
        
        store.put(serde_json::json!({"v": 1}), Some(IDBKey::Number(1))).unwrap();
        store.put(serde_json::json!({"v": 2}), Some(IDBKey::Number(2))).unwrap();
        store.put(serde_json::json!({"v": 3}), Some(IDBKey::Number(3))).unwrap();
        
        let all = store.get_all(None, None);
        assert_eq!(all.len(), 3);
        
        let range = IDBKeyRange::bound(IDBKey::Number(1), IDBKey::Number(2), false, false);
        let filtered = store.get_all(Some(&range), None);
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_create_index() {
        let params = IDBObjectStoreParameters {
            key_path: None,
            auto_increment: false,
        };
        let mut store = IDBObjectStore::new("test".to_string(), params);
        
        let index_params = IDBIndexParameters {
            unique: false,
            multi_entry: false,
        };
        
        assert!(store.create_index("nameIndex".to_string(), "name".to_string(), index_params).is_ok());
        assert!(store.index("nameIndex").is_ok());
    }

    #[test]
    fn test_cursor() {
        let keys = vec![
            IDBKey::Number(1),
            IDBKey::Number(2),
            IDBKey::Number(3),
        ];
        
        let mut cursor = IDBCursor::new(keys, IDBCursorDirection::Next);
        assert_eq!(cursor.key(), Some(&IDBKey::Number(1)));
        
        cursor.continue_cursor();
        assert_eq!(cursor.key(), Some(&IDBKey::Number(2)));
        
        cursor.advance(1);
        assert_eq!(cursor.key(), Some(&IDBKey::Number(3)));
        
        cursor.continue_cursor();
        assert!(!cursor.has_value());
    }

    #[test]
    fn test_transaction() {
        let db = IDBDatabase::new("testdb".to_string(), 1);
        let result = db.transaction(vec!["store1".to_string()], IDBTransactionMode::ReadOnly);
        assert!(result.is_err()); // Store doesn't exist
    }

    #[test]
    fn test_transaction_with_store() {
        let mut db = IDBDatabase::new("testdb".to_string(), 1);
        let params = IDBObjectStoreParameters::default();
        db.create_object_store("store1".to_string(), params).unwrap();
        
        let tx = db.transaction(vec!["store1".to_string()], IDBTransactionMode::ReadOnly).unwrap();
        assert_eq!(tx.mode(), IDBTransactionMode::ReadOnly);
        assert!(tx.is_active());
    }
}
