use crate::parsing::Parse;

pub trait LanguageData: Parse {
    type FunctionData;
}
