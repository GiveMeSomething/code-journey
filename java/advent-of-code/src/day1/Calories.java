package day1;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Scanner;

public class Calories {
    public ArrayList<ArrayList<Integer>> readElfItems() {
        // Path is read from root -> src -> packages
        File itemFile = new File("src/day1/items.txt");
        Scanner reader = null;
        try {
            reader = new Scanner(itemFile);
        } catch (FileNotFoundException e) {
            System.out.println("File not found. Aborting...");
            return null;
        }

        ArrayList<ArrayList<Integer>> elfItems = new ArrayList<>();

        // Init tracking variable and result array
        int currentElf = 0;
        elfItems.add(new ArrayList<>());

        while(reader.hasNextLine()) {
            String item = reader.nextLine();

            // Is an empty line
            if(item.isEmpty()) {
                currentElf++;
                elfItems.add(new ArrayList<>());
                continue;
            }

            // Try to read number value from file
            try {
                int itemValue = Integer.parseInt(item);
                elfItems.get(currentElf).add(itemValue);
            }catch (NumberFormatException e) {
                System.out.println(e.getMessage());
            }
        }

        return elfItems;
    }
    public int maxCalories(ArrayList<ArrayList<Integer>> elfItems) {
        if(elfItems == null) {
            return -1;
        }

        int maxCalories = 0;
        for (ArrayList<Integer> items : elfItems) {
            int sum = 0;
            for(int item: items) {
                sum += item;
            }

            maxCalories = Math.max(maxCalories, sum);
        }

        return maxCalories;
    }

    public int topThreeCalories(ArrayList<ArrayList<Integer>> elfItems) {
        if(elfItems == null) {
            return -1;
        }

        int sumTopThree = 0;
        for(int i = 0; i < 3; i++) {
            int max = 0;
            int maxIndex = 0;
            for(int j = 0; j < elfItems.size(); j++) {
                var items = elfItems.get(j);
                int sum = 0;
                for(int item: items) {
                    sum += item;
                }

                if(sum > max) {
                    max = sum;
                    maxIndex = j;
                }
            }

            sumTopThree += max;
            elfItems.remove(maxIndex);
        }

        return sumTopThree;
    }
}
