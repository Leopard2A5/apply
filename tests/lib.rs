extern crate apply;

use apply::Apply;

#[test]
fn test_it_works() {
    struct Xample {
        num: i32
    }

    impl Xample {
        fn new(i: i32) -> Self {
            Xample {
                num: i
            }
        }
    }

    fn do_something_with_an_xample(x: &Xample) -> i32 {
        // complicated code goes here...
        x.num
    }

    let x = 3i32
        .apply(|x| x * 2)
        .apply(Xample::new)
        .apply_ref(do_something_with_an_xample);

    assert_eq!(x, 6);
}