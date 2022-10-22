use parseal::{
    language_formats::{DefineList, ImportData},
    parsing::{
        self,
        tokens,
        Group, Identifier, Parse, List,
    },
    Parsable,
};

use super::DoubleColon;

#[derive(Debug, Clone, Parsable)]
pub enum ImportPart {
    Name(Identifier),
    List(Group<tokens::Brace, List<ImportPart, tokens::Comma>>),
    Path(Identifier, DoubleColon, List<ImportPart, DoubleColon>),
}

impl ImportPart {
    pub fn names(&self) -> Vec<Identifier> {
        match self {
            Self::Name(name) => vec![name.clone()],
            Self::List(group) => {
                let mut list = Vec::new();
                for part in group.clone() {
                    list.extend(part.names());
                }
                list
            },
            Self::Path(_, _, list) => {
                let list: Vec<ImportPart> = list.clone().into();
                list.last().unwrap().names()
            },
        }
    }
}

#[derive(Debug, Clone, Parsable)]
pub struct Import {
    #[value("use")]
    keyword: Identifier,
    part: ImportPart,
    end: tokens::Semicolon,
}

impl DefineList<ImportData> for Import {
    fn names(&self) -> Vec<Identifier> {
        self.part.names()
    }
}

impl Into<Vec<ImportData>> for Import {
    fn into(self) -> Vec<ImportData> {
        todo!()
    }
}
