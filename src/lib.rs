#![no_std]

use core::ops::*;

pub enum Zero {}
pub struct Succ<N: Natural>(N);

pub trait Natural {
    fn toUsize() -> usize;
}

impl Natural for Zero {
    fn toUsize() -> usize { 0 }
}

impl<N: Natural> Natural for Succ<N> {
    fn toUsize() -> usize { 1+N::toUsize() }
}

impl<N: Natural> Add<N> for Zero {
    type Output = N;
    fn add(self: Zero, n: N) -> N { n }
}

impl<M: Natural, N: Natural> Add<N> for Succ<M> where M: Add<N>, <M as Add<N>>::Output: Natural {
    type Output = Succ<<M as Add<N>>::Output>;
    fn add(self: Succ<M>, n: N) -> Self::Output { let Succ(m) = self; Succ(m+n) }
}
