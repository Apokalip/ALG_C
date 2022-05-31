use super::encoded_chunk::EncodedChunk;
use std::thread;

//helper method for vec procedures
fn find_subsequence<T>(haystack: &[T], needle: &[T]) -> Option<usize>
    where for<'a> &'a [T]: PartialEq
{
    haystack.windows(needle.len()).rposition(|window| window == needle)
}
// Vector based solution, It is on avarage 20-30% faster
// Ofcourse Graphemes would be simply better memory wise and speed wise, but interview tests are no place for external libs/mods/crates.
fn encode_alg_c_vec_proc(input_data: &[char], look_ahead: usize) -> Vec<EncodedChunk<char>>{

    let data_len = input_data.len();
    // search buffer
    let mut sbuffer: Vec<char> = Vec::with_capacity(data_len);
    // result from the encoding
    let mut encoded_data: Vec<EncodedChunk<char>> = vec![];
    // main iterator
    let mut i = 0;

    while i < data_len {
        // if even the first char is not found
        let first_char = input_data[i];
        if !sbuffer.contains(&first_char){
            encoded_data.push(EncodedChunk::new(0,0,first_char));
            sbuffer.push(first_char);
            i += 1;
        } else {
            //caching patterns and idxs while searching
            let mut longest_pattern:&[char] = &[];
            let mut sbuffer_pattern_idx = 0;

            // take the most limiting length for the pattern search, be it input string size or look ahead limit,
            // as the string may not be split perfectly unless the buffersize/lookahead may not be multiples of the length
            let max_pattern_len = data_len.min(i + look_ahead);
            //pattern search

            for pattern_it in i..(max_pattern_len - 1){
                
                match find_subsequence(&sbuffer, &input_data[i..(pattern_it + 1)]){
                    Some(idx) => {
                        longest_pattern = &input_data[i..(pattern_it + 1)];
                        sbuffer_pattern_idx = idx;
                    },
                    None => {
                        break
                    },
                };
            }
            // fill the search buffer with the new found pattern + the last char
            sbuffer.extend(&input_data[i..i +(longest_pattern.len() + 1)]);

            let mut pattern_byte_len: usize = 0;
            longest_pattern.iter().for_each(|&ch| pattern_byte_len += ch.len_utf8());

            // pattern offset from the main 'cursor' i
            let mut pattern_byte_offset: usize = 0;
            input_data[sbuffer_pattern_idx..i].iter().for_each(|&ch| pattern_byte_offset += ch.len_utf8());

            encoded_data.push(EncodedChunk::new(
                pattern_byte_offset,
                pattern_byte_len,
                input_data[i +(longest_pattern.len())] 
            ));

            //move cursor over after the last char
            i += longest_pattern.len() + 1;
        }
    }
    encoded_data
}
// Vector based solution, but encoding over u8s and not char, want to see if it handles utf8 faster and how much memory it saves or wastes
// Ofcourse Graphemes would be simply better memory wise and speed wise, but interview tests are no place for external libs/mods/crates.
fn encode_alg_c_u8_vec_proc(input_data: &[u8], look_ahead: usize) -> Vec<EncodedChunk<u8>>{

    let data_len = input_data.len();
    // search buffer
    let mut sbuffer: Vec<u8> = vec![];
    // result from the encoding
    let mut encoded_data: Vec<EncodedChunk<u8>> = Vec::with_capacity(input_data.len());
    // main iterator
    let mut i = 0;

    while i < data_len {
        // if even the first char is not found
        let first_char = input_data[i];
        if !sbuffer.contains(&first_char){
            encoded_data.push(EncodedChunk::new(0,0,first_char));
            sbuffer.push(first_char);
            i += 1;
        } else {
            //caching patterns and idxs while searching
            let mut longest_pattern:&[u8] = &[];
            let mut sbuffer_pattern_idx = 0;

            // take the most limiting length for the pattern search, be it input string size or look ahead limit,
            // as the string may not be split perfectly unless the buffersize/lookahead may not be multiples of the length
            let max_pattern_len = data_len.min(i + look_ahead);
            //pattern search

            for pattern_it in i..(max_pattern_len - 1){
                
                match find_subsequence(&sbuffer, &input_data[i..(pattern_it + 1)]){
                    Some(idx) => {
                        longest_pattern = &input_data[i..(pattern_it + 1)];
                        sbuffer_pattern_idx = idx;
                    },
                    None => {
                        break
                    },
                };
            }
            // fill the search buffer with the new found pattern + the last char
            sbuffer.extend(&input_data[i..i +(longest_pattern.len() + 1)]);

            encoded_data.push(EncodedChunk::new(
                i - sbuffer_pattern_idx,
                longest_pattern.len(),
                input_data[i +(longest_pattern.len())] 
            ));

            //move cursor over after the last char
            i += longest_pattern.len() + 1;
        }
    }
    encoded_data
}
/// Single thread procedure relying on strings, not face for utf8
fn encode_alg_c_proc(input_string: &str, look_ahead: usize) -> Vec<EncodedChunk<char>>{

    // search buffer
    let mut sbuffer = String::new();
    // result from the encoding
    let mut encoded_data: Vec<EncodedChunk<char>> = vec![];
    // main iterator
    let mut i = 0;

    while i < input_string.len() {

        // if even the first char is not found
        let first_char = input_string.chars().nth(i).unwrap();
        if !sbuffer.contains(first_char){
            encoded_data.push(EncodedChunk::new(0,0,first_char));
            sbuffer.push(first_char);
            i += 1;
        } else {
            //caching patterns and idxs while searching
            let mut longest_pattern = "";
            let mut sbuffer_pattern_idx = 0;

            // take the most limiting length for the pattern search, be it input string size or look ahead limit,
            // as the string may not be split perfectly unless the buffersize/lookahead may not be multiples of the length
            let max_pattern_len = input_string.len().min(i + look_ahead);

            //pattern search
            for pattern_it in i..(max_pattern_len-1){
  
                match sbuffer.rfind(&input_string[i..(pattern_it+1)]){
                    Some(idx) => {
                        longest_pattern = &input_string[i..(pattern_it+1)];
                        sbuffer_pattern_idx = idx;
                    },
                    None => {
                        break
                    },
                };
            }

            // fill the search buffer with the new found pattern + the last char
            sbuffer.push_str(&input_string[i..i +(longest_pattern.len() + 1)]);
            
            encoded_data.push(EncodedChunk::new(
                i - sbuffer_pattern_idx,
                longest_pattern.len(),
                input_string.chars().nth(i +(longest_pattern.len())).unwrap() 
            ));
            //move cursor over after the last char
            i += longest_pattern.len() + 1;
        }
    }
    encoded_data
}
// correction if user gave invalid lengths
fn correct_lens(input_len: usize, sbuff_len: usize, look_ahead: usize) -> (usize, usize){
    
    let cor_sbuff_len = match sbuff_len{
        len if len < 1 => input_len,
        _ => sbuff_len
    };

    (cor_sbuff_len, match look_ahead{
        len if len < 1 => cor_sbuff_len,
        _ => look_ahead
    })
}
// handles thread spawns and collect results from procedure
// char based
pub fn _priv_encode_alg_c_mt(input_string: &str, sbuff_len: usize, look_ahead: usize) -> Option<Vec<EncodedChunk<char>>> {

    let input_data: Vec<char> = input_string.chars().collect();

    let (cor_sbuff_len, cor_look_ahead) = correct_lens(input_data.len(), sbuff_len, look_ahead);
    // num of threads is going to be +1 if there is left over
    let num_threads = input_data.len() / cor_sbuff_len;
    //get the left over if we could not split the string into perfect parts
    let left_over_len = input_data.len() - num_threads*cor_sbuff_len;
    let mut thread_handles = vec![];
    
    for i in 0..num_threads{
        let data_chunk = input_data[i*cor_sbuff_len .. (i+1)*cor_sbuff_len].to_vec();
           thread_handles.push(thread::spawn(move || {
            encode_alg_c_vec_proc(&data_chunk, cor_look_ahead)
        }));
    }
    // ! We can handle the left over 2 ways, leave it to the last spawned thread but then we need to pass searchbuffer size
    // to encode_alg_c_proc() which will have more calculations. The other choice is to create an extra thread, which will skip 
    // the need for search_buffer_size to be passed. Testing showed the second case to be a little fast but neglegable;
    if left_over_len > 0 {
        let data_chunk = input_data[cor_sbuff_len*num_threads..].to_vec();
        thread_handles.push(thread::spawn(move || {
            encode_alg_c_vec_proc(&data_chunk, cor_look_ahead)
           }));
    }

    // join all threads and collect results
    let mt_vecs = thread_handles.into_iter().map(|h| h.join().expect("Could not join encoding thread!"));

    let mut res: Vec<EncodedChunk<char>> = vec![];
    for v in mt_vecs{
        res.extend(v);
    }
    
    Some(res)
}
// handles thread spawns and collects results from procedure
// u8 based, byte based encoding, more of an experiment for utf8
pub fn _priv_encode_alg_c_u8_mt(input_data: &[u8], sbuff_len: usize, look_ahead: usize) -> Option<Vec<EncodedChunk<u8>>> {

    let (cor_sbuff_len, cor_look_ahead) = correct_lens(input_data.len(), sbuff_len, look_ahead);
    // num of threads is going to be +1 if there is left over
    let num_threads = input_data.len() / cor_sbuff_len;
    //get the left over if we could not split the string into perfect parts
    let left_over_len = input_data.len() - num_threads*cor_sbuff_len;
    let mut thread_handles = vec![];
    
    for i in 0..num_threads{
        let data_chunk = input_data[i*cor_sbuff_len .. (i+1)*cor_sbuff_len].to_vec();
           thread_handles.push(thread::spawn(move || {
            encode_alg_c_u8_vec_proc(&data_chunk, cor_look_ahead)
        }));
    }
    // ! We can handle the left over 2 ways, leave it to the last spawned thread but then we need to pass searchbuffer size
    // to encode_alg_c_proc() which will have more calculations. The other choice is to create an extra thread, which will skip 
    // the need for search_buffer_size to be passed. Testing showed the second case to be a little fast but neglegable;
    if left_over_len > 0 {
        let data_chunk = input_data[cor_sbuff_len*num_threads..].to_vec();
        thread_handles.push(thread::spawn(move || {
            encode_alg_c_u8_vec_proc(&data_chunk, cor_look_ahead)
           }));
    }

    // join all threads and collect results
    let mt_vecs = thread_handles.into_iter().map(|h| h.join().expect("Could not join encoding thread!"));

    let mut res: Vec<EncodedChunk<u8>> = vec![];
    for v in mt_vecs{
        res.extend(v);
    }
    
    Some(res)
}
// encoding macro to call different encoding methods easier
// last argument should be the type of encoding if its based on char or u8
// TODO: This macro can be made better when specilization is in stable so we can base of the type of data we are given
#[macro_export]
macro_rules! encode_alg_c{
    ($is:expr, $sblen:expr, $lalen:expr, char) => {
        _priv_encode_alg_c_mt($is, $sblen, $lalen)
    };
    ($is:expr, $sblen:expr, $lalen:expr, u8) => {
        _priv_encode_alg_c_u8_mt($is, $sblen, $lalen)
    };
}
// decoding using primaraly &str so there is no actual iteration and everything is based on the data of the EncodedChunk*s
pub fn decode_alg_c(input_vec: Vec<EncodedChunk<char>>) -> Result<String, &'static str>{

    let mut res = String::new();
    
    for chunk in input_vec{
        let res_len = res.len();
        
        //check for invalid encodings
        if res_len < chunk.offset || chunk.offset < chunk.len {
            return Err("Invalid Encoding")
        }
        //str slice params
        let start = res_len - chunk.offset;
        let end = start + chunk.len;

        // create new string from string slice over res itself, so we clone only the slice and not the whole string
        res.push_str(&res[start..end].to_string());
        // push the tuple character last
        res.push(chunk.identf);
    }
    Ok(res)
}
pub fn decode_alg_c_u8(input_vec: Vec<EncodedChunk<u8>>) -> Result<String, &'static str>{

    let mut res: Vec<u8> = vec![];
    
    for chunk in input_vec.iter(){
        let res_len = res.len();
        
        //check for invalid encodings
        if res_len < chunk.offset || chunk.offset < chunk.len {
            return Err("Invalid encoding");
        }
        //str slice params
        let start = res_len - chunk.offset;
        let end = start + chunk.len;

        // create new string from string slice over res itself, so we clone only the slice and not the whole string
        res.extend(&res[start..end].to_vec());
        // push the tuple character last
        res.push(chunk.identf);
    }
    
    match String::from_utf8(res){
        Ok(s) => Ok(s),
        Err(_) => panic!("Could not convert u8 to string utf8"),
    }
}

// decoding macro to call different encoding methods easier
#[macro_export]
macro_rules! decode_alg_c{
    ($is:expr, char) => {
        decode_alg_c($is)
    };
    ($is:expr, u8) => {
        decode_alg_c_u8($is)
    };
}