use std::io;
use view::View;
use reader::Reader;

pub struct Pipette<R: io::Read, W: io::Write> {
    reader: Reader<R>,
    writer: W,
    candidates: Vec<String>,
    view: View
}

impl<R: io::Read, W: io::Write> Pipette<R, W> {
    pub fn new(reader: R, writer: W, candidates: Vec<String>) -> Pipette<R, W> {
        Pipette {
            reader: Reader::new(reader),
            writer: writer,
            candidates: candidates,
            view: View::new()
        }
    }

    pub fn exec(&mut self) -> Option<String> {
        use reader::Key::*;
        let mut selected = 0;
        let mut prompt = "";
        loop {
            self.view.update(&mut self.writer, prompt, &self.candidates,
                             selected);
            match self.reader.read() {
                Some(Ok(key)) => match key {
                    CtrlM => return Some(self.candidates[selected].clone()),
                    CtrlN => {
                        if selected + 1 < self.candidates.len() {
                            selected += 1
                        }
                    }
                    _ => unimplemented!()
                },
                _ => unimplemented!()
            }
        }
    }
}

#[test]
fn test_pipette_exec() {
    let input = b"\x0e\x0e\x0e\x0e\x0d";
    let mut out = vec![];
    let candidates = vec!["foo".into(), "bar".into(), "baz".into()];
    {
        let mut pipette = Pipette::new(&input[..], &mut out, candidates);
        assert_eq!(pipette.exec(), Some("baz".into()));
    }
    assert_eq!(out, "\
!?25l\r!K>\x20
\r!K!37;41mfoo!0m
\r!Kbar
\r!Kbaz!3A!3G!?25h\
!?25l\r!K>\x20
\r!Kfoo
\r!K!37;41mbar!0m
\r!Kbaz!3A!3G!?25h\
!?25l\r!K>\x20
\r!Kfoo
\r!Kbar
\r!K!37;41mbaz!0m!3A!3G!?25h\
!?25l\r!K>\x20
\r!Kfoo
\r!Kbar
\r!K!37;41mbaz!0m!3A!3G!?25h\
!?25l\r!K>\x20
\r!Kfoo
\r!Kbar
\r!K!37;41mbaz!0m!3A!3G!?25h\
".replace("!", "\x1b[").as_bytes());
}
