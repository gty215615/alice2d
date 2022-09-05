

#[derive(Debug,Clone, Copy)]
pub struct Cursor {
    pub index:usize,
    pub at_return_after:bool
}

impl Cursor {
    pub fn new( index: usize ) -> Self {
        Self { index, at_return_after:false }
    }
}



