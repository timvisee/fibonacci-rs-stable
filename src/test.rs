extern crate test;

use self::test::Bencher;

use num::BigUint;

use Fibonacci;

#[bench]
fn fib_u64(b: &mut Bencher) {
  b.iter(|| Fibonacci::<u64>::default().take(92).collect::<Vec<_>>());
}

#[bench]
fn fib_biguint(b: &mut Bencher) {
  b.iter(|| Fibonacci::<BigUint>::default().take(92).collect::<Vec<_>>());
}

#[test]
fn first_ten() {
  let ten: Vec<u8> = Fibonacci::default().take(10).collect();
  assert_eq!(ten, vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
}

#[test]
fn overflow() {
  assert_eq!(92, Fibonacci::<u64>::default().take(93).collect::<Vec<_>>().len());
}

#[test]
fn non_overflow() {
  assert_eq!(300, Fibonacci::<BigUint>::default().take(300).collect::<Vec<_>>().len());
}
