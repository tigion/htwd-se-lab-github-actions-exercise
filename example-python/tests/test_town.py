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

    def test_town_constructor_sets_and_gets_correct_values(self):
        """T01: Constructor sets and gets correct values."""

        # Arrange
        t = Town(self.test_value1_name, self.test_value1_residents)

        # Act & Assert
        self.assertEqual(t.name, self.test_value1_name)
        self.assertEqual(t.residents, self.test_value1_residents)

    def test_town_name_setter_sets_and_gets_correct_value(self):
        """T02: Name setter sets and gets correct value."""

        # Arrange
        t = Town(self.test_value1_name, self.test_value1_residents)

        # Act
        t.name = self.test_value2_name

        # Assert
        self.assertEqual(t.name, self.test_value2_name)

    def test_town_residents_setter_sets_and_gets_correct_value(self):
        """T03: Residents setter sets and gets correct value."""

        # Arrange
        t = Town(self.test_value1_name, self.test_value1_residents)

        # Act
        t.residents = self.test_value2_residents

        # Assert
        self.assertEqual(t.residents, self.test_value2_residents)

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
        t = Town(self.test_value1_name, self.test_value1_residents)

        # Act & Assert
        for name in self.INVALID_NAMES:
            with self.assertRaises(ValueError):
                t.name = name

    def test_town_constructor_rejects_negative_residents(self):
        """T06: Constructor rejects negative residents."""

        # Act & Assert
        with self.assertRaises(ValueError):
            Town(self.test_value1_name, -1)

    def test_town_residents_setter_rejects_negative_residents(self):
        """T07: Setter rejects negative residents."""

        # Arrange
        t = Town(self.test_value1_name, self.test_value1_residents)

        # Act & Assert
        with self.assertRaises(ValueError):
            t.residents = -1

    def test_town_str_returns_human_readable_string(self):
        """T08: Get correct town as human readable string."""

        # Arrange
        t = Town(self.test_value1_name, self.test_value1_residents)
        expected = (
            f"{self.test_value1_name} has {self.test_value1_residents} residents."
        )

        # Act, Assert
        self.assertEqual(str(t), expected)

    def test_town_repr_returns_object_representation(self):
        """T09: Get correct town as object representation string."""

        # Arrange
        t = Town(self.test_value1_name, self.test_value1_residents)
        expected = f"Town({self.test_value1_name!r}, {self.test_value1_residents!r})"

        # Act, Assert
        self.assertEqual(repr(t), expected)


if __name__ == "__main__":
    unittest.main()
