mod preprocessing;
mod constants;

use constants::ROUND_CONSTANTS;
// Shadows f32::constants::SQRT_X from std library
use constants::SQRT_2;
use constants::SQRT_3;
use constants::SQRT_5;
use constants::SQRT_7;
use constants::SQRT_11;
use constants::SQRT_13;
use constants::SQRT_17;
use constants::SQRT_19;

fn transfer_to_msg_schedule(block_data: u128, message_schedule: &mut [u32; 4]) {
    message_schedule[0] = (block_data >> 96) as u32;
    message_schedule[1] = (block_data >> 64) as u32;
    message_schedule[2] = (block_data >> 32) as u32;
    message_schedule[3] = (block_data >> 0) as u32;
}

fn create_message_schedule(block: [u128;4]) -> [u32; 64] {
    let mut w: [u32; 64] = [0; 64];

    // Place the block data in the first 16 u32
    for i in 0..4 {
        
        // Turns a slice into a array reference of known size
        let w_slice: &mut [u32;4] = &mut w[i*4..i*4+4].try_into().expect("Incorrect slice size, must be 4");
        
        transfer_to_msg_schedule(
            block[i],
            w_slice
        );
    }

    // Expand data to the whole message schedule array
    for i in 16..64 {
        let s_0 = u32::rotate_right(w[i-15], 7) ^ u32::rotate_right(w[i-15], 18) ^ (w[i-15] >> 3);
        let s_1 = u32::rotate_right(w[i-2], 17) ^ u32::rotate_right(w[i-2], 19) ^ (w[i-2] >> 10);
        w[i] = w[i-16] + s_0 + w[i-7] + s_1;
    }

    w
}

fn compress_block(h: [u32;8], w: &[u32; 64]) -> [u32;8] {
    let mut h = h; // make mutable

    for i in 0..64 {
        let s_1 = u32::rotate_right(h[4], 6) ^ u32::rotate_right(h[4], 11) ^ u32::rotate_right(h[4], 25);
        let ch = (h[4] & h[5]) ^ ((!h[4]) & h[6]);
        let tmp_1 = h[7] + s_1 + ch + ROUND_CONSTANTS[i] + w[i];
        let s_0 = u32::rotate_right(h[0], 2) ^ u32::rotate_right(h[0], 13) ^ u32::rotate_right(h[0], 22);
        let maj = (h[0] & h[1]) ^ (h[0] & h[2]) ^ (h[1] ^ h[2]);
        let tmp_2 = s_0 + maj;

        h[7] = h[6];
        h[6] = h[5];
        h[5] = h[4];
        h[4] = h[3] + tmp_1;
        h[3] = h[2];
        h[2] = h[1];
        h[1] = h[0];
        h[0] = tmp_1 + tmp_2;
    }

    h
}

pub fn hash(message: &[u8]) -> [u8;32] {
    // Initial hash value
    let mut h: [u32; 8] = [SQRT_2, SQRT_3, SQRT_5, SQRT_7, SQRT_11, SQRT_13, SQRT_17, SQRT_19];

    let blocks = preprocessing::blockify_msg(message);

    for block in blocks {
        let w = create_message_schedule(block);
        let h_comp = compress_block(h.clone(), &w);

        // Add the compressed block to the current hash
        h[0] += h_comp[0];
        h[1] += h_comp[1];
        h[2] += h_comp[2];
        h[3] += h_comp[3];
        h[4] += h_comp[4];
        h[5] += h_comp[5];
        h[6] += h_comp[6];
        h[7] += h_comp[7];
    }

    // Prepare the final hash as a byte array
    let mut hash: [u8; 32] = [0; 32];
    for i in 0..8 {
        hash.copy_from_slice(u32::to_be_bytes(h[i]).as_slice());
    }
    
    hash
}