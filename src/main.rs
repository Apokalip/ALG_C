use std::env;
use self::encoding::*;

mod encoding;

fn main() {
    let args: Vec<String> = env::args().collect();

    let encoded = encode_alg_c(&args[1],0,0).expect("Could not encode provided string");
    println!("encoded {:?}", encoded);
    let decoded = decode_alg_c(encoded).expect("Decoding failed");
    println!("decoded {:?}", decoded);
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
    fn test_large_100k_16threads(){
        let now = time::Instant::now();
        let test_str = fs::read_to_string("src/test_string.txt").expect("Could not read test_string");
        let encoded = encode_alg_c(&test_str,test_str.len()/16,0).unwrap();
        let decoded = decode_alg_c(encoded).unwrap();
        assert_eq!(test_str, decoded);
        println!("Testing single string of len 100k alphanumerics; sbuff_size = 100k/cores(16) : {:?}", now.elapsed());
    }


    #[test]
    fn test_large_100k_utf8_16threads(){
        let now = time::Instant::now();
        let test_str = fs::read_to_string("src/test_string_utf8.txt").expect("Could not read test_string");
        let encoded = encode_alg_c(&test_str,test_str.len()/16,0).unwrap();
        let decoded = decode_alg_c(encoded).unwrap();
        assert_eq!(test_str, decoded);
        println!("Testing single string of len 100k utf8; sbuff_size = 100k/cores(16) : {:?}", now.elapsed());
    }
}
