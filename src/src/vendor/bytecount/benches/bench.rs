#[macro_use]
extern crate bencher;
extern crate rand;
extern crate bytecount;

use rand::Rng;

use bytecount::{count, naive_count, naive_count_32};

fn random_bytes(len: usize) -> Vec<u8> {
    rand::thread_rng().gen_iter::<u8>().take(len).collect::<Vec<_>>()
}

macro_rules! bench {
    ($i: expr, $name_naive: ident, $name_32: ident, $name_hyper: ident) => {
        fn $name_naive(b: &mut bencher::Bencher) {
            let haystack = random_bytes($i);
            b.iter(|| naive_count(&haystack, 10));
        }

        fn $name_32(b: &mut bencher::Bencher) {
            let haystack = random_bytes($i);
            b.iter(|| naive_count_32(&haystack, 10));
        }

        fn $name_hyper(b: &mut bencher::Bencher) {
            let haystack = random_bytes($i);
            b.iter(|| count(&haystack, 10));
        }
    };
}

bench!(0, bench_00000_naive, bench_00000_32, bench_00000_hyper);
bench!(10, bench_00010_naive, bench_00010_32, bench_00010_hyper);
bench!(20, bench_00020_naive, bench_00020_32, bench_00020_hyper);
bench!(30, bench_00030_naive, bench_00030_32, bench_00030_hyper);

bench!(40, bench_00040_naive, bench_00040_32, bench_00040_hyper);
bench!(50, bench_00050_naive, bench_00050_32, bench_00050_hyper);
bench!(60, bench_00060_naive, bench_00060_32, bench_00060_hyper);
bench!(70, bench_00070_naive, bench_00070_32, bench_00070_hyper);
bench!(80, bench_00080_naive, bench_00080_32, bench_00080_hyper);
bench!(90, bench_00090_naive, bench_00090_32, bench_00090_hyper);
bench!(100, bench_00100_naive, bench_00100_32, bench_00100_hyper);
bench!(120, bench_00120_naive, bench_00120_32, bench_00120_hyper);
bench!(140, bench_00140_naive, bench_00140_32, bench_00140_hyper);
bench!(170, bench_00170_naive, bench_00170_32, bench_00170_hyper);
bench!(210, bench_00210_naive, bench_00210_32, bench_00210_hyper);
bench!(250, bench_00250_naive, bench_00250_32, bench_00250_hyper);
bench!(300, bench_00300_naive, bench_00300_32, bench_00300_hyper);

bench!(400, bench_00400_naive, bench_00400_32, bench_00400_hyper);
bench!(500, bench_00500_naive, bench_00500_32, bench_00500_hyper);
bench!(600, bench_00600_naive, bench_00600_32, bench_00600_hyper);
bench!(700, bench_00700_naive, bench_00700_32, bench_00700_hyper);
bench!(800, bench_00800_naive, bench_00800_32, bench_00800_hyper);
bench!(900, bench_00900_naive, bench_00900_32, bench_00900_hyper);
bench!(1_000, bench_01000_naive, bench_01000_32, bench_01000_hyper);
bench!(1_200, bench_01200_naive, bench_01200_32, bench_01200_hyper);
bench!(1_400, bench_01400_naive, bench_01400_32, bench_01400_hyper);
bench!(1_700, bench_01700_naive, bench_01700_32, bench_01700_hyper);
bench!(2_100, bench_02100_naive, bench_02100_32, bench_02100_hyper);
bench!(2_500, bench_02500_naive, bench_02500_32, bench_02500_hyper);
bench!(3_000, bench_03000_naive, bench_03000_32, bench_03000_hyper);

bench!(4_000, bench_04000_naive, bench_04000_32, bench_04000_hyper);
bench!(5_000, bench_05000_naive, bench_05000_32, bench_05000_hyper);
bench!(6_000, bench_06000_naive, bench_06000_32, bench_06000_hyper);
bench!(7_000, bench_07000_naive, bench_07000_32, bench_07000_hyper);
bench!(8_000, bench_08000_naive, bench_08000_32, bench_08000_hyper);
bench!(9_000, bench_09000_naive, bench_09000_32, bench_09000_hyper);
bench!(10_000, bench_10000_naive, bench_10000_32, bench_10000_hyper);
bench!(12_000, bench_12000_naive, bench_12000_32, bench_12000_hyper);
bench!(14_000, bench_14000_naive, bench_14000_32, bench_14000_hyper);
bench!(17_000, bench_17000_naive, bench_17000_32, bench_17000_hyper);
bench!(21_000, bench_21000_naive, bench_21000_32, bench_21000_hyper);
bench!(25_000, bench_25000_naive, bench_25000_32, bench_25000_hyper);
bench!(30_000, bench_30000_naive, bench_30000_32, bench_30000_hyper);

