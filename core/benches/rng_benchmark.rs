use std::num::NonZeroU8;

use criterion::{criterion_group, criterion_main, Criterion};
use lazy_static::lazy_static;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

fn rng_fastrand() -> i32 {
    fastrand::i32(1..=6)
}

fn rng_fastrand8x3() -> (u8, u8, u8) {
    let r = fastrand::u8(..);
    (r % 6 + 1, r / 6 % 6 + 1, r / 6 / 6 % 6 + 1)
}

unsafe fn rng_fastrand8x3_buf() -> u8 {
    static mut BUF: (Option<NonZeroU8>, Option<NonZeroU8>) = (None, None);

    if BUF.0.is_none() {
        if BUF.1.is_none() {
            let r = fastrand::u8(..);
            let (r1, r2, r3) = (r % 6 + 1, r / 6 % 6 + 1, r / 6 / 6 % 6 + 1);
            BUF.0 = Some(NonZeroU8::new_unchecked(r2));
            BUF.1 = Some(NonZeroU8::new_unchecked(r3));
            r1
        } else {
            std::mem::replace(&mut BUF.1, None).unwrap_unchecked().get()
        }
    } else {
        std::mem::replace(&mut BUF.0, None).unwrap_unchecked().get()
    }
}

unsafe fn rng_fastrand16x6_buf() -> u8 {
    static mut BUF: (
        Option<NonZeroU8>,
        Option<NonZeroU8>,
        Option<NonZeroU8>,
        Option<NonZeroU8>,
        Option<NonZeroU8>,
        Option<NonZeroU8>,
    ) = (None, None, None, None, None, None);
    if BUF.0.is_some() {
        return std::mem::replace(&mut BUF.0, None).unwrap_unchecked().get();
    }
    if BUF.1.is_some() {
        return std::mem::replace(&mut BUF.1, None).unwrap_unchecked().get();
    }
    if BUF.2.is_some() {
        return std::mem::replace(&mut BUF.2, None).unwrap_unchecked().get();
    }
    if BUF.3.is_some() {
        return std::mem::replace(&mut BUF.3, None).unwrap_unchecked().get();
    }
    if BUF.4.is_some() {
        return std::mem::replace(&mut BUF.4, None).unwrap_unchecked().get();
    }
    if BUF.5.is_some() {
        return std::mem::replace(&mut BUF.5, None).unwrap_unchecked().get();
    }

    let r = fastrand::u16(..);
    let (r0, r1, r2, r3, r4, r5) = (
        r % 6 + 1,
        r / 6 % 6 + 1,
        r / 6 / 6 % 6 + 1,
        r / 6 / 6 / 6 % 6 + 1,
        r / 6 / 6 / 6 / 6 % 6 + 1,
        r / 6 / 6 / 6 / 6 / 6 % 6 + 1,
    );
    BUF.0 = Some(NonZeroU8::new_unchecked(r0 as u8));
    BUF.1 = Some(NonZeroU8::new_unchecked(r1 as u8));
    BUF.2 = Some(NonZeroU8::new_unchecked(r2 as u8));
    BUF.3 = Some(NonZeroU8::new_unchecked(r3 as u8));
    BUF.4 = Some(NonZeroU8::new_unchecked(r4 as u8));
    r5 as u8
}

unsafe fn rng_fastrand16x6_buf_loop() -> u8 {
    static mut BUF: [Option<NonZeroU8>; 6] = [None; 6];
    if let Some(x) = BUF.iter_mut().skip_while(|x| x.is_none()).next() {
        return std::mem::replace(x, None).unwrap_unchecked().get();
    }

    let mut r = fastrand::u16(..);
    let ret = r % 6 + 1;
    (1..6).for_each(|i| {
        r /= 6;
        let a = r % 6 + 1;
        BUF[i] = Some(NonZeroU8::new_unchecked(a as u8));
    });

    ret as u8
}

unsafe fn rng_fastrand32x24_buf_loop() -> u8 {
    static mut BUF: [Option<NonZeroU8>; 12] = [None; 12];
    if let Some(x) = BUF.iter_mut().skip_while(|x| x.is_none()).next() {
        return std::mem::replace(x, None).unwrap_unchecked().get();
    }

    let mut r = fastrand::u32(..);
    let ret = r % 6 + 1;
    (1..12).for_each(|i| {
        r /= 6;
        let a = r % 6 + 1;
        BUF[i] = Some(NonZeroU8::new_unchecked(a as u8));
    });

    ret as u8
}

unsafe fn rng_fastrand8x3_single_buffer() -> u8 {
    const VALS_PER_GEN: u8 = 3;
    static mut BUF: u8 = 0;
    static mut BUF_REMAINING: u8 = 0;

    if BUF_REMAINING > 0 {
        BUF /= 6;
        BUF_REMAINING -= 1;
        return (BUF % 6 + 1) as u8;
    }

    let mut r = fastrand::u8(..);
    let ret = r % 6 + 1;
    BUF = r;
    BUF_REMAINING = VALS_PER_GEN - 1;

    ret as u8
}

unsafe fn rng_fastrand16x6_single_buffer() -> u8 {
    const VALS_PER_GEN: u8 = 6;
    static mut BUF: u16 = 0;
    static mut BUF_REMAINING: u8 = 0;

    if BUF_REMAINING > 0 {
        BUF /= 6;
        BUF_REMAINING -= 1;
        return (BUF % 6 + 1) as u8;
    }

    let mut r = fastrand::u16(..);
    let ret = r % 6 + 1;
    BUF = r;
    BUF_REMAINING = VALS_PER_GEN - 1;

    ret as u8
}

unsafe fn rng_fastrand16x5_single_buffer_recursive() -> u8 {
    const VALS_PER_GEN: u8 = 5;
    static mut BUF: u16 = 0;
    static mut BUF_REMAINING: u8 = 0;

    if BUF_REMAINING > 0 {
        BUF /= 8;
        BUF_REMAINING -= 1;
        let r = (BUF % 8 + 1) as u8;
        return if r >= 7 {
            rng_fastrand16x5_single_buffer_recursive()
        } else {
            r
        };
    }

    let mut r = fastrand::u16(..);
    let ret = r % 8 + 1;
    BUF = r;
    BUF_REMAINING = VALS_PER_GEN - 1;

    return if r >= 7 {
        rng_fastrand16x5_single_buffer_recursive()
    } else {
        ret as u8
    };
}

unsafe fn rng_fastrand32x12_single_buffer() -> u8 {
    const VALS_PER_GEN: u8 = 12;
    static mut BUF: u32 = 0;
    static mut BUF_REMAINING: u8 = 0;

    if BUF_REMAINING > 0 {
        BUF /= 6;
        BUF_REMAINING -= 1;
        return (BUF % 6 + 1) as u8;
    }

    let mut r = fastrand::u32(..);
    let ret = r % 6 + 1;
    BUF = r;
    BUF_REMAINING = VALS_PER_GEN - 1;

    ret as u8
}

fn rng_rand() -> i32 {
    rand::thread_rng().gen_range(1..=6)
}

fn rng_no_range() -> f32 {
    fastrand::f32()
}

fn rng_rand8x3() -> (u8, u8, u8) {
    let r = rand::thread_rng().gen::<u8>();
    (r % 6 + 1, r / 6 % 6 + 1, r / 6 / 6 % 6 + 1)
}

lazy_static! {
    static ref STEP: Uniform<i32> = Uniform::new(1, 7);
}
fn rng_uniform() -> i32 {
    unsafe { STEP.sample(&mut rand::thread_rng()) }
}

fn rng_benchmark(c: &mut Criterion) {
    c.bench_function("rng_fastrand8x3_single_buffer", |b| {
        b.iter(|| unsafe { rng_fastrand8x3_single_buffer() })
    });
    c.bench_function("rng_fastrand16x6_single_buffer", |b| {
        b.iter(|| unsafe { rng_fastrand16x6_single_buffer() })
    });
    c.bench_function("rng_fastrand32x12_single_buffer", |b| {
        b.iter(|| unsafe { rng_fastrand32x12_single_buffer() })
    });
    c.bench_function("rng_fastrand16x5_single_buffer_recursive", |b| {
        b.iter(|| unsafe { rng_fastrand16x5_single_buffer_recursive() })
    });
    c.bench_function("fastrand_i32", |b| b.iter(|| rng_fastrand()));
    c.bench_function("fastrand_u8x3", |b| b.iter(|| rng_fastrand8x3()));
    c.bench_function("fastrand_u8x3_buf", |b| {
        b.iter(|| unsafe { rng_fastrand8x3_buf() })
    });
    c.bench_function("fastrand_u16x6_buf", |b| {
        b.iter(|| unsafe { rng_fastrand16x6_buf() })
    });

    c.bench_function("fastrand_u16x6_buf_loop", |b| {
        b.iter(|| unsafe { rng_fastrand32x24_buf_loop() })
    });

    c.bench_function("rand_u8x3", |b| b.iter(|| rng_rand8x3()));
    c.bench_function("rand_i32", |b| b.iter(|| rng_rand()));
    c.bench_function("fastrand_f32", |b| b.iter(|| rng_no_range()));
    c.bench_function("rand_uniform_i32", |b| b.iter(|| rng_uniform()));
}

criterion_group!(benches, rng_benchmark);
criterion_main!(benches);
