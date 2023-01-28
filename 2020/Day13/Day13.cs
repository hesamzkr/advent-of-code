using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class MainClass
{
    public static void Main(string[] args)
    {
        List<string> input = File.ReadLines("./Input.txt").ToList();

        // Console.WriteLine($"Part1 value: {Part1(input)}");
        Console.WriteLine($"Part2 value: {Part2(input[1])}");
    }

    private static long Part1(List<string> input)
    {
        long earliestTime = Int64.Parse(input[0]);
        List<long> busIds = input[1].Split(',').Where(a => a != "x").Select(Int64.Parse).ToList();
        long closestDiff = 10000;
        long closestId = 0;

        foreach (long id in busIds)
        {
            long divide = (long) earliestTime / id + 1;
            long diff = id * divide - earliestTime;
            if (diff < closestDiff)
            {
                closestDiff = diff;
                closestId = id;
            }
        }

        return closestId * closestDiff;
    }

    private static long Part2(string input)
    {
        List<string> buses = input.Split(',').ToList();
        long firstNumber = Int64.Parse(buses[0]);
        int numOfEmpties = input.Count(f => f == 'x');
        
        for (long i = 0; i < 1000000000; i++)
        {
            long initialValue = firstNumber * i;
            int count = 0;

            for (int j = 1; j < buses.Count; j++)
            {
                if (buses[j] != "x") {
                    long nextNumber = Int64.Parse(buses[j]);
                    if (((initialValue + j) % nextNumber) == 0)
                    {
                        count++;
                    }
                }
            }
            
            
            if (count == buses.Count - numOfEmpties - 1)
            {
                return firstNumber * i;
            }

        }

        return -1;
    }

}