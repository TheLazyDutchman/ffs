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
    List(Group<tokens::Brace, List<Box<ImportPart>, tokens::Comma>>),
    Path(Identifier, Option<Box<(DoubleColon, ImportPart)>>),
}

impl ImportPart {
    pub fn names(&self) -> Vec<Identifier> {
        match self {
            Self::List(group) => {
                let mut list = Vec::new();
                for part in group.clone() {
                    list.extend(part.names());
                }
                list
            },
            Self::Path(_, Some(part)) => {
                part.as_ref().1.names()
            }
            Self::Path(name, None) => {
                vec![name.clone()]
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
