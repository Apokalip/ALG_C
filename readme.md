# ALG_C
ALG_C personal test private

ALG_C compression based [Manta_Coding_Challenge.pdf](https://github.com/Apokalip/ALG_C/files/8775547/Manta_Coding_Challenge.pdf)

There are 2 solutions: one based on strings, and one based on vectors. Both are multithreaded(parallel).
encode_alg_c_proc() for the string "naive" approach
encode_alg_c_vec_proc() for the vector based

String Solution follow the example given in the PDF and tries to take the largest pattern which is closest to the main iterating "cursor". 
In this solution was the "naive" solution following the example.

Vector solution is faster and safer, as we use char and we collect data through Iterators which should ensure we are actually collecting Unicode scalar values.
On string that are smaller than 50 chars, String solution may seem faster because of the creation of vectors, but the vector solution scales a lot better.
The Vector solution showed 20-30% better performance using 100k length random generated string.
  
The best and safest solution would be using Graphemes, but as a test external libraries/modules/crates are not recommended. 
I tried to stick to the standalone and things I wrote on the spot.
Implementing Graphemes on the spot would have taken a lot more time than the allowed time so I did not go that route.