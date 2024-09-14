use super::arithmetic::{Arithmetic, Floating};
use thiserror::Error;
use std::iter::Iterator;

/// NumericAttribute represents some extra parsed attribute found on a number.
/// For instance, a cell may represent a percentage value in which case we 
/// should treat it as 1/100 of the raw numerical value in the cell for 
/// some computations.
#[derive(Clone, Debug, PartialEq)]
pub enum NumericAttribute {
    Percent,
    Currency(String),
}

/// Numeric is a number with attributes attached, like percentage or currency.
#[derive(Clone, Debug)]
pub struct Numeric<T=f64> 
where T: Arithmetic {
    number: T,
    attr: Option<NumericAttribute>,
}

impl<T> Numeric<T> 
where T: Arithmetic {
    /// Get the raw value of this cell which will be used for computations.
    ///
    /// This is not necessarily just the number in the cell.
    pub fn value(&self) -> T {
        match self.attr {
            None => self.number,
            Some(ref attr) => {
                match attr {
                    NumericAttribute::Percent => self.number / Floating::from_f64(100.0),
                    _ => self.number,
                }
            }
        }
    }

    /// Tries to add another number. This may fail if we add two numerics with
    /// different attributes.
    pub fn try_add(self, other: Self) -> Option<Self> {
        match self.attr {
            None => {
                Some(Self{number: self.value() + other.value(), attr: other.attr.clone()})
            },
            Some(ref attr) => {
                match other.attr {
                    None => Some(Self{number: self.value() + other.value(), attr: self.attr.clone()}),
                    Some(ref other_attr) => {
                        if attr == other_attr {
                            Some(Self{number: self.value() + other.value(), attr: self.attr.clone()})
                        } else {
                            None
                        }
                    }
                }
            }
        }
    }
}

/// A primitive type which a cell may represent.
pub enum Primitive<T=f64> 
where T: Arithmetic {
    Number(Numeric<T>),
    Bool(bool),
    Date(chrono::NaiveDate),
    Time(chrono::TimeDelta),
    IPAddress([u8; 4]),
}

impl<T: Arithmetic> TryFrom<&str> for Primitive<T> {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

#[derive(Error, Debug)]
enum ColumnParseError {
    #[error("Encountered unexpected char {0}")]
    UnexpectedChar(char),

    #[error("Did not start with alphabetical character")]
    DidntStartAlpha,

    #[error("Did not contain numerical characters")]
    DidntContainNumber,
}

const fn ipow(x: u64, pow: u64) -> u64 {
    match pow {
        0 => 1,
        1 => x,
        _ => x * ipow(x, pow-1)
    }
}

const fn column_to_u64(column: &str) -> Result<u64, ColumnParseError> {
    let bytes = column.as_bytes();
    match bytes.len() {
        0 => Ok(0),
        _ => {
            let c = bytes[0];
            let (_, s2) = bytes.split_at(1);
            match c as char {
                'A'..'Z' => {
                    let res = ((c - b'A') as u64 * ipow(26, s2.len() as u64));
                    unsafe {
                        match column_to_u64(std::str::from_utf8_unchecked(s2)) {
                            Ok(v) => Ok(v + res),
                            Err(e) => Err(e),
                        }
                    }
                },
                'a'..'z' => {
                    let res = ((c - b'a') as u64 * ipow(26, s2.len() as u64));
                    unsafe {
                        match column_to_u64(std::str::from_utf8_unchecked(s2)) {
                            Ok(v) => Ok(v + res),
                            Err(e) => Err(e),
                        }
                    }
                },
                _ => Err(ColumnParseError::UnexpectedChar(c as char)),
            }
        },
    }
}

const fn split_id(s: &str) -> Result<(&str, &str), ColumnParseError> {
    Err(ColumnParseError::DidntStartAlpha)
}

macro_rules! xl {
    ($s:expr) => {{
        const parts = split_id.unwrap();
    }}
}

/// CellId represents the id of a cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellId {
    row: u32,
    col: u32,
}

impl CellId {
    fn new(row: u32, col: u32) -> Self {
        Self{row, col}
    }
}

pub enum FunctionKind {
    Sum,
    Prod,
    If,
    Sqrt,
    Sdev,
    Offset,
}

pub enum Formula<T: Arithmetic> {
    CellRef(CellId),
    CellRange(CellId, CellId),
    Function{
        kind: FunctionKind,
        arguments: Vec<Value<T>>,
    },
    Add(Box<Value<T>>, Box<Value<T>>),
    Mul(Box<Value<T>>, Box<Value<T>>),
    Sub(Box<Value<T>>, Box<Value<T>>),
    Div(Box<Value<T>>, Box<Value<T>>),
    Cmp(Box<Value<T>>, Box<Value<T>>),
    Lt(Box<Value<T>>, Box<Value<T>>),
    Gr(Box<Value<T>>, Box<Value<T>>),
}

impl<T: Arithmetic> TryFrom<&str> for Formula<T> {
    type Error=FormulaParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

#[derive(Error, Debug)]
pub enum FormulaParseError {
    #[error("unknown function")]
    UnknownFunction(String),
}

pub enum Value<T=f64>
where T: Arithmetic {
    Raw,
    Primitive(Primitive<T>),
    Formula(Formula<T>),
    FormulaParseError(FormulaParseError),
}

impl<T: Arithmetic> From<&str> for Value<T> {
    fn from(value: &str) -> Self {
        let value = value.trim();
        if !value.starts_with('=') {
            if let Ok(primitive) = Primitive::try_from(value) {
                Self::Primitive(primitive)
            } else {
                Self::Raw
            }
        } else {
            let (_, remainder) = value.split_at(1);
            match Formula::try_from(remainder) {
                Ok(formula) => Self::Formula(formula),
                Err(e) => Self::FormulaParseError(e),
            }
        }
    }
}

pub struct Cell<T: Arithmetic> {
    raw: String,
    value: Value<T>,
}

impl<T: Arithmetic> From<String> for Cell<T> {
    fn from(s: String) -> Self {
        let value = s.as_str().into();
        Self{raw: s, value}
    }
}

pub trait Kernel<E: std::error::Error, T: Arithmetic=f64> {
    fn get_cell(&self, cell_id: CellId) -> Option<Cell<T>>;
    fn evaluate_cell(&self, cell_id: CellId) -> Result<Value, E>;
    fn set_cell(&mut self, cell_id: CellId, data: String);
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn it_works() {
//        let result = add(2, 2);
//        assert_eq!(result, 4);
//    }
//}
