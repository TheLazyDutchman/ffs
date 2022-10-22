use parseal::{
    Parsable,
    language_formats::{Define, VariableData},
    parsing::{self, Identifier, Parse},
};

#[derive(Debug, Clone, Parsable)]
pub struct Variable {
    #[value("let")]
    keyword: Identifier,
    name: Identifier,
}

impl Define<VariableData> for Variable {
    fn name(&self) -> Identifier {
        self.name.clone()
    }
}

impl Into<VariableData> for Variable {
    fn into(self) -> VariableData {
        todo!()
    }
}