bench!(100_000, bench_big_0100000_naive, bench_big_0100000_32, bench_big_0100000_hyper);
bench!(1_000_000, bench_big_1000000_naive, bench_big_1000000_32, bench_big_1000000_hyper);

benchmark_group!(bench,
    bench_00000_naive, bench_00000_32, bench_00000_hyper,
    bench_00010_naive, bench_00010_32, bench_00010_hyper,
    bench_00020_naive, bench_00020_32, bench_00020_hyper,
    bench_00030_naive, bench_00030_32, bench_00030_hyper,

    bench_00040_naive, bench_00040_32, bench_00040_hyper,
    bench_00050_naive, bench_00050_32, bench_00050_hyper,
    bench_00060_naive, bench_00060_32, bench_00060_hyper,
    bench_00070_naive, bench_00070_32, bench_00070_hyper,
    bench_00080_naive, bench_00080_32, bench_00080_hyper,
    bench_00090_naive, bench_00090_32, bench_00090_hyper,
    bench_00100_naive, bench_00100_32, bench_00100_hyper,
    bench_00120_naive, bench_00120_32, bench_00120_hyper,
    bench_00140_naive, bench_00140_32, bench_00140_hyper,
    bench_00170_naive, bench_00170_32, bench_00170_hyper,
    bench_00210_naive, bench_00210_32, bench_00210_hyper,
    bench_00250_naive, bench_00250_32, bench_00250_hyper,
    bench_00300_naive, bench_00300_32, bench_00300_hyper,

    bench_00400_naive, bench_00400_32, bench_00400_hyper,
    bench_00500_naive, bench_00500_32, bench_00500_hyper,
    bench_00600_naive, bench_00600_32, bench_00600_hyper,
    bench_00700_naive, bench_00700_32, bench_00700_hyper,
    bench_00800_naive, bench_00800_32, bench_00800_hyper,
    bench_00900_naive, bench_00900_32, bench_00900_hyper,
    bench_01000_naive, bench_01000_32, bench_01000_hyper,
    bench_01200_naive, bench_01200_32, bench_01200_hyper,
    bench_01400_naive, bench_01400_32, bench_01400_hyper,
    bench_01700_naive, bench_01700_32, bench_01700_hyper,
    bench_02100_naive, bench_02100_32, bench_02100_hyper,
    bench_02500_naive, bench_02500_32, bench_02500_hyper,
    bench_03000_naive, bench_03000_32, bench_03000_hyper,

    bench_04000_naive, bench_04000_32, bench_04000_hyper,
    bench_05000_naive, bench_05000_32, bench_05000_hyper,
    bench_06000_naive, bench_06000_32, bench_06000_hyper,
    bench_07000_naive, bench_07000_32, bench_07000_hyper,
    bench_08000_naive, bench_08000_32, bench_08000_hyper,
    bench_09000_naive, bench_09000_32, bench_09000_hyper,
    bench_10000_naive, bench_10000_32, bench_10000_hyper,
    bench_12000_naive, bench_12000_32, bench_12000_hyper,
    bench_14000_naive, bench_14000_32, bench_14000_hyper,
    bench_17000_naive, bench_17000_32, bench_17000_hyper,
    bench_21000_naive, bench_21000_32, bench_21000_hyper,
    bench_25000_naive, bench_25000_32, bench_25000_hyper,
    bench_30000_naive, bench_30000_32, bench_30000_hyper,

    bench_big_0100000_naive, bench_big_0100000_32, bench_big_0100000_hyper,
    bench_big_1000000_naive, bench_big_1000000_32, bench_big_1000000_hyper);

benchmark_main!(bench);
