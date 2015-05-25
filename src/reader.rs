use std::io;

pub struct Reader<R: io::Read> {
    inner: R
}
