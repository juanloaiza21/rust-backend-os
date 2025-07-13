pub mod data_lector;
pub mod disk_hash;
pub mod filters;
pub mod pagination;
pub mod trip_struct;

const CSV_PATH: &str = "src/data/data.csv";

use filters::{
    TripFilter, filter_to_file, filter_with_pagination, get_filter_stats, get_popular_destinations,
    get_trip_by_index,
};
use pagination::{PagedResult, Pagination};
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;
use trip_struct::Trip;

pub struct FilterResult {
    count: usize,
    time: std::time::Duration,
    output_file: String,
}

pub struct StatsResult {
    stats: HashMap<String, f64>,
    time: std::time::Duration,
}

pub struct PopularDestinationsResult {
    destinations: Vec<(String, usize)>,
    time: std::time::Duration,
}

pub fn get_trips_by_index(index: &str) -> Result<Option<Trip>, Box<dyn Error>> {
    get_trip_by_index(CSV_PATH, index)
}

pub fn get_trips_by_price_range(
    min_price: f64,
    max_price: f64,
    pagination: Pagination,
) -> Result<PagedResult<Trip>, Box<dyn Error>> {
    let filter = TripFilter::Price {
        min: Some(min_price),
        max: Some(max_price),
    };

    filter_with_pagination(CSV_PATH, filter, pagination)
}

pub fn get_trips_by_destination(
    destination: &str,
    pagination: Pagination,
) -> Result<PagedResult<Trip>, Box<dyn Error>> {
    let filter = TripFilter::Destination(destination.to_string());

    filter_with_pagination(CSV_PATH, filter, pagination)
}

pub fn get_trips_with_complex_filter(
    filter: TripFilter,
    pagination: Pagination,
) -> Result<PagedResult<Trip>, Box<dyn Error>> {
    filter_with_pagination(CSV_PATH, filter, pagination)
}
