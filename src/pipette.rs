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
}
