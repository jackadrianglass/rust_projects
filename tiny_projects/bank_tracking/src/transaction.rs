use chrono::prelude::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord, Clone, Copy)]
pub enum TransactionKind {
    None,
    Income,
    Essential,
    Recurring,
    Investment,
    EssentialFood,
    FunFood,
    Fun,
}

impl TransactionKind {
    pub fn from_usize(val: usize) -> Option<Self> {
        match val {
            0 => Some(Self::None),
            1 => Some(Self::Income),
            2 => Some(Self::Essential),
            3 => Some(Self::Recurring),
            4 => Some(Self::Investment),
            5 => Some(Self::EssentialFood),
            6 => Some(Self::FunFood),
            7 => Some(Self::Fun),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub kind: TransactionKind,
    pub date: NaiveDate,
    pub description: String,
    pub cad: f32,
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date && self.description == other.description && self.cad == other.cad
    }
}
