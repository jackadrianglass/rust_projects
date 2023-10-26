#![allow(dead_code)]
pub mod rbc;
pub mod storage;
pub mod transaction;

use crate::rbc::*;
use crate::storage::*;
use crate::transaction::*;
use chrono::Datelike;
use clap::{CommandFactory, Parser, Subcommand};
use itertools::Itertools;
use std::io::stdin;
use std::path::Path;

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
    Report { year: i32 },
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

            transaction.kind = TransactionKind::from_usize(choice).unwrap();
            break;
        }
    }
}

fn sort_new_data(file_path: &str) {
    let data_path = Path::new("storage/");
    let mut transactions = read_raw_data(file_path);
    transactions.sort_by(|a, b| a.date.partial_cmp(&b.date).unwrap());

    let mut year_months: Vec<_> = transactions
        .iter()
        .map(|t| (t.date.year(), t.date.month()))
        .collect();
    year_months.dedup();
    let old_transactions = read_many_months(&data_path, &year_months);
    transactions.retain(|t| !old_transactions.contains(&t));

    categorize_transactions(&mut transactions);
    transactions.retain(|t| t.kind != TransactionKind::None);

    transactions.extend(old_transactions.into_iter());
    transactions.sort_by(|a, b| a.date.partial_cmp(&b.date).unwrap());

    output_monthly(&data_path, &transactions);
}

fn print_report(year: i32) {
    let data_path = Path::new("storage/");

    for month in 1..=12 {
        let transactions = read_month(data_path, year, month);
        if transactions.is_empty() {
            println!("{year}/{month} no records");
            continue;
        }

        let total: f32 = transactions.iter().map(|v| v.cad).sum();
        println!("{year}/{month} total spent {total}$");
        for (key, group) in &transactions
            .iter()
            .sorted_by_key(|v| v.kind)
            .group_by(|v| v.kind)
        {
            let sum: f32 = group.into_iter().map(|v| v.cad).sum();
            let d = 7;
            let p = 3;
            println!(
                "    {:d$.2}$ ({:p$.0}%) in {:?} ",
                sum,
                100.0 * (sum / total),
                key
            );
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let Some(command) = cli.command else {
        let _ = Cli::command().print_help();
        return;
    };

    match command {
        Commands::Read { file_path } => sort_new_data(&file_path),
        Commands::Report { year } => print_report(year),
    }
}
