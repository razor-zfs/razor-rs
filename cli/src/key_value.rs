use std::error::Error;
use std::str::FromStr;

// #[derive(Debug)]
// pub struct Property {
//     name: String,
//     value: String,
// }

// impl clap::builder::ValueParserFactory for Property {
//     type Parser = PropertyParser;

//     fn value_parser() -> Self::Parser {
//         todo!()
//     }
// }

// #[derive(Clone, Debug)]
// pub struct PropertyParser;

// impl clap::builder::TypedValueParser for PropertyParser {
//     type Value = Property;

//     fn parse_ref(
//         &self,
//         cmd: &clap::Command,
//         arg: Option<&clap::Arg>,
//         value: &std::ffi::OsStr,
//     ) -> Result<Self::Value, clap::Error> {
//         todo!()
//     }
// }

pub fn parse<T, U>(text: &str) -> anyhow::Result<(T, U)>
where
    T: FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let (key, value) = text
        .split_once('=')
        .ok_or_else(|| anyhow::anyhow!("Invalid format: no '=' found in '{text}'"))?;
    let key = key.parse()?;
    let value = value.parse()?;
    Ok((key, value))
}
