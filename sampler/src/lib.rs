use rand::random;

/// 蓄水池算法采样器
trait ReservoirSampler {
    /// 每种采样器只会在一种总体中采样，而总体中所有个体都属于相同类型
    type Item;

    /// 流式采样器无法知道总体数据有多少个样本，因此只逐个处理，并返回是否将样本纳入
    /// 样本池的结果，以及可能被替换出来的样本
    fn sample(&mut self, it: Self::Item) -> (bool, Option<Self::Item>);

    /// 任意时候应当知道当前蓄水池的状态。如果蓄水池未满，使用 `None` 来填充
    fn samples(&self) -> &[Option<Self::Item>];

    /// 锁定采样器结果。这个过程中如果采集到的样本不足，则会再次打乱顺序。
    fn lock(mut self) -> Vec<Option<Self::Item>>;
}

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

    fn sample(&mut self, it: Self::Item) -> (bool, Option<Self::Item>) {
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

        (r <= pool_cap, replaced)
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
