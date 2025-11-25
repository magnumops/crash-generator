# Project Magnum (Logos Ops Suite)

## Архитектура "Матрешка"
* **Outer Shell (Rust):** CLI Orchestrator (`magnum`). Управляет процессами, аргументами и портами.
* **Inner Core (Python):** Submodule `vendor/logos`. Содержит бизнес-логику, Z3 Solver и коннекторы к API.

## Протокол Первого Запуска

1. **Клонирование и Сборка:**
   ```bash
   git clone --recursive <repo_url>
   cd crash-generator
   python3 -m venv venv
   source venv/bin/activate
   pip install -r vendor/logos/requirements.txt  # или вручную pandas requests z3-solver fastapi uvicorn httpx
   cargo build
   ```

2. **Режим 1: The Pathologist (Анализ)**
   ```bash
   # Анализ CSV файла на предмет манипуляций ликвидностью
   cargo run -- analyze --file /path/to/evidence.csv
   ```
   *Критерий успеха:* Вывод `LIQUIDITY_VOID_DETECTED` (если сделка фейк) или `CLEAN`.

3. **Режим 2: The Crash Generator (Атака)**
   ```bash
   # Запуск прокси-сервера (Fake Exchange) на порту 8080
   cargo run -- crash --port 8080
   ```
   *Активация атаки:* GET запрос с параметром `crash_mode=true`.

## Структура Зависимостей
* `magnum` (Rust) -> зависит от `venv/bin/python3`
* `logos` (Python) -> зависит от `z3-solver`
