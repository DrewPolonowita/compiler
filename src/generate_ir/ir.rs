use std::fmt::Display;
use crate::parser::parse_tree::Operator;


pub enum IRLine {
    ThreeAddress(ThreeAddress),
    SingleAddress(SingleAddress),
    CmpSingleAddress(CmpSingleAddress),
    CmpThreeAddress(CmpThreeAddress),
    Goto(Goto),
    Label(Label),
}

impl Display for IRLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use IRLine::*;
        match self {
            ThreeAddress(line) => write!(f, "{}", line),
            SingleAddress(line) => write!(f, "{}", line),
            CmpSingleAddress(line) => write!(f, "{}", line),
            CmpThreeAddress(line) => write!(f, "{}", line),
            Goto(line) => write!(f, "{}", line),
            Label(line) => write!(f, "{}", line),
        }
    }
}

pub struct ThreeAddress {
    pub name: String,
    pub operator: Operator,
    pub left: String,
    pub right: String
}

impl ThreeAddress {
    pub fn new(name: String, operator: Operator, left: String, right: String) -> Self {
        Self {name, operator, left, right}
    }
}

impl Display for ThreeAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {} {} {}", self.name, self.left, self.operator.symbol(), self.right)
    }
}

pub struct SingleAddress {
    pub name: String,
    pub variable: String,
}

impl SingleAddress {
    pub fn new(name: String, variable: String) -> Self {
        Self { name, variable }
    }
}

impl Display for SingleAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.name, self.variable)
    }
}

pub struct CmpThreeAddress {
    pub comparison: Comparison,
    pub left: String,
    pub right: String,
    pub label: String,
}

impl CmpThreeAddress {
    pub fn new(comparison: Comparison, left: String, right: String, label: String) -> Self {
        Self {comparison, left, right, label}
    }
}

impl Display for CmpThreeAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "if {} {} {} goto {}", self.left, self.comparison, self.right, self.label)
    }
}

pub struct CmpSingleAddress {
    pub comparison: Comparison,
    pub variable: String,
    pub label: String,
}

impl CmpSingleAddress {
    pub fn new(comparison: Comparison, variable: String, label: String) -> Self {
        Self { comparison, variable, label }
    }
}

impl Display for CmpSingleAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "if {} {} goto {}", self.comparison, self.variable, self.label)
    }
}

pub struct Goto {
    pub label: String
}

impl Goto {
    pub fn new(label: String) -> Self {
        Self { label }
    }
}

impl Display for Goto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "goto {}", self.label)
    }
}

pub struct Label {
    pub label: String
}

impl Label {
    pub fn new(label: String) -> Self {
        Self { label }
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: ", self.label)
    }
}

pub enum Comparison {
    Eq,
    Neq,
    Lt,
    Gt,
    Leq,
    Geq,

    True,
    False
}

impl Display for Comparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Comparison::*;
        match self {
            Eq => write!(f, "=="),
            Neq => write!(f, "!="),
            Lt => write!(f, "<"),
            Gt => write!(f, ">"),
            Leq => write!(f, "<="),
            Geq => write!(f, ">="),

            True => write!(f, "true"),
            False => write!(f, "false"),
        }
    }
}

impl From<&Operator> for Comparison {
    fn from(op: &Operator) -> Self {
        use Operator::*;
        match op {
            Eq => Comparison::Eq,
            Neq => Comparison::Neq,
            Lt => Comparison::Lt,
            Gt => Comparison::Gt,
            Leq => Comparison::Leq,
            Geq => Comparison::Geq,
            _ => unreachable!()
        }
    }
}

impl From<Operator> for Comparison {
    fn from(op: Operator) -> Self {
        Comparison::from(&op)
    }
}