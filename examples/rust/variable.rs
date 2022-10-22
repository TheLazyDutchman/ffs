use parseal::{
    language_formats::{DefineList, VariableData},
    parsing::{self, Identifier, Parse},
    Parsable,
};

#[derive(Debug, Clone, Parsable)]
pub struct Variable {
    #[value("let")]
    keyword: Identifier,
    name: Identifier,
}

impl DefineList<VariableData> for Variable {
    fn names(&self) -> Vec<Identifier> {
        todo!()
    }
}

impl Into<Vec<VariableData>> for Variable {
    fn into(self) -> Vec<VariableData> {
        todo!()
    }
}
