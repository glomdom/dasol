use std::{env, fs::File, io::Read, process};

use ariadne::{Label, Report, ReportKind, Source};
use pest::{error::InputLocation, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DASOLParser;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut code = String::new();
    let mut file = File::open(&args[1])?;
    file.read_to_string(&mut code)?;

    let ast = match DASOLParser::parse(Rule::document, &code) {
        Ok(mut pairs) => pairs.next().unwrap(),
        Err(error) => {
            let range = match error.location {
                InputLocation::Pos(pos) => pos..pos,
                InputLocation::Span((start, end)) => start..end,
            };

            Report::build(ReportKind::Error, &args[1], range.start)
                .with_code("P001")
                .with_label(
                    Label::new((&args[1], range))
                        .with_message(error.variant.message())
                )
                .finish()
                .print((&args[1], Source::from(&code)))
                .unwrap();

            process::exit(1);
        }
    };

    println!("{:#?}", ast);

    Ok(())
}
