use super::encoded_chunk::EncodedChunk;
use std::thread;

//helper method for vec procedure
fn find_subsequence(haystack: &[char], needle: &[char]) -> Option<usize>
{	
    haystack.windows(needle.len()).rposition(|window| window == needle)
}

// Vector based solution, It is on avarage 20-30% faster
// Ofcourse Graphemes would be simply better memory wise and speed wise, but interview tests are no place for external libs/mods/crates.
fn encode_alg_c_vec_proc(input_data: &[char], look_ahead: usize) -> Option<Vec<EncodedChunk>>{

    let data_len = input_data.len();
    // search buffer
    let mut sbuffer: Vec<char> = Vec::with_capacity(data_len);
    // result from the encoding
    let mut encoded_data: Vec<EncodedChunk> = vec![];
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
    Some(encoded_data)
}

/// Single thread procedure relying on strings, not face for utf8
fn encode_alg_c_proc(input_string: &str, look_ahead: usize) -> Option<Vec<EncodedChunk>>{

    // search buffer
    let mut sbuffer = String::new();
    // result from the encoding
    let mut encoded_data: Vec<EncodedChunk> = vec![];
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
    Some(encoded_data)
}

// handles thread spawns and collect results from procedure
fn encode_alg_c_mt(input_string: &str, sbuff_len: usize, look_ahead: usize) -> Option<Vec<EncodedChunk>> {


    let input_data: Vec<char> = input_string.chars().collect();

    // num of threads is going to be +1 if there is left over
    let num_threads = input_data.len() / sbuff_len;
    //get the left over if we could not split the string into perfect parts
    let left_over_len = input_data.len() - num_threads*sbuff_len;

    let mut thread_handles = vec![];

    for i in 0..num_threads{
        let data_chunk = input_data[i*sbuff_len .. (i+1)*sbuff_len].to_vec();
           thread_handles.push(thread::spawn(move || {
            encode_alg_c_vec_proc(&data_chunk, look_ahead)
        }));
    }

    // ! We can handle the left over 2 ways, leave it to the last spawned thread but then we need to pass searchbuffer size
    // to encode_alg_c_proc() which will have more calculations. The other choice is to create an extra thread, which will skip 
    // the need for search_buffer_size to be passed. Testing showed the second case to be a little fast but neglegable;
    if left_over_len > 0 {
        let data_chunk = input_data[sbuff_len*num_threads..].to_vec();
        thread_handles.push(thread::spawn(move || {
            encode_alg_c_vec_proc(&data_chunk, look_ahead)
           }));
    }

    // join all threads and collect results
    let mt_vecs = thread_handles.into_iter().map(|h| h.join().expect("Could not join encoding thread!"));

    let mut res: Vec<EncodedChunk> = vec![];
    for v in mt_vecs{
        res.extend(v.expect("Failed to get vector from thread"));
    }
    
    Some(res)
}

// actual encoding method, checks for sbuff_len and look_ahead to be ready for procedure
pub fn encode_alg_c(input_string: &str, sbuff_len: usize, look_ahead: usize) -> Option<Vec<EncodedChunk>> {
    let cor_sbuff_len = match sbuff_len{
        len if len < 1 => input_string.len(),
        _ => sbuff_len
    };

    let cor_look_ahead_len = match look_ahead{
        len if len < 1 => cor_sbuff_len,
        _ => look_ahead
    };

    encode_alg_c_mt(input_string, cor_sbuff_len, cor_look_ahead_len)
}

// decoding using primaraly &str so there is no actual iteration and everything is based on the data of the EncodedChunk*s
pub fn decode_alg_c(input_vec: Vec<EncodedChunk>) -> Option<String>{

    let mut res = String::new();
    
    for chunk in input_vec{
        let res_len = res.len();
        
        //check for invalid encodings
        if res_len < chunk.offset || chunk.offset < chunk.len {
            return None
        }
        //str slice params
        let start = res_len - chunk.offset;
        let end = start + chunk.len;

        // create new string from string slice over res itself, so we clone only the slice and not the whole string
        res.push_str(&res[start..end].to_string());
        // push the tuple character last
        res.push(chunk.identf);
    }
    Some(res)
}


