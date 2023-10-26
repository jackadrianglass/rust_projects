use chrono::prelude::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord, Clone, Copy)]
pub enum TransactionKind {
    None,
    EssentialFood,
    FunFood,
    Recurring,
    Essential,
    Investment,
    Fun,
}

impl TransactionKind {
    pub fn from_usize(val: usize) -> Option<Self> {
        match val {
            0 => Some(Self::None),
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
