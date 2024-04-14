use std::env;

pub struct ParsedArgs {
    pub filename: String,
    pub base: usize,
}

impl ParsedArgs {
    pub fn get() -> Result<ParsedArgs, &'static str> {
        let args: Vec<String> = env::args().collect();
        let len = args.len();
        if len == 1 || len > 3 {
            return Err(
                "Need target base and optionally a filename as arguments.\nExample: bxx 64 hello.c",
            );
        }

        let base_arg = &args[1];
        let base: usize = match base_arg.parse() {
            Ok(num) => match num {
                32 => 32,
                64 => 64,
                _ => return Err("Invalid base"),
            },
            Err(_) => return Err("Could not parse base arg"),
        };

        let filename = match len {
            2 => "-".to_string(),
            3 => {
                let filename_arg = &args[2];
                filename_arg.to_string()
            }
            _ => return Err("Too many arguments"),
        };

        let pa = ParsedArgs { filename, base };
        Ok(pa)
    }
}
