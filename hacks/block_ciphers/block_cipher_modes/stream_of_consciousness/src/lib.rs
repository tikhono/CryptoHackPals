use ascii::IntoAsciiString;
use ecb_cbc_wtf::get_response;
use std::collections::BTreeMap;

pub fn receive_and_print_ciphertexts() {
    let mut map = BTreeMap::new();

    for _ in 0..100 {
        let addr = "http://aes.cryptohack.org/stream_consciousness/encrypt/".to_string();
        let ciphertext = get_response(addr);

        map.insert(ciphertext.len(), ciphertext);
    }
    for ciph in &map {
        println!("{}", ciph.1);
    }

    for ciph in &map {
        print!("{:?}\t", ciph.0 / 2);
        for chunk in ciph.1.as_bytes().chunks(32) {
            print!("{:?}", chunk.into_ascii_string().unwrap());
        }
        println!();
    }
}

mod tests {
    use _19_break_fixed_nonce_ctr_mode_usong_substitutions::fixed_xor;
    use _19_break_fixed_nonce_ctr_mode_usong_substitutions::score_keystreams;

    #[test]
    fn capture_the_flag() {
        let texts = ["0666472831a644a06ae23e043e",
                "0b66413758d15bb026e16b056935897a5d5a40a0",
                "087d513758d15fb12be12756683d903547560de764ca",
                "0d61506462dc41b821e42511213b903e157e44e27c8d6273081e",
                "2a614c67659e57b279f47e417369ca376a411efb25d753274f38ea42e32b01e7",
                "00671574709f0bad6aef2e5675358c34155c58fa3cc46e630e47e502f47c0cf4cb8024f92a697bf2e7c36207",
                "0033467f709d40f926e23813213f883f474a59e6798a6b361b09e856ba7019ba8c8735f92b6778bdf7c765425f",
                "1d7b477274d14eb633fe6b04743490335b5401ae60886d6f1309eb56b56b4df2849032bc302235cef0d47f460b0fb520",
                "077c193758d640b56aea24566834de2e5a1369e17c8875361b09e856a07a01f6cb8a24ab637d61eff4cf61410547bb749f",
                "017c4237618343ac2ead2a18657a963b454354ae78812b7a1647ee13f46805ff85c229bc636970e9e6866b505109bb758eec",
                "1e7b4c37759e0cad22e832566635de355b135def798a787f1400ac17ba7b4df89e8b2dbd2a6072bdf4ca6a09050fb1219fa49e7534",
                "0033467f709d40f56ac46c1a6d7a923546560deb66817e6f0e0fe518b33f04fccb8a24f9276170eefb8172091208b964cbaf927360b4",
                "1d7b503765945eab23ef2713212e96335b540de763c4787e1b13ac02bc7a4dea8a9135f9206f7bbae186644c5113bb7385ed9c657fba5a6dfe0b8846f6cfab17779e61",
                "1e7c407b75d165f922ec3d1321389b365c565beb74c4787e1f09ac02bc7e19baa2c222b6366271bde7c3674a1947a77488a5d3746eea4c7cad429353f6d5b1156a81261d967fe56299",
                "1976477f70815ff922e86b1e6029de375c405eeb74c4787e1f47f804b57603ba8a8c25f92a7d35fff4c56d09131ef46f84badd305cfb5660fe0f9347b39dac0d6e8423158362e363c818",
                "00345837649f44b83afd325a2113de3e504048fc66812c7f0e4bac02bc7a4dfc8a972dad647d35f0fcc863055105a175cb84d47d2bef567cbf128c4cf6dca81423992719c265eb61c3159c2e3910e989cf7581",
                "057c43723dd15cab25ef2a146d23c17a615b48f7308063785d13ac1dba701aba838d36f9277c70fce7df26400547bd72c7ed9b7f7cba5061b30b905cb7c9ad1664c36152c262e2698678ca233209fc89c37ecbcdc1aea241654983f15f",
                "0d7c597b68d15bb026e16b0269339031154745ef64c445311747e013b56904f48cc220f9306b76f2fbc226410414b66085a9d37165fe1860b6038815a2d5a10a668b200e8736c32ccb4ccf327c0fecdbc77cd6cdc0a3eb446f1b92f7140d2efebc2c6ef3",
                "1d7b506474d144b638fe2e052d7a8a325c400ded71967e7f1b00e956f93f05f59cc208f92f6174e9fdc326440814b16d8ded9a7e2bee507dad429f54a4cfad1964886f51c262e269df1ece237c1df5c58278c69e99e6a958751bafbf024538e0bf6267b87373e3d92a81a266cd4768b430bbba95f2b21c",
                "1e7b546331900cb525f96b19677a8a325c5d4afd309064770e47f81eb1714de98e872cbc272e61f2b5cb63090208f46c8abf857567f65761ad429d5bb29db11662993b1d8b78eb6eca5c9066341defcc8272ca8ecdabae0d685595f6164330eaba2168b3713ab1983780e732d14a2dad78b3b393e8fc7b23bc1a1a502077279391487ac916d296f29ca5a4fa6a59f332440d2f38a9dd4b74b81727e3cb4fc6",].to_vec();

        let bytes: Vec<Vec<u8>> = texts
            .iter()
            .map(|x| hex::decode(x).expect("invalid base64"))
            .collect();

        let key = score_keystreams(bytes.clone());

        for text in bytes {
            println!(
                "{}",
                String::from_utf8_lossy(&fixed_xor(&key.get(0..text.len()).unwrap(), &text))
            );
        }
        println!();
    }
}
