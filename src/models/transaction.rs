use serde::{Serialize,Deserialize};

#[derive(Deserialize,Serialize,Debug,Clone)]
pub enum ExpenseType {
    Spend,
    Earned,
}

#[allow(dead_code)]
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Transaction {
    pub id : usize,
    pub amount : f64,
    pub description : String,
    pub date : String,
    pub category : String,
    pub expense_type : ExpenseType,
}