#[cfg(test)]
mod tests {
    use aes::Aes128;
    use block_modes::block_padding::Pkcs7;
    use block_modes::{BlockMode, Cbc};
    use sha1::{Digest, Sha1};

    type Aes128Cbc = Cbc<Aes128, Pkcs7>;

    #[test]
    fn capture_the_flag() {
        /*
        sage: def SmartAttack(P,Q,p):
        ....:     E = P.curve()
        ....:     Eqp = EllipticCurve(Qp(p, 2), [ ZZ(t) + randint(0,p)*p for t in E.a_invariants() ])
        ....:
        ....:     P_Qps = Eqp.lift_x(ZZ(P.xy()[0]), all=True)
        ....:     for P_Qp in P_Qps:
        ....:         if GF(p)(P_Qp.xy()[1]) == P.xy()[1]:
        ....:             break
        ....:
        ....:     Q_Qps = Eqp.lift_x(ZZ(Q.xy()[0]), all=True)
        ....:     for Q_Qp in Q_Qps:
        ....:         if GF(p)(Q_Qp.xy()[1]) == Q.xy()[1]:
        ....:             break
        ....:
        ....:     p_times_P = p*P_Qp
        ....:     p_times_Q = p*Q_Qp
        ....:
        ....:     x_P,y_P = p_times_P.xy()
        ....:     x_Q,y_Q = p_times_Q.xy()
        ....:
        ....:     phi_P = -(x_P/y_P)
        ....:     phi_Q = -(x_Q/y_Q)
        ....:     k = phi_Q/phi_P
        ....:     return ZZ(k)
        sage: p = 0xa15c4fb663a578d8b2496d3151a946119ee42695e18e13e90600192b1d0abdbb6f787f90c8d102ff88e284dd4526f5f6b6c980bf88f1d0490714b67e8a2a2b77
        ....: a = 0x5e009506fcc7eff573bc960d88638fe25e76a9b6c7caeea072a27dcd1fa46abb15b7b6210cf90caba982893ee2779669bac06e267013486b22ff3e24abae2d42
        ....: b = 0x2ce7d1ca4493b0977f088f6d30d9241f8048fdea112cc385b793bce953998caae680864a7d3aa437ea3ffd1441ca3fb352b0b710bb3f053e980e503be9a7fece
        sage: E = EllipticCurve(GF(p), [a, b])
        sage: P = E(303471280937553790810298875011338244400875853944897275058152581090063424339217270368490525749098254377523363001170737518904130243694510639561731
        ....: 2498769005, 498664509858261641569007408223781762442433333907496936452754810704287617548089413257639961102784740287988557413012505084271005229187026810
        ....: 1817275410204850)
        sage: Q = E(474819837289540486675211176662642192748197151948347138381304400569938831765039531519392222670460493745474260823312483187049363600372520030768393
        ....: 9875286865, 242187330900227984102179136988448330805149721579801750980530204110246831063682206070735078977606521260689048970659736952656233625627225854
        ....: 4226688832663757)
        sage: n = SmartAttack(P, Q, p)
        sage: B = E(0x7f0489e4efe6905f039476db54f9b6eac654c780342169155344abc5ac90167adc6b8dabacec643cbe420abffe9760cbc3e8a2b508d24779461c19b20e242a38, 0xdd04134e74
        ....: 7354e5b9618d8cb3f60e03a74a709d4956641b234daa8a65d43df34e18d00a59c070801178d198e8905ef670118c15b0906d3a00a662d3a2736bf)
        sage: B*n
        (8216782777192629016736082577047876662181587556895841333932300215083185803392455455078234452846594885807223123796905544359993306809106491336354148716965075 : 148242255193707745238490492470982569663420403534103630669750159547273939511000254936452583511650128798498874821400341065904433354709622703123910071981138 : 1)
        */

        let shared_secret = "8216782777192629016736082577047876662181587556895841333932300215083185803392455455078234452846594885807223123796905544359993306809106491336354148716965075";

        let mut hasher = Sha1::new();
        hasher.update(shared_secret);

        let key = hasher.finalize().to_vec();
        let key = key.get(0..16).unwrap();

        let iv = hex::decode("719700b2470525781cc844db1febd994").unwrap();

        let flag = hex::decode("335470f413c225b705db2e930b9d460d3947b3836059fb890b044e46cbb343f0")
            .unwrap();

        let cipher = Aes128Cbc::new_from_slices(key, &iv).unwrap();
        let mut buf = flag;

        if cipher.decrypt(&mut buf).is_ok() {};
        println!("{}", String::from_utf8_lossy(&buf));
    }
}
