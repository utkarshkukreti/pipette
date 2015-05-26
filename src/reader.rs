use std::io;

pub struct Reader<R: io::Read> {
    inner: io::Chars<R>
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
            inner: inner.chars()
        }
    }

    pub fn read(&mut self) -> Option<Result<Key, io::CharsError>> {
        use self::Key::*;

        self.inner.next().map(|result| {
            result.map(|char| {
                match char as u32 {
                    3 => CtrlC,
                    8 => CtrlH,
                    10 => CtrlJ,
                    13 => CtrlM,
                    14 => CtrlN,
                    16 => CtrlP,
                    21 => CtrlU,
                    127 => Delete,
                    0 ... 31 => Other,
                    _ => Char(char)
                }
            })
        })
    }
}

#[test]
fn test_reader_read() {
    use self::Key::*;

    let input = [3, 8, 10, 13, 14, 16, 21, 127, 1,
                 32,
                 207, 128,
                 226, 153, 152,
                 240, 159, 153, 128];
    let mut reader = Reader::new(&input[..]);

    assert_eq!(reader.read().unwrap().unwrap(), CtrlC);
    assert_eq!(reader.read().unwrap().unwrap(), CtrlH);
    assert_eq!(reader.read().unwrap().unwrap(), CtrlJ);
    assert_eq!(reader.read().unwrap().unwrap(), CtrlM);
    assert_eq!(reader.read().unwrap().unwrap(), CtrlN);
    assert_eq!(reader.read().unwrap().unwrap(), CtrlP);
    assert_eq!(reader.read().unwrap().unwrap(), CtrlU);
    assert_eq!(reader.read().unwrap().unwrap(), Delete);
    assert_eq!(reader.read().unwrap().unwrap(), Other);
    assert_eq!(reader.read().unwrap().unwrap(), Char(' '));
    assert_eq!(reader.read().unwrap().unwrap(), Char('π'));
    assert_eq!(reader.read().unwrap().unwrap(), Char('♘'));
    assert_eq!(reader.read().unwrap().unwrap(), Char('🙀'));
    assert!(reader.read().is_none());
}
