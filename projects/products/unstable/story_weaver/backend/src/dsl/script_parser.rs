use crate::diagnostics::Error;
use crate::dsl::Script;
use crate::io::JsonCodec;

pub struct ScriptParser;

impl ScriptParser {
    pub fn parse(input: &str) -> Result<Script, Error> {
        JsonCodec::decode(input)
    }
}
