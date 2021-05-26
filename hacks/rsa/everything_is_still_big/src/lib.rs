#![feature(destructuring_assignment)]

use num::bigint::BigInt;
use num::ToPrimitive;

fn is_perfect_square(n: BigInt) -> bool {
    let sq_mod256 = [
        1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
        0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
        0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1,
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
        0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0,
        0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1,
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
        0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
    ];
    if sq_mod256[(n.clone() % BigInt::from(256)).to_usize().unwrap()] == 0 {
        return false;
    }
    let t = vec![
        vec![1, 1, 0, 0, 1, 0, 0, 1, 0],
        vec![1, 1, 0, 0, 1],
        vec![1, 1, 1, 0, 1, 0, 0],
        vec![1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1],
        vec![1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1],
    ];

    let a: BigInt = n.clone() % ((((9 * 5) * 7) * 13) * 17);

    for t in t.iter() {
        if t[a.clone().to_usize().unwrap() % t.len()] == 0 {
            return false;
        }
    }
    n.sqrt().pow(2) == n
}

fn rational_to_contfrac(mut x: i32, mut y: i32) -> Vec<i32> {
    let mut res = Vec::new();
    while y > 0 {
        let a = x / y;
        res.push(a);
        (x, y) = (y, x - a * y);
    }
    res
}

fn contfrac_to_rational_iter(contfrac: Iterable<i32>)  {

    let (n0, d0) = (0, 1);
    let (n1, d1) = (1, 0);
    for q in contfrac {
        let n = ((q*n1) + n0);
        let d = ((q*d1) + d0);
//yield is unimplemented;
        let (n0, d0) = (n1, d1);
        let (n1, d1) = (n, d);
    }
}
/*
fn convergents_from_contfrac(contfrac: Iterable<i32>)  {
    "
    ref: https://www.cits.ruhr-uni-bochum.de/imperia/md/content/may/krypto2ss08/shortsecretexponents.pdf Section.3
    ";
    let (n_, d_) = (1, 0);
    for (i, (n, d)) in contfrac_to_rational_iter(contfrac).iter().enumerate() {
        if (i % 2) == 0 {
//yield is unimplemented;
        } else {
//yield is unimplemented;
        }
        let (n_, d_) = (n, d);
    }
}
fn attack(e: i32, n: i32) -> Option<i32> {
    "
    ref: https://www.cits.ruhr-uni-bochum.de/imperia/md/content/may/krypto2ss08/shortsecretexponents.pdf Section.4

    >>> attack(2621, 8927)
    5
    >>> attack(6792605526025, 9449868410449)
    569
    >>> attack(30749686305802061816334591167284030734478031427751495527922388099381921172620569310945418007467306454160014597828390709770861577479329793948103408489494025272834473555854835044153374978554414416305012267643957838998648651100705446875979573675767605387333733876537528353237076626094553367977134079292593746416875606876735717905892280664538346000950343671655257046364067221469807138232820446015769882472160551840052921930357988334306659120253114790638496480092361951536576427295789429197483597859657977832368912534761100269065509351345050758943674651053419982561094432258103614830448382949765459939698951824447818497599, 109966163992903243770643456296093759130737510333736483352345488643432614201030629970207047930115652268531222079508230987041869779760776072105738457123387124961036111210544028669181361694095594938869077306417325203381820822917059651429857093388618818437282624857927551285811542685269229705594166370426152128895901914709902037365652575730201897361139518816164746228733410283595236405985958414491372301878718635708605256444921222945267625853091126691358833453283744166617463257821375566155675868452032401961727814314481343467702299949407935602389342183536222842556906657001984320973035314726867840698884052182976760066141)
    4221909016509078129201801236879446760697885220928506696150646938237440992746683409881141451831939190609743447676525325543963362353923989076199470515758399
    ";
    let f_ = rational_to_contfrac(e, n);
    let iter = convergents_from_contfrac(f_)
    for (k, dg) in  iter{
        let edg = e*dg;
        let phi = edg / k;
        let x = (n - phi) + 1;
        if (x % 2) == 0&&is_perfect_square((x / 2).pow(2) - n) {
            let g = edg - (phi*k);
            return dg / g;
        }
    }
    return None;
}


 */

#[cfg(test)]
mod tests {
    use crate::{is_perfect_square, rational_to_contfrac};
    use diffie_hellman_starter_1::mod_inverse;
    use num::bigint::BigInt;
    use num::{Num, One};

    #[test]
    fn perfect_square() {
        assert_eq!(true, is_perfect_square(BigInt::from(100)));
        assert_eq!(
            true,
            is_perfect_square(
                BigInt::from_str_radix("2000000000000000000000000000", 10)
                    .unwrap()
                    .pow(2)
            )
        );
        assert_eq!(
            false,
            is_perfect_square(
                BigInt::from_str_radix("2000000000000000000000000000", 10)
                    .unwrap()
                    .pow(2)
                    + BigInt::one()
            )
        );
    }

    #[test]
    fn rational() {
        assert_eq!(vec![0, 2, 1, 3], rational_to_contfrac(4, 11))
    }
    
    
    #[test]
    fn capture_the_flag() {
        // I used http://factordb.com, or it can be used Wienner attack
        // https://github.com/orisano/owiener

        let n = BigInt::from_str_radix(
            "665166804cd78e8197073f65f58bca14e019982245fcc7cad74535e948a4e0258b2e919bf3720968a00e5240c5e1d6b8831d8fec300d969fccec6cce11dde826d3fbe0837194f2dc64194c78379440671563c6c75267f0286d779e6d91d3e9037c642a860a894d8c45b7ed564d341501cedf260d3019234f2964ccc6c56b6de8a4f66667e9672a03f6c29d95100cdf5cb363d66f2131823a953621680300ab3a2eb51c12999b6d4249dde499055584925399f3a8c7a4a5a21f095878e80bbc772f785d2cbf70a87c6b854eb566e1e1beb7d4ac6eb46023b3dc7fdf34529a40f5fc5797f9c15c54ed4cb018c072168e9c30ca3602e00ea4047d2e5686c6eb37b9",
            16,
        )
            .unwrap();
        println!("{}", n);

        let q = BigInt::from_str_radix(
            "131205304707717699800023219057082007986286045823683571663112014612188606710079038751853416273709729039622908861933527111469616900188875912430487264576215232569029320804579614330240773622645122871884209068761138439268551367198798009790636662892148063583135747945604771740458352899202428704645256790931460695949",
            10,
        ).unwrap();
        let p = BigInt::from_str_radix(
            "98444549679044409506244239144443867459824227934526036052949278261505813439015297459200379108752444235232667213138464076415095486907288282630595622287237215801470940146886371515679909322090871473412384894540642399950010296214525469622505798526072170187467562765920044646574445427364231529083610955760228212701",
            10,
        )
            .unwrap();
        let phi: BigInt = (p - 1) * (q - 1);
        let e = BigInt::from_str_radix("2c998e57bc651fe4807443dbb3e794711ca22b473d7792a64b7a326538dc528a17c79c72e425bf29937e47b2d6f6330ee5c13bfd8564b50e49132d47befd0ee2e85f4bfe2c9452d62ef838d487c099b3d7c80f14e362b3d97ca4774f1e4e851d38a4a834b077ded3d40cd20ddc45d57581beaa7b4d299da9dec8a1f361c808637238fa368e07c7d08f5654c7b2f8a90d47857e9b9c0a81a46769f6307d5a4442707afb017959d9a681fa1dc8d97565e55f02df34b04a3d0a0bf98b7798d7084db4b3f6696fa139f83ada3dc70d0b4c57bf49f530dec938096071f9c4498fdef9641dfbfe516c985b27d1748cc6ce1a4beb1381fb165a3d14f61032e0f76f095d", 16).unwrap();
        let ct = BigInt::from_str_radix(
            "503d5dd3bf3d76918b868c0789c81b4a384184ddadef81142eabdcb78656632e54c9cb22ac2c41178607aa41adebdf89cd24ec1876365994f54f2b8fc492636b59382eb5094c46b5818cf8d9b42aed7e8051d7ca1537202d20ef945876e94f502e048ad71c7ad89200341f8071dc73c2cc1c7688494cad0110fca4854ee6a1ba999005a650062a5d55063693e8b018b08c4591946a3fc961dae2ba0c046f0848fbe5206d56767aae8812d55ee9decc1587cf5905887846cd3ecc6fc069e40d36b29ee48229c0c79eceab9a95b11d15421b8585a2576a63b9f09c56a4ca1729680410da237ac5b05850604e2af1f4ede9cf3928cbb3193a159e64482928b585ac",
            16,
        )
            .unwrap();

        let d = mod_inverse(e, phi).unwrap();
        let pt = ct.modpow(&d, &n);
        println!("{}", String::from_utf8_lossy(&pt.to_signed_bytes_be()));
    }
}
