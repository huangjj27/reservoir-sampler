use reservoir_sampler::{Reservoir, ReservoirSampler};

mod builder;
mod pos;

pub use crate::builder::{BuildChoosenError, ChoosenBuilder};
pub use crate::pos::{Position, PositionType, PositionTypeError};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ChoosenError {
    NoOneIsChoosen,
}

pub struct Choosen<P, PT = Position>
where
    PT: PositionType,
{
    positions: Vec<PT>,
    lucky: Reservoir<P>,
}

impl<P, PT> Choosen<P, PT>
where
    PT: PositionType,
{
    pub fn poll_one(&mut self, it: P) -> (usize, usize, Option<P>) {
        self.lucky.sample(it)
    }

    pub fn lucky(&self) -> &[Option<P>] {
        self.lucky.samples()
    }

    pub fn release(self) -> Result<Vec<(String, Vec<P>)>, ChoosenError> {
        let mut final_lucky = self.lucky.lock();

        if !final_lucky.iter().any(|it| it.is_some()) {
            return Err(ChoosenError::NoOneIsChoosen);
        }

        let mut counted = 0;
        let mut result = Vec::new();
        for p in self.positions {
            let mut luck = Vec::with_capacity(p.cap());

            for i in 0..p.cap() {
                if let Some(it) = final_lucky[counted + i].take() {
                    luck.push(it);
                }
            }

            result.push((p.name().into(), luck));
            counted += p.cap();
        }

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn have_something_choosen() -> Result<(), BuildChoosenError> {
        let v = vec![8, 1, 1, 9, 2];
        let mut choosen = ChoosenBuilder::<Position>::new()
            .add_position("一等奖", 1)?
            .add_position("二等奖", 1)?
            .add_position("三等奖", 4)?
            .build::<usize>()?;

        for it in v {
            choosen.poll_one(it);
            println!("{:?}", choosen.lucky());
        }

        println!("{:?}", choosen.release().unwrap());
        Ok(())
    }

    #[test]
    fn have_nothhing_choosen() -> Result<(), BuildChoosenError> {
        let choosen = ChoosenBuilder::<Position>::new()
            .add_position("一等奖", 3)?
            .add_position("三等奖", 4)?
            .build::<usize>()?;

        assert_eq!(choosen.release().err(), Some(ChoosenError::NoOneIsChoosen));

        Ok(())
    }
}
