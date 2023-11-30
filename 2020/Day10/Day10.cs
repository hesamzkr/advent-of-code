using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class MainClass
{
    public static void Main(string[] args)
    {
        List<string> input = File.ReadLines("./Input.txt").ToList();

        List<int> adaptors = input.Select(int.Parse).ToList();

        adaptors.Add(0);
        adaptors.Add(adaptors.Max() + 3);
        adaptors.Sort();

        Console.WriteLine($"Part1 value: {Part1(adaptors)}");
        Console.WriteLine($"Part2 arrangements: {Part2(adaptors)}");
    }

    private static int Part1(List<int> adaptors)
    {
        int oneDiff = 0;
        int threeDiff = 0;
        int currentAdaptor = 0;
        adaptors.Sort();

        foreach (int adaptor in adaptors)
        {
            int diff = Math.Abs(currentAdaptor - adaptor);
            if (diff == 1)
            {
                oneDiff++;
                currentAdaptor = adaptor;
            }
            else if (diff == 3)
            {
                threeDiff++;
                currentAdaptor = adaptor;
            }
        }
        
        return oneDiff * threeDiff;

    }


    private static long Part2(List<int> adaptors)
    {
        List<int> important = new List<int>();
        important.Add(adaptors[0]);
        for (int i = 0; i < adaptors.Count - 1; i++)
        {
            if (adaptors[i + 1] - adaptors[i] == 3)
            {
                if (!important.Contains(adaptors[i]))
                {
                    important.Add(adaptors[i]);
                }
                important.Add(adaptors[i + 1]);
            }
            else if (adaptors.Count - 2 == i)
            {
                important.Add(adaptors[i + 1]);
            } 
        }

        List<List<int>> unimportant = new List<List<int>>();
        for (int i = 0; i < adaptors.Count; i++)
        {
            if (!important.Contains(adaptors[i]))
            {
                unimportant[unimportant.Count - 1].Add(adaptors[i]);
            }
            else
            {
                if (unimportant.Count > 0)
                {
                    unimportant[unimportant.Count - 1].Add(adaptors[i]);
                }
                List<int> temp = new List<int>() { adaptors[i] };
                unimportant.Add(temp);
            }
        }

        long totalArrangements = 1;
        for (int i = 0; i < unimportant.Count; i++)
        {
            if (unimportant[i].Count > 2)
            {
                int range = unimportant[i].Max() - unimportant[i].Min();
                if (range <= 3)
                {
                    totalArrangements *= (unimportant[i].Count - 2) * 2;
                }
                else if (range == 4)
                {
                    int[] intArray = {1, 3, 7};
                    totalArrangements *= (intArray)[unimportant[i].Count - 3];
                }   
            }
        }

        return totalArrangements;
    }
}