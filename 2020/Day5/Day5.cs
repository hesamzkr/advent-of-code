using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class MainClass
{
    public static void Main(string[] args)
    {
        List<string> boardingPasses = File.ReadLines("./Input.txt").ToList();

        Console.WriteLine($"Part1 highest seat ID: {Part1(boardingPasses)}");
        Console.WriteLine($"Part2 ID: {Part2(boardingPasses)}");
    }

    private static int IdCalculator(string boardingPass)
    {
            int lowerRowBound = 0;
            int upperRowBound = 127;

            int lowerColBound = 0;
            int upperColBound = 7;

            int row = 0;
            int col = 0;

            for (int i = 0; i < 7; i++)
            {
                if (boardingPass[i] == 'F')
                {
                    upperRowBound = (int)Math.Floor((decimal)(upperRowBound + lowerRowBound) / 2);
                    
                    row = upperRowBound;
                }
                else
                {
                    lowerRowBound = (int)Math.Ceiling((decimal)(upperRowBound + lowerRowBound) / 2);
                    row = lowerRowBound;
                }
            }

            for (int j = 7; j < boardingPass.Length; j++)
            {
                if (boardingPass[j] == 'L')
                {
                    upperColBound = (int)Math.Floor((decimal)(upperColBound + lowerColBound) / 2);
                    col = upperColBound;
                }
                else
                {
                    lowerColBound = (int)Math.Ceiling((decimal)(upperColBound + lowerColBound) / 2);
                    col = lowerColBound;
                }
            }

            return row * 8 + col;
    }

    private static int Part1(List<string> boardingPasses)
    {
        int highestId = 0;

        foreach (string pass in boardingPasses)
        {
            int newId = IdCalculator(pass);
            if (newId > highestId)
            {
                highestId = newId;
            }
        }

        return highestId;
    }

    private static int Part2(List<string> boardingPasses)
    {
        List<int> ids = new List<int>();

        foreach (string pass in boardingPasses)
        {
            ids.Add(IdCalculator(pass));
        }

        ids.Sort();

        for (int i = ids[0]; i < ids[ids.Count - 1]; i++)
        {
            if (!ids.Contains(i))
            {
                return i;
            }
        }

        return -1;
    }
}