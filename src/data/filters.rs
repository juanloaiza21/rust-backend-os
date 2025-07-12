use super::disk_hash::DiskHashTable;
use super::trip_struct::Trip;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use std::sync::Mutex;
use std::sync::Once;

const HASH_DIR: &str = "tmp/hash_index";
static HASH_INIT: Once = Once::new();
static HASH_TABLE: LazyLock<Mutex<Option<DiskHashTable>>> = LazyLock::new(|| Mutex::new(None));

pub enum TripFilter {
    Price { min: Option<f64>, max: Option<f64> },
    Index(String),
    Destination(String),
    And(Vec<TripFilter>),
    Or(Vec<TripFilter>),
}

impl TripFilter {
    pub fn matches(&self, trip: &Trip) -> bool {
        match self {
            TripFilter::Price { min, max } => {
                let price = trip.total_amount.parse::<f64>().unwrap_or(0.0);
                let min_check = min.map_or(true, |min_val| price >= min_val);
                let max_check = max.map_or(true, |max_val| price <= max_val);

                min_check && max_check
            }
            TripFilter::Index(target_index) => trip.index == *target_index,
            TripFilter::Destination(target_dest) => trip.do_location_id == *target_dest,
            TripFilter::And(filters) => filters.iter().all(|filter| filter.matches(trip)),
            TripFilter::Or(filters) => filters.iter().any(|filter| filter.matches(trip)),
        }
    }
}

fn get_or_initialize_hash_table<P: AsRef<Path>>(
    csv_path: P,
) -> Result<&'static Mutex<Option<DiskHashTable>>, Box<dyn Error>> {
    HASH_INIT.call_once(|| {
        println!("Inicializando tabla hash en disco...");
        let hash_path = PathBuf::from(HASH_DIR);
        let needs_build = if hash_path.exists() {
            let table_path = hash_path.join("hash_table.bin");
            let data_path = hash_path.join("trip_data.bin");

            !(table_path.exists()
                && data_path.exists()
                && fs::metadata(&table_path)
                    .map(|m| m.len() > 0)
                    .unwrap_or(false))
        } else {
            true
        };

        if !hash_path.exists() {
            if let Err(e) = fs::create_dir_all(&hash_path) {
                eprintln!("Error al crear directorio hash: {}", e);
                return;
            }
        }
        match DiskHashTable::new(&hash_path) {
            Ok(hash_table) => {
                let mut table_ref = HASH_TABLE.lock().unwrap();
                *table_ref = Some(hash_table);

                if needs_build {
                    println!("Construyendo índice hash desde CSV...");

                    // Convertir ambos paths a String para asegurar que sean del mismo tipo
                    let csv_path_str = csv_path.as_ref().to_string_lossy().to_string();
                    let hash_path_str = hash_path.to_string_lossy().to_string();

                    // Pasar ambos como referencias a String
                    match DiskHashTable::build_hash_table_from_csv(&csv_path_str, &hash_path_str) {
                        Ok(count) => println!("Índice hash construido con {} registros", count),
                        Err(e) => eprintln!("Error al construir índice hash: {}", e),
                    }
                } else {
                    println!("Usando índice hash existente");
                }
            }
            Err(e) => {
                eprintln!("Error al inicializar tabla hash: {}", e);
            }
        }
    });

    Ok(&HASH_TABLE)
}

fn can_use_hash_index(filter: &TripFilter) -> Option<String> {
    match filter {
        TripFilter::Index(idx) => Some(idx.clone()),
        TripFilter::And(filters) => {
            for f in filters {
                if let TripFilter::Index(idx) = f {
                    return Some(idx.clone());
                }
            }
            None
        }
        _ => None,
    }
}

pub fn filter_to_file<P: AsRef<Path>>(
    csv_path: P,
    output_file: P,
    filter: TripFilter,
    max_results: Option<usize>,
) -> Result<usize, Box<dyn Error>> {
    let output_file = output_file.as_ref();

    if let Some(parent) = output_file.parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(output_file)?;
    let mut writer = BufWriter::new(file);
    writeln!(
        writer,
        "vendor_id,tpep_pickup_datetime,tpep_dropoff_datetime,passenger_count,trip_distance,ratecode_id,store_and_fwd_flag,pu_location_id,do_location_id,payment_type,fare_amount,extra,mta_tax,tip_amount,tolls_amount,improvement_surcharge,total_amount,congestion_surcharge,index"
    )?;

    let mut count = 0;
    if let Some(index) = can_use_hash_index(&filter) {
        println!(
            "Usando índice hash para búsqueda rápida por índice: {}",
            index
        );
        let hash_table_ref = get_or_initialize_hash_table(&csv_path)?;
        if let Some(hash_table) = hash_table_ref.lock().unwrap().as_ref() {
            if let Ok(Some(trip)) = hash_table.get(&index) {
                if filter.matches(&trip) {
                    writeln!(
                        writer,
                        "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                        trip.vendor_id,
                        trip.tpep_pickup_datetime,
                        trip.tpep_dropoff_datetime,
                        trip.passenger_count,
                        trip.trip_distance,
                        trip.ratecode_id,
                        trip.store_and_fwd_flag,
                        trip.pu_location_id,
                        trip.do_location_id,
                        trip.payment_type,
                        trip.fare_amount,
                        trip.extra,
                        trip.mta_tax,
                        trip.tip_amount,
                        trip.tolls_amount,
                        trip.improvement_surcharge,
                        trip.total_amount,
                        trip.congestion_surcharge,
                        trip.index
                    )?;
                    count = 1;
                }
            }
            writer.flush()?;
            return Ok(count);
        }
    }

    println!("Usando escaneo secuencial de CSV para filtrado");

    super::data_lector::stream_process_csv(csv_path, |trip| {
        if filter.matches(trip) {
            writeln!(
                writer,
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                trip.vendor_id,
                trip.tpep_pickup_datetime,
                trip.tpep_dropoff_datetime,
                trip.passenger_count,
                trip.trip_distance,
                trip.ratecode_id,
                trip.store_and_fwd_flag,
                trip.pu_location_id,
                trip.do_location_id,
                trip.payment_type,
                trip.fare_amount,
                trip.extra,
                trip.mta_tax,
                trip.tip_amount,
                trip.tolls_amount,
                trip.improvement_surcharge,
                trip.total_amount,
                trip.congestion_surcharge,
                trip.index
            )?;

            count += 1;

            if let Some(max) = max_results {
                if count >= max {
                    return Err("Límite de resultados alcanzado".into());
                }
            }
        }

        Ok(())
    })
    .or_else(|e| {
        if e.to_string() == "Límite de resultados alcanzado" {
            Ok(())
        } else {
            Err(e)
        }
    })?;

    writer.flush()?;

    Ok(count)
}

