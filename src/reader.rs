use std::io;

pub struct Reader<R: io::Read> {
    inner: R
}

pub enum Key {
    Char(char),
    CtrlC,
    CtrlH,
    CtrlJ,
    CtrlM,
    CtrlN,
    CtrlP,
    CtrlU,
    Delete,
    Other
}
