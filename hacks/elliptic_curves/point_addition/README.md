[CryptoHack – Elliptic Curves challenges](https://cryptohack.org/challenges/ecc/)

> While working with elliptic curve cryptography, we will need to add points together. In the background challenges, we did this geometrically by finding a line that passed through two points, finding the third intersection and then reflecting along the y-axis.
>
> It turns out that there is an efficient algorithm for calculating the point addition law for an elliptic curve.
>
> Taken from "An Introduction to Mathematical Cryptography", _Jeffrey Hoffstein, Jill Pipher, Joseph H. Silverman_, the following algorithm will calculate the addition of two points on an elliptic curve
>
> **Algorithm for the addition of two points: P + Q**
>
> (a) If P = O, then P + Q = Q.  
> (b) Otherwise, if Q = O, then P + Q = P.  
> (c) Otherwise, write P = (x1, y1) and Q = (x2, y2).  
> (d) If x1 = x2 and y1 = −y2, then P + Q = O.  
> (e) Otherwise:  
>   (e1) if P ≠ Q: λ = (y2 - y1) / (x2 - x1)  
>   (e2) if P = Q: λ = (3x12 + a) / 2y1  
> (f) x3 = λ2 − x1 − x2,     y3 = λ(x1 −x3) − y1  
> (g) P + Q = (x3, y3)
>
>
> We are working with a finite field, so the above calculations should be done `mod p`, and we do not "divide" by an integer, we instead multiply by the modular inverse of a number. e.g. `1 / 5 = 9 mod 11`.
>
> We will work with the following elliptic curve, and prime:
>
> E: Y2 = X3 + 497 X + 1768, p: 9739
>
> You can test your algorithm by asserting: `X + Y = (1024, 4440)` and `X + X = (7284, 2107)` for `X = (5274, 2841)` and `Y = (8669, 740)`.
>
> Using the above curve, and the points `P = (493, 5564), Q = (1539, 4742), R = (4403,5202)`, find the point `S(x,y) = P + P + Q + R` by implementing the above algorithm.
>
> After calculating `S`, substitute the coordinates into the curve. Assert that the point `S` is in `E(Fp)`

> ### How to:
> Run all tests from this package:
>
>     cargo test --package point_addition --lib tests
>
> Capture the flag:
>
>     cargo test --package point_addition --lib tests::capture_the_flag -- --exact
>
> Test addition:
>
>     cargo test --package point_addition --lib tests::test_addition -- --exact
