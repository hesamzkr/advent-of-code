using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class MainClass
{
    public static void Main(string[] args)
    {
        List<string> rules = File.ReadLines("./Input.txt").ToList();
        // List<string> part1TargetBags = new List<string>() {"shiny gold"};
        Dictionary<string, int> initialBag = new Dictionary<string, int>() {"shiny gold", 0};
        List<Dictionary<string, int>> part2TargetBags = new List<Dictionary<string, int>>() {initialBag};

        // Console.WriteLine($"Part1 color bags: {Part1(part1TargetBags, rules)}");
        Console.WriteLine($"Part2 answer: {Part2(part2TargetBags, rules)}");
    }

    private static int Part1(List<string> targetBags, List<string> rules)
    {
        List<string> alreadyChecked = new List<string>();
        int count = 0;

        while (targetBags.Count > 0)
        {
            for (int i = 0; i < targetBags.Count; i++)
            {
                foreach (string rule in rules)
                {
                    string firstBag = rule.Split(new string[] { " bags" }, StringSplitOptions.None)[0];
                    if (rule.IndexOf(targetBags[i]) != -1 && firstBag != targetBags[i] && alreadyChecked.IndexOf(firstBag) == -1)
                    {
                        targetBags.Add(firstBag);
                        alreadyChecked.Add(firstBag);
                        count++;
                    }
                }

                targetBags.RemoveAt(i);
            }
        }

        return count;
    }

    private static int Part2(List<Dictionary<string, int>> targetBags, List<string> rules)
    {
        // List<string> alreadyChecked = new List<string>();
        // int count = 0;

        while (targetBags.Count > 0)
        {
            for (int i = 0; i < targetBags.Count; i++)
            {
                string targetBag = targetBags[i];
                foreach (string rule in rules)
                {
                    string splittedRule = rule.Split(new string[] { " bags" }, StringSplitOptions.None);
                    if (splittedRule[0] == targetBag)
                    {
                        targetBags.Add(firstBag);
                        alreadyChecked.Add(firstBag);
                        count++;
                    }
                }

                targetBags.RemoveAt(i);
            }
        }

        //calcs
        return count;
    }
}