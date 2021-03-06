use std::borrow::Borrow;
use std::io;

pub struct View;

impl View {
    pub fn new() -> View {
        View
    }

    pub fn update<W, S>(&mut self,
                        writer: &mut W,
                        prompt: &str,
                        candidates: &[S],
                        selected: usize) -> io::Result<()>
        where S: Borrow<str>, W: io::Write
    {
        // Hide the cursor.
        try!(write!(writer, "\x1b[?25l"));

        // Clear the line and print the prompt.
        try!(write!(writer, "\r\x1b[K> {}", prompt));

        for (i, candidate) in candidates.iter().enumerate() {
            // Go to the beginning of next line, and clear the line.
            try!(write!(writer, "\n\r\x1b[K"));

            if i == selected {
                // Red background; White foreground.
                try!(write!(writer, "\x1b[37;41m"))
            }

            // Print the candidate
            try!(write!(writer, "{}", candidate.borrow()));

            if i == selected {
                // Reset all SGR attributes.
                try!(write!(writer, "\x1b[0m"))
            }
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
        v.update(&mut out, "f", &["foo", "bar", "baz", "quux"], 2).unwrap();
    }
    assert_eq!(out, "!?25l\r!K> f
\r!Kfoo
\r!Kbar
\r!K!37;41mbaz!0m
\r!Kquux!4A!4G!?25h".replace("!", "\x1b[").as_bytes());
}
