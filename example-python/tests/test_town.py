import unittest

from town_example.town import Town


class TestTown(unittest.TestCase):
    INVALID_NAMES = (None, "", "  ")

    # Runs once before all tests.
    @classmethod
    def setUpClass(cls): ...

    # Runs before each test.
    def setUp(self):
        self.test_value1_name = "Dresden"
        self.test_value1_residents = 556780
        self.test_value2_name = "Leipzig"
        self.test_value2_residents = 593145

    # Runs after each test.
    def tearDown(self): ...

    # Runs once after all tests.
    @classmethod
    def tearDownClass(cls): ...

    def test_town_constructor_sets_values(self):
        """T01: Constructor sets the town values."""

        # Arrange
        town = Town(self.test_value1_name, self.test_value1_residents)

        # Act & Assert
        self.assertEqual(self.test_value1_name, town.name)
        self.assertEqual(self.test_value1_residents, town.residents)

    def test_town_name_setter_updates_name(self):
        """T02: Name setter updates the town name."""

        # Arrange
        town = Town(self.test_value1_name, self.test_value1_residents)

        # Act
        town.name = self.test_value2_name

        # Assert
        self.assertEqual(self.test_value2_name, town.name)

    def test_town_residents_setter_updates_residents(self):
        """T03: Residents setter updates the town residents."""

        # Arrange
        town = Town(self.test_value1_name, self.test_value1_residents)

        # Act
        town.residents = self.test_value2_residents

        # Assert
        self.assertEqual(self.test_value2_residents, town.residents)

    def test_town_constructor_rejects_invalid_name(self):
        """
        T04: Constructor rejects invalid town names:
        - None
        - empty string
        - whitespace only
        """

        # Act & Assert
        for name in self.INVALID_NAMES:
            with self.assertRaises(ValueError):
                Town(name, self.test_value1_residents)

    def test_town_name_setter_rejects_invalid_name(self):
        """
        T05: Setter rejects invalid town names:
        - None
        - empty string
        - whitespace only
        """

        # Arrange
        town = Town(self.test_value1_name, self.test_value1_residents)

        # Act & Assert
        for name in self.INVALID_NAMES:
            with self.assertRaises(ValueError):
                town.name = name

    def test_town_constructor_rejects_negative_residents(self):
        """T06: Constructor rejects negative residents."""

        # Act & Assert
        with self.assertRaises(ValueError):
            Town(self.test_value1_name, -1)

    def test_town_residents_setter_rejects_negative_residents(self):
        """T07: Setter rejects negative residents."""

        # Arrange
        town = Town(self.test_value1_name, self.test_value1_residents)

        # Act & Assert
        with self.assertRaises(ValueError):
            town.residents = -1

    def test_town_str_returns_human_readable_string(self):
        """T08: `str()` returns human-readable string."""

        # Arrange
        town = Town(self.test_value1_name, self.test_value1_residents)
        expected = (
            f"{self.test_value1_name} has {self.test_value1_residents} residents."
        )

        # Act, Assert
        self.assertEqual(expected, str(town))

    def test_town_repr_returns_object_representation(self):
        """T09: `repr()` returns object representation."""

        # Arrange
        town = Town(self.test_value1_name, self.test_value1_residents)
        expected = f"Town({self.test_value1_name!r}, {self.test_value1_residents!r})"

        # Act, Assert
        self.assertEqual(expected, repr(town))


if __name__ == "__main__":
    unittest.main()
