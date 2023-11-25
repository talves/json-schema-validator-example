use boon::*;
use serde_json;
use std::{error::Error, fs::File};

fn main() {
    match validate() {
        Ok(valid) => println!("{}", valid),
        Err(e) => println!("{}", e),
    };
}

fn validate() -> Result<bool, Box<dyn Error>> {
    let mut schemas = Schemas::new(); // container for compiled schemas
    let mut compiler = Compiler::new();
    let sch_index = compiler.compile("./schema.json", &mut schemas)?;
    let instance: serde_json::Value = serde_json::from_reader(File::open("./config.json")?)?;
    let result = schemas.validate(&instance, sch_index);
    if let Err(e) = &result {
        for line in format!("{e}").lines() {
            println!("        {line}");
        }
        for line in format!("{e:#}").lines() {
            println!("        {line}");
        }
        println!("{:#}", e.detailed_output());
    }
    // assert!(result.is_ok());

    Ok(result.is_ok())
}
