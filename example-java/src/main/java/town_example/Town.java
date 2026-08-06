package town_example;

/**
 * Represents a town with name and number of residents.
 */
public class Town {

  /** The name of the town. */
  private String name;

  /** The number of residents in the town. */
  private int residents;

  public Town(String name, int residents) {
    this.setName(name);
    this.setResidents(residents);
  }

  public String getName() {
    return this.name;
  }

  public int getResidents() {
    return this.residents;
  }

  public void setName(String name) {
    if (name == null || name.isBlank())
      throw new IllegalArgumentException("Name must be a non-empty string.");
    this.name = name.strip();
  }

  public void setResidents(int residents) {
    // FIX: Add non-negative residents validation.

    this.residents = residents;
  }

  /** Returns human-readable town information. */
  public String getInfo() {
    return this.name + " has " + this.residents + " residents.";
  }

  /** Returns a string representation of the object. */
  @Override
  public String toString() {
    return "Town('" + name + "', " + residents + ")";
  }
}
