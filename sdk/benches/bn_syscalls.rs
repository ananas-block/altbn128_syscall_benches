#![feature(test)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(unused_assignments)]
extern crate test;

use solana_program::alt_bn128::{
    alt_bn128_addition,
    alt_bn128_multiplication,
    alt_bn128_pairing
};
use {
    ark_bn254,
    ark_ec,
    ark_std::{test_rng, UniformRand},
        };
use ark_std::rand::prelude::StdRng;
use ark_ff::{
    PrimeField,
    BigInteger,
    Fp256,
    bytes::ToBytes,
};
use ark_ec::AffineCurve;

type G2 = ark_ec::short_weierstrass_jacobian::GroupAffine::<ark_bn254::g2::Parameters>;

type G1 = ark_ec::short_weierstrass_jacobian::GroupAffine::<ark_bn254::g1::Parameters>;

#[bench]
fn alt_bn128_addition_test(b: &mut Bencher) {
    use serde::Deserialize;
    use std::time::Instant;
    let TEST_DATA = r#"[
        {
            "Input": "18b18acfb4c2c30276db5411368e7185b311dd124691610c5d3b74034e093dc9063c909c4720840cb5134cb9f59fa749755796819658d32efc0d288198f3726607c2b7f58a84bd6145f00c9c2bc0bb1a187f20ff2c92963a88019e7c6a014eed06614e20c147e940f2d70da3f74c9a17df361706a4485c742bd6788478fa17d7",
            "Expected": "2243525c5efd4b9c3d3c45ac0ca3fe4dd85e830a4ce6b65fa1eeaee202839703301d1d33be6da8e509df21cc35964723180eed7532537db9ae5e7d48f195c915",
            "Name": "chfast1",
            "Gas": 150,
            "NoBenchmark": false
        }
    ]"#;

    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct TestCase {
        input: String,
        expected: String,
        name: String,
    }

    let test_cases: Vec<TestCase> = serde_json::from_str(TEST_DATA).unwrap();

    b.iter(|| {

        test_cases.iter().for_each(|test| {

            let input = array_bytes::hex2bytes_unchecked(&test.input);

            let result = alt_bn128_addition(&input);
            assert!(result.is_ok());
            let result = result.unwrap();

            let expected = array_bytes::hex2bytes_unchecked(&test.expected);
            assert_eq!(result, expected);
        });
    });
}

#[bench]
fn alt_bn128_multiplication_test(b: &mut Bencher) {
    use serde::Deserialize;

    let TEST_DATA = r#"[
        {
            "Input": "2bd3e6d0f3b142924f5ca7b49ce5b9d54c4703d7ae5648e61d02268b1a0a9fb721611ce0a6af85915e2f1d70300909ce2e49dfad4a4619c8390cae66cefdb20400000000000000000000000000000000000000000000000011138ce750fa15c2",
            "Expected": "070a8d6a982153cae4be29d434e8faef8a47b274a053f5a4ee2a6c9c13c31e5c031b8ce914eba3a9ffb989f9cdd5b0f01943074bf4f0f315690ec3cec6981afc",
            "Name": "chfast1",
            "Gas": 6000,
            "NoBenchmark": false
        },{
            "Input": "070a8d6a982153cae4be29d434e8faef8a47b274a053f5a4ee2a6c9c13c31e5c031b8ce914eba3a9ffb989f9cdd5b0f01943074bf4f0f315690ec3cec6981afc30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd46",
            "Expected": "025a6f4181d2b4ea8b724290ffb40156eb0adb514c688556eb79cdea0752c2bb2eff3f31dea215f1eb86023a133a996eb6300b44da664d64251d05381bb8a02e",
            "Name": "chfast2",
            "Gas": 6000,
            "NoBenchmark": false
        },{
            "Input": "025a6f4181d2b4ea8b724290ffb40156eb0adb514c688556eb79cdea0752c2bb2eff3f31dea215f1eb86023a133a996eb6300b44da664d64251d05381bb8a02e183227397098d014dc2822db40c0ac2ecbc0b548b438e5469e10460b6c3e7ea3",
            "Expected": "14789d0d4a730b354403b5fac948113739e276c23e0258d8596ee72f9cd9d3230af18a63153e0ec25ff9f2951dd3fa90ed0197bfef6e2a1a62b5095b9d2b4a27",
            "Name": "chfast3",
            "Gas": 6000,
            "NoBenchmark": false
        },{
            "Input": "1a87b0584ce92f4593d161480614f2989035225609f08058ccfa3d0f940febe31a2f3c951f6dadcc7ee9007dff81504b0fcd6d7cf59996efdc33d92bf7f9f8f6ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            "Expected": "2cde5879ba6f13c0b5aa4ef627f159a3347df9722efce88a9afbb20b763b4c411aa7e43076f6aee272755a7f9b84832e71559ba0d2e0b17d5f9f01755e5b0d11",
            "Name": "cdetrio1",
            "Gas": 6000,
            "NoBenchmark": false
        },{
            "Input": "17c139df0efee0f766bc0204762b774362e4ded88953a39ce849a8a7fa163fa901e0559bacb160664764a357af8a9fe70baa9258e0b959273ffc5718c6d4cc7cffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            "Expected": "29e587aadd7c06722aabba753017c093f70ba7eb1f1c0104ec0564e7e3e21f6022b1143f6a41008e7755c71c3d00b6b915d386de21783ef590486d8afa8453b1",
            "Name": "cdetrio6",
            "Gas": 6000,
            "NoBenchmark": false
        },{
            "Input": "039730ea8dff1254c0fee9c0ea777d29a9c710b7e616683f194f18c43b43b869073a5ffcc6fc7a28c30723d6e58ce577356982d65b833a5a5c15bf9024b43d98ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
            "Expected": "00a1a234d08efaa2616607e31eca1980128b00b415c845ff25bba3afcb81dc00242077290ed33906aeb8e42fd98c41bcb9057ba03421af3f2d08cfc441186024",
            "Name": "cdetrio11",
            "Gas": 6000,
            "NoBenchmark": false
        }
    ]"#;

    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct TestCase {
        input: String,
        expected: String,
    }

    let test_cases: Vec<TestCase> = serde_json::from_str(TEST_DATA).unwrap();
    println!("test_cases len: {:?}",test_cases.len() );
    b.iter(|| {

        test_cases.iter().for_each(|test| {
            let input = array_bytes::hex2bytes_unchecked(&test.input);
            let result = alt_bn128_multiplication(&input);
            assert!(result.is_ok());
            let result = result.unwrap();

            let expected = array_bytes::hex2bytes_unchecked(&test.expected);
            assert_eq!(result, expected);
        });
    });
}

fn get_random_g1(rng: &mut StdRng) -> [u8;64] {
    let success = false;
    let mut res : Option<G1>;
    while !success {

        let fp = Fp256::<ark_bn254::FqParameters>::rand(rng).into_repr().to_bytes_le();
        res = match <G1 as AffineCurve>::from_random_bytes(&fp[..]) {
            Some(res) => {
                let mut bytes = [0u8;65];
                <G1 as ToBytes>::write(&res, &mut bytes[..]).unwrap();
                return bytes[..64].try_into().unwrap();
            },
            None => None,
        };
    }
    [0u8;64]
}

fn get_random_g2(rng: &mut  StdRng) -> [u8;128] {

    let success = false;
    let mut res : Option<G2>;
    while !success {

        let fp = Fp256::<ark_bn254::FqParameters>::rand(rng).into_repr().to_bytes_le();
        res = match <G2 as AffineCurve>::from_random_bytes(&fp[..]) {
            Some(res) => {
                let mut bytes = [0u8;129];
                <G2 as ToBytes>::write(&res, &mut bytes[..]).unwrap();
                return bytes[..128].try_into().unwrap();
            },
            None => None,
        };
    }
    [0u8;128]
}

fn to_be_64(bytes: &[u8]) -> Vec<u8> {
    let mut vec = Vec::new();
    for b in bytes.chunks(32) {
        for byte in b.iter().rev() {
            vec.push(*byte);
        }
    }
    vec
}
fn to_be_128(bytes: &[u8]) -> Vec<u8> {
    let mut vec = Vec::new();
    for b in bytes.chunks(64) {
        for byte in b.iter().rev() {
            vec.push(*byte);
        }
    }
    vec
}
use criterion::{criterion_group, criterion_main, Criterion};


criterion_group!(benches,
    // alt_bn128_pairing_test_2,
    // // alt_bn128_pairing_test_rnd,
    alt_bn128_pairing_test_rnd_2,
    alt_bn128_multiplication_test_rnd
);
criterion_main!(benches);

const TEST_DATA: [[([u8; 64], [u8; 128]); 3];3 ] = [
    [
        (
            [169, 188, 126, 23, 234, 181, 49, 44, 76, 155, 186, 163, 180, 151, 19, 153, 6, 220, 171, 29, 119, 54, 44, 34, 82, 130, 81, 172, 144, 32, 252, 41, 51, 218, 77, 129, 230, 75, 37, 139, 138, 25, 61, 229, 38, 121, 209, 134, 47, 83, 24, 40, 105, 229, 156, 143, 191, 172, 172, 88, 204, 23, 187, 29],
            [133, 52, 151, 123, 19, 114, 157, 14, 21, 62, 189, 188, 4, 178, 35, 99, 225, 132, 32, 193, 205, 86, 200, 15, 25, 57, 244, 156, 6, 174, 131, 16, 112, 192, 162, 11, 208, 105, 38, 25, 207, 152, 137, 184, 141, 148, 183, 25, 137, 165, 117, 9, 241, 106, 140, 254, 1, 125, 113, 17, 96, 189, 169, 2, 253, 248, 3, 180, 29, 86, 110, 90, 49, 229, 224, 58, 22, 188, 76, 132, 220, 16, 176, 51, 132, 26, 126, 45, 224, 132, 17, 56, 248, 37, 12, 7, 23, 2, 42, 116, 42, 173, 235, 102, 244, 191, 177, 1, 93, 177, 63, 151, 44, 150, 232, 54, 181, 66, 207, 138, 144, 211, 104, 119, 163, 198, 6, 17]
        ),
        (
            [220, 210, 225, 96, 65, 152, 212, 86, 43, 63, 222, 140, 149, 68, 69, 209, 141, 89, 0, 170, 89, 149, 222, 17, 80, 181, 170, 29, 142, 207, 12, 12, 195, 251, 228, 187, 136, 200, 161, 205, 225, 188, 70, 173, 169, 183, 19, 63, 115, 136, 119, 101, 133, 250, 123, 233, 146, 120, 213, 224, 177, 91, 158, 15],
            [237, 246, 146, 217, 92, 189, 222, 70, 221, 218, 94, 247, 212, 34, 67, 103, 121, 68, 92, 94, 102, 0, 106, 66, 118, 30, 31, 18, 239, 222, 0, 24, 194, 18, 243, 174, 183, 133, 228, 151, 18, 231, 169, 53, 51, 73, 170, 241, 37, 93, 251, 49, 183, 191, 96, 114, 58, 72, 13, 146, 147, 147, 142, 25, 157, 127, 130, 113, 21, 192, 57, 239, 17, 247, 45, 92, 40, 131, 175, 179, 205, 23, 182, 243, 53, 212, 164, 109, 62, 50, 165, 5, 205, 239, 155, 29, 236, 101, 90, 7, 58, 177, 115, 230, 153, 59, 190, 247, 93, 57, 54, 219, 199, 36, 117, 24, 9, 172, 177, 203, 179, 175, 209, 136, 162, 196, 93, 39]
        ),
        (
            [181, 129, 186, 7, 53, 61, 26, 93, 210, 29, 170, 46, 100, 150, 94, 3, 69, 237, 166, 21, 152, 146, 211, 52, 142, 103, 21, 166, 133, 176, 141, 24, 57, 122, 149, 35, 146, 161, 222, 19, 116, 168, 229, 88, 0, 246, 241, 65, 134, 237, 213, 24, 65, 254, 219, 138, 55, 223, 50, 68, 107, 147, 187, 32],
            [83, 221, 254, 184, 55, 148, 227, 43, 133, 7, 18, 158, 114, 71, 125, 201, 138, 190, 192, 0, 56, 234, 29, 190, 13, 53, 55, 124, 65, 213, 82, 16, 190, 225, 85, 93, 216, 143, 253, 91, 162, 249, 28, 124, 77, 137, 187, 191, 41, 63, 204, 124, 190, 22, 134, 112, 142, 91, 162, 209, 153, 210, 182, 31, 36, 167, 184, 235, 213, 41, 254, 96, 37, 227, 187, 127, 87, 12, 115, 172, 212, 196, 214, 182, 240, 132, 194, 165, 181, 15, 200, 254, 250, 69, 45, 32, 97, 149, 114, 77, 166, 31, 30, 137, 84, 29, 211, 14, 204, 3, 70, 171, 70, 14, 213, 156, 243, 16, 201, 200, 211, 247, 42, 95, 196, 13, 58, 48]
        )
    ],
    [
        (
            [143,  15, 147,  99,  79,  60,  78,  50,   8, 203, 226,
               62,  60, 109, 217, 225, 121,  35,  63, 247,  36, 118,
               48,  28,  46, 227, 216, 210, 143, 152, 178,  32, 196,
               95, 169, 192,  62, 112, 118, 209,  62,  38,  48, 221,
               92, 177,  39,   6, 209, 164, 125, 146,  25,  41,  79,
               58,  75,   8,  43,  65, 211, 110, 225,  30],
          [133, 52, 151, 123, 19, 114, 157, 14, 21, 62, 189, 188, 4, 178, 35, 99, 225, 132, 32, 193, 205, 86, 200, 15, 25, 57, 244, 156, 6, 174, 131, 16, 112, 192, 162, 11, 208, 105, 38, 25, 207, 152, 137, 184, 141, 148, 183, 25, 137, 165, 117, 9, 241, 106, 140, 254, 1, 125, 113, 17, 96, 189, 169, 2, 253, 248, 3, 180, 29, 86, 110, 90, 49, 229, 224, 58, 22, 188, 76, 132, 220, 16, 176, 51, 132, 26, 126, 45, 224, 132, 17, 56, 248, 37, 12, 7, 23, 2, 42, 116, 42, 173, 235, 102, 244, 191, 177, 1, 93, 177, 63, 151, 44, 150, 232, 54, 181, 66, 207, 138, 144, 211, 104, 119, 163, 198, 6, 17]
        ),
        (
            [220, 210, 225, 96, 65, 152, 212, 86, 43, 63, 222, 140, 149, 68, 69, 209, 141, 89, 0, 170, 89, 149, 222, 17, 80, 181, 170, 29, 142, 207, 12, 12, 195, 251, 228, 187, 136, 200, 161, 205, 225, 188, 70, 173, 169, 183, 19, 63, 115, 136, 119, 101, 133, 250, 123, 233, 146, 120, 213, 224, 177, 91, 158, 15],
            [173, 107, 171, 22, 221, 71, 45, 8, 196, 71, 21, 41, 91, 194, 234, 150, 169, 187, 191, 168, 232, 15, 151, 135, 154, 78, 26, 82, 238, 227, 241, 40, 226, 243, 148, 20, 235, 209, 68, 253, 43, 11, 170, 29, 250, 120, 231, 225, 205, 97, 222, 24, 170, 83, 144, 237, 88, 237, 120, 135, 51, 94, 186, 31, 225, 243, 95, 76, 78, 195, 89, 183, 200, 17, 179, 211, 10, 171, 25, 250, 102, 190, 107, 2, 80, 178, 187, 180, 75, 67, 5, 167, 39, 0, 171, 13, 198, 43, 144, 117, 20, 112, 3, 248, 251, 68, 197, 76, 168, 116, 200, 43, 119, 58, 222, 243, 112, 199, 3, 134, 49, 71, 184, 111, 92, 200, 89, 4]
        ),
        (
            [43, 199, 220, 200, 152, 163, 210, 104, 247, 237,   3,
               10,  42, 146, 151, 211,  32, 128,  69, 115, 173, 153,
              226, 245, 198,  70, 127,  50, 105, 103,  69,   5, 225,
              143, 168, 217,  93,  12,  51, 233, 218, 140, 240,  72,
               95,  27,  69, 243,  32, 194, 245, 194, 132,  60,  63,
              203, 107, 244, 113, 109,  83, 157, 100,  21],
          [83, 221, 254, 184, 55, 148, 227, 43, 133, 7, 18, 158, 114, 71, 125, 201, 138, 190, 192, 0, 56, 234, 29, 190, 13, 53, 55, 124, 65, 213, 82, 16, 190, 225, 85, 93, 216, 143, 253, 91, 162, 249, 28, 124, 77, 137, 187, 191, 41, 63, 204, 124, 190, 22, 134, 112, 142, 91, 162, 209, 153, 210, 182, 31, 36, 167, 184, 235, 213, 41, 254, 96, 37, 227, 187, 127, 87, 12, 115, 172, 212, 196, 214, 182, 240, 132, 194, 165, 181, 15, 200, 254, 250, 69, 45, 32, 97, 149, 114, 77, 166, 31, 30, 137, 84, 29, 211, 14, 204, 3, 70, 171, 70, 14, 213, 156, 243, 16, 201, 200, 211, 247, 42, 95, 196, 13, 58, 48]
        )
    ],
    [
        (
            [34, 122, 253, 204, 243,  16, 201, 133, 161, 151,  13,
              130,  78, 126,  94, 163, 224,  32, 110, 105,  60, 173,
               80, 225,   5, 251, 211,  85,  42, 227, 225,  17,  66,
               75, 107, 118, 161, 223,  82, 148,  65, 172,  88, 173,
                9, 109, 108, 229, 250,  87, 112, 159, 113, 219, 102,
               31, 149,  48,  83,  81, 141, 139, 169,  17],
          [133, 52, 151, 123, 19, 114, 157, 14, 21, 62, 189, 188, 4, 178, 35, 99, 225, 132, 32, 193, 205, 86, 200, 15, 25, 57, 244, 156, 6, 174, 131, 16, 112, 192, 162, 11, 208, 105, 38, 25, 207, 152, 137, 184, 141, 148, 183, 25, 137, 165, 117, 9, 241, 106, 140, 254, 1, 125, 113, 17, 96, 189, 169, 2, 253, 248, 3, 180, 29, 86, 110, 90, 49, 229, 224, 58, 22, 188, 76, 132, 220, 16, 176, 51, 132, 26, 126, 45, 224, 132, 17, 56, 248, 37, 12, 7, 23, 2, 42, 116, 42, 173, 235, 102, 244, 191, 177, 1, 93, 177, 63, 151, 44, 150, 232, 54, 181, 66, 207, 138, 144, 211, 104, 119, 163, 198, 6, 17]
        ),
        (
            [220, 210, 225, 96, 65, 152, 212, 86, 43, 63, 222, 140, 149, 68, 69, 209, 141, 89, 0, 170, 89, 149, 222, 17, 80, 181, 170, 29, 142, 207, 12, 12, 195, 251, 228, 187, 136, 200, 161, 205, 225, 188, 70, 173, 169, 183, 19, 63, 115, 136, 119, 101, 133, 250, 123, 233, 146, 120, 213, 224, 177, 91, 158, 15],
            [27, 204, 124, 11, 165, 70, 231, 141, 30, 176, 235, 127, 5, 147, 187, 136, 179, 176, 39, 54, 240, 245, 69, 79, 225, 2, 29, 28, 30, 92, 220, 14, 154, 121, 195, 133, 58, 138, 48, 178, 244, 161, 30, 12, 144, 147, 201, 94, 26, 26, 180, 238, 105, 53, 232, 123, 16, 26, 111, 42, 131, 150, 17, 32, 184, 189, 171, 1, 21, 45, 85, 39, 172, 64, 214, 75, 179, 42, 172, 248, 41, 111, 116, 204, 218, 37, 202, 100, 74, 134, 56, 35, 193, 179, 194, 47, 24, 25, 165, 85, 203, 222, 32, 43, 140, 89, 155, 150, 92, 130, 129, 161, 37, 230, 36, 249, 77, 180, 149, 50, 16, 212, 248, 81, 4, 241, 71, 46]
        ),
        (
            [208,  81,  69, 193, 208, 184,   9, 149,   1,  84, 164,
              160,  88, 157,  70, 224, 244, 253,  90, 181,  20,  25,
              183, 146, 153, 228, 241, 189, 117, 142, 186,  30, 161,
              103,  48,  84,  73,  70, 218, 115, 168, 176, 143,  92,
              214,  13, 203,   2,  34, 146,  69,  99,  20,  32, 206,
              167, 153,  85,  92,  14, 242, 134,  25,   5],
          [83, 221, 254, 184, 55, 148, 227, 43, 133, 7, 18, 158, 114, 71, 125, 201, 138, 190, 192, 0, 56, 234, 29, 190, 13, 53, 55, 124, 65, 213, 82, 16, 190, 225, 85, 93, 216, 143, 253, 91, 162, 249, 28, 124, 77, 137, 187, 191, 41, 63, 204, 124, 190, 22, 134, 112, 142, 91, 162, 209, 153, 210, 182, 31, 36, 167, 184, 235, 213, 41, 254, 96, 37, 227, 187, 127, 87, 12, 115, 172, 212, 196, 214, 182, 240, 132, 194, 165, 181, 15, 200, 254, 250, 69, 45, 32, 97, 149, 114, 77, 166, 31, 30, 137, 84, 29, 211, 14, 204, 3, 70, 171, 70, 14, 213, 156, 243, 16, 201, 200, 211, 247, 42, 95, 196, 13, 58, 48]
        )
    ],


];

