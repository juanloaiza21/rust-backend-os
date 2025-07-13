FROM rustlang/rust:nightly as builder

# Instalar Python y dependencias necesarias
RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    pkg-config \
    libssl-dev \
    sed

WORKDIR /usr/src/app
COPY . .

# Ejecutar el script para generar el CSV
RUN cd src/data && python3 datagen.py

# Modificar temporalmente Cargo.toml para cambiar la edición a 2021
RUN sed -i 's/edition = "2024"/edition = "2021"/' Cargo.toml

# Habilitar las características inestables necesarias
RUN touch src/lib.rs
RUN echo '#![feature(lazy_cell)]' > src/lib.rs
RUN echo '#![feature(file_create_new)]' >> src/lib.rs

# Compilar para release
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Crear la estructura de directorios necesaria
RUN mkdir -p /app/src/data

# Copia el binario compilado
COPY --from=builder /usr/src/app/target/release/rust-backend-os /app/rust-backend-os

# Copia los archivos de datos generados
COPY --from=builder /usr/src/app/src/data/data.csv /app/src/data/data.csv

# Puerto expuesto - usar variable de entorno PORT para Cloud Run
EXPOSE 8080

# Comando para ejecutar con soporte para PORT variable
CMD ["sh", "-c", "PORT=${PORT:-3000} ./rust-backend-os"]
