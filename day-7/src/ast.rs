pub struct Gate<'a> {
    pub op: Op<'a>,
    pub out: &'a str,
}

impl<'a> Gate<'a> {
    pub fn rshift(g: Source<'a>, w: u8, d: &'a str) -> Gate<'a> {
        Gate { op: Op::Rshift(g, w), out: d }
    }

    pub fn or(a: Source<'a>, b: Source<'a>, d: &'a str) -> Gate<'a> {
        Gate { op: Op::Or(a, b), out: d }
    }

    pub fn pass(a: Source<'a>, d: &'a str) -> Gate<'a> {
        Gate { op: Op::Pass(a), out: d }
    }

    pub fn not(a: Source<'a>, d: &'a str) -> Gate<'a> {
        Gate { op: Op::Not(a), out: d }
    }

    pub fn and(a: Source<'a>, b: Source<'a>, d: &'a str) -> Gate<'a> {
        Gate { op: Op::And(a, b), out: d }
    }

    pub fn lshift(g: Source<'a>, w: u8, d: &'a str) -> Gate<'a> {
        Gate { op: Op::Lshift(g, w), out: d }
    }
}

pub enum Op<'a> {
    Rshift(Source<'a>, u8),
    Or(Source<'a>, Source<'a>),
    Pass(Source<'a>),
    Not(Source<'a>),
    And(Source<'a>, Source<'a>),
    Lshift(Source<'a>, u8),
}

pub enum Source<'a> {
    Wire(&'a str),
    Lit(u16),
}
