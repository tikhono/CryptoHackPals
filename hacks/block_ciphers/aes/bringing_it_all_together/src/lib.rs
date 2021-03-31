use diffusion_through_permutation::*;
use nalgebra::Matrix4;

const N_ROUNDS: u8 = 10;
const KEY: &[u8; 16] = b"\xc3,\\\xa6\xb5\x80^\x0c\xdb\x8d\xa5z*\xb6\xfe\\";
const CIPHERTEXT: &[u8; 16] = b"\xd1O\x14j\xa4+O\xb6\xa1\xc4\x08B)\x8f\x12\xdd";

fn expand_key<T0, RT>(master_key: T0) -> RT {
    "
    Expands and returns a list of KEY matrices for the given master_key.
    ";
    let r_con = [
        0, 1, 2, 4, 8, 16, 32, 64, 128, 27, 54, 108, 216, 171, 77, 154, 47, 94, 188, 99, 198, 151,
        53, 106, 212, 179, 125, 250, 239, 197, 145, 57,
    ];
    let mut key_columns = bytes2matrix(master_key);
    let iteration_size = (master_key.len() / 4);
    let columns_per_iteration = key_columns.len();
    let mut i = 1;
    while key_columns.len() < ((N_ROUNDS + 1) * 4) {
        let mut word = key_columns[-1].collect::<Vec<_>>();
        if (key_columns.len() % iteration_size) == 0 {
            word.append(word.pop(0));
            word = word.iter().map(|b| s_box[b]).collect::<Vec<_>>();
            word[0] ^= r_con[i];
            i += 1;
        } else {
            if master_key.len() == 32 && (key_columns.len() % iteration_size) == 4 {
                word = word.iter().map(|b| s_box[b]).collect::<Vec<_>>();
            }
        }
        word = bytes(
            zip(word, key_columns[-(iteration_size)])
                .iter()
                .map(|(i, j)| (i ^ j))
                .collect::<Vec<_>>(),
        );
        key_columns.append(word);
    }
    return (0..(key_columns.len() / 4))
        .iter()
        .map(|i| key_columns[(4 * i)..(4 * (i + 1))])
        .collect::<Vec<_>>();
}
fn decrypt<T0, T1, RT>(KEY: T0, CIPHERTEXT: T1) -> RT {
    let round_keys = expand_key(KEY);
    for i in ((N_ROUNDS - 1)..0).step_by(-1) { /*pass*/ }
    return plaintext;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
