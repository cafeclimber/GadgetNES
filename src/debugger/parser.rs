use std::borrow::Cow;
use std::str::{self, FromStr};
use std::num;

use nom::{IResult, digit, is_hex_digit};

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Step(usize),
    Run,
    Breakpoint(u16),
    ListBreakPoints,
    ClearBreakpoint(usize),
    Print(u16),
    PrintRange(u16, u16),
    Help,
    Quit,
}

impl FromStr for Command {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match command(s.as_bytes()) {
            IResult::Done(_, c) => Ok(c),
            err => Err(format!("Unable to parse command {}: {:?}", s, err).into())
        }
    }
}

fn from_hex(input: &[u8]) -> Result<u16, num::ParseIntError> {
    let hex_string = str::from_utf8(input).unwrap_or("0");
    u16::from_str_radix(hex_string, 16)
}

named!(hex_primary<&[u8], u16>,
    map_res!(take_while!(is_hex_digit), from_hex)
);

// TODO: Allow commands to take hex prefixed with '$'
named!(command<Command>,
    alt_complete!(
        breakpoint  |
        list        |
        clear_bp    |
        step        |
        run         |
        print       |
        print_range |
        help        |
        quit
    )
);

named!(step<Command>,
    do_parse!(
        alt!(
            tag!("s") | tag!("step")
        ) >>
        num_steps: opt!(complete!(ws!(usize_parser))) >>
        (Command::Step(num_steps.unwrap_or(1)))
    )
);

named!(run<Command>,
    do_parse!(
        alt!(
            tag!("r") | tag!("run")
        ) >>
        (Command::Run)
    )
);

named!(breakpoint<Command>,
    do_parse!(
        alt!(
            tag!("b") | tag!("break")
        ) >>
        addr: ws!(hex_primary) >>
        (Command::Breakpoint(addr))
    )
);

named!(list<Command>,
    do_parse!(
        alt!(
            tag!("l") | tag!("list")
        ) >>
        (Command::ListBreakPoints)
    )
);

named!(clear_bp<Command>,
    do_parse!(
        alt!(
            tag!("cb") | tag!("clear")
        ) >>
        num: ws!(usize_parser) >>
        (Command::ClearBreakpoint(num))
    )
);

named!(print<Command>,
    do_parse!(
        alt!(
            tag!("p") | tag!("print")
        ) >>
        addr: ws!(hex_primary) >>
        (Command::Print(addr))
    )
);

named!(print_range<Command>,
    do_parse!(
        tag!("pr") >>
        low_addr: ws!(hex_primary) >>
        alt!(char!(',') | char!(':')) >>
        high_addr: ws!(hex_primary) >>
        (Command::PrintRange(low_addr, high_addr))
    )
);

// TODO: Allow help for specific commands
named!(help<Command>,
    do_parse!(
        alt!(
            tag!("h") | tag!("help")
        ) >>
        (Command::Help)
    )
);

named!(quit<Command>,
    do_parse!(
        alt!(
            tag!("q") | tag!("quit")
        ) >>
        (Command::Quit)
    )
);

named!(usize_parser<usize>,
    map_res!(
        map_res!(
            digit,
            str::from_utf8
        ),
        FromStr::from_str
    )
);
