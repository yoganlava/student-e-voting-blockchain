
//! Very Very Very Naive Implementation of SHA256

fn pad_message(message: &[u8]) -> Vec<u8> {
    // begin with the original message of length L bits
    let len = (message.len() * 8) as u64;
    let mut padded_message = message.to_vec();
    // append a single '1' bit
    // 128d = 1000 0000b
    padded_message.push(128);
    // append K '0' bits, where K is the minimum number >= 0 such that (L + 1 + K + 64) is a multiple of 512
    if (padded_message.len() % 64) < 56 {
        padded_message.extend(vec![0; 56 - (padded_message.len() % 64)]);
    } else {
        // pad 8 0s to create a new chunk and then fill in the rest
        padded_message.extend(vec![0; 8 + 56 - (padded_message.len() % 64)]);
    }

    // append L as a 64-bit big-endian integer, making the total post-processed length a multiple of 512 bits
    padded_message.extend(len.to_be_bytes());
    padded_message
}

pub fn hash(message: &[u8]) -> [u32; 8]{
    let padded_message = pad_message(message);
    let mut hash_state = crate::crypto::consts::H.clone();

    // break message into 512-bit chunks
    for chunk in padded_message.as_slice().chunks(64) {
        // create a 64-entry message schedule array w[0..63] of 32-bit words
        // (The initial values in w[0..63] don't matter, so many implementations zero them here)
        let mut w = [0u32; 64];
        // copy chunk into first 16 words w[0..15] of the message schedule array
        for i in 0..16 {
            let j = i * 4;
            w[i] = u32::from_be_bytes([chunk[j],chunk[j+1],chunk[j+2],chunk[j+3]]);
        }

        // Extend the first 16 words into the remaining 48 words w[16..63] of the message schedule array:
        for i in 16..64 {
            // s0 := (w[i-15] rightrotate  7) xor (w[i-15] rightrotate 18) xor (w[i-15] rightshift  3)
            let s0 = w[i-15].rotate_right(7) ^ w[i-15].rotate_right(18) ^ (w[i-15] >> 3);
            // s1 := (w[i-2] rightrotate 17) xor (w[i-2] rightrotate 19) xor (w[i-2] rightshift 10)
            let s1 = w[i-2].rotate_right(17) ^ w[i-2].rotate_right(19) ^ (w[i-2] >> 10);
            // w[i] := w[i-16] + s0 + w[i-7] + s1
            w[i] = w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1);
        }

        // Initialize working variables to current hash value:
        //    a := h0 (state[0])
        //    b := h1 (state[1])
        //    c := h2 (state[2])
        //    d := h3 (state[3])
        //    e := h4 (state[4])
        //    f := h5 (state[5])
        //    g := h6 (state[6])
        //    h := h7 (state[7])

        let mut state = hash_state.clone();

        // Compression function main loop:
        for i in 0..64 {
            // S1 := (e rightrotate 6) xor (e rightrotate 11) xor (e rightrotate 25)
            let S1 =  state[4].rotate_right(6) ^ state[4].rotate_right(11) ^ state[4].rotate_right(25);
            // ch := (e and f) xor ((not e) and g)
            let ch = (state[4] & state[5]) ^ (!state[4] & state[6]);
            // temp1 := h + S1 + ch + k[i] + w[i]
            let temp1 = state[7].wrapping_add(S1).wrapping_add(ch).wrapping_add(crate::crypto::consts::K[i]).wrapping_add(w[i]);
            // S0 := (a rightrotate 2) xor (a rightrotate 13) xor (a rightrotate 22)
            let S0 = state[0].rotate_right(2) ^ state[0].rotate_right(13) ^ state[0].rotate_right(22);
            // maj := (a and b) xor (a and c) xor (b and c)
            let maj =  (state[0] & state[1]) ^ (state[0] & state[2]) ^ (state[1] & state[2]);
            // temp2 := S0 + maj
            let temp2 = S0.wrapping_add(maj);
            // h := g
            // g := f
            // f := e
            // e := d + temp1
            // d := c
            // c := b
            // b := a
            // a := temp1 + temp2

            state[7] = state[6];
            state[6] = state[5];
            state[5] = state[4];
            state[4] = state[3].wrapping_add(temp1);
            state[3] = state[2];
            state[2] = state[1];
            state[1] = state[0];
            state[0] = temp1.wrapping_add(temp2);
        }

        // Add the compressed chunk to the current hash value:
        // h0 := h0 + a
        // h1 := h1 + b
        // h2 := h2 + c
        // h3 := h3 + d
        // h4 := h4 + e
        // h5 := h5 + f
        // h6 := h6 + g
        // h7 := h7 + h
        for i in 0..hash_state.len() {
            hash_state[i] = hash_state[i].wrapping_add(state[i]);
        }
    }

    // Produce the final hash value (big-endian):
    // hash := h0 append h1 append h2 append h3 append h4 append h5 append h6 append h7
    hash_state
}