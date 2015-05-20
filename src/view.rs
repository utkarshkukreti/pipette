use std::io;

pub struct View;

impl View {
    pub fn new() -> View {
        View
    }

    pub fn update<W: io::Write>(&mut self,
                                writer: &mut W,
                                prompt: &str,
                                candidates: &[&str]) -> io::Result<()> {
        // Hide the cursor.
        try!(write!(writer, "\x1b[?25l"));

        // Clear the line and print the prompt.
        try!(write!(writer, "\r\x1b[K> {}", prompt));

        for candidate in candidates {
            // Go to the next line, clear the line, and print the candidate.
            try!(write!(writer, "\n\r\x1b[K{}", candidate))
        }

        // Jump back to the last char of the prompt.
        try!(write!(writer, "\x1b[{}A\x1b[{}G",
                    candidates.len(),
                    prompt.chars().count() + 3));

        // Show the cursor.
        try!(write!(writer, "\x1b[?25h"));

        writer.flush()
    }
}

#[test]
fn test_view_update() {
    let mut out = vec![];
    {
        let mut v = View::new();
        v.update(&mut out, "f", &["foo", "bar", "baz", "quux"]).unwrap();
    }
    assert_eq!(out, "!?25l\r!K> f
\r!Kfoo
\r!Kbar
\r!Kbaz
\r!Kquux!4A!4G!?25h".replace("!", "\x1b[").as_bytes());
}
