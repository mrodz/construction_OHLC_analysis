use chrono::{DateTime, TimeZone, Utc};

use crate::{Node, Rule, TsOhlcParser};

#[derive(Debug)]
pub struct Header {
    symbol: Symbol,
    work_time: WorkTime,
}

impl TsOhlcParser {
	pub fn header(input: Node) -> Header {
		assert!(input.as_rule() == Rule::header);

		let mut children = input.children();

		let _description = children.next().unwrap();
		let symbol = children.next().unwrap();
		let work_time = children.next().unwrap();

		Header {
			symbol: Self::symbol(symbol),
			work_time: Self::work_time(work_time),
		}
	}
}

#[derive(Debug)]
pub struct Symbol(String);

impl TsOhlcParser {
    pub fn symbol(input: Node) -> Symbol {
        assert!(input.as_rule() == Rule::symbol, "not symbol: {:?}", input.as_rule());
        let ticker = input.children().single().expect("only one ticker");
        assert!(ticker.as_rule() == Rule::ticker);
        Symbol(ticker.as_str().to_owned())
    }
}

#[derive(Debug)]
pub struct Date(DateTime<Utc>);

impl TsOhlcParser {
	fn date(input: Node) -> Date {
        assert!(input.as_rule() == Rule::date);
        let children = input.children();

        let mut dmy_items = [0u32; 3];

        for (i, dmy) in children.enumerate() {
            assert!(dmy.as_rule() == Rule::dmy);
            dmy_items[i] = dmy
                .as_str()
                .parse()
                .expect("should have been ASCII_DIGIT{1,2}");
        }

        Date(Utc.with_ymd_and_hms(
            dmy_items[2].try_into().expect("non-standard year"),
            dmy_items[0],
            dmy_items[1],
            0,
            0,
            0,
        ).earliest().expect("error with date"))
    }
}

#[derive(Debug)]
pub struct WorkTime {
    start: Date,
    end: Date,
}

impl TsOhlcParser {
	pub fn work_time(input: Node) -> WorkTime {
		assert!(input.as_rule() == Rule::work_time);
		let mut children = input.children();

		let start = children.next().unwrap();
		let end = children.next().unwrap();

		WorkTime {
			start: Self::date(start),
			end: Self::date(end),
		}
	}
}
