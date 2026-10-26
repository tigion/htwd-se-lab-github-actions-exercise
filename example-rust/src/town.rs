use std::fmt;

/// Represents a town with name and number of residents.
#[derive(Debug)]
pub struct Town {
    name: String,
    residents: u32,
}

/// The maximum number of residents allowed in a town.
pub const MAX_RESIDENTS: u32 = 1_000_000_000;

#[derive(Debug, PartialEq, Eq)]
pub enum TownError {
    /// The town name is empty or contains only whitespace.
    InvalidName,

    /// The number of residents exceeds the maximum allowed value.
    TooManyResidents,
}

impl Town {
    /// Creates a new town after validating its values.
    ///
    /// # Errors
    ///
    /// Returns an error message if the name is empty or consists only of
    /// whitespace, or if the residents value is invalid.
    pub fn new(name: impl Into<String>, residents: u32) -> Result<Self, TownError> {
        let name = Self::validate_name(&name.into())?;
        let residents = Self::validate_residents(residents)?;

        Ok(Self { name, residents })
    }

    /// Returns the name of the town.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the number of residents in the town.
    pub fn residents(&self) -> u32 {
        self.residents
    }

    /// Sets the name of the town.
    ///
    /// # Errors
    ///
    /// Returns an error if the name is invalid.
    pub fn set_name(&mut self, name: impl Into<String>) -> Result<(), TownError> {
        self.name = Self::validate_name(&name.into())?;

        Ok(())
    }

    /// Sets the number of residents in the town.
    ///
    /// # Errors
    ///
    /// Returns an error if the residents value is invalid.
    pub fn set_residents(&mut self, residents: u32) -> Result<(), TownError> {
        self.residents = Self::validate_residents(residents)?;

        Ok(())
    }

    /// Returns human-readable town information.
    pub fn info(&self) -> String {
        // FIX: Add pluralization and handle zero residents.
        format!("{} has {} residents.", self.name, self.residents)
    }

    /// Returns the validated town name if it is valid.
    ///
    /// # Errors
    ///
    /// Returns [`TownError::InvalidName`] if the name is empty or consists only
    /// of whitespace.
    fn validate_name(name: &str) -> Result<String, TownError> {
        let name = name.trim();

        if name.is_empty() {
            return Err(TownError::InvalidName);
        }

        Ok(name.to_owned())
    }

    /// Returns the number of residents if it is valid.
    ///
    /// # Errors
    ///
    /// Returns [`TownError::TooManyResidents`] if the number of residents
    /// exceeds [`MAX_RESIDENTS`].
    fn validate_residents(residents: u32) -> Result<u32, TownError> {
        if residents > MAX_RESIDENTS {
            return Err(TownError::TooManyResidents);
        }

        Ok(residents)
    }
}

/// Formats the town as `Town('<name>', <residents>)`.
impl fmt::Display for Town {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Town('{}', {})", self.name, self.residents)
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import items from the parent module (above) for testing.

    // Test data shared by multiple unit tests.

    const INVALID_NAMES: [&str; 5] = ["", " ", "   ", "\n", "\t"];

    const TOO_MANY_RESIDENTS: u32 = MAX_RESIDENTS + 1;

    struct TestTown {
        name: &'static str,
        residents: u32,
    }

    const DRESDEN: TestTown = TestTown {
        name: "Dresden",
        residents: 556_780,
    };

    const LEIPZIG: TestTown = TestTown {
        name: "Leipzig",
        residents: 593_145,
    };

    // Unit tests for the `Town` struct.

    /// T01: Constructor sets the town values.
    #[test]
    fn constructor_sets_values() {
        // Arrange
        let town = Town::new(DRESDEN.name, DRESDEN.residents).expect("valid town values");

        // Act & Assert
        assert_eq!(DRESDEN.name, town.name());
        assert_eq!(DRESDEN.residents, town.residents());
    }

    /// T02: `set_name()` updates the name.
    #[test]
    fn set_name_updates_name() {
        // Arrange
        let mut town = Town::new(DRESDEN.name, DRESDEN.residents).expect("valid town values");

        // Act
        town.set_name(LEIPZIG.name).expect("valid name");

        // Assert
        assert_eq!(LEIPZIG.name, town.name());
    }

    /// T03: `set_residents()` updates the number of residents.
    #[test]
    fn set_residents_updates_residents() {
        // Arrange
        let mut town = Town::new(DRESDEN.name, DRESDEN.residents).expect("valid town values");

        // Act
        town.set_residents(LEIPZIG.residents)
            .expect("valid residents");

        // Assert
        assert_eq!(LEIPZIG.residents, town.residents());
    }

    /// T04: Constructor rejects invalid town names.
    #[test]
    fn constructor_rejects_invalid_name() {
        // Act & Assert
        for name in INVALID_NAMES {
            assert_eq!(
                TownError::InvalidName,
                Town::new(name, DRESDEN.residents).unwrap_err()
            );
        }
    }

    /// T05: `set_name()` rejects invalid town names.
    #[test]
    fn set_name_rejects_invalid_name() {
        // Arrange
        let mut town = Town::new(DRESDEN.name, DRESDEN.residents).expect("valid town values");

        // Act & Assert
        for name in INVALID_NAMES {
            assert_eq!(TownError::InvalidName, town.set_name(name).unwrap_err())
        }
    }

    /// T06: Constructor rejects too many residents.
    #[test]
    fn constructor_rejects_too_many_residents() {
        // Act & Assert
        assert_eq!(
            TownError::TooManyResidents,
            Town::new(DRESDEN.name, TOO_MANY_RESIDENTS).unwrap_err()
        );
    }

    /// T07: `set_residents()` rejects too many residents.
    #[test]
    fn set_residents_rejects_too_many_residents() {
        // Arrange
        let mut town = Town::new(DRESDEN.name, DRESDEN.residents).expect("valid town values");

        // Act & Assert
        assert_eq!(
            TownError::TooManyResidents,
            town.set_residents(TOO_MANY_RESIDENTS).unwrap_err()
        );
    }

    /// T08: `info()` uses "no residents" for zero residents.
    #[test]
    fn info_uses_no_residents_for_zero() {
        // Arrange
        let town = Town::new(DRESDEN.name, 0).expect("valid town values");
        let expected = format!("{} has no residents.", DRESDEN.name);

        // Act & Assert
        assert_eq!(expected, town.info());
    }

    /// T09: `info()` uses singular form for one resident.
    #[test]
    fn info_uses_singular_for_one_resident() {
        // Arrange
        let town = Town::new(DRESDEN.name, 1).expect("valid town values");
        let expected = format!("{} has 1 resident.", DRESDEN.name);

        // Act & Assert
        assert_eq!(expected, town.info());
    }

    /// T10: `info()` uses plural form for multiple residents.
    #[test]
    fn info_uses_plural_for_multiple_residents() {
        // Arrange
        let town = Town::new(DRESDEN.name, DRESDEN.residents).expect("valid town values");
        let expected = format!("{} has {} residents.", DRESDEN.name, DRESDEN.residents);

        // Act & Assert
        assert_eq!(expected, town.info());
    }

    /// T11: `Display()` returns object representation.
    #[test]
    fn display_returns_object_representation() {
        // Arrange
        let town = Town::new(DRESDEN.name, DRESDEN.residents).expect("valid town values");
        let expected = format!("Town('{}', {})", DRESDEN.name, DRESDEN.residents);

        // Act & Assert
        assert_eq!(expected, format!("{}", town)); //town.to_string());
    }
}
