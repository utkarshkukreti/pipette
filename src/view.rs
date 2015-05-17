use std::io;

pub struct View<W: io::Write> {
    writer: W
}

impl<W: io::Write> View<W> {
    pub fn update(&mut self, prompt: &str, _candidates: &[&str]) -> io::Result<()> {
        // Hide the cursor.
        try!(write!(self.writer, "\x1b[?25l"));

        // Clear the line and print the prompt.
        try!(write!(self.writer, "\r\x1b[K> {}", prompt));

        // Show the cursor.
        write!(self.writer, "\x1b[?25h")
    }
}

#[test]
fn test_view_update() {
    let mut out = vec![];
    {
        let mut v = View {
            writer: &mut out
        };
        v.update("f", &["foo", "bar", "baz", "quux"]).unwrap();
    }
    assert_eq!(out, "!?25l\r!K> f!?25h".replace("!", "\x1b[").as_bytes());
}
