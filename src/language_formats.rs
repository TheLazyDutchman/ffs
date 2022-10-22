use crate::{
    parsing::{self, Identifier, Parse},
    Parsable,
};

pub trait Define<Data>: Parse + Into<Data> {
    fn name(&self) -> Identifier;
}

pub trait DefineList<Data>: Parse + Into<Vec<Data>> {
    fn names(&self) -> Vec<Identifier>;
}

pub struct FunctionData {}

pub struct ImportData {}

pub struct VariableData {}

#[derive(Clone, Parsable, Debug)]
pub enum Definition<L>
where
    L: LanguageData,
{
    Function(L::Function),
    Import(L::Import),
    Variable(L::Variable),
}

pub trait LanguageData: Parse {
    type Function: Define<FunctionData>;
    type Import: DefineList<ImportData>;
    type Variable: DefineList<VariableData>;
}
