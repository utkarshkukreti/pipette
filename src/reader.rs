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

    pub fn read(&mut self) -> io::Result<Key> {
        use self::Key::*;

        let mut buf = [0];
        try!(self.inner.read(&mut buf));
        Ok(match buf[0] {
            3 => CtrlC,
            8 => CtrlH,
            10 => CtrlJ,
            13 => CtrlM,
            14 => CtrlN,
            16 => CtrlP,
            21 => CtrlU,
            127 => Delete,
            _ => Other
        })
    }
}

#[test]
fn test_reader_read() {
    use self::Key::*;

    let input = [3, 8, 10, 13, 14, 16, 21, 127, 1];
    let mut reader = Reader::new(&input[..]);

    assert_eq!(reader.read().unwrap(), CtrlC);
    assert_eq!(reader.read().unwrap(), CtrlH);
    assert_eq!(reader.read().unwrap(), CtrlJ);
    assert_eq!(reader.read().unwrap(), CtrlM);
    assert_eq!(reader.read().unwrap(), CtrlN);
    assert_eq!(reader.read().unwrap(), CtrlP);
    assert_eq!(reader.read().unwrap(), CtrlU);
    assert_eq!(reader.read().unwrap(), Delete);
    assert_eq!(reader.read().unwrap(), Other);
}
