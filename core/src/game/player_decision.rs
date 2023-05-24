use anyhow::bail;

/// A helper trait for easier strategy implementation directly in Rust
pub trait PlayerStrategy {
    extern "C" fn decide(w1: i32, w2: i32, round_score: i32, total_score: i32, num_throws: i32, d: i32, h: i32, b: i32, w: i32) -> i32;
}

/// w1, w2, round_score, total_score, num_throws, d, h, b, w -> Decision(as i32)
pub type DecideFn = extern "C" fn(i32, i32, i32, i32, i32, i32, i32, i32, i32) -> i32;

#[derive(Debug)]
pub enum Decision {
    Continue,
    End,
    D,
    H,
    B,
    W,
}

impl TryFrom<i32> for Decision {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use Decision::{B, Continue, D, End, H, W};

        match value {
            0 => Ok(Continue),
            1 => Ok(End),
            2 => Ok(D),
            3 => Ok(H),
            4 => Ok(B),
            5 => Ok(W),
            _ => bail!("Invalid decision value"),
        }
    }
}

impl Into<i32> for Decision {
    fn into(self) -> i32 {
        use Decision::{B, Continue, D, End, H, W};

        match self {
            Continue => 0,
            End => 1,
            D => 2,
            H => 3,
            B => 4,
            W => 5,
        }
    }
}
