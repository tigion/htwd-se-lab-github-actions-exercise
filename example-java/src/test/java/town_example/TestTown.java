package town_example;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

class TestTown {
  private static final String[] INVALID_NAMES = { null, "", " " };

  private String testValue1Name, testValue2Name;
  private int testValue1Residents, testValue2Residents;

  // Runs once before all tests.
  @BeforeAll
  static void setUpBeforeClass() throws Exception {
  }

  // Runs before each test.
  @BeforeEach
  void setUp() {
    testValue1Name = "Dresden";
    testValue1Residents = 556780;
    testValue2Name = "Leipzig";
    testValue2Residents = 593145;
  }

  // Runs after each test.
  @AfterEach
  void tearDown() {
  }

  // Runs once after all tests.
  @AfterAll
  static void tearDownAfterClass() throws Exception {
  }

  @DisplayName("T01: Constructor sets the town values")
  @Test
  void constructorSetsValues() {
    // Arrange
    Town town = new Town(testValue1Name, testValue1Residents);

    // Act & Assert
    assertEquals(testValue1Name, town.getName());
    assertEquals(testValue1Residents, town.getResidents());
  }

  @DisplayName("T02: `setName()` updates the name")
  @Test
  void setNameUpdatesName() {
    // Arrange
    Town town = new Town(testValue1Name, testValue1Residents);

    // Act
    town.setName(testValue2Name);

    // Assert
    assertEquals(testValue2Name, town.getName());
  }

  @DisplayName("T03: `setResidents()` updates the number of residents")
  @Test
  void setResidentsUpdatesResidents() {
    // Arrange
    Town town = new Town(testValue1Name, testValue1Residents);

    // Act
    town.setResidents(testValue2Residents);

    // Assert
    assertEquals(testValue2Residents, town.getResidents());
  }

  @DisplayName("T04: Constructor rejects invalid name")
  @Test
  void constructorRejectsInvalidName() {
    // Act & Assert
    for (String name : INVALID_NAMES) {
      assertThrows(IllegalArgumentException.class, () -> {
        new Town(name, testValue1Residents);
      });
    }
  }

  @DisplayName("T05: `setName()` rejects invalid name")
  @Test
  void setNameRejectsInvalidName() {
    // Arrange
    Town town = new Town(testValue1Name, testValue1Residents);

    // Act & Assert
    for (String name : INVALID_NAMES) {
      assertThrows(IllegalArgumentException.class, () -> {
        town.setName(name);
      });
    }
  }

  @DisplayName("T06: Constructor rejects negative residents")
  @Test
  void constructorRejectsNegativeResidents() {
    // Act & Assert
    assertThrows(IllegalArgumentException.class, () -> {
      new Town(testValue1Name, -1);
    });
  }

  @DisplayName("T07: `setResidents()` rejects negative residents")
  @Test
  void setResidentsRejectsNegativeResidents() {
    // Arrange
    Town town = new Town(testValue1Name, testValue1Residents);

    // Act & Assert
    assertThrows(IllegalArgumentException.class, () -> {
      town.setResidents(-1);
    });
  }

  @DisplayName("T08: `getInfo()` uses no residents for zero")
  @Test
  void getInfoUsesNoResidentsForZero() {
    // Arrange
    Town town = new Town(testValue1Name, 0);
    String expected = "%s has no residents.".formatted(testValue1Name);

    // Act, Assert
    assertEquals(expected, town.getInfo());
  }

  @DisplayName("T09: `getInfo()` uses singular for one resident")
  @Test
  void getInfoUsesSingularForOneResident() {
    // Arrange
    Town town = new Town(testValue1Name, 1);
    String expected = "%s has 1 resident.".formatted(testValue1Name);

    // Act, Assert
    assertEquals(expected, town.getInfo());
  }

  @DisplayName("T10: `getInfo()` uses plural for multiple residents")
  @Test
  void getInfoUsesPluralForMultipleResidents() {
    // Arrange
    Town town = new Town(testValue1Name, testValue1Residents);
    String expected = "%s has %d residents.".formatted(testValue1Name, testValue1Residents);

    // Act, Assert
    assertEquals(expected, town.getInfo());
  }

  @DisplayName("T11: `toString()` returns object representation")
  @Test
  void toStringReturnsObjectRepresentation() {
    // Arrange
    Town town = new Town(testValue1Name, testValue1Residents);
    String expected = "Town('%s', %d)".formatted(testValue1Name, testValue1Residents);

    // Act, Assert
    assertEquals(expected, town.toString());
  }
}
