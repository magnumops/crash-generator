# --- STAGE 1: BUILDER (RUST) ---
FROM rust:1.82-slim-bookworm as builder
WORKDIR /usr/src/magnum
COPY Cargo.toml .
COPY src ./src
RUN cargo build --release

# --- STAGE 2: RUNTIME (PYTHON) ---
FROM python:3.12-slim-bookworm
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends curl && rm -rf /var/lib/apt/lists/*

# FIX: Ставим uvicorn[standard] для полноценной работы WebSocket
RUN pip install --no-cache-dir \
    fpdf2 matplotlib pandas \
    requests \
    z3-solver \
    fastapi \
    "uvicorn[standard]" \
    httpx

COPY --from=builder /usr/src/magnum/target/release/magnum /usr/local/bin/magnum
COPY vendor/logos /app/vendor/logos
ENV MAGNUM_PYTHON_EXEC=/usr/local/bin/python
ENV MAGNUM_IN_DOCKER=true
ENV PYTHONPATH=/app/vendor/logos

ENTRYPOINT ["magnum"]
CMD ["--help"]
