use super::trip_struct::Trip;
use odht::{Config, FxHashFn, HashTable, HashTableOwned};
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::fs::{File, OpenOptions, create_dir_all};
use std::hash::{Hash, Hasher};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

struct TripHashConfig;

impl Config for TripHashConfig {
    type Key = u64;
    type Value = u32; //Se usa U8 porque lo solicita la libreria.

    type EncodedKey = [u8; 8];
    type EncodedValue = [u8; 4];

    type H = FxHashFn;

    #[inline]
    fn encode_key(k: &Self::Key) -> Self::EncodedKey {
        k.to_le_bytes()
    }

    #[inline]
    fn encode_value(v: &Self::Value) -> Self::EncodedValue {
        v.to_le_bytes()
    }

    #[inline]
    fn decode_key(k: &Self::EncodedKey) -> Self::Key {
        u64::from_le_bytes(*k)
    }

    #[inline]
    fn decode_value(v: &Self::EncodedValue) -> Self::Value {
        u32::from_le_bytes(*v)
    }
}

fn calculate_hash(key: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

pub struct DiskHashTable {
    table_path: PathBuf,
    data_path: PathBuf,
    //De esta forma no almacenamos la tabla en la memoria permanentemente.
}

impl DiskHashTable {
    //Create
    pub fn new<P: AsRef<Path>>(dir_path: P) -> Result<Self, Box<dyn Error>> {
        let dir_path = dir_path.as_ref();
        create_dir_all(dir_path)?;

        let table_path = dir_path.join("hash_table.bin");
        let data_path = dir_path.join("trip_data.bin");

        if !table_path.exists() {
            let builder = HashTableOwned::<TripHashConfig>::with_capacity(4, 90);
            let data = builder.raw_bytes();

            let mut file = File::create(&table_path)?;
            file.write_all(data)?;
        }

        if !data_path.exists() {
            File::create_new(&data_path)?;
        }

        Ok(Self {
            table_path,
            data_path,
        })
    }

    pub fn insert(&self, key: String, trip: Trip) -> Result<(), Box<dyn Error>> {
        let key_hash = calculate_hash(&key);
        let mut data_file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(&self.data_path)?;
        let position = data_file.metadata()?.len() as u32;
        let trip_bytes = serde_json::to_vec(&trip)?;
        let data_size = trip_bytes.len() as u32;
        data_file.write_all(&data_size.to_le_bytes())?;
        data_file.write_all(&trip_bytes)?;
        let mut table_file = OpenOptions::new().read(true).open(&self.table_path)?;
        let mut table_data = Vec::new();
        table_file.read_to_end(&mut table_data)?;
        let mut builder = HashTableOwned::<TripHashConfig>::from_raw_bytes(&table_data)?;
        builder.insert(&key_hash, &position);
        let mut table_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.table_path)?;

        table_file.write_all(builder.raw_bytes())?;

        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<Trip>, Box<dyn Error>> {
        let key_hash = calculate_hash(key);
        let mut table_file = File::open(&self.table_path)?;
        let mut table_data = Vec::new();
        table_file.read_to_end(&mut table_data)?;
        let table = HashTable::<TripHashConfig, &[u8]>::from_raw_bytes(&table_data)?;
        if let Some(position) = table.get(&key_hash) {
            let mut data_file = File::open(&self.data_path)?;
            data_file.seek(SeekFrom::Start(position as u64))?;
            let mut size_bytes = [0u8; 4];
            data_file.read_exact(&mut size_bytes)?;
            let data_size = u32::from_le_bytes(size_bytes) as usize;
            let mut trip_bytes = vec![0u8; data_size];
            data_file.read_exact(&mut trip_bytes)?;
            let trip = serde_json::from_slice(&trip_bytes)?;
            Ok(Some(trip))
        } else {
            Ok(None)
        }
    }

    pub fn count_entries(&self) -> Result<usize, Box<dyn Error>> {
        let table_file = File::open(&self.table_path)?;
        let mut reader = BufReader::new(table_file);
        let mut table_data = Vec::new();
        reader.read_to_end(&mut table_data)?;
        let table = HashTable::<TripHashConfig, &[u8]>::from_raw_bytes(&table_data)?;
        Ok(table.len())
    }

    pub fn build_hash_table_from_csv<P: AsRef<Path>>(
        csv_path: P,
        hash_dir: P,
    ) -> Result<usize, Box<dyn Error>> {
        let hash_table = DiskHashTable::new(&hash_dir)?;
        let data_path = hash_dir.as_ref().join("trip_data.bin");
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&data_path)?;
        let builder = HashTableOwned::<TripHashConfig>::with_capacity(16, 90);
        let table_path = hash_dir.as_ref().join("hash_table.bin");
        let mut table_file = File::create(&table_path)?;
        table_file.write_all(builder.raw_bytes())?;
        let mut count = 0;
        super::data_lector::stream_process_csv(csv_path, |trip| {
            let key = trip.index.clone();
            hash_table.insert(key, trip.clone())?;

            count += 1;
            if count % 1000 == 0 {
                println!("Procesados {} registros...", count);
            }

            Ok(())
        })?;

        println!("Total de registros procesados: {}", count);

        Ok(count)
    }
}
