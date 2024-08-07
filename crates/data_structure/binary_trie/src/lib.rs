use std::num::NonZeroUsize;
use std::ops::{BitXor, BitXorAssign, Shl, Shr};

#[derive(Clone, Copy, Debug, Default)]
struct TrieNode {
    /// offset ^ digit == child index
    offset: Option<NonZeroUsize>,
    /// parrent index
    parrent: usize,
    /// subtree node count
    cnt: usize,
}

#[derive(Debug, Default)]
pub struct BinaryTrie<T> {
    data: Vec<TrieNode>,
    lazy: T,
}

impl<T: UInt> BinaryTrie<T> {
    pub fn new() -> Self {
        Self {
            // [root, dummy]
            data: vec![Default::default(); 2],
            lazy: Default::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.data[0].cnt
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn insert(&mut self, value: T) {
        let value = value ^ self.lazy;
        let mut idx = 0;
        self.data[idx].cnt += 1;
        for i in (0..T::BITS).rev() {
            if self.data[idx].offset.is_none() {
                let offset = self.push();
                self.data[idx].offset = NonZeroUsize::new(offset);
                self.data[offset].parrent = idx;
                self.data[offset ^ 1].parrent = idx;
            }
            let offset = self.data[idx].offset.unwrap().get();
            idx = offset | ((value >> i).as_usize() & 1);
            self.data[idx].cnt += 1;
        }
    }

    fn push(&mut self) -> usize {
        let len = self.data.len();
        self.data.resize(len + 2, Default::default());
        len
    }

    pub fn remove(&mut self, value: T) {
        if let Some(mut idx) = self.find(value) {
            while idx != 0 {
                self.data[idx].cnt -= 1;
                idx = self.data[idx].parrent;
            }
            self.data[idx].cnt -= 1;
        }
    }

    fn find(&self, value: T) -> Option<usize> {
        let value = value ^ self.lazy;
        let mut idx = 0;
        for i in (0..T::BITS).rev() {
            let offset = self.data[idx].offset?.get();
            idx = offset | ((value >> i).as_usize() & 1);
        }
        (self.data[idx].cnt != 0).then_some(idx)
    }

    pub fn count(&self, value: T) -> usize {
        self.find(value).map_or(0, |idx| self.data[idx].cnt)
    }

    pub fn xor_all(&mut self, value: T) {
        self.lazy ^= value;
    }

    pub fn min(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let mut ret = T::default();
        let mut idx = 0;
        for i in (0..T::BITS).rev() {
            let offset = self.data[idx].offset.unwrap().get();
            idx = offset ^ ((self.lazy >> i).as_usize() & 1);
            if self.data[idx].cnt == 0 {
                idx ^= 1;
                ret ^= T::one() << i;
            }
        }
        Some(ret)
    }
}

pub trait UInt:
    Default
    + Copy
    + BitXorAssign
    + BitXor<Output = Self>
    + Shl<u32, Output = Self>
    + Shr<u32, Output = Self>
{
    const BITS: u32;
    fn as_usize(self) -> usize;
    fn one() -> Self;
}

impl UInt for u32 {
    const BITS: u32 = u32::BITS;

    fn as_usize(self) -> usize {
        self as usize
    }
    fn one() -> Self {
        1
    }
}

impl UInt for u64 {
    const BITS: u32 = u64::BITS;

    fn as_usize(self) -> usize {
        self as usize
    }
    fn one() -> Self {
        1
    }
}

impl UInt for usize {
    const BITS: u32 = usize::BITS;

    fn as_usize(self) -> usize {
        self
    }
    fn one() -> Self {
        1
    }
}
