use std::borrow::Cow;
use std::str::{self, FromStr};

use nom::{IResult, digit};

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Step(usize),
    Run,
    Breakpoint(usize),
    ListBreakPoints,
    ClearBreakpoint(usize),
    Print(usize),
    PrintRange(usize, usize),
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

// TODO: Allow commands to take hex prefixed with '$'
named!(command<Command>,
    alt_complete!(
        step        |
        run         |
        breakpoint  |
        list        |
        clear_bp    |
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
            tag!("r") | tag!("run") | tag!("c") | tag!("continue")
        ) >>
        (Command::Run)
    )
);

named!(breakpoint<Command>,
    do_parse!(
        alt!(
            tag!("b") | tag!("break")
        ) >>
        addr: ws!(usize_parser) >>
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
        addr: ws!(usize_parser) >>
        (Command::ClearBreakpoint(addr))
    )
);

named!(print<Command>,
    do_parse!(
        alt!(
            tag!("p") | tag!("print")
        ) >>
        addr: ws!(usize_parser) >>
        (Command::Print(addr))
    )
);

named!(print_range<Command>,
    do_parse!(
        tag!("pr") >>
        low_addr: ws!(usize_parser) >>
        alt!(char!(',') | char!(':')) >>
        high_addr: ws!(usize_parser) >>
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
