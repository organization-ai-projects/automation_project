use crate::diagnostics::error::SpreadsheetError;
use crate::formula::ast::{BinOpKind, Expr};
use crate::model::cell_id::CellId;
use crate::model::cell_value::CellValue;
use crate::model::sheet::Sheet;

pub struct Evaluator<'a> {
    sheet: &'a Sheet,
}

impl<'a> Evaluator<'a> {
    pub fn new(sheet: &'a Sheet) -> Self {
        Self { sheet }
    }

    pub fn eval(&self, expr: &Expr) -> Result<CellValue, SpreadsheetError> {
        match expr {
            Expr::Number(n) => Ok(CellValue::Number(*n)),
            Expr::Text(s) => Ok(CellValue::Text(s.clone())),
            Expr::CellRef(id) => Ok(self.sheet.get_value(id)),
            Expr::RangeRef(from, to) => {
                // Range used outside function context — collect as single value error
                Err(SpreadsheetError::EvalError(format!(
                    "range {}:{} used outside function context",
                    from, to
                )))
            }
            Expr::Neg(inner) => {
                let v = self.eval(inner)?;
                match v {
                    CellValue::Number(n) => Ok(CellValue::Number(-n)),
                    _ => Err(SpreadsheetError::TypeError("negation requires a number".into())),
                }
            }
            Expr::BinOp { op, lhs, rhs } => {
                let lv = self.eval(lhs)?;
                let rv = self.eval(rhs)?;
                match (lv, rv) {
                    (CellValue::Number(a), CellValue::Number(b)) => match op {
                        BinOpKind::Add => Ok(CellValue::Number(a + b)),
                        BinOpKind::Sub => Ok(CellValue::Number(a - b)),
                        BinOpKind::Mul => Ok(CellValue::Number(a * b)),
                        BinOpKind::Div => {
                            if b == 0.0 {
                                Err(SpreadsheetError::DivisionByZero)
                            } else {
                                Ok(CellValue::Number(a / b))
                            }
                        }
                    },
                    _ => Err(SpreadsheetError::TypeError("arithmetic requires numbers".into())),
                }
            }
            Expr::FunctionCall { name, args } => self.eval_function(name, args),
        }
    }

    fn collect_range_values(&self, from: &CellId, to: &CellId) -> Vec<CellValue> {
        let mut values = Vec::new();
        for row in from.row..=to.row {
            for col in from.col..=to.col {
                values.push(self.sheet.get_value(&CellId::new(row, col)));
            }
        }
        values
    }

    fn collect_arg_values(&self, args: &[Expr]) -> Result<Vec<CellValue>, SpreadsheetError> {
        let mut values = Vec::new();
        for arg in args {
            match arg {
                Expr::RangeRef(from, to) => {
                    values.extend(self.collect_range_values(from, to));
                }
                _ => {
                    values.push(self.eval(arg)?);
                }
            }
        }
        Ok(values)
    }

    fn eval_function(&self, name: &str, args: &[Expr]) -> Result<CellValue, SpreadsheetError> {
        let values = self.collect_arg_values(args)?;

        match name {
            "SUM" => {
                let mut sum = 0.0;
                for v in &values {
                    match v {
                        CellValue::Number(n) => sum += n,
                        CellValue::Empty => {}
                        CellValue::Error(e) => {
                            return Err(SpreadsheetError::EvalError(e.clone()));
                        }
                        _ => return Err(SpreadsheetError::TypeError("SUM requires numbers".into())),
                    }
                }
                Ok(CellValue::Number(sum))
            }
            "MIN" => {
                let nums: Vec<f64> = values.iter().filter_map(|v| {
                    if let CellValue::Number(n) = v { Some(*n) } else { None }
                }).collect();
                if nums.is_empty() {
                    Ok(CellValue::Number(f64::INFINITY))
                } else {
                    Ok(CellValue::Number(nums.iter().cloned().fold(f64::INFINITY, f64::min)))
                }
            }
            "MAX" => {
                let nums: Vec<f64> = values.iter().filter_map(|v| {
                    if let CellValue::Number(n) = v { Some(*n) } else { None }
                }).collect();
                if nums.is_empty() {
                    Ok(CellValue::Number(f64::NEG_INFINITY))
                } else {
                    Ok(CellValue::Number(nums.iter().cloned().fold(f64::NEG_INFINITY, f64::max)))
                }
            }
            _ => Err(SpreadsheetError::EvalError(format!("unknown function: {}", name))),
        }
    }
}
