use nalgebra::Matrix4;

pub const XTIME: fn(u8) -> u8 = |a| {
    if a >= 128 {
        ((a << 1) ^ 27) & 255
    } else {
        a << 1
    }
};

pub fn mix_single_column(a: &mut [u8]) {
    let t = a[0] ^ a[1] ^ a[2] ^ a[3];
    let u = a[0];
    a[0] ^= t ^ XTIME(a[0] ^ a[1]);
    a[1] ^= t ^ XTIME(a[1] ^ a[2]);
    a[2] ^= t ^ XTIME(a[2] ^ a[3]);
    a[3] ^= t ^ XTIME(a[3] ^ u);
}

pub fn mix_columns(state: &mut Matrix4<u8>) {
    for mut column in state.column_iter_mut() {
        mix_single_column(column.as_mut_slice());
    }
}

pub fn inv_mix_single_column(a: &mut [u8]) {
    let u = XTIME(XTIME(a[0] ^ a[2]));
    let v = XTIME(XTIME(a[1] ^ a[3]));
    a[0] ^= u;
    a[1] ^= v;
    a[2] ^= u;
    a[3] ^= v;
}

pub fn inv_mix_columns(state: &mut Matrix4<u8>) {
    for mut column in state.column_iter_mut() {
        inv_mix_single_column(column.as_mut_slice());
    }
    mix_columns(state);
}

pub fn shift_rows(state: &mut Matrix4<u8>) {
    //shift 1 row left by 1
    state.swap((1, 0), (1, 1));
    state.swap((1, 1), (1, 2));
    state.swap((1, 2), (1, 3));
    //shift 2 row left by 2
    state.swap((2, 0), (2, 2));
    state.swap((2, 1), (2, 3));
    //shift 3 row left by 3
    state.swap((3, 3), (3, 2));
    state.swap((3, 2), (3, 1));
    state.swap((3, 1), (3, 0));
}

pub fn inv_shift_rows(state: &mut Matrix4<u8>) {
    //shift 1 row right by 1
    state.swap((1, 3), (1, 2));
    state.swap((1, 2), (1, 1));
    state.swap((1, 1), (1, 0));
    //shift 2 row right by 2
    state.swap((2, 0), (2, 2));
    state.swap((2, 1), (2, 3));
    //shift 3 row right by 3
    state.swap((3, 0), (3, 1));
    state.swap((3, 1), (3, 2));
    state.swap((3, 2), (3, 3));
}

#[cfg(test)]
mod tests {
    use crate::{inv_mix_columns, inv_shift_rows, mix_columns, mix_single_column, shift_rows};
    use ascii::IntoAsciiString;
    use nalgebra::Matrix4;
    use structure_of_aes::matrix2bytes;

    #[test]
    fn test_shift() {
        let mut state = Matrix4::new(
            206, 243, 61, 34, 171, 11, 93, 31, 16, 200, 91, 108, 150, 3, 194, 51,
        );
        let test = Matrix4::new(
            206, 243, 61, 34, 171, 11, 93, 31, 16, 200, 91, 108, 150, 3, 194, 51,
        );
        shift_rows(&mut state);
        inv_shift_rows(&mut state);
        assert_eq!(state, test);
    }

    #[test]
    fn test_mix_column() {
        //https://en.wikipedia.org/wiki/Rijndael_MixColumns
        let initial_state: &mut [u8] = &mut [198, 198, 198, 198];
        mix_single_column(initial_state);
        let destination_state: &[u8] = &[198, 198, 198, 198];
        assert_eq!(destination_state, initial_state);

        let initial_state: &mut [u8] = &mut [1, 1, 1, 1];
        mix_single_column(initial_state);
        let destination_state: &[u8] = &[1, 1, 1, 1];
        assert_eq!(destination_state, initial_state);

        let initial_state: &mut [u8] = &mut [212, 212, 212, 213];
        mix_single_column(initial_state);
        let destination_state: &[u8] = &[213, 213, 215, 214];
        assert_eq!(destination_state, initial_state);

        let initial_state: &mut [u8] = &mut [45, 38, 49, 76];
        mix_single_column(initial_state);
        let destination_state: &[u8] = &[77, 126, 189, 248];
        assert_eq!(destination_state, initial_state);

        let initial_state: &mut [u8] = &mut [219, 19, 83, 69];
        mix_single_column(initial_state);
        let destination_state: &[u8] = &[142, 77, 161, 188];
        assert_eq!(destination_state, initial_state);

        let initial_state: &mut [u8] = &mut [242, 10, 34, 92];
        mix_single_column(initial_state);
        let destination_state: &[u8] = &[159, 220, 88, 157];
        assert_eq!(destination_state, initial_state);
    }

    #[test]
    fn capture_the_flag() {
        let mut state = Matrix4::new(
            108, 106, 71, 86, 96, 62, 38, 72, 42, 184, 92, 209, 94, 79, 8, 54,
        );
        state.transpose_mut(); //There are problems to take mut slice from row because matrix is column ordered
        inv_mix_columns(&mut state);
        inv_shift_rows(&mut state);
        state.transpose_mut();
        let bytes = matrix2bytes(state).into_ascii_string().unwrap();
        println!("{:?}", bytes);
    }
}
