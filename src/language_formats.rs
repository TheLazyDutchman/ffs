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

pub struct TypeData {}

#[derive(Clone, Parsable, Debug)]
pub enum Definition<L>
where
    L: LanguageData,
{
    Function(L::Function),
    Import(L::Import),
    Variable(L::Variable),
    Type(L::Type),
}

pub trait LanguageData: Parse {
    type Function: Define<FunctionData>;
    type Import: DefineList<ImportData>;
    type Variable: DefineList<VariableData>;
    type Type: Define<TypeData>;
}
