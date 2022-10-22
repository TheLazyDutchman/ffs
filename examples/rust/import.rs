use parseal::{
    Parsable,
    language_formats::{Define, ImportData},
    parsing::{self, Identifier, Parse},
};

#[derive(Debug, Clone, Parsable)]
pub struct Import {
    #[value("use")]
    keyword: Identifier,
    name: Identifier,
}

impl Define<ImportData> for Import {
    fn name (&self) -> Identifier {
        self.name.clone()
    }
}

impl Into<ImportData> for Import {
    fn into(self) -> ImportData {
        todo!()
    }
}
