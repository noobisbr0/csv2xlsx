# csv2xlsx

Простой консольный инструмент на Rust для конвертации CSV файлов в формат Excel (`.xlsx`) с автоматическим определением типов данных и форматированием шапки.

## Требования

У вас должен быть установлен [Rust](https://www.rust-lang.org/) (Cargo).

## Сборка

```bash
cargo build --release
```

## Использование

```bash
cargo run -- <input.csv> [output.xlsx]
```

* `<input.csv>` — путь к исходному CSV файлу.
* `[output.xlsx]` — необязательный путь к итоговому файлу Excel (если не указан, сохранится рядом с CSV с тем же именем, но расширением `.xlsx`).

### Пример:

```bash
cargo run -- data.csv result.xlsx