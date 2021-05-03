use reservoir_sampler::ReservoirSampler;
use rand::random;

pub struct Reservoir<T>{
    total: usize,
    pool: Vec<Option<T>>,
}

impl<T: Clone> Reservoir<T> {
    pub fn with_capacity(n: usize) -> Self {
        Self {
            total: 0,
            pool: vec![None; n],
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

fn main() {
    let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut reservoir = Reservoir::<i32>::with_capacity(15);

    for &it in &list {
        let (r, total, replaced) = reservoir.sample(it);
        println!("current: {:?}", reservoir.samples());
    }

    println!("result: {:?}", reservoir.lock());
}
