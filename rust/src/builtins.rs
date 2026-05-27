use std::collections::HashMap;

use crate::value::Value;

#[derive(Debug, Clone)]
pub struct Env {
    pub fields: Vec<Value>,
    pub nr: usize,
    pub fnr: usize,
    pub filename: String,
    pub fs: String,
    pub rstart: usize,
    pub rlength: isize,
    pub vars: HashMap<String, Value>,
    pub arrays: HashMap<String, HashMap<String, Value>>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            fields: vec![],
            nr: 0,
            fnr: 0,
            filename: String::new(),
            fs: String::from(" "),
            rstart: 0,
            rlength: -1,
            vars: HashMap::new(),
            arrays: HashMap::new(),
        }
    }

    pub fn set_filename(&mut self, name: &str) {
        self.filename = name.to_string();
    }

    pub fn set_line(&mut self, line: &str) {
        self.nr += 1;
        self.fnr += 1;
        self.split_fields(line);
    }

    pub fn get_field(&self, n: usize) -> Value {
        if n < self.fields.len() {
            self.fields[n].clone()
        } else {
            Value::Str(String::new())
        }
    }

    pub fn set_field(&mut self, n: usize, val: Value) {
        if n == 0 {
            self.split_fields(&val.as_str());
        } else if n < self.fields.len() {
            self.fields[n] = val;
        }
    }

    pub fn get_var(&self, name: &str) -> Value {
        match name {
            "NF" => Value::Num((self.fields.len().saturating_sub(1)) as f64),
            "NR" => Value::Num(self.nr as f64),
            "FNR" => Value::Num(self.fnr as f64),
            "FILENAME" => Value::Str(self.filename.clone()),
            "FS" => Value::Str(self.fs.clone()),
            "RSTART" => Value::Num(self.rstart as f64),
            "RLENGTH" => Value::Num(self.rlength as f64),
            _ => self.vars.get(name).cloned().unwrap_or(Value::Num(0.0)),
        }
    }

    pub fn reset_fnr(&mut self) {
        self.fnr = 0;
    }

    pub fn get_array(&self, name: &str, index: &str) -> Value {
        self.arrays
            .get(name)
            .and_then(|arr| arr.get(index))
            .cloned()
            .unwrap_or(Value::Str(String::new()))
    }

    pub fn set_var(&mut self, name: &str, val: Value) {
        if name == "FS" {
            self.fs = val.as_str();
        }
        self.vars.insert(name.to_string(), val);
    }

    fn split_fields(&mut self, line: &str) {
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
}
