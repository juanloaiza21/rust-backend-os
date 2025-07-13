pub mod data_lector;
pub mod disk_hash;
pub mod filters;
pub mod trip_struct;

const CSV_PATH: &str = "src/data/data.csv";

use filters::{TripFilter, filter_to_file, get_filter_stats, get_popular_destinations};
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

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

pub fn filter_by_index(index: &str) -> Result<FilterResult, Box<dyn Error>> {
    let start = Instant::now();

    let filter = TripFilter::Index(index.to_string());
    let output_file = format!("output/filtered_by_index_{}.csv", index);

    let count = filter_to_file(CSV_PATH, &output_file, filter, None)?;

    Ok(FilterResult {
        count,
        time: start.elapsed(),
        output_file,
    })
}

pub fn filter_by_price_range(
    min_price: f64,
    max_price: f64,
) -> Result<FilterResult, Box<dyn Error>> {
    let start = Instant::now();

    let filter = TripFilter::Price {
        min: Some(min_price),
        max: Some(max_price),
    };
    let output_file = format!("output/filtered_by_price_{}_{}.csv", min_price, max_price);

    let count = filter_to_file(CSV_PATH, &output_file, filter, Some(1000))?;

    Ok(FilterResult {
        count,
        time: start.elapsed(),
        output_file,
    })
}

pub fn filter_by_destination(destination: &str) -> Result<FilterResult, Box<dyn Error>> {
    let start = Instant::now();

    let filter = TripFilter::Destination(destination.to_string());
    let output_file = format!("output/filtered_by_destination_{}.csv", destination);

    let count = filter_to_file(CSV_PATH, &output_file, filter, Some(1000))?;

    Ok(FilterResult {
        count,
        time: start.elapsed(),
        output_file,
    })
}

pub fn get_stats_for_price_range(
    min_price: f64,
    max_price: f64,
) -> Result<StatsResult, Box<dyn Error>> {
    let start = Instant::now();

    let filter = TripFilter::Price {
        min: Some(min_price),
        max: Some(max_price),
    };

    let stats = get_filter_stats(CSV_PATH, filter)?;

    Ok(StatsResult {
        stats,
        time: start.elapsed(),
    })
}

pub fn find_popular_destinations(
    limit: usize,
) -> Result<PopularDestinationsResult, Box<dyn Error>> {
    let start = Instant::now();

    let destinations = get_popular_destinations(CSV_PATH, limit)?;

    Ok(PopularDestinationsResult {
        destinations,
        time: start.elapsed(),
    })
}