fn alt_bn128_pairing_test_2(c: &mut Criterion) {

    let input = vec![to_be_64(&TEST_DATA[0][0].0), to_be_128(&TEST_DATA[0][0].1), to_be_64(&TEST_DATA[0][1].0), to_be_128(&TEST_DATA[0][1].1)].concat();


    c.bench_function("pairing 2", |b| b.iter(|| alt_bn128_pairing(&input[..])));
    let input = vec![input, to_be_64(&TEST_DATA[0][2].0), to_be_128(&TEST_DATA[0][2].1)].concat();

    c.bench_function("pairing 3", |b| b.iter(|| alt_bn128_pairing(&input[..])));
    let input = vec![input, to_be_64(&TEST_DATA[1][0].0), to_be_128(&TEST_DATA[1][0].1)].concat();

    c.bench_function("pairing 4", |b| b.iter(|| alt_bn128_pairing(&input[..])));
    let input = vec![input, to_be_64(&TEST_DATA[1][1].0), to_be_128(&TEST_DATA[1][1].1)].concat();

    c.bench_function("pairing 5", |b| b.iter(|| alt_bn128_pairing(&input[..])));
    let input = vec![input, to_be_64(&TEST_DATA[1][2].0), to_be_128(&TEST_DATA[1][2].1)].concat();

    c.bench_function("pairing 6", |b| b.iter(|| alt_bn128_pairing(&input[..])));
    let input = vec![input, to_be_64(&TEST_DATA[2][0].0), to_be_128(&TEST_DATA[2][0].1)].concat();

    c.bench_function("pairing 7", |b| b.iter(|| alt_bn128_pairing(&input[..])));
    let input = vec![input, to_be_64(&TEST_DATA[2][1].0), to_be_128(&TEST_DATA[2][1].1)].concat();

    c.bench_function("pairing 8", |b| b.iter(|| alt_bn128_pairing(&input[..])));
    let input = vec![input, to_be_64(&TEST_DATA[2][2].0), to_be_128(&TEST_DATA[2][2].1)].concat();

    c.bench_function("pairing 9", |b| b.iter(|| alt_bn128_pairing(&input[..])));

}

