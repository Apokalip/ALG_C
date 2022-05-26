use std::env;
use self::encoding::*;
use std::fs;

mod encoding;

fn main() {
    
    let test_str = fs::read_to_string("src/test_string.txt").expect("Could not read test_string");
    let encoded = encode_alg_c(&test_str,6,0).unwrap();
    let decoded = decode_alg_c(encoded).unwrap();
}

#[cfg(test)]
mod tests {
    use std::*;
    use super::*;

    #[test]
    fn test_small_example(){
        let now = time::Instant::now();
        let test_str = "ababcbababaa";
        //testing with no limited size of search buffer or look ahead
        let encoded = encode_alg_c(test_str,0,0).unwrap();
        println!("ENCODED: {:?}",encoded);
        let decoded = decode_alg_c(encoded).unwrap();
        assert_eq!(test_str, decoded);
        println!("Example test small speed {:?}", now.elapsed());
    }

    #[test]
    fn test_small_sbuff_size(){
        let now = time::Instant::now();

        for i in 0..100{
            let test_str = "ababcbababaaababcbababaaa";
            let encoded = encode_alg_c(test_str,i,0).unwrap();
            let decoded = decode_alg_c(encoded).unwrap();
            assert_eq!(test_str, decoded);
        }
        println!("testing searchbuffer small size from 0..1000 encode/decode string of size 25 : {:?}", now.elapsed());
    }

    #[test]
    fn test_small_look_ahead(){
        let now = time::Instant::now();

        for i in 0..100{
            let test_str = "ababcbababaaababcbababaaa";
            let encoded = encode_alg_c(test_str,0,i).unwrap();
            let decoded = decode_alg_c(encoded).unwrap();
            assert_eq!(test_str, decoded);
        }
        println!("testing lookahead small length from 0..1000 encode/decode string of size 25 : {:?}", now.elapsed());
    }

    #[test]
    fn test_large_1mil_16threads(){
        let now = time::Instant::now();
        let test_str = fs::read_to_string("src/test_string.txt").expect("Could not read test_string");
        let encoded = encode_alg_c(&test_str,test_str.len()/16,0).unwrap();
        let decoded = decode_alg_c(encoded).unwrap();
        assert_eq!(test_str, decoded);
        println!("Testing single string of len 100k alphanumerics; sbuff_size = 1mil/cores(16) : {:?}", now.elapsed());
    }
}
