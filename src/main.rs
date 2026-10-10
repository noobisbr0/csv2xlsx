use std::env;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use csv::ReaderBuilder;
use rust_xlsxwriter::{Format, FormatBorder, Workbook};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input.csv> [output.xlsx]", args[0]);
        std::process::exit(1);
    }

    let input_path = Path::new(&args[1]);
    let output_path = if args.len() >= 3 {
        PathBuf::from(&args[2])
    } else {
        input_path.with_extension("xlsx")
    };

    let file = File::open(input_path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(file);

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Fixed: set_border_bottom instead of set_bottom
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
                write_typed_cell(worksheet, row, col, field)?;
            }
        }
    }

    worksheet.autofit();
    workbook.save(&output_path)?;
    println!("Successfully converted {:?} to {:?}", input_path, output_path);

    Ok(())
}

fn write_typed_cell(
    worksheet: &mut rust_xlsxwriter::Worksheet,
    row: u32,
    col: u16,
    val: &str,
) -> Result<(), Box<dyn Error>> {
    let trimmed = val.trim();

    // If empty, skipping the cell leaves it naturally blank in Excel.
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
