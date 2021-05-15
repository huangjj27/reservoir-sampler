use crate::pos::{Position, PositionType, PositionTypeError};
use crate::Choosen;
use reservoir_sampler::Reservoir;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BuildChoosenError {
    EmptyBuilder,
    WrongPositionType(PositionTypeError),
    PositionOutBound(usize),
}

/// A `Builder` promises that only after all positions are confirmed would we
/// start to choose. Besides, Confirming positions can be divided into separate
/// steps
pub struct ChoosenBuilder<PT = Position>
where
    PT: PositionType + Default + Clone,
{
    positions: Vec<PT>,
}

impl<PT> ChoosenBuilder<PT>
where
    PT: PositionType + Default + Clone,
{
    pub fn new() -> Self {
        ChoosenBuilder {
            positions: Vec::<PT>::new(),
        }
    }

    pub fn add_position(&mut self, name: &str, cap: usize) -> Result<&mut Self, BuildChoosenError> {
        let mut pos = PT::default();
        pos.set_name(name)
            .map_err(|e| BuildChoosenError::WrongPositionType(e))?;
        pos.set_cap(cap)
            .map_err(|e| BuildChoosenError::WrongPositionType(e))?;
        self.positions.push(pos);
        Ok(self)
    }

    fn check_idx(&self, idx: usize) -> Result<(), BuildChoosenError> {
        if idx < self.positions.len() {
            Ok(())
        } else {
            Err(BuildChoosenError::PositionOutBound(idx))
        }
    }

    pub fn set_position_name(
        &mut self,
        name: &str,
        idx: usize,
    ) -> Result<&mut Self, BuildChoosenError> {
        self.check_idx(idx)?;
        self.positions[idx]
            .set_name(name)
            .map_err(|e| BuildChoosenError::WrongPositionType(e))?;
        Ok(self)
    }

    pub fn set_position_cap(
        &mut self,
        new_cap: usize,
        idx: usize,
    ) -> Result<&mut Self, BuildChoosenError> {
        self.check_idx(idx)?;
        self.positions[idx]
            .set_cap(new_cap)
            .map_err(|e| BuildChoosenError::WrongPositionType(e))?;
        Ok(self)
    }

    pub fn remove_position(&mut self, idx: usize) -> Result<&mut Self, BuildChoosenError> {
        self.check_idx(idx)?;
        self.positions.remove(idx);
        Ok(self)
    }

    pub fn positions(&self) -> &[PT] {
        &self.positions
    }

    pub fn build<P: Clone>(&self) -> Result<Choosen<P, PT>, BuildChoosenError> {
        if self.positions.is_empty() {
            return Err(BuildChoosenError::EmptyBuilder);
        }

        let lucky_cap = self.positions.iter().map(|p| p.cap()).sum::<usize>();

        Ok(Choosen {
            positions: self.positions.clone(),
            lucky: Reservoir::<P>::with_capacity(lucky_cap),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let builder = ChoosenBuilder::<Position>::new();
        assert_eq!(builder.positions.len(), 0);
    }

    #[test]
    fn add_position() {
        let mut builder = ChoosenBuilder::<Position>::new();

        let _ = builder.add_position("pos", 13);
        assert_eq!(builder.positions.len(), 1);
        assert_eq!(builder.positions[0].name(), "pos");
        assert_eq!(builder.positions[0].cap(), 13);
    }

    #[test]
    fn check_idx() -> Result<(), BuildChoosenError> {
        let mut builder = ChoosenBuilder::<Position>::new();

        let result = builder.check_idx(0);
        assert_eq!(result.err(), Some(BuildChoosenError::PositionOutBound(0)));

        let result = builder.add_position("some_pos", 1)?.check_idx(0);

        assert_eq!(result, Ok(()));

        Ok(())
    }

    #[test]
    fn empty_position_name() {
        let mut builder = ChoosenBuilder::<Position>::new();

        let result = builder.add_position("", 1);
        assert_eq!(
            result.err(),
            Some(BuildChoosenError::WrongPositionType(
                PositionTypeError::EmptyName
            ))
        );
    }

    #[test]
    fn zero_position_cap() {
        let mut builder = ChoosenBuilder::<Position>::new();

        let result = builder.add_position("new_name", 0);
        assert_eq!(
            result.err(),
            Some(BuildChoosenError::WrongPositionType(
                PositionTypeError::ZeroCapacity
            ))
        );
    }

    #[test]
    fn remove_valid_position() -> Result<(), BuildChoosenError> {
        let mut builder = ChoosenBuilder::<Position>::new();
        builder.add_position("pos", 13)?.add_position("pos2", 34)?;

        builder.remove_position(1)?;
        assert_eq!(builder.positions.len(), 1);
        assert_eq!(builder.positions[0].name(), "pos");
        assert_eq!(builder.positions[0].cap(), 13);

        Ok(())
    }

    #[test]
    fn remove_invalid_position() -> Result<(), BuildChoosenError> {
        let mut builder = ChoosenBuilder::<Position>::new();
        let result = builder.add_position("pos", 13)?.remove_position(1);

        assert_eq!(result.err(), Some(BuildChoosenError::PositionOutBound(1)));

        Ok(())
    }

    #[test]
    fn set_name() -> Result<(), BuildChoosenError> {
        let mut builder = ChoosenBuilder::<Position>::new();
        builder
            .add_position("pos", 13)?
            .set_position_name("changed_pos", 0)?;

        assert_eq!(builder.positions.len(), 1);
        assert_eq!(builder.positions[0].name(), "changed_pos");
        assert_eq!(builder.positions[0].cap(), 13);

        Ok(())
    }

    #[test]
    fn set_name_out_boud() -> Result<(), BuildChoosenError> {
        let mut builder = ChoosenBuilder::<Position>::new();
        let result = builder
            .add_position("pos", 13)?
            .set_position_name("changed_pos", 1);

        assert_eq!(result.err(), Some(BuildChoosenError::PositionOutBound(1)));
        Ok(())
    }

    #[test]
    fn set_cap() -> Result<(), BuildChoosenError> {
        let mut builder = ChoosenBuilder::<Position>::new();
        builder.add_position("pos", 13)?.set_position_cap(20, 0)?;

        assert_eq!(builder.positions.len(), 1);
        assert_eq!(builder.positions[0].name(), "pos");
        assert_eq!(builder.positions[0].cap(), 20);

        Ok(())
    }

    #[test]
    fn set_cap_out_boud() -> Result<(), BuildChoosenError> {
        let mut builder = ChoosenBuilder::<Position>::new();
        let result = builder.add_position("pos", 13)?.set_position_cap(1, 1);

        assert_eq!(result.err(), Some(BuildChoosenError::PositionOutBound(1)));
        Ok(())
    }

    #[test]
    fn positions() -> Result<(), BuildChoosenError> {
        let mut builder = ChoosenBuilder::<Position>::new();
        let result = builder.add_position("some_pos", 4)?;

        assert_eq!(result.positions().len(), 1);
        assert_eq!(result.positions()[0].name(), "some_pos");
        assert_eq!(result.positions()[0].cap(), 4);

        Ok(())
    }

    #[test]
    fn empty_builder_should_not_build() -> Result<(), BuildChoosenError> {
        let mut builder = ChoosenBuilder::<Position>::new();

        let result = builder.build::<usize>();
        assert_eq!(result.err(), Some(BuildChoosenError::EmptyBuilder));

        let result = builder
            .add_position("some_pos", 1)?
            .remove_position(0)?
            .build::<usize>();
        assert_eq!(result.err(), Some(BuildChoosenError::EmptyBuilder));

        Ok(())
    }

    #[test]
    fn build() -> Result<(), BuildChoosenError> {
        let mut builder = ChoosenBuilder::<Position>::new();
        let choosen = builder
            .add_position("some_pos", 1)?
            .remove_position(0)?
            .add_position("another", 7)?
            .build::<usize>()?;
        assert_eq!(choosen.positions.len(), 1);
        assert_eq!(choosen.positions[0].name(), "another");
        assert_eq!(choosen.positions[0].cap(), 7);

        Ok(())
    }
}
