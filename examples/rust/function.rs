use parseal::{
    Parsable,
    language_formats::{Define, FunctionData},
    parsing::{self, Identifier, Parse},
};

#[derive(Debug, Clone, Parsable)]
pub struct Function {
    #[value("fn")]
    keyword: Identifier,
    name: Identifier,
}

impl Define<FunctionData> for Function {
    fn name(&self) -> Identifier {
        self.name.clone()
    }
}

impl Into<FunctionData> for Function {
    fn into(self) -> FunctionData {
        todo!()
    }
}
