#[cfg(test)]
mod tests {
    use diffie_hellman_starter_1::mod_inverse;
    use num::bigint::BigInt;
    use num::Num;

    #[test]
    fn capture_the_flag() {
        let n = BigInt::from_str_radix(
            "171731371218065444125482536302245915415603318380280392385291836472299752747934607246477508507827284075763910264995326010251268493630501989810855418416643352631102434317900028697993224868629935657273062472544675693365930943308086634291936846505861203914449338007760990051788980485462592823446469606824421932591",
            10,
        )
        .unwrap();
        let e = BigInt::from(65537);
        let ct = BigInt::from_str_radix(
            "161367550346730604451454756189028938964941280347662098798775466019463375610700074840105776873791605070092554650190486030367121011578171525759600774739890458414593857709994072516290998135846956596662071379067305011746842247628316996977338024343628757374524136260758515864509435302781735938531030576289086798942",
            10,
        )
        .unwrap();

        let d = mod_inverse(e, n.clone() - 1).unwrap();
        let pt = ct.modpow(&d, &n);
        println!("{}", String::from_utf8_lossy(&pt.to_signed_bytes_be()));
    }
}
