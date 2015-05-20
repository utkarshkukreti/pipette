pub struct Pipette {
    candidates: Vec<String>
}

impl Pipette {
    pub fn new(candidates: Vec<String>) -> Pipette {
        Pipette {
            candidates: candidates
        }
    }
}
