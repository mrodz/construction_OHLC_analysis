mod cli;
mod header;

use anyhow::Result;
use clap::Parser as _;
use cli::Args;
use header::Header;

#[derive(pest_consume::Parser)]
#[grammar = "grammar.pest"]
struct TsOhlcParser;

pub(crate) type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[derive(Debug)]
pub(crate) struct TsCSVFile {
    header: Header,
}

#[pest_consume::parser]
impl TsOhlcParser {
    pub fn file(input: Node) -> TsCSVFile {
        assert!(input.as_rule() == Rule::file);

        let mut children = input.children();

        let header = children.next().unwrap();

        TsCSVFile {
            header: Self::header(header),
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let file = std::fs::read_to_string(args.input)?;

    let root = <TsOhlcParser as pest_consume::Parser>::parse(Rule::file, &file)?.next().unwrap();

    let file = TsOhlcParser::file(root);

    println!("{file:?}");

    Ok(())
}
