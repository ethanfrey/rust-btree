extern crate quickcheck;
extern crate rand;

extern crate btree;

use self::quickcheck::{Arbitrary, Gen, QuickCheck, StdGen};

/*
Model: insert and get

* generate a sequence of operations
* run this sequence against our btree as well as BTreeMap
* at every step, verify that get has same return values

*/

// btree: u32 -> u64
#[derive(Clone, Debug)]
enum Op {
    Insert(u32, u64),
    Get(u32),
}
use Op::{Insert, Get};

#[derive(Clone, Debug)]
struct Ops(Vec<Op>);

const KEY_SPACE: u32 = 50;
const NUM_OPS: u32 = 200;

impl Arbitrary for Op {
        fn arbitrary<G: Gen>(g: &mut G) -> Op {
            let k : u32 = g.gen_range(1, KEY_SPACE);

            if g.gen_weighted_bool(2) {
                return Insert(k, g.next_u64());
            }
            Get(k)
        }
}

impl Arbitrary for Ops {
    fn arbitrary<G: Gen>(g: &mut G) -> Ops {
        let mut res = vec![];
        for _ in 0..NUM_OPS {
            res.push(Op::arbitrary(g))
        }
        Ops(res)
    }

    fn shrink(&self) -> Box<Iterator<Item = Self>> {
        // this is basically the default Vec shrink impl,
        let mut smaller = vec![];

        for i in 0..self.0.len() {
            let mut clone = self.clone();
            clone.0.remove(i);
            smaller.push(clone);
        }

        Box::new(smaller.into_iter())
    }
}

fn btree_matches_std(ops: Ops) -> bool {
    let mut tree = btree::Tree::default();
    let mut std = std::collections::BTreeMap::new();

    for op in ops.0 {
        match op {
            Insert(k, v) => {
                tree.insert(k, v);
                std.insert(k, v);
            },
            Get(k) => {
                if tree.get(&k) != std.get(&k) {
                    return false;
                }
            }
        }
    }
    true
}

#[test]
fn medium_property_works() {
    QuickCheck::new()
        .gen(StdGen::new(rand::thread_rng(), 1))
        .tests(1000)
        .max_tests(10000)
        .quickcheck(btree_matches_std as fn(Ops) -> bool);
}

