use std::hash::Hash;
use std::fmt::Debug;

pub trait Index: Clone + Debug + Hash + PartialEq + Eq {}

impl Index for usize {}
impl Index for u8 {}
impl Index for u16 {}
impl Index for u32 {}
impl Index for u64 {}
impl Index for u128 {}
impl Index for i8 {}
impl Index for i16 {}
impl Index for i32 {}
impl Index for i64 {}
impl Index for i128 {}
impl Index for &str {}
impl Index for String {}
impl Index for [u8; 32] {}
