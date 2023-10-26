use crate::transaction::{Transaction, TransactionKind};
use chrono::prelude::NaiveDate;
use csv::StringRecord;

/// Expecting the record to be in the format of
/// - Account type
/// - Account number
/// - Transaction date
/// - Cheque number
/// - Description 1
/// - Description 2
/// - cad
/// - usd
pub fn get_transactions(records: &[StringRecord]) -> Vec<Transaction> {
    records
        .iter()
        .map(|val| Transaction {
            kind: TransactionKind::None,
            date: NaiveDate::parse_from_str(&val[2], "%m/%d/%Y").unwrap(),
            description: val[4].to_string(),
            cad: val[6].parse().unwrap(),
        })
        .collect()
}

pub fn read_raw_data(file_path: &str) -> Vec<Transaction> {
    let mut reader = csv::Reader::from_path(file_path).expect("Can't find the data");
    let records: Vec<_> = reader
        .records()
        .into_iter()
        .map(|val| val.expect("Did you remember to remove the trailing commas?"))
        .collect();

    get_transactions(&records)
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
            kind: TransactionKind::None,
            date: NaiveDate::from_ymd_opt(2023, 9, 3).unwrap().into(),
            description: "BRAGG CREEK ESSO BRAGG CREEK AB".to_string(),
            cad: -6.7,
        }];

        assert_eq!(transactions, expected);
    }
}
