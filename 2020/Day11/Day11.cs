using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class MainClass
{
    public static void Main(string[] args)
    {
        List<string> seats = File.ReadLines("./Input.txt").ToList();

        Console.WriteLine($"Part1 value: {Part1(seats)}");
        Console.WriteLine($"Part2 value: {Part2(seats)}");
    }

    private static int Part1(List<string> seats)
    {
        bool hasChanged = true;

        List<string> newSeats = new List<string>(seats);

        while (hasChanged)
        {
            hasChanged = false;
            
            seats = new List<string>(newSeats);

            for (int i = 0; i < seats.Count; i++)
            {
                string row = seats[i];
                for (int j = 0; j < row.Length; j++)
                {
                    int occupied = 0;

                    if (j != row.Length - 1 && seats[i][j + 1] == '#') occupied++;

                    if (j != 0 && seats[i][j - 1] == '#') occupied++;

                    if (i != seats.Count - 1 && seats[i + 1][j] == '#') occupied++;
                    
                    if (i != 0 && seats[i - 1][j] == '#') occupied++;

                    if (i != 0 && j != row.Length - 1 && seats[i - 1][j + 1] == '#') occupied++;

                    if (i != 0 && j != 0 && seats[i - 1][j - 1] == '#') occupied++;

                    if (i != seats.Count - 1 && j != row.Length - 1 && seats[i + 1][j + 1] == '#') occupied++;

                    if (i != seats.Count - 1 && j != 0 && seats[i + 1][j - 1] == '#') occupied++;

                    if (row[j] == 'L' && occupied == 0)
                    {
                        char[] ch = newSeats[i].ToCharArray();
                        ch[j] = '#';
                        newSeats[i] = new string (ch);
                        hasChanged = true;
                    }
                    else if (row[j] == '#' && occupied >= 4)
                    {
                        char[] ch = newSeats[i].ToCharArray();
                        ch[j] = 'L';
                        newSeats[i] = new string (ch);
                        hasChanged = true;
                    }
                }
            }
        }

        int occupiedSeats = 0;
        foreach (string row in seats)
        {
            occupiedSeats += row.Count(c => c == '#');
        }

        return occupiedSeats;
    }

    private static int Part2(List<string> seats)
    {
        bool hasChanged = true;

        List<string> newSeats = new List<string>(seats);

        while (hasChanged)
        {
            hasChanged = false;
            
            seats = new List<string>(newSeats);

            for (int i = 0; i < seats.Count; i++)
            {
                string row = seats[i];
                for (int j = 0; j < row.Length; j++)
                {
                    int occupied = 0;
                    bool a,b,c,d,e,f,g,h;
                    a=b=c=d=e=f=g=h = true;
                    for (int x = 1; x < seats.Count; x++)
                    {
                        if (a && j + x <= row.Length - 1 && seats[i][j + x] != '.')
                        {
                            a = false;
                            if (seats[i][j + x] == '#') occupied++;
                        }

                        if (b && j - x >= 0 && seats[i][j - x] != '.')
                        {
                            b = false;
                            if (seats[i][j - x] == '#') occupied++;
                        }

                        if (c && i + x <= seats.Count - 1 && seats[i + x][j] != '.')
                        {
                            c = false;
                            if (seats[i + x][j] == '#') occupied++;
                        }

                        if (d && i - x >= 0 && seats[i - x][j] != '.')
                        {
                            d = false;
                            if (seats[i - x][j] == '#') occupied++;
                        }

                        if (e && i - x >= 0 && j + x <= row.Length - 1 && seats[i - x][j + x] != '.')
                        {
                            e = false;
                            if (seats[i - x][j + x] == '#') occupied++;
                        }

                        if (f && i - x >= 0 && j - x >= 0 && seats[i - x][j - x] != '.')
                        {
                            f = false;
                            if (seats[i - x][j - x] == '#') occupied++;
                        }

                        if (g && i + x <= seats.Count - 1 && j + x <= row.Length - 1 && seats[i + x][j + x] != '.')
                        {
                            g = false;
                            if (seats[i + x][j + x] == '#') occupied++;
                        }

                        if (h && i + x <= seats.Count - 1 && j - x >= 0 && seats[i + x][j - x] != '.')
                        {
                            h = false;
                            if (seats[i + x][j - x] == '#') occupied++;
                        }
                    }

                    if (row[j] == 'L' && occupied == 0)
                    {
                        char[] ch = newSeats[i].ToCharArray();
                        ch[j] = '#';
                        newSeats[i] = new string (ch);
                        hasChanged = true;
                    }
                    else if (row[j] == '#' && occupied >= 5)
                    {
                        char[] ch = newSeats[i].ToCharArray();
                        ch[j] = 'L';
                        newSeats[i] = new string (ch);
                        hasChanged = true;     
                    }
                }
            }
        }

        int occupiedSeats = 0;
        foreach (string row in seats)
        {
            occupiedSeats += row.Count(c => c == '#');
        }

        return occupiedSeats;
    }
}