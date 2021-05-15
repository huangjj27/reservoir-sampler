use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PositionTypeError {
    EmptyName,
    ZeroCapacity,
}

pub trait PositionType {
    fn name(&self) -> &str;
    fn set_name(&mut self, name: &str) -> Result<&mut Self, PositionTypeError>;
    fn cap(&self) -> usize;
    fn set_cap(&mut self, new_cap: usize) -> Result<&mut Self, PositionTypeError>;
}

#[derive(Clone, Debug)]
pub struct Position {
    name: String,
    cap: usize,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            name: String::from("default_name"),
            cap: 1,
        }
    }
}

impl PositionType for Position {
    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: &str) -> Result<&mut Self, PositionTypeError> {
        if name.len() == 0 {
            return Err(PositionTypeError::EmptyName);
        }
        self.name = name.into();
        Ok(self)
    }

    fn cap(&self) -> usize {
        self.cap
    }

    fn set_cap(&mut self, new_cap: usize) -> Result<&mut Self, PositionTypeError> {
        if new_cap == 0 {
            return Err(PositionTypeError::ZeroCapacity);
        }
        self.cap = new_cap;
        Ok(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default() {
        let pos = Position::default();
        assert_eq!(pos.name(), "default_name");
        assert_eq!(pos.cap(), 1);
    }

    #[test]
    fn test_set_valid_name() {
        let _ = Position::default().set_name("some other anme");
    }

    #[test]
    fn test_set_empty_name() {
        let mut pos = Position::default();
        let result = pos.set_name("");
        assert_eq!(result.err(), Some(PositionTypeError::EmptyName));
    }

    #[test]
    fn test_set_valid_cap() {
        let _ = Position::default().set_cap(15);
    }

    #[test]
    fn test_set_zero_cap() {
        let mut pos = Position::default();
        let result = pos.set_cap(0);
        assert_eq!(result.err(), Some(PositionTypeError::ZeroCapacity));
    }
}
