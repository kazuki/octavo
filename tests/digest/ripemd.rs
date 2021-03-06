use octavo::digest::ripemd::*;

use digest::Test;

const TESTS: &'static [Test<'static>] =
    &[Test {
          input: b"",
          output: &[0x9c, 0x11, 0x85, 0xa5, 0xc5, 0xe9, 0xfc, 0x54, 0x61, 0x28, 0x08, 0x97, 0x7e,
                    0xe8, 0xf5, 0x48, 0xb2, 0x25, 0x8d, 0x31],
      },
      Test {
          input: b"a",
          output: &[0x0b, 0xdc, 0x9d, 0x2d, 0x25, 0x6b, 0x3e, 0xe9, 0xda, 0xae, 0x34, 0x7b, 0xe6,
                    0xf4, 0xdc, 0x83, 0x5a, 0x46, 0x7f, 0xfe],
      },
      Test {
          input: b"abc",
          output: &[0x8e, 0xb2, 0x08, 0xf7, 0xe0, 0x5d, 0x98, 0x7a, 0x9b, 0x04, 0x4a, 0x8e, 0x98,
                    0xc6, 0xb0, 0x87, 0xf1, 0x5a, 0x0b, 0xfc],
      },
      Test {
          input: b"message digest",
          output: &[0x5d, 0x06, 0x89, 0xef, 0x49, 0xd2, 0xfa, 0xe5, 0x72, 0xb8, 0x81, 0xb1, 0x23,
                    0xa8, 0x5f, 0xfa, 0x21, 0x59, 0x5f, 0x36],
      },
      Test {
          input: b"abcdefghijklmnopqrstuvwxyz",
          output: &[0xf7, 0x1c, 0x27, 0x10, 0x9c, 0x69, 0x2c, 0x1b, 0x56, 0xbb, 0xdc, 0xeb, 0x5b,
                    0x9d, 0x28, 0x65, 0xb3, 0x70, 0x8d, 0xbc],
      },
      Test {
          input: b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
          output: &[0x12, 0xa0, 0x53, 0x38, 0x4a, 0x9c, 0x0c, 0x88, 0xe4, 0x05, 0xa0, 0x6c, 0x27,
                    0xdc, 0xf4, 0x9a, 0xda, 0x62, 0xeb, 0x2b],
      },
      Test {
          input: b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
          output: &[0xb0, 0xe2, 0x0b, 0x6e, 0x31, 0x16, 0x64, 0x02, 0x86, 0xed, 0x3a, 0x87, 0xa5,
                    0x71, 0x30, 0x79, 0xb2, 0x1f, 0x51, 0x89],
      }];

#[test]
fn simple_test_vectors() {
    for test in TESTS {
        test.test(Ripemd160::default());
    }
}

digest_quick!(Ripemd160, Type::RIPEMD160);
