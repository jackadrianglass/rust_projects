#![allow(dead_code)]
use chrono::prelude::NaiveDate;
use chrono::Datelike;
use clap::{CommandFactory, Parser, Subcommand};
use csv::StringRecord;
use serde::{Deserialize, Serialize};
use std::io::stdin;
use std::path::{Path};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// read in a new data set for later analysis
    Read { file_path: String },
    /// print out report of overall spending
    Report,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum TransactionKind {
    EssentialFood,
    FunFood,
    Recurring,
    Essential,
    Investment,
    Fun,
}

impl TransactionKind {
    fn from_usize(val: usize) -> Option<Self> {
        match val {
            1 => Some(Self::EssentialFood),
            2 => Some(Self::FunFood),
            3 => Some(Self::Recurring),
            4 => Some(Self::Essential),
            5 => Some(Self::Fun),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    kind: Option<TransactionKind>,
    date: NaiveDate,
    description: String,
    cad: f32,
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date && self.description == other.description && self.cad == other.cad
    }
}

/// Expecting the record to be in the format of
/// - Account type
/// - Account number
/// - Transaction date
/// - Cheque number
/// - Description 1
/// - Description 2
/// - cad
/// - usd
fn get_transactions(records: &[StringRecord]) -> Vec<Transaction> {
    records
        .iter()
        .map(|val| Transaction {
            kind: None,
            date: NaiveDate::parse_from_str(&val[2], "%m/%d/%Y").unwrap(),
            description: val[4].to_string(),
            cad: val[6].parse().unwrap(),
        })
        .collect()
}

fn categorize_transactions(transactions: &mut [Transaction]) {
    println!("Sort the transactions into these categories");
    println!("0. Ignore");
    println!("1. Essential Food");
    println!("2. Food for fun");
    println!("3. Recurring Expenses");
    println!("4. Essential Expenses");
    println!("5. Fun");

    for transaction in transactions.iter_mut() {
        println!(
            "Date {:?} - {} - {}$",
            transaction.date, transaction.description, transaction.cad
        );

        loop {
            let mut input = String::new();
            stdin()
                .read_line(&mut input)
                .expect("Did not enter a correct string");

            let Ok(choice) = input.trim().parse::<usize>() else {
                println!("Please input a number");
                continue;
            };

            if choice > 5 {
                println!("Input out of range");
                continue;
            }

            transaction.kind = TransactionKind::from_usize(choice);
            break;
        }
    }
}

fn month_csv(csvs_folder: &Path, year: i32, month: u32) -> String {
    csvs_folder
        .join(format!("transactions-{}-{}.csv", year, month))
        .into_os_string()
        .into_string()
        .unwrap()
}

fn output_monthly(output_dir: &Path, transactions: &[Transaction]) {
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

fn read_monthly(output_dir: &Path, year_months: &[(i32, u32)]) -> Vec<Transaction> {
    year_months
        .iter()
        .map(|(y, m)| month_csv(&output_dir, *y, *m))
        .map(|p| {
            let mut reader =
                csv::Reader::from_path(&p).expect(&format!("Can't find the data {}", &p));
            reader
                .records()
                .into_iter()
                .map(|val| val.unwrap().deserialize::<Transaction>(None).unwrap())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn read_raw_data(file_path: &str) -> Vec<Transaction> {
    let mut reader = csv::Reader::from_path(file_path).expect("Can't find the data");
    let records: Vec<_> = reader
        .records()
        .into_iter()
        .map(|val| val.unwrap())
        .collect();

    let mut transactions = get_transactions(&records);
    transactions.sort_by(|a, b| a.date.partial_cmp(&b.date).unwrap());

    transactions
}

fn sort_new_data(file_path: &str) {
    let data_path = Path::new("monthly_transactions/");
    let mut transactions = read_raw_data(file_path);

    let mut year_months: Vec<_> = transactions.iter().map(|t| (t.date.year(), t.date.month())).collect();
    year_months.dedup();
    let old_transactions = read_monthly(&data_path, &year_months);
    transactions.retain(|t| !old_transactions.contains(&t));

    categorize_transactions(&mut transactions);
    output_monthly(&data_path, &transactions);
}

fn main() {
    let cli = Cli::parse();

    let Some(command) = cli.command else {
        let _ = Cli::command().print_help();
        return;
    };

    match command {
        Commands::Read { file_path } => sort_new_data(&file_path),
        Commands::Report => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use csv::ReaderBuilder;

    #[test]
    fn test_get_transactions() {
        let test_str = r#"
        "Account Type","Account Number","Transaction Date","Cheque Number","Description 1","Description 2","CAD$","USD$"
        Visa,12344565790,9/3/2023,,"BRAGG CREEK ESSO BRAGG CREEK AB",,-6.70,"#;

        let mut rdr = ReaderBuilder::new().from_reader(test_str.as_bytes());
        let records = rdr
            .records()
            .collect::<Result<Vec<StringRecord>, csv::Error>>()
            .unwrap();

        let transactions = get_transactions(&records);

        let expected = vec![Transaction {
            kind: None,
            date: NaiveDate::from_ymd_opt(2023, 9, 3).unwrap().into(),
            description: "BRAGG CREEK ESSO BRAGG CREEK AB".to_string(),
            cad: -6.7,
        }];

        assert_eq!(transactions, expected);
    }
}
