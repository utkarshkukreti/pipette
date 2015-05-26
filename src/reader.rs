use std::io;

pub struct Reader<R: io::Read> {
    inner: R
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

impl<R: io::Read> Reader<R> {
    pub fn new(inner: R) -> Reader<R> {
        Reader {
            inner: inner
        }
    }
}
