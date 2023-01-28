using System;
using System.IO;
using System.Collections.Generic;

class MainClass
{
    public static void Main(string[] args)
    {
        List<int> list = new List<int>();
        foreach (string line in File.ReadLines("./Input.txt"))
        {
            list.Add(Int32.Parse(line));
        }

        Console.WriteLine($"Part1: {Part1(list)}");
        Console.WriteLine($"Part2: {Part2(list)}");
    }


    private static int Part1(List<int> numbers)
    {
        for (int i = 0; i < numbers.Count - 1; i++)
        {
            for (int j = i + 1; j < numbers.Count - 1; j++)
            {
                if (numbers[i] + numbers[j] == 2020)
                {
                    return (numbers[i] * numbers[j]);
                }
            }
        }

        return -1;
    }

    private static int Part2(List<int> numbers)
    {
        for (int i = 0; i < numbers.Count - 1; i++)
        {
            for (int j = i + 1; j < numbers.Count - 1; j++)
            {
                for (int x = j + 1; x < numbers.Count - 1; x++)
                {
                    if (numbers[i] + numbers[j] + numbers[x] == 2020)
                    {
                        return (numbers[i] * numbers[j] * numbers[x]);
                    }
                }
            }
        }

        return -1;
    }
}