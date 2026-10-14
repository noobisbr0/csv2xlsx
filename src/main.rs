use std::env;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

use csv::ReaderBuilder;
use rust_xlsxwriter::{Format, FormatBorder, Workbook};
use spreadsheet_ods::style::CellStyle;
use spreadsheet_ods::{Sheet, WorkBook as OdsWorkBook};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Xlsx,
    Ods,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input.csv> [output.xlsx | output.ods]", args[0]);
        std::process::exit(1);
    }

    let input_path = Path::new(&args[1]);
    let output_path = if args.len() >= 3 {
        PathBuf::from(&args[2])
    } else {
        input_path.with_extension("xlsx")
    };

    let format = match output_path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_ascii_lowercase())
        .as_deref()
    {
        Some("ods") => OutputFormat::Ods,
        Some("xlsx") => OutputFormat::Xlsx,
        Some(other) => {
            eprintln!(
                "Warning: unknown extension '.{}', defaulting to XLSX.",
                other
            );
            OutputFormat::Xlsx
        }
        None => OutputFormat::Xlsx,
    };

    match format {
        OutputFormat::Xlsx => convert_to_xlsx(input_path, &output_path)?,
        OutputFormat::Ods => convert_to_ods(input_path, &output_path)?,
    }

    println!("Successfully converted {:?} to {:?}", input_path, output_path);
    Ok(())
}

fn convert_to_xlsx(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(input_path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(file);

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let header_format = Format::new()
        .set_bold()
        .set_border_bottom(FormatBorder::Thin);

    for (row_idx, result) in rdr.records().enumerate() {
        let record = result?;
        let row = row_idx as u32;

        for (col_idx, field) in record.iter().enumerate() {
            let col = col_idx as u16;

            if row == 0 {
                worksheet.write_string_with_format(row, col, field, &header_format)?;
            } else {
                write_xlsx_typed_cell(worksheet, row, col, field)?;
            }
        }
    }

    worksheet.autofit();
    workbook.save(output_path)?;
    Ok(())
}

fn write_xlsx_typed_cell(
    worksheet: &mut rust_xlsxwriter::Worksheet,
    row: u32,
    col: u16,
    val: &str,
) -> Result<(), Box<dyn Error>> {
    let trimmed = val.trim();

    if trimmed.is_empty() {
        return Ok(());
    }

    if let Ok(int_val) = trimmed.parse::<i64>() {
        worksheet.write_number(row, col, int_val as f64)?;
    } else if let Ok(float_val) = trimmed.parse::<f64>() {
        worksheet.write_number(row, col, float_val)?;
    } else if let Ok(bool_val) = trimmed.parse::<bool>() {
        worksheet.write_boolean(row, col, bool_val)?;
    } else {
        worksheet.write_string(row, col, val)?;
    }

    Ok(())
}

fn convert_to_ods(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(input_path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(file);

    let mut wb = OdsWorkBook::new_empty();
    let mut sheet = Sheet::new("Sheet1");

    let mut header_style = CellStyle::new_empty();
    header_style.set_name("HeaderStyle");
    header_style.set_font_bold();
    let header_style_ref = wb.add_cellstyle(header_style);

    for (row_idx, result) in rdr.records().enumerate() {
        let record = result?;
        let row = row_idx as u32;

        for (col_idx, field) in record.iter().enumerate() {
            let col = col_idx as u32;

            if row == 0 {
                sheet.set_value(row, col, field);
                sheet.set_cellstyle(row, col, &header_style_ref);
            } else {
                write_ods_typed_cell(&mut sheet, row, col, field);
            }
        }
    }

    wb.push_sheet(sheet);
    spreadsheet_ods::write_ods(&mut wb, output_path)?;
    Ok(())
}

fn write_ods_typed_cell(sheet: &mut Sheet, row: u32, col: u32, val: &str) {
    let trimmed = val.trim();

    if trimmed.is_empty() {
        return;
    }

    if let Ok(int_val) = trimmed.parse::<i64>() {
        sheet.set_value(row, col, int_val);
    } else if let Ok(float_val) = trimmed.parse::<f64>() {
        sheet.set_value(row, col, float_val);
    } else if let Ok(bool_val) = trimmed.parse::<bool>() {
        sheet.set_value(row, col, bool_val);
    } else {
        sheet.set_value(row, col, val);
    }
}
