# csv2xlsx

Консольная утилита на Rust для конвертации CSV в форматы таблиц Excel (`.xlsx`) и OpenDocument Spreadsheet (`.ods`) с автоматическим определением типов данных и выделением строки заголовков.

## Требования

Установленный [Rust](https://www.rust-lang.org/) (Cargo).

## Сборка

```bash
cargo build --release
```

## Использование

```bash
cargo run -- <input.csv> [output.xlsx | output.ods]
```

* `<input.csv>` — путь к входному файлу CSV.
* `[output]` — путь к итоговому файлу. Целевой формат (`xlsx` или `ods`) определяется автоматически по расширению. Если аргумент не указан, по умолчанию создается файл `.xlsx`.

### Примеры

Конвертация в Excel:
```bash
cargo run -- data.csv report.xlsx
```

Конвертация в ODS (LibreOffice / OpenOffice):
```bash
cargo run -- data.csv report.ods
```
