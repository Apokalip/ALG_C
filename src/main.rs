use std::env;
use self::encoding::*;

mod encoding;

fn main() {
    let args: Vec<String> = env::args().collect();

    let encoded = encode_alg_c!(&args[1],0,0, char).expect("Could not encode provided string");
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
        let test_str_size = test_str.len();

        let encoded = encode_alg_c!(test_str,0,0, char).unwrap();
        let encoded_size = encoded.capacity();

        let decoded = decode_alg_c!(encoded, char).unwrap();
        assert_eq!(test_str, decoded);

        println!("test_small_example: {:?} |bsize: {} | esize: {}\n", now.elapsed(),test_str_size, encoded_size);
    }

    #[test]
    fn test_small_sbuff_size(){
        let now = time::Instant::now();

        for i in 0..100{
            let test_str = "ababcbababaaababcbababaaa";
            let encoded = encode_alg_c!(test_str,i,0, char).unwrap();
            let decoded = decode_alg_c!(encoded, char).unwrap();
            assert_eq!(test_str, decoded);
        }
        println!("testing searchbuffer small size from 0..1000 encode/decode string of size 25 : {:?}\n", now.elapsed());
    }

    #[test]
    fn test_small_look_ahead(){
        let now = time::Instant::now();

        for i in 0..100{
            let test_str = "ababcbababaaababcbababaaa";
            let encoded = encode_alg_c!(test_str,0,i, char).unwrap();
            let decoded = decode_alg_c!(encoded, char).unwrap();
            assert_eq!(test_str, decoded);
        }
        println!("testing lookahead small length from 0..1000 encode/decode string of size 25 : {:?}\n", now.elapsed());
    }
    #[test]
    fn test_large_100k_16threads(){
        let now = time::Instant::now();
        let test_str = fs::read_to_string("src/test_string.txt").expect("Could not read test_string");
        let test_str_size = test_str.capacity();

        let encoded = encode_alg_c!(&test_str,test_str.len()/16,0, char).unwrap();
        let encoded_size = encoded.capacity();

        let decoded = decode_alg_c!(encoded, char).unwrap();
        assert_eq!(test_str, decoded);

        println!("test_large_100k_16threads: {:?} |bsize: {} | esize: {}\n", now.elapsed(),test_str_size, encoded_size);
    }
    #[test]
    fn test_large_100k_utf8_16threads(){
        let now = time::Instant::now();
        let test_str = fs::read_to_string("src/test_string_utf8.txt").expect("Could not read test_string");
        let test_str_size = test_str.capacity();

        let encoded = encode_alg_c!(&test_str,test_str.len()/16,0, char).unwrap();
        let encoded_size = encoded.capacity();

        let decoded = decode_alg_c!(encoded, char).expect("Could not convert decoded chars to string");
        assert_eq!(test_str, decoded);

        println!("test_large_100k_utf8_16threads: {:?} |bsize: {} | esize: {}\n", now.elapsed(),test_str_size, encoded_size);
    }
    #[test]
    fn test_large_100k_u8_16threads(){
        let now = time::Instant::now();
        let test_u8s = fs::read("src/test_string.txt").expect("Could not read test_string");
        let test_u8s_size = test_u8s.capacity();

        let encoded = encode_alg_c!(&test_u8s,test_u8s.len()/16,0, u8).unwrap();
        let encoded_size = encoded.capacity();

        let decoded = decode_alg_c!(encoded, u8).unwrap();
        assert_eq!(String::from_utf8(test_u8s).expect("Could not convert decoded bytes to string"), decoded);

        println!("test_large_100k_u8_16threads: {:?} |bsize: {} | esize: {}\n", now.elapsed(),test_u8s_size, encoded_size);
    }
    #[test]
    fn test_large_100k_utf8_u8_16threads(){
        let now = time::Instant::now();
        let test_u8s = fs::read("src/test_string_utf8.txt").expect("Could not read test_string");
        let test_u8s_size = test_u8s.capacity();
        let encoded = encode_alg_c!(&test_u8s,test_u8s.len()/16,0, u8).unwrap();
        let encoded_size = encoded.capacity();
        let decoded = decode_alg_c!(encoded, u8).unwrap();
        assert_eq!(String::from_utf8(test_u8s).expect("Could not convert decoded bytes to string"), decoded);

        println!("test_large_100k_utf8_u8_16threads: {:?} |bsize: {} | esize: {} \n", now.elapsed(),test_u8s_size, encoded_size);
    }
}
