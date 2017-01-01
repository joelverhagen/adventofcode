use decompresstokens::DecompressToken;

#[derive(Clone, Copy, Debug)]
pub enum DecompressError {
    CouldNotReadFile,
    NonAsciiCharEncountered,
    ExpectedInteger,
    ExpectedX,
    ExpectedCloseParenthesis,
    CouldNotParseInteger,
    InvalidRepeatDirective,
    UnexpectedToken(DecompressToken),
    ExpectedMoreCharacters,
}
