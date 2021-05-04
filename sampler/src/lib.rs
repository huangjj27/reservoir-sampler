//! This crate only provide a "standard" trait about what a streaming sampler
//! using the [Reservoir Algorithm] can do. In My opinion, given a `Whole`
//! consists of a same type of `Item`, the sampler can decide whether it should
//! sample an item when the item passes through, and no matter when, the sampler
//! should know which samples it currently holds. When the sampler decided not
//! to accept any new sample more, it can `lock` the result.
//!
//! [Reservoir Algorithm](https://en.wikipedia.org/wiki/Reservoir_sampling)
use rand::random;

pub trait ReservoirSampler {
    /// Each sampler only processes the same type of items.
    type Item;

    /// A sampler processes exactly one item each time, for the items come in as
    /// a stream.
    ///
    /// ## Return
    /// the `sample` function return a tuple contains 3 elements:
    /// - a `usize` stands for what random number the current item gets
    /// - a `usize` stands for how many items has been passed through so far
    /// - an option of item that is replaced by the current item.
    fn sample(&mut self, it: Self::Item) -> (usize, usize, Option<Self::Item>);

    /// A reservoir should know which items are held no matter if the sampling
    /// process is finished.
    fn samples(&self) -> &[Option<Self::Item>];

    /// End the sampling process. Shuffling the order of the result is allowed.
    fn lock(self) -> Vec<Option<Self::Item>>;
}

/// A `Reservoir` is a just a pool, but for random number generation, `total`
/// items' count passed through is known.
pub struct Reservoir<T> {
    total: usize,
    pool: Vec<Option<T>>,
}

impl<T: Clone> Reservoir<T> {
    pub fn with_capacity(n: usize) -> Self {
        Self {
            total: 0,
            pool: std::vec::from_elem(Option::<T>::None, n),
        }
    }
}

impl<T> ReservoirSampler for Reservoir<T> {
    type Item = T;

    fn sample(&mut self, it: Self::Item) -> (usize, usize, Option<Self::Item>) {
        let pool_cap = self.pool.capacity();

        self.total += 1;

        // 概率渐小的随机替换
        let r = random::<usize>() % self.total + 1;
        let mut replaced = None;
        if r <= pool_cap {
            replaced = self.pool[r - 1].take();
            self.pool[r - 1] = Some(it);
        }

        if self.total <= pool_cap && r < self.total {
            self.pool[self.total - 1] = replaced.take();
        }

        (r, self.total, replaced)
    }

    fn samples(&self) -> &[Option<Self::Item>] {
        &self.pool[..]
    }

    fn lock(mut self) -> Vec<Option<Self::Item>> {
        let mut i = self.total;
        while i < self.pool.capacity() {
            i += 1;

            let r = random::<usize>() % i + 1;
            if r <= self.pool.capacity() {
                self.pool[i - 1] = self.pool[r - 1].take();
            }
        }

        self.pool
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut reservoir = Reservoir::<i32>::with_capacity(15);

        for &it in &list {
            let _ = reservoir.sample(it);
            println!("current: {:?}", reservoir.samples());
        }

        println!("result: {:?}", reservoir.lock());
    }
}
