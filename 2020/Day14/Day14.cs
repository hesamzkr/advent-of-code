using System;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;
using System.Collections.Generic;

class MainClass
{

    private static Dictionary<string, string> memory = new Dictionary<string, string>();
    private static string mask = "";
    private static string value = "";

    public static void Main(string[] args)
    {
        List<string> input = File.ReadLines("./Input.txt").ToList();

        Console.WriteLine($"Part1 sum: {Part1(input)}");
        Console.WriteLine($"Part2 sum: {Part2(input)}");
    }

    private static long Part1(List<string> input)
    {
        mask = "";
        value = "";
        memory.Clear();
        foreach (string line in input)
        {
            string[] lineSplit = line.Split(' ');

            if (lineSplit[0] == "mask")
            {
                mask = lineSplit[2];
            }
            else
            {
                string memoryLoc = Regex.Match(lineSplit[0], @"[0-9]+").Value;

                string value = ApplyMask(ConvertToBits(lineSplit[2]), 'X');

                memory[memoryLoc] = value;
            }
        }

        long sum = 0;
        foreach (KeyValuePair<string, string> m in memory)
        {
            sum += Convert.ToInt64(m.Value, 2);
        }

        return sum;
    }


    private static long Part2(List<string> input)
    {
        mask = "";
        value = "";
        memory.Clear();
        foreach (string line in input)
        {
            string[] lineSplit = line.Split(' ');

            if (lineSplit[0] == "mask")
            {
                mask = lineSplit[2];
            }
            else
            {
                string memoryLoc = ApplyMask(ConvertToBits(Regex.Match(lineSplit[0], @"[0-9]+").Value), '0');

                value = lineSplit[2];

                Permutations(new List<string>() {memoryLoc});
            }
        }

        long sum = 0;
        foreach (KeyValuePair<string, string> m in memory)
        {
            sum += Convert.ToInt64(m.Value);
        }

        return sum;
    }

    private static string ApplyMask(string bits, char delimiter)
    {
        for (int i = 0; i < mask.Length; i++)
        {
            if (mask[i] != delimiter)
            {
                char[] ch = bits.ToCharArray();
                ch[i] = mask[i];
                bits = new string (ch);
            }
        }

        return bits;
    }

    private static void Permutations(List<string> bitsList)
    {
        List<string> nextIter = new List<string>();

        if (bitsList.Count > 0)
        {
            foreach (string bits in bitsList)
            {
                if (bits.Contains("X"))
                {
                    int first = 0;
                    for (int i = 0; i < bits.Length; i++)
                    {
                        if (bits[i] == 'X')
                        {
                            first = i;
                            break;
                        }
                    }

                    string newBits = bits.Replace("X", "0");
                    string newBits2 = bits.Replace("X", "1");

                    char[] ch1 = newBits.ToCharArray();
                    char[] ch2 = newBits.ToCharArray();
                    char[] ch3 = newBits2.ToCharArray();
                    char[] ch4 = newBits2.ToCharArray();

                    ch1[first] = '0';
                    ch2[first] = '1';
                    ch3[first] = '0';
                    ch4[first] = '1';

                    memory[new string (ch1)] = value;
                    memory[new string (ch2)] = value;
                    memory[new string (ch3)] = value;
                    memory[new string (ch4)] = value;

                    ch1 = bits.ToCharArray();
                    ch2 = bits.ToCharArray();
                    ch1[first] = '0';
                    ch2[first] = '1';

                    nextIter.Add(new string (ch1));
                    nextIter.Add(new string (ch2));
                }
            }

            Permutations(nextIter);
        }
    }

    private static string ConvertToBits(string numberStr)
    {
        long number = Int32.Parse(numberStr);
        string value = Convert.ToString(number, 2);

        string temp = "";
        for (int i = 0; i < 36 - value.Length; i++)
        {
            temp += "0";
        }

        return temp + value;
    }
}