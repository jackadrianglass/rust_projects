use crate::grammar::*;

pub fn evaluate(expr: &Expr) -> Result<f64, ()> {
    match expr {
        Expr::Literal(lit) => match lit {
            Literal::Float(val) => Ok(*val),
            Literal::Int(val) => Ok((*val).into()),
            _ => Err(()),
        },
        Expr::UnaryExpr(op, expr) => {
            let val = evaluate(expr)?;
            match op {
                UnaryOp::Not => Err(()),
                UnaryOp::Negative => Ok(-val),
            }
        }
        Expr::BinaryExpr(left, op, right) => {
            let left_val = evaluate(left)?;
            let right_val = evaluate(right)?;
            match op {
                BinaryOp::Plus => Ok(left_val + right_val),
                BinaryOp::Minus => Ok(left_val - right_val),
                BinaryOp::Times => Ok(left_val * right_val),
                BinaryOp::Divide => Ok(left_val / right_val),
                _ => Err(()),
            }
        }
        Expr::Grouping(expr) => evaluate(expr),
    }
}
