using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class MainClass
{
    public static void Main(string[] args)
    {
        List<string> lines = File.ReadLines("./Input.txt").ToList();
        

        Console.WriteLine($"Part1 correct passwords: {Part1(lines)}");
        Console.WriteLine($"Part2 correct passwords: {Part2(lines)}");
    }

    private static int Part1(List<string> lines)
    {
        int totalCorrect = 0;
        foreach(string line in lines)
        {
            string[] args = line.Split(' ');

            int lowerBound = Int32.Parse(args[0].Split('-')[0]);
            int higherBound = Int32.Parse(args[0].Split('-')[1]);

            char checkFor = args[1][0];

            string password = args[2];

            int count = 0;
            foreach (char letter in password) {
                if (letter == checkFor) {
                    count++;
                }
            }

            if (count >= lowerBound && count <= higherBound) {
                totalCorrect++;
            }
        }

        return totalCorrect;
    }

    private static int Part2(List<string> lines)
    {
        int totalCorrect = 0;
        foreach(string line in lines)
        {
            string[] args = line.Split(' ');

            string[] positions = args[0].Split('-');

            char checkFor = args[1][0];

            string password = args[2];

            bool firstCheck = false;
            if (Int32.Parse(positions[0]) - 1 <= password.Length) {
                firstCheck = password[Int32.Parse(positions[0]) - 1] == checkFor ? true : false;
            }
            
            bool secondCheck = false;
            if (Int32.Parse(positions[1]) - 1 <= password.Length) {
                secondCheck = password[Int32.Parse(positions[1]) - 1] == checkFor ? true : false;
            }

            if (firstCheck ^ secondCheck) {
                totalCorrect++;
            }
        }

        return totalCorrect;
    }
}