# --- STAGE 1: BUILDER (RUST) ---
FROM rust:1.82-slim-bookworm as builder

WORKDIR /usr/src/magnum

# Копируем манифесты
COPY Cargo.toml .
# Копируем исходный код
COPY src ./src

# Собираем релизную версию (оптимизирована по скорости и размеру)
RUN cargo build --release

# --- STAGE 2: RUNTIME (PYTHON) ---
FROM python:3.12-slim-bookworm

WORKDIR /app

# Устанавливаем системные зависимости (если нужны для Z3 или Pandas)
# Обычно slim образ уже содержит минимум необходимого
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Устанавливаем Python-зависимости
# Мы делаем это одним слоем для экономии места
RUN pip install --no-cache-dir \
    pandas \
    requests \
    z3-solver \
    fastapi \
    uvicorn \
    httpx

# Копируем скомпилированный бинарник Rust из первого этапа
COPY --from=builder /usr/src/magnum/target/release/magnum /usr/local/bin/magnum

# Копируем Python-ядро (Logos)
COPY vendor/logos /app/vendor/logos

# Настраиваем переменные окружения для Magnum
ENV MAGNUM_PYTHON_EXEC=/usr/local/bin/python
ENV MAGNUM_IN_DOCKER=true
ENV PYTHONPATH=/app/vendor/logos

# Точка входа
ENTRYPOINT ["magnum"]
CMD ["--help"]
