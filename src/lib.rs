#![no_std]

use core::ops::*;

#[derive(Copy)] pub enum Zero {}
#[derive(Clone, Copy)] pub struct Succ<N: Natural>(N);

impl Clone for Zero {
    fn clone(&self) -> Self { match *self {} }
}

pub trait Natural {
    const as_usize: usize;
}

impl Natural for Zero {
    const as_usize: usize = 0;
}

impl<N: Natural> Natural for Succ<N> {
    const as_usize: usize = 1 + N::as_usize;
}

impl<N: Natural> Add<N> for Zero {
    type Output = N;
    fn add(self: Zero, n: N) -> N { n }
}

impl<M: Natural, N: Natural> Add<N> for Succ<M> where M: Add<N>, <M as Add<N>>::Output: Natural {
    type Output = Succ<<M as Add<N>>::Output>;
    fn add(self: Succ<M>, n: N) -> Self::Output { let Succ(m) = self; Succ(m+n) }
}