pub fn get_filter_stats<P: AsRef<Path>>(
    csv_path: P,
    filter: TripFilter,
) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    let mut stats = HashMap::new();
    let mut count = 0;
    let mut total_distance = 0.0;
    let mut total_amount = 0.0;
    let mut total_passengers = 0;
    if let Some(index) = can_use_hash_index(&filter) {
        println!("Usando índice hash para estadísticas por índice: {}", index);

        let hash_table_ref = get_or_initialize_hash_table(&csv_path)?;
        if let Some(hash_table) = hash_table_ref.lock().unwrap().as_ref() {
            if let Ok(Some(trip)) = hash_table.get(&index) {
                if filter.matches(&trip) {
                    count = 1;
                    total_distance = trip.trip_distance.parse::<f64>().unwrap_or(0.0);
                    total_amount = trip.total_amount.parse::<f64>().unwrap_or(0.0);
                    total_passengers = trip.passenger_count.parse::<i32>().unwrap_or(0);

                    stats.insert("count".to_string(), count as f64);
                    stats.insert("avg_distance".to_string(), total_distance);
                    stats.insert("avg_amount".to_string(), total_amount);
                    stats.insert("avg_passengers".to_string(), total_passengers as f64);
                    stats.insert("total_amount".to_string(), total_amount);

                    return Ok(stats);
                }
            }
        }
    }

    println!("Usando escaneo secuencial de CSV para estadísticas");

    super::data_lector::stream_process_csv(csv_path, |trip| {
        if filter.matches(trip) {
            count += 1;
            total_distance += trip.trip_distance.parse::<f64>().unwrap_or(0.0);
            total_amount += trip.total_amount.parse::<f64>().unwrap_or(0.0);
            total_passengers += trip.passenger_count.parse::<i32>().unwrap_or(0);
        }

        Ok(())
    })?;

    stats.insert("count".to_string(), count as f64);

    if count > 0 {
        stats.insert("avg_distance".to_string(), total_distance / count as f64);
        stats.insert("avg_amount".to_string(), total_amount / count as f64);
        stats.insert(
            "avg_passengers".to_string(),
            total_passengers as f64 / count as f64,
        );
        stats.insert("total_amount".to_string(), total_amount);
    }

    Ok(stats)
}

pub fn get_popular_destinations<P: AsRef<Path>>(
    csv_path: P,
    limit: usize,
) -> Result<Vec<(String, usize)>, Box<dyn Error>> {
    let mut dest_counts: HashMap<String, usize> = HashMap::new();

    super::data_lector::stream_process_csv(csv_path, |trip| {
        let dest = &trip.do_location_id;
        *dest_counts.entry(dest.clone()).or_insert(0) += 1;

        Ok(())
    })?;

    let mut dest_vec: Vec<(String, usize)> = dest_counts.into_iter().collect();
    dest_vec.sort_by(|a, b| b.1.cmp(&a.1));

    // Limitar resultados
    let result = dest_vec.into_iter().take(limit).collect();

    Ok(result)
}

pub fn initialize_hash_index<P: AsRef<Path>>(csv_path: P) -> Result<usize, Box<dyn Error>> {
    println!("Inicializando índice hash manualmente...");
    let hash_path = PathBuf::from(HASH_DIR);

    if hash_path.exists() {
        println!("Eliminando índice hash existente...");
        fs::remove_dir_all(&hash_path)?;
    }

    fs::create_dir_all(&hash_path)?;

    println!("Construyendo nuevo índice hash...");

    let csv_path_str = csv_path.as_ref().to_string_lossy().to_string();
    let hash_path_str = hash_path.to_string_lossy().to_string();

    let count = DiskHashTable::build_hash_table_from_csv(&csv_path_str, &hash_path_str)?;
    let hash_table = DiskHashTable::new(&hash_path)?;
    let mut table_ref = HASH_TABLE.lock().unwrap();
    *table_ref = Some(hash_table);

    println!("Índice hash inicializado con {} registros", count);

    Ok(count)
}
