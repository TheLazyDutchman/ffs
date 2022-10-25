use parseal::{
    language_formats::{DefineList, ImportData},
    parsing::{
        self,
        tokens,
        Group, Identifier, Parse, List,
    },
    Parsable,
};

#[derive(Debug, Clone, Parsable)]
pub enum UsePart {
    List(Group<tokens::Brace, List<Box<UsePart>, tokens::Comma>>),
    Path(Identifier, Option<Box<(tokens::DoubleColon, UsePart)>>),
}

impl UsePart {
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
pub struct Use {
    #[value("use")]
    keyword: Identifier,
    part: UsePart,
    end: tokens::Semicolon,
}

#[derive(Debug, Clone, Parsable)]
pub enum Import {
    Use(Use),
    Mod(#[value("mod")] Identifier, Identifier, tokens::Semicolon),
}

impl DefineList<ImportData> for Import {
    fn names(&self) -> Vec<Identifier> {
        match self {
            Self::Use(value) => value.part.names(),
            Self::Mod(_, name, _) => vec![name.clone()],
        }
    }
}

impl Into<Vec<ImportData>> for Import {
    fn into(self) -> Vec<ImportData> {
        todo!()
    }
}
