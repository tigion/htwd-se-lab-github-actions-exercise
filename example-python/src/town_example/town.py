class Town:
    """
    Represents a town with name and number of residents.

    :param str name: The name of the town.
    :param int residents: The number of residents in the town.
    """

    def __init__(self, name, residents):
        self.name = name
        self.residents = residents

    @property
    def name(self):
        """The name of the town."""
        return self.__name

    @name.setter
    def name(self, value):
        if isinstance(value, str) and value and not value.isspace():
            self.__name = value.strip()
        else:
            raise ValueError("Name must be a non-empty string.")

    @property
    def residents(self):
        """The number of residents in the town."""
        return self.__residents

    @residents.setter
    def residents(self, value):
        if type(value) is not int:
            raise ValueError("Residents must be an integer.")
        if value < 0:
            raise ValueError("Residents cannot be negative.")

        self.__residents = value

    def __str__(self):
        """Returns a human-readable string of the object."""

        # FIX: Add pluralization and handle zero residents.
        return f"{self.name} has {self.residents} residents."

    def __repr__(self):
        """Returns a string representation of the object."""
        return f"{type(self).__name__}({self.name!r}, {self.residents!r})"
