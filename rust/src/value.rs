use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Num(f64),
}

impl Value {
    pub fn as_str(&self) -> String {
        match self {
            Value::Str(s) => s.clone(),
            Value::Num(n) => {
                if *n == n.floor() {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
        }
    }

    pub fn as_f64(&self) -> f64 {
        match self {
            Value::Str(s) => s.parse().unwrap_or(0.0),
            Value::Num(n) => *n,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Str(s) => write!(f, "{}", s),
            Value::Num(n) => {
                if *n == n.floor() {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
        }
    }
}
