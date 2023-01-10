#![cfg(feature = "derive")]

use arbitrary::{Arbitrary, Unstructured};

fn arbitrary_from<'a, T: Arbitrary<'a>>(input: &'a [u8]) -> T {
    let mut buf = Unstructured::new(input);
    T::arbitrary(&mut buf).expect("can create arbitrary instance OK")
}

/// This wrapper trait *implies* `Arbitrary`, but the compiler isn't smart enough to work that out
/// so when using this wrapper we *must* opt-out of the auto-generated `T: Arbitrary` bounds.
pub trait WrapperTrait: for<'a> Arbitrary<'a> {}

impl WrapperTrait for u32 {}

#[derive(Arbitrary)]
#[arbitrary(bound = "T: WrapperTrait")]
struct GenericSingleBound<T: WrapperTrait> {
    t: T,
}

#[test]
fn single_bound() {
    let v: GenericSingleBound<u32> = arbitrary_from(&[0, 0, 0, 0]);
    assert_eq!(v.t, 0);
}

#[derive(Arbitrary)]
#[arbitrary(bound = "T: WrapperTrait, U: WrapperTrait")]
struct GenericMultipleBounds<T: WrapperTrait, U: WrapperTrait> {
    t: T,
    u: U,
}

#[test]
fn multiple_bounds() {
    let v: GenericMultipleBounds<u32, u32> = arbitrary_from(&[1, 0, 0, 0, 2, 0, 0, 0]);
    assert_eq!(v.t, 1);
    assert_eq!(v.u, 2);
}
