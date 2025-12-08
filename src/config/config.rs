pub enum Command {
    Expense {
        description: String,
        category: String,
        amount: String,
    },
    Income {
        description: String,
        category: String,
        amount: String,
    },
    Summary,
}

#[allow(dead_code)]
pub struct Config {
    pub command: Command,
}
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next(); // skip program name

        let query = args.next().ok_or("Missing command")?;

        match query.as_str() {
            "expense" => {
                let description = args.next().ok_or("Missing description")?;
                let category = args.next().ok_or("Missing category")?;
                let amount = args.next().ok_or("Missing amount")?;

                Ok(Config {
                    command: Command::Expense {
                        description,
                        category,
                        amount,
                    },
                })
            }

            "income" => {
                let description = args.next().ok_or("Missing description")?;
                let category = args.next().ok_or("Missing category")?;
                let amount = args.next().ok_or("Missing amount")?;

                Ok(Config {
                    command: Command::Income {
                        description,
                        category,
                        amount,
                    },
                })
            }

            "summary" => {
                Ok(Config {
                    command: Command::Summary,
                })
            }

            _ => Err("Unknown command"),
        }
    }
}
