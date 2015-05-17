use std::io;

pub struct View<W: io::Write> {
    writer: W
}
