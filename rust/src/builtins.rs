use std::collections::HashMap;

use crate::value::Value;

#[derive(Debug, Clone)]
pub struct Env {
    pub fields: Vec<Value>,
    pub nr: usize,
    pub filename: String,
    pub vars: HashMap<String, Value>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            fields: vec![],
            nr: 0,
            filename: String::new(),
            vars: HashMap::new(),
        }
    }

    pub fn set_filename(&mut self, name: &str) {
        self.filename = name.to_string();
    }

    pub fn set_line(&mut self, line: &str, field_sep: &str) {
        self.nr += 1;
        let raw: Vec<&str> = if field_sep == " " {
            line.split_whitespace().collect()
        } else {
            line.split(field_sep).collect()
        };
        let mut fields = Vec::new();
        fields.push(Value::Str(line.to_string()));
        for f in raw {
            fields.push(Value::Str(f.to_string()));
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
            _ => self.vars.get(name).cloned().unwrap_or(Value::Num(0.0)),
        }
    }

    pub fn set_var(&mut self, name: &str, val: Value) {
        self.vars.insert(name.to_string(), val);
    }
}
