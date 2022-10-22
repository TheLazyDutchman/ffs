use crate::{Parsable, parsing::{self, Parse, Identifier}};

pub trait Define<Data>: Parse + Into<Data> {
    fn name(&self) -> Identifier;
}

pub struct FunctionData {
}

pub struct ImportData {
}

pub struct VariableData {
}

#[derive(Clone, Parsable, Debug)]
pub enum Definition<L> where L: LanguageData {
    Function(L::Function),
    Import(L::Import),
    Variable(L::Variable),
}

pub trait LanguageData: Parse {
    type Function: Define<FunctionData>;
    type Import: Define<ImportData>;
    type Variable: Define<VariableData>;
}
