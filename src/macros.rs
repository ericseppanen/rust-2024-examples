#![allow(unused_macros)]

macro_rules! unwrap_or_explode {
    ($e:expr) => {
        match $e {
            Some(val) => val,
            None => detonate(),
        }
    };
}
