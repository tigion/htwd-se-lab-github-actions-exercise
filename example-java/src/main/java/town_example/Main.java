package town_example;

public class Main {

  public static void main(String[] args) {
    Town t = new Town("Dresden", 556780);
    System.out.println(t.getInfo());
    System.out.println(t.toString());
    System.out.println(t.getName());
    System.out.println(t.getResidents());
  }

}
