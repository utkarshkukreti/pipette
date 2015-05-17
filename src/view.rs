use std::io;

pub struct View<W: io::Write> {
    writer: W
}

impl<W: io::Write> View<W> {
    pub fn update(&mut self, _prompt: &str, _candidates: &[&str]) -> io::Result<()> {
        Ok(())
    }
}
