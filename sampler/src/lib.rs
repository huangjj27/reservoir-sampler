//! This crate only provide a "standard" trait about what a streaming sampler
//! using the [Reservoir Algorithm] can do. In My opinion, given a `Whole`
//! consists of a same type of `Item`, the sampler can decide whether it should
//! sample an item when the item passes through, and no matter when, the sampler
//! should know which samples it currently holds. When the sampler decided not
//! to accept any new sample more, it can `lock` the result.
//!
//! [Reservoir Algorithm](https://en.wikipedia.org/wiki/Reservoir_sampling)

trait ReservoirSampler {
    // Each sampler only processes the same type of items.
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
