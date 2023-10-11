#![allow(dead_code)]
use chrono::prelude::NaiveDate;
use csv::StringRecord;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Transaction {
    transaction_date: NaiveDate,
    description: String,
    cad: f32,
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
            transaction_date: NaiveDate::parse_from_str(&val[2], "%m/%d/%Y").unwrap(),
            description: val[4].to_string(),
            cad: val[6].parse().unwrap(),
        })
        .collect()
}

fn main() {
    let mut reader = csv::Reader::from_path("data.csv").expect("Can't find the data");
    let records: Vec<_> = reader
        .records()
        .into_iter()
        .map(|val| val.unwrap())
        .collect();

    // TODO
    // - split up transactions by month
    let _transactions = get_transactions(&records);

    // TODO
    // - split this up into a vector of categories and a vector of transaction vectors
    let mut transactions_by_category = HashMap::<String, Vec<Transaction>>::new();
    transactions_by_category.insert("Essential Food".to_string(), Vec::new());
    transactions_by_category.insert("Food For Fun".to_string(), Vec::new());
    transactions_by_category.insert("Recurring Expenses".to_string(), Vec::new());
    transactions_by_category.insert("Essential Expenses".to_string(), Vec::new());
    transactions_by_category.insert("Fun".to_string(), Vec::new());

    // TODO: Prompt the user per transaction to place into one of the categories (probably need a
    // tui)

    // TODO:
    // - Output results into some format for later consumption
    // - Print out total money spent in each category and percentages of total
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
            transaction_date: NaiveDate::from_ymd_opt(2023, 9, 3).unwrap().into(),
            description: "BRAGG CREEK ESSO BRAGG CREEK AB".to_string(),
            cad: -6.7,
        }];

        assert_eq!(transactions, expected);
    }
}
