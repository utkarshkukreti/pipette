use std::io;

pub struct View<W: io::Write> {
    writer: W
}

impl<W: io::Write> View<W> {
    pub fn update(&mut self, _prompt: &str, _candidates: &[&str]) -> io::Result<()> {
        // Hide the cursor.
        try!(write!(self.writer, "\x1b[?25l"));

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
    assert_eq!(out, b"\x1b[?25l\x1b[?25h");
}
