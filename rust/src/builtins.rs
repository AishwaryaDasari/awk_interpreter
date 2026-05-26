use std::collections::HashMap;

use crate::value::Value;

#[derive(Debug, Clone)]
pub struct Env {
    pub fields: Vec<Value>,
    pub nr: usize,
    pub filename: String,
    pub fs: String,
    pub vars: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            fields: vec![],
            nr: 0,
            filename: String::new(),
            fs: String::from(" "),
            vars: HashMap::new(),
        }
    }

    pub fn set_filename(&mut self, name: &str) {
        self.filename = name.to_string();
    }

    pub fn set_line(&mut self, line: &str) {
        self.nr += 1;
        let raw: Vec<String> = if self.fs == " " {
            line.split_whitespace().map(|s| s.to_string()).collect()
        } else if self.fs.is_empty() {
            line.chars().map(|c| c.to_string()).collect()
        } else {
            line.split(&self.fs).map(|s| s.to_string()).collect()
        };
        let mut fields = vec![Value::Str(line.to_string())];
        for f in raw {
            fields.push(Value::Str(f));
        }
        self.fields = fields;
    }

    pub fn get_field(&self, n: usize) -> Value {
        if n < self.fields.len() {
            self.fields[n].clone()
        } else {
            Value::Str(String::new())
        }
    }

    pub fn get_var(&self, name: &str) -> Value {
        match name {
            "NF" => Value::Num((self.fields.len().saturating_sub(1)) as f64),
            "NR" => Value::Num(self.nr as f64),
            "FILENAME" => Value::Str(self.filename.clone()),
            "FS" => Value::Str(self.fs.clone()),
            _ => self.vars.get(name).cloned().unwrap_or(Value::Num(0.0)),
        }
    }

    pub fn set_var(&mut self, name: &str, val: Value) {
        if name == "FS" {
            self.fs = val.as_str();
        }
        self.vars.insert(name.to_string(), val);
    }
}
