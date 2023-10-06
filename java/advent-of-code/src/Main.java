import day1.Calories;

public class Main {
    public static void main(String[] args) {
        System.out.println("Hello World");

        runDay1();
    }

    private static void runDay1() {
        Calories caloriesCounter = new Calories();

        var elfItems = caloriesCounter.readElfItems();
        int maxCalories = caloriesCounter.maxCalories(elfItems);
        int topThreeCalories = caloriesCounter.topThreeCalories(elfItems);

        System.out.printf("Max calories carrying is %d%n", maxCalories);
        System.out.printf("Top three calories carrying is %d%n", topThreeCalories);
    }
}