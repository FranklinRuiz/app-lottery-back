# Etapa de construcción
FROM rust:1.78-alpine as builder

# Instala las herramientas necesarias
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    pkgconfig \
    build-base \
    openssl-libs-static

# Establece el directorio de trabajo
WORKDIR /app

# Copia los archivos Cargo.toml y Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Copia todo el código fuente
COPY src ./src

# Construye el proyecto
RUN cargo build --release

# Verifica el contenido del directorio target
RUN ls -l /app/target/release

# Etapa final
FROM alpine:3.20.1

# Instala librerías necesarias
RUN apk add --no-cache \
    libgcc \
    libssl3 \
    ca-certificates

# Copia el binario construido desde la etapa de construcción
COPY --from=builder /app/target/release/app-lottery /usr/local/bin/app-lottery

# Establece el comando de inicio para ejecutar tu binario
CMD ["app-lottery"]

# Expone el puerto en el que tu aplicación se ejecuta
EXPOSE 9080