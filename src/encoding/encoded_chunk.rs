use std::fmt::{Debug, Result, Formatter};

// struct holding the encoded tuple (offset,len,char)
// makes it easier to implement tests
#[derive(Clone,Copy)]
pub struct EncodedChunk{
    pub offset: usize,
    pub len: usize,
    pub identf: char,
}

impl EncodedChunk{

    pub fn new(offset: usize, len: usize, identf: char) -> Self {
        EncodedChunk{
            offset,
            len,
            identf,
        }
    }
}

impl PartialEq<EncodedChunk> for EncodedChunk{
    fn eq(&self, other: &EncodedChunk) -> bool{
        self.offset == other.offset &&
        self.len == other.len &&
        self.identf == other.identf
    }
}

impl Debug for EncodedChunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({},{},{})", self.offset, self.len, self.identf)
    }
}