use std::error::Error;

// There are 10^9 lamports in one SOL
const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

// There are 10^6 micro-lamports in one lamport
const MICRO_LAMPORTS_PER_LAMPORT: u64 = 1_000_000;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let result= match (config.from, config.to) {
        (Unit::Sol, Unit::Lamport) => sol_to_lamports(config.amount),
        (Unit::Sol, Unit::MicroLamport) => sol_to_micro_lamports(config.amount),

        (Unit::Lamport, Unit::Sol) => lamports_to_sol(config.amount),
        (Unit::Lamport, Unit::MicroLamport) => lamports_to_micro_lamports(config.amount),

        (Unit::MicroLamport, Unit::Lamport) => micro_lamports_to_lamports(config.amount),
        (Unit::MicroLamport, Unit::Sol) => micro_lamports_to_sol(config.amount),

        (Unit::Sol, Unit::Sol) => config.amount,
        (Unit::Lamport, Unit::Lamport) => config.amount,
        (Unit::MicroLamport, Unit::MicroLamport) => config.amount,
    };

    println!("{result}");
    
    Ok(())
}

fn sol_to_lamports(sol: f64) -> f64 {
    sol * LAMPORTS_PER_SOL as f64
}

fn lamports_to_sol(lamports: f64) -> f64 {
    lamports / LAMPORTS_PER_SOL as f64
}

fn lamports_to_micro_lamports(lamports: f64) -> f64 {
    lamports * MICRO_LAMPORTS_PER_LAMPORT as f64
}

fn micro_lamports_to_lamports(micro_lamports: f64) -> f64 {
    micro_lamports / MICRO_LAMPORTS_PER_LAMPORT as f64
}

fn sol_to_micro_lamports(sol: f64) -> f64 {
    lamports_to_micro_lamports(sol_to_lamports(sol))
}

fn micro_lamports_to_sol(micro_lamports: f64) -> f64 {
    lamports_to_sol(micro_lamports_to_lamports(micro_lamports))
}

pub struct Config {
    from: Unit,
    to: Unit,
    amount: f64,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let from = match args.next() {
            Some(arg) => {
                match Unit::from_str(&arg) {
                    Ok(unit) => unit,
                    Err(_) => return Err("Unexpected unit at first argument")
                }
            },
            None => return Err("Didn't get a source unit"),
        };

        let to = match args.next() {
            Some(arg) => {
                match Unit::from_str(&arg) {
                    Ok(unit) => unit,
                    Err(_) => return Err("Unexpected unit at second argument"),
                }
            },
            None => return Err("Didn't get a target unit"),
        };

        let amount: f64 = match args.next() {
            Some(arg) => match arg.parse() {
                Ok(value) => {
                    if value < 0.0 {
                        return Err("Amount must be a non-negative number");
                    }
                    value
                },
                Err(_) => return Err("Amount is not a number"),
            },
            None => return Err("Didn't get an amount"),
        };

        Ok(Config {
            from,
            to,
            amount,
        })
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Unit {
    Sol,
    Lamport,
    MicroLamport,
}

impl Unit {
    fn from_str(s: &str) -> Result<Unit, &'static str> {
        match s {
            "sol" => Ok(Unit::Sol),
            "lamport" => Ok(Unit::Lamport),
            "micro_lamport" => Ok(Unit::MicroLamport),
            _ => Err("Invalid unit type"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        assert_eq!(sol_to_lamports(1.0), 1_000_000_000.0);
        assert_eq!(lamports_to_sol(1_000_000_000.0), 1.0);
        assert_eq!(lamports_to_micro_lamports(1.0), 1_000_000.0);
        assert_eq!(micro_lamports_to_lamports(1_000_000.0), 1.0);
    }

    #[test]
    fn test_unit_from_str_valid_values() {
        assert_eq!(Unit::from_str("sol").unwrap(), Unit::Sol);
        assert_eq!(Unit::from_str("lamport").unwrap(), Unit::Lamport);
        assert_eq!(Unit::from_str("micro_lamport").unwrap(), Unit::MicroLamport);
    }

    #[test]
    fn test_config_build() {
        let args = vec![
            "program".to_string(),
            "sol".to_string(),
            "lamport".to_string(),
            "1.0".to_string(),
        ];
        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(config.from, Unit::Sol);
        assert_eq!(config.to, Unit::Lamport);
        assert_eq!(config.amount, 1.0);
    }
}