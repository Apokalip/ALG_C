use super::encoded_chunk::EncodedChunk;
use std::thread;

//helper method for vec procedure
fn find_subsequence(haystack: &[char], needle: &[char]) -> Option<usize>
{	
	haystack.windows(needle.len()).position(|window| window == needle)
}

// Vector based solution, It is on avarage 20-30% faster, but it handles writing offset taking the first occuring pattern
// and not the last as the example was given in the PDF. In decoding we use &str so that does not matter for performance as we do not iterate
// over the already decoded data but simply use the tuples to figure out the params for the slices
fn encode_alg_c_vec_proc(input_string: &str, look_ahead: usize) -> Option<Vec<EncodedChunk>>{

    // search buffer
    let mut sbuffer: Vec<char> = vec![];
	let input_data: Vec<char> = input_string.chars().collect();
    // result from the encoding
    let mut encoded_data: Vec<EncodedChunk> = vec![];
    // main iterator
    let mut i = 0;

	let data_len = input_data.len();

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
				
                match (find_subsequence(&sbuffer, &input_data[i..(pattern_it + 1)])){
                    Some(idx) => {
                        longest_pattern = &input_data[i..(pattern_it+1)];
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
    Some(encoded_data)
}

/// Single thread procedure relying on strings but cannot handle safely utf8
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

	let mut res: Vec<EncodedChunk> = vec![];
	// num of threads is going to be +1 if there is left over
	let num_threads = input_string.len() / sbuff_len;
	//get the left over if we could not split the string into perfect parts
	let left_over_len = input_string.len() - num_threads*sbuff_len;

	let mut thread_handles = vec![];

	for i in 0..num_threads{
		let str_chunk = input_string[i*sbuff_len .. (i+1)*sbuff_len].to_string();
       	thread_handles.push(thread::spawn(move || {
			encode_alg_c_proc(&str_chunk, look_ahead)
			//encode_alg_c_vec_proc(&str_chunk, look_ahead)
        }));
	}

	// ! We can handle the left over 2 ways, leave it to the last spawned thread but then we need to pass searchbuffer size
	// to encode_alg_c_proc() which will have more calculations. The other choice is to create an extra thread, which will skip 
	// the need for search_buffer_size to be passed. Testing showed the second case to be a little fast but neglegable;
	if left_over_len > 0 {
		let str_chunk = input_string[sbuff_len*num_threads..].to_string();
		thread_handles.push(thread::spawn(move || {
			encode_alg_c_proc(&str_chunk, look_ahead)
			//encode_alg_c_vec_proc(&str_chunk, look_ahead)
   		}));
	}

	// join all threads and collect results
	let mt_vecs = thread_handles.into_iter().map(|h| h.join().expect("Could not join encoding thread!"));

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
// The vector solution takes the pattern from the first as it is best for perfomance as reversing vectors is slow.
// The string solution takes the closest/largest pattern to the cursor as searching for subslice is easier like in the example given.
pub fn decode_alg_c(input_vec: Vec<EncodedChunk>) -> Option<String>{

    let mut res = String::new();
    
    for chunk in input_vec{
        let mut res_len = res.len();
        
        //check for invalid encodings
        if res.len() < chunk.offset {
        	return None
        }
        let start = res_len - chunk.offset;
        let end = start + chunk.len;
        // create new string from string slice over res itself, so we clone only the slice and not the whole string
        res.push_str(&res[start..end].to_string());
        // push the tuple character last
        res.push(chunk.identf);
    }

    Some(res)
}


