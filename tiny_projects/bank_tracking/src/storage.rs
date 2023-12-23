use chrono::Datelike;
use std::path::Path;

use crate::transaction::*;

pub fn month_csv(csvs_folder: &Path, year: i32, month: u32) -> String {
    csvs_folder
        .join(format!("transactions-{}-{}.csv", year, month))
        .into_os_string()
        .into_string()
        .unwrap()
}

pub fn output_monthly(output_dir: &Path, transactions: &[Transaction]) {
    if !(output_dir.exists() && output_dir.is_dir()) {
        return;
    }

    for (year, month) in transactions.iter().map(|v| (v.date.year(), v.date.month())) {
        let mut writer = csv::Writer::from_path(month_csv(&output_dir, year, month)).unwrap();

        for t in transactions
            .iter()
            .filter(|v| v.date.year() == year && v.date.month() == month)
        {
            writer.serialize(t).unwrap();
        }
    }
}

pub fn read_many_months(output_dir: &Path, year_months: &[(i32, u32)]) -> Vec<Transaction> {
    year_months
        .iter()
        .map(|(y, m)| read_month(output_dir, *y, *m))
        .flatten()
        .collect()
}

pub fn read_month(output_dir: &Path, year: i32, month: u32) -> Vec<Transaction> {
    let month_csv_path = month_csv(&output_dir, year, month);

    let Ok(mut reader) =
                csv::Reader::from_path(&month_csv_path) else {
                return Vec::new();
            };

    reader
        .records()
        .into_iter()
        .map(|val| val.unwrap().deserialize::<Transaction>(None).unwrap())
        .collect::<Vec<_>>()
}
