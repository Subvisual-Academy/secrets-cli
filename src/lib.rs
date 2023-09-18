use std::error::Error;
use std::str::FromStr;

pub enum Operation {
    Encrypt,
    Decrypt,
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Operation, Self::Err> {
        match s {
            "encrypt" => Ok(Operation::Encrypt),
            "decrypt" => Ok(Operation::Decrypt),
            _ => Err("invalid operation type."),
        }
    }
}

pub struct Secret {
    pub operation: Operation,
    pub argument: String,
}

impl Secret {
    pub fn build(args: &[String]) -> Result<Secret, &str> {
        if args.len() < 3 {
            return Err("not enough arguments.");
        }

        let operation = Operation::from_str(&args[1])?;
        let argument = args[2].clone();
        Ok(Secret {
            operation,
            argument,
        })
    }
}

pub fn run(secret: Secret) -> Result<(), Box<dyn Error>> {
    match secret.operation {
        Operation::Encrypt => {}
        Operation::Decrypt => {}
    }
    Ok(())
}

