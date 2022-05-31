use std::{vec::Vec, iter::Iterator,  fmt::{Debug, Result as fmtResult, Formatter}};

//helper struct to make comparisons and debug easier for testing
#[derive(Clone,Copy)]
pub struct EncodedChunk<T: Clone>{
    pub offset: usize,
    pub len: usize,
    pub identf: T,
}

impl<T: Clone> EncodedChunk<T>{

    pub fn new(offset: usize, len: usize, identf: T) -> Self {
        EncodedChunk{
            offset,
            len,
            identf,
        }
    }
}

impl<T: PartialEq + Clone> PartialEq<EncodedChunk<T>> for EncodedChunk<T>{
    
    fn eq(&self, other: &EncodedChunk<T>) -> bool{
        self.offset == other.offset &&
        self.len == other.len &&
        self.identf == other.identf
    }
}

impl<T: std::fmt::Display + Clone> Debug for EncodedChunk<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "({},{},{})", self.offset, self.len, self.identf)
    }
}