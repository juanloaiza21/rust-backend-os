# Rust Backend OS

A Rust-based backend system developed for an Operating Systems bachelor class, featuring efficient data indexing and API endpoints.

## 📋 Project Overview

This project demonstrates the implementation of a high-performance backend system using Rust, focusing on data processing, indexing, and efficient memory management principles relevant to operating systems concepts.

## 🏗️ Repository Structure

```
.
├── Cargo.toml          # Project dependencies and configuration
├── Dockerfile          # Container configuration for deployment
└── src/
    ├── data/           # Data generation and storage
    │   ├── data.csv    # Our data file generated.
    │   ├── datagen.py  # Python script for generating test data
    │   ├── data_lector.rs  # Rust lector for the csv data
    │   ├── disk_hash.rs  # ODHT implementation for mem efficency
    │   ├── filters.rs  # Filter implementation for an agile search on our generated hash tables
    │   ├── mod.rs      # Middleware with the logic of the module for its use on endpoints
    │   ├── pagination.rs  # mem efficency filte for the generation of results
    │   ├── trip_struct.rs  # Trip data struct
    ├── router_local/   # API routing definitions
    │   ├── mod.rs      # Main router configuration
    │   ├── trip_rorutes.rs # Trip-specific routes
    │   └── data_intput_struct.rs # Input data structures
    ├── utils/          # Utility functions and helpers
    └── main.rs         # Application entry point
```

## 🛠️ Tech Stack

- **Backend Framework**: [Axum](https://github.com/tokio-rs/axum) - A modern Rust web framework
- **Async Runtime**: [Tokio](https://tokio.rs/) - Asynchronous runtime for Rust
- **Data Processing**: CSV parsing with the `csv` crate
- **Data Structure**: [ODHT](https://docs.rs/odht/latest/odht/) (On Disk Hash Table) for efficient indexing
- **Serialization**: Serde for JSON handling
- **Middleware**: Tower and Tower-HTTP for service composition
- **Deployment**: Google Cloud Platform

## ⚙️ Data Handling and Indexing

### Dataset Reduction Strategy

A significant challenge encountered in this project was managing dataset size constraints during cloud deployment:

- **Problem**: The complete dataset indexing process exceeded Google Cloud's deployment time limits, causing failed deployments.
  
- **Solution**: Implemented a strategic dataset reduction approach:
  1. Reduced the original dataset to a more manageable size while preserving key data distribution
  2. Modified `datagen.py` to generate a smaller but representative dataset
  3. Optimized the indexing algorithm in Rust for faster processing
  4. Implemented lazy loading patterns to defer non-critical data processing
  
- **Technical Implementation**:
  - Used [ODHT](https://docs.rs/odht/latest/odht/) (On Disk Hash Table) for memory-efficient (RAM and ROM) indexing
  - Optimized CSV parsing to handle only essential fields
  - Implemented streaming processing where appropriate to reduce memory footprint

This approach successfully reduced deployment times while maintaining the system's core functionality and demonstrating important OS concepts like efficient memory management and resource allocation.

## 🚀 Running Locally

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (nightly toolchain)
- Python 3 (for data generation)

### Setup and Run

```bash
# Clone the repository
git clone https://github.com/juanloaiza21/rust-backend-os.git
cd rust-backend-os

# Generate the test data
cd src/data
python3 datagen.py
cd ../..

# Run the application
cargo run
```

## 🐳 Docker Deployment

The project includes a Dockerfile for containerized deployment:

```bash
# Build the Docker image
docker build -t rust-backend-os .

# Run the container
docker run -p 8080:8080 rust-backend-os
```

## ☁️ Google Cloud Deployment

The application is configured to deploy on Google Cloud Platform, particularly Cloud Run:

```bash
# Build and push to Google Container Registry
gcloud builds submit --tag gcr.io/YOUR-PROJECT-ID/rust-backend-os

# Deploy to Cloud Run
gcloud run deploy --image gcr.io/YOUR-PROJECT-ID/rust-backend-os --platform managed
```

Note: The container is configured to use the `PORT` environment variable that Cloud Run provides.

## 🔍 API Endpoints and Usage

The API provides several endpoints for data access:

### Base Endpoints

```
GET /           - Hello world test endpoint
GET /api        - API status endpoint
```

### Trip Endpoints

```
GET /trip/{id}                    - Get trip by ID
GET /trip/price                   - Get trips by price range (with query parameters)
GET /trip/destination/{dest}      - Get trips by destination (with pagination)
```

### Example API Calls with curl

#### Hello World Test
```bash
curl http://localhost:8080/
```
Expected response: `hello_world!`

#### API Status
```bash
curl http://localhost:8080/api
```
Expected response: OK status

#### Get Trip by ID
```bash
curl http://localhost:8080/trip/ABC123
```
Expected response: JSON with trip details for ID "ABC123"

#### Get Trips by Price Range
```bash
curl "http://localhost:8080/trip/price?min=100&max=500&page=1&per_page=10"
```
Expected response: JSON with trips priced between $100 and $500, showing the first page with 10 results per page

#### Get Trips by Destination
```bash
curl "http://localhost:8080/trip/destination/Paris?page=1&per_page=20"
```
Expected response: JSON with trips to destination "Paris", showing the first page with 20 results per page

### Request Parameters

#### For Price Range Queries:
- `min`: Minimum price (optional, defaults to 0.0)
- `max`: Maximum price (optional, defaults to maximum possible value)
- `page`: Page number (optional, defaults to 1)
- `per_page`: Results per page (optional, defaults to 50)

#### For Destination Queries:
- `page`: Page number (optional, defaults to 1)
- `per_page`: Results per page (optional, defaults to 50)

## 🧪 Future Improvements

- Add authentication and authorization
- Develop a more comprehensive test suite
- Further optimize indexing for larger datasets
- Implement incremental indexing capability
- Add more advanced filtering options for trip searches

## 📝 License

This project is open source and available under the [MIT License](LICENSE).
