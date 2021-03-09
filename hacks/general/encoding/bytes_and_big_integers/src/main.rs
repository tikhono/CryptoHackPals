
use uint::construct_uint;

construct_uint!{pub struct U1024(16);}

fn main() {
    let string = "11515195063862318899931685488813747395775516287289682636499965282714637259206269";
    let num=U1024::from_dec_str(string).unwrap();
    let bytes:&mut[u8] =&mut[0;16*8];
    num.to_big_endian(bytes);
    for i in 0..bytes.len() {
        print!("{}", bytes[i] as char);
    }
}
