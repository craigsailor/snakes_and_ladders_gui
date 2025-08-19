use class_group::primitives::vdf::VDF;
use class_group::{ABDeltaTriple, BinaryQF, pari_init};
use curv::BigInt;
use curv::arithmetic::traits::*;
use curv::cryptographic_primitives::hashing::HmacExt;
use hmac::Hmac;
use sha2::Sha512;
use std::ops::Shl;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::time::Duration;

pub fn start_search(a_b_delta: ABDeltaTriple) -> Receiver<BigInt> {
    let (tx, rx) = channel();
    thread::spawn(move || {
        unsafe {
            class_group::pari_init(1000000000, 2);
        }
        let g = BinaryQF::binary_quadratic_form_disc(&a_b_delta).reduce();
        let mut y = g.clone();
        let hundred = BigInt::from(100);
        if y.a.mod_floor(&hundred).is_zero() {
            tx.send(BigInt::zero()).unwrap();
        }
        let mut iteration = BigInt::from(1);
        let one = BigInt::from(1);
        loop {
            y = y.compose(&y).reduce();
            if y.a.mod_floor(&hundred).is_zero() {
                tx.send(iteration.clone()).unwrap();
            }
            iteration += &one;
            thread::sleep(Duration::from_millis(5)); // Adjust duration as needed
        }
    });
    rx
}

/*
Auxillary functions, taken from vdf.rs */
pub fn custom_setup(x: &BigInt) -> ABDeltaTriple {
    let disc = BigInt::from_str_radix(
        "-33113823931246733065610185160556059556094015405298556868184554415304275770508484956550367629901423347856001088308353083506236980600018729315119158888545170400248173152829933177518867657744268262446452892187819691675188700067377284304929290024952792667257440456310106172327122846283386191071754104113516886289900697664534434962391227639705115239359835498839137436278040307655519949916627736216445696327070203290002138952858567696222579847232658415685807710091074341642589939921525639",
        10,
    ).unwrap();

    unsafe {
        pari_init(1000000000, 2);
    }
    let (a, b) = h_g(&disc, x);
    ABDeltaTriple { a, b, delta: disc }
}

/// helper function H_G(x)
/// Claudio algorithm:
/// 1) i = 0,
/// 2) r = prng(x,i)
/// 3) b = 2r + 1 // guarantee division by 4 later
/// 4) u = (b^2 - delta^2) / 4   // = ac
/// 5) choose small c at random and check if u/c is integral
/// 6) if true: take a = u/c
/// 7) if false : i++; goto 2.
fn h_g(disc: &BigInt, x: &BigInt) -> (BigInt, BigInt) {
    let mut i = 0;
    let two = BigInt::from(2);
    let max = BigInt::from(20);
    let mut b = &two * prng(x, i, disc.bit_length()) + BigInt::one();
    let mut c = two.clone();
    let mut b2_minus_disc: BigInt = b.pow(2) - disc;
    let four = BigInt::from(4);
    let mut u = b2_minus_disc.div_floor(&four);
    while u.mod_floor(&c) != BigInt::zero() {
        b = &two * prng(x, i, disc.bit_length()) + BigInt::one();
        b2_minus_disc = b.pow(2) - disc;
        u = b2_minus_disc.div_floor(&four);
        i += 1;
        c = (&c.next_prime()).mod_floor(&max);
    }
    let a = u.div_floor(&c);
    (a, b)
}

fn prng(seed: &BigInt, i: usize, bitlen: usize) -> BigInt {
    let i_bn = BigInt::from(i as i32);
    let mut res = Hmac::<Sha512>::new_bigint(&i_bn)
        .chain_bigint(seed)
        .result_bigint();
    let mut tmp: BigInt = res.clone();
    let mut res_bit_len = res.bit_length();
    while res_bit_len < bitlen {
        tmp = Hmac::<Sha512>::new_bigint(&i_bn)
            .chain_bigint(&tmp)
            .result_bigint();
        res = &res.shl(res_bit_len) + &tmp;
        res_bit_len = res.bit_length();
    }
    // prune to get |res| = bitlen
    res >> (res_bit_len - bitlen)
}
