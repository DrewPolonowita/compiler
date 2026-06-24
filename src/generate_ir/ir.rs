use std::fmt::Display;
use std::hash::Hash;
use crate::parser::parse_tree::Operator;

#[derive(Clone)]
pub enum Value {
    Temp(Temp),
    Prim(String)
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Value::*;
        match self {
            Temp(temp) => write!(f, "{}", temp),
            Prim(value) => write!(f, "{}", value)
        }
    }
}

pub struct LabelMaker<T> {
    curr_num: usize,
    _output: std::marker::PhantomData<T>
}

impl<T: FromId> LabelMaker<T> {
    pub fn next(&mut self) -> T {
        let id = self.curr_num;
        self.curr_num += 1;
        T::from_id(id)
    }

    pub fn new() -> Self {
        Self {
            curr_num: 0usize,
            _output: std::marker::PhantomData
        }
    }
}

#[derive(Clone)]
pub struct Temp {
    id: usize
}

#[derive(Clone)]
pub struct Label {
    id: usize
}

impl FromId for Label {
    fn from_id(id: usize) -> Self {
        Label { id }
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "l{}", self.id)
    }
}

impl FromId for Temp {
    fn from_id(id: usize) -> Self {
        Temp { id }
    }
}

pub trait FromId {
    fn from_id(id: usize) -> Self;
}

impl Display for Temp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "t{}", self.id)
    }
}


pub enum IRLine {
    ThreeAddress(ThreeAddress),
    SingleAddress(SingleAddress),
    CmpSingleAddress(CmpSingleAddress),
    CmpThreeAddress(CmpThreeAddress),
    Goto(Goto),
    Label(LabelBlock),
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
    pub name: Temp,
    pub operator: Operator,
    pub left: Value,
    pub right: Value
}

impl ThreeAddress {
    pub fn new(name: Temp, operator: Operator, left: Value, right: Value) -> Self {
        Self {name, operator, left, right}
    }
}

impl Display for ThreeAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {} {} {}", self.name, self.left, self.operator.symbol(), self.right)
    }
}

pub struct SingleAddress {
    pub name: Temp,
    pub variable: Value,
}

impl SingleAddress {
    pub fn new(name: Temp, variable: Value) -> Self {
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
    pub left: Value,
    pub right: Value,
    pub label: Label,
}

impl CmpThreeAddress {
    pub fn new(comparison: Comparison, left: Value, right: Value, label: Label) -> Self {
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
    pub variable: Value,
    pub label: Label,
}

impl CmpSingleAddress {
    pub fn new(comparison: Comparison, variable: Value, label: Label) -> Self {
        Self { comparison, variable, label }
    }
}

impl Display for CmpSingleAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "if {} {} goto {}", self.comparison, self.variable, self.label)
    }
}

pub struct Goto {
    pub label: Label
}

impl Goto {
    pub fn new(label: Label) -> Self {
        Self { label }
    }
}

impl Display for Goto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "goto {}", self.label)
    }
}

pub struct LabelBlock {
    pub label: Label
}

impl LabelBlock {
    pub fn new(label: Label) -> Self {
        Self { label }
    }
}

impl Display for LabelBlock {
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