use reservoir_sampler::{Reservoir, ReservoirSampler};
use std::fmt::Debug;

pub struct Choosen<P> {
    positions: Vec<Position>,
    lucky: Reservoir<P>,
}

#[derive(Clone, Debug)]
pub struct Position {
    name: String,
    cap: usize,
}

impl<P> Choosen<P> {
    pub fn poll_one(&mut self, it: P) -> (usize, usize, Option<P>) {
        self.lucky.sample(it)
    }

    pub fn lucky(&self) -> &[Option<P>] {
        self.lucky.samples()
    }

    pub fn release(self) -> Result<Vec<(String, Vec<P>)>, &'static str> {
        let mut final_lucky = self.lucky.lock();

        if !final_lucky.iter().any(|it| it.is_some()) {
            return Err("No one is choosen!");
        }

        let mut counted = 0;
        let mut result = Vec::new();
        for p in self.positions {
            let mut luck = Vec::with_capacity(p.cap);

            for i in 0..p.cap {
                if let Some(it) = final_lucky[counted + i].take() {
                    luck.push(it);
                }
            }

            result.push((p.name, luck));
            counted += p.cap;
        }

        Ok(result)
    }
}

/// A `Builder` promises that only after all positions are confirmed would we
/// start to choose. Besides, Confirming positions can be divided into separate
/// steps
pub struct ChoosenBuilder {
    positions: Vec<Position>,
}

impl ChoosenBuilder {
    pub fn new() -> Self {
        ChoosenBuilder {
            positions: Vec::new(),
        }
    }

    pub fn add_position(&mut self, name: &str, cap: usize) -> &mut Self {
        self.positions.push(Position {
            name: name.into(),
            cap,
        });
        self
    }

    pub fn remove_position(&mut self, idx: usize) -> &mut Self {
        self.positions.remove(idx);
        self
    }

    pub fn set_position_name(&mut self, name: &str, idx: usize) -> &mut Self {
        self.positions[idx].name = name.into();
        self
    }

    pub fn positions(&self) -> &[Position] {
        &self.positions
    }

    pub fn build<P: Clone>(&self) -> Choosen<P> {
        let lucky_cap = self.positions.iter().map(|p| p.cap).sum::<usize>();

        Choosen {
            positions: self.positions.clone(),
            lucky: Reservoir::<P>::with_capacity(lucky_cap),
        }
    }
}

#[cfg(test)]
mod test_builder {
    use super::ChoosenBuilder;

    #[test]
    fn test() {
        let mut builder = ChoosenBuilder::new();
        assert_eq!(builder.positions.len(), 0);

        builder.add_position("test_pos", 13);

        assert_eq!(builder.positions.len(), 1);
        assert_eq!(builder.positions[0].name, "test_pos");
        assert_eq!(builder.positions[0].cap, 13);

        builder.add_position("test_pos2", 34);

        assert_eq!(builder.positions.len(), 2);
        assert_eq!(builder.positions[1].name, "test_pos2");
        assert_eq!(builder.positions[1].cap, 34);

        let choosen = builder.build::<usize>();
        assert_eq!(choosen.positions.len(), 2);
        assert_eq!(choosen.lucky().len(), 13 + 34);
    }
}

#[cfg(test)]
mod test_choosen {
    use super::*;
    #[test]
    fn have_something_choosen() {
        let v = vec![8, 1, 1, 9, 2];
        let mut choosen = ChoosenBuilder::new()
            .add_position("一等奖", 1)
            .add_position("二等奖", 1)
            .add_position("三等奖", 4)
            .build::<usize>();

        for it in v {
            choosen.poll_one(it);
            println!("{:?}", choosen.lucky());
        }

        println!("{:?}", choosen.release().unwrap());
    }

    #[test]
    fn have_nothhing_choosen() {
        let choosen = ChoosenBuilder::new()
            .add_position("一等奖", 3)
            .add_position("三等奖", 4)
            .build::<usize>();

        assert_eq!(choosen.release(), Err("No one is choosen!"));
    }
}
