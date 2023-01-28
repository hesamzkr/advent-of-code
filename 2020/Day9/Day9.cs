using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class MainClass
{
    public static void Main(string[] args)
    {
        List<string> numbers = File.ReadLines("./Input.txt").ToList();

        // Console.WriteLine($"Part1 value: {Part1(numbers, 25)}");
        Console.WriteLine($"Part2 value: {Part2(numbers, Part1(numbers, 25))}");
    }

    private static int Part1(List<string> numbers, int preambleLength)
    {
        for (int i = preambleLength; i < numbers.Count ; i++)
        {
            int startingIndex = i - preambleLength;
            string[] slicedArray = numbers.GetRange(startingIndex, preambleLength).ToArray();

            int[] prevNums = Array.ConvertAll(slicedArray, s => Int32.Parse(s));
            int num = Int32.Parse(numbers[i]);
            if (!Validator(prevNums, num))
            {
                Console.WriteLine($"{i}");
                return num;
            }
        }
        
        return -1;
    }

    private static long Part2(List<string> numbers, int target)
    {
        for (int i = 0; i < numbers.Count; i++)
        {
            long num = Int64.Parse(numbers[i]);
            List<long> sumList = new List<long>() {num};
            long sum = num;
            for (int j = i + 1; j < numbers.Count; j++)
            {
                sumList.Add(Int64.Parse(numbers[j]));
                sum += Int64.Parse(numbers[j]);

                if (sum == target)
                {
                    sumList.Sort();
                    return sumList[0] + sumList[sumList.Count - 1];
                }
            }
        }

        return -1;
    }

    // private static bool Recursion(int numIndex, int target, List<string> numbers)
    // {
    //     for (int j = 1; j < 5; j++)
    //     {
    //         if ()
    //     }
    // }

    private static bool Validator(int[] prevNums, int number)
    {
        for (int i = 0; i < prevNums.Length; i++)
        {
            for (int j = 0; j < prevNums.Length - 1; j++)
            {
                if (prevNums[i] + prevNums[j] == number)
                    return true;
            }
        }

        return false;
    }
}