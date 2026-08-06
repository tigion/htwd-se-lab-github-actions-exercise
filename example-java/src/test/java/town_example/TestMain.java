package town_example;

import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

class TestMain {
  @DisplayName("T01: `main()` runs without throwing an exception")
  @Test
  void mainRunsWithoutException() {
    // Act & Assert
    assertDoesNotThrow(() -> Main.main(new String[0]));
  }
}