fn alt_bn128_pairing_test_rnd_2(c: &mut Criterion) {
    let mut rng = test_rng();
    let mut input = Vec::new();
    for _ in 0..10000 {
        input.push([&to_be_64(&get_random_g1(&mut rng))[..], &to_be_128(&get_random_g2(&mut rng))[..]].concat());
    }
    let mut i = 0;
    c.bench_function("pairing rnd", |b| b.iter(|| {

        alt_bn128_pairing(&input[i][..]);
        i+=1;
    }));

    let mut i = 9_999;
    c.bench_function("pairing 2 rnd", |b| b.iter(|| {

        alt_bn128_pairing(&[&input[i][..], &input[i-1][..]].concat());
        i-=1;
    }));

}
use ark_ff::BigInteger256;
fn alt_bn128_multiplication_test_rnd(c: &mut Criterion) {
    let mut rng = test_rng();
    let mut input = Vec::new();
    for _ in 0..1_000_000 {
        let bigint = BigInteger256::rand(&mut rng);
        input.push([&to_be_64(&get_random_g1(&mut rng))[..], &bigint.to_bytes_be()[..]].concat());
    }
    let mut i = 0;
    c.bench_function("mul rnd", |b| b.iter(|| {
        alt_bn128_multiplication(&input[i][..]);
        i+=1;
    }));

}

fn alt_bn128_pairing_test_rnd(c: &mut Criterion) {


    for i in 0..3 {
        for j in 0..3 {
            let input = vec![to_be_64(&TEST_DATA[i][j].0), to_be_128(&TEST_DATA[i][j].1)].concat();
            c.bench_function("pairing 1 rnd", |b| b.iter(|| alt_bn128_pairing(&input[..])));

        }

    }

    for i in 0..3 {
        for j in 0..3 {
            let input = vec![to_be_64(&TEST_DATA[0][0].0), to_be_128(&TEST_DATA[0][0].1), to_be_64(&TEST_DATA[i][j].0), to_be_128(&TEST_DATA[i][j].1)].concat();
            c.bench_function("pairing 2 rnd", |b| b.iter(|| alt_bn128_pairing(&input[..])));
        }
    }


}
