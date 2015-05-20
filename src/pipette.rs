use std::io;
use view::View;

pub struct Pipette<R: io::Read, W: io::Write> {
    reader: R,
    writer: W,
    candidates: Vec<String>,
    view: View
}

impl<R: io::Read, W: io::Write> Pipette<R, W> {
    pub fn new(reader: R, writer: W, candidates: Vec<String>) -> Pipette<R, W> {
        Pipette {
            reader: reader,
            writer: writer,
            candidates: candidates,
            view: View::new()
        }
    }
}
