using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class MainClass
{
    public static void Main(string[] args)
    {
        List<string> rows = File.ReadLines("./Input.txt").ToList();
        

        Console.WriteLine($"Part1 trees: {Part1(rows, 3, 1)}");
        Console.WriteLine($"Part2 multiplied trees: {Part2(rows)}");
    }

    private static long Part1(List<string> rows, int right, int down)
    {
        int treeCount = 0;
        int column = 0;
        for (int i = 0; i < rows.Count; i += down)
        {
            if (rows[i][column] == '#') {
                treeCount++;
            }
            column = (column + right) % rows[0].Length;
        }

        return (long)treeCount;
    }

    private static long Part2(List<string> rows)
    {
        long a = Part1(rows, 1, 1);
        long b = Part1(rows, 3, 1);
        long c = Part1(rows, 5, 1);
        long d = Part1(rows, 7, 1);
        long e = Part1(rows, 1, 2);

        return (a * b * c * d * e);
    }
}