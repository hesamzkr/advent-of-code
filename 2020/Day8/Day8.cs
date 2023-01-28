using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class MainClass
{
    public static void Main(string[] args)
    {
        List<string> instructions = File.ReadLines("./Input.txt").ToList();

        Console.WriteLine($"Part1 value: {Part1(instructions)}");
        Console.WriteLine($"Part2 value: {Part2(instructions)}");
    }

    private static int Part1(List<string> instructions)
    {
        int accValue = 0;
        List<int> checkedInstructions = new List<int>();

        int i = 0;
        while (!checkedInstructions.Contains(i) && i < instructions.Count - 1)
        {
            string instruction = instructions[i];
            string operation = instruction.Substring(0,3);
            int argument = Int32.Parse(instruction.Substring(3));

            checkedInstructions.Add(i);

            if (operation == "acc")
            {
                accValue += argument;
            }
            else if (operation == "jmp")
            {
                i += argument;
                continue;
            }
            
            i++;
        }

        return accValue;
    }

    private static int Part2(List<string> instructions)
    {
        for (int i = 0; i < instructions.Count(); i++)
        {
            List<string> newList = new List<string>(instructions);
            if (newList[i].Substring(0,3) == "jmp")
            {
                newList[i] = newList[i].Replace("jmp", "nop");
            }
            else
            {
                newList[i] = newList[i].Replace("nop", "jmp");
            }

            
            if (Validator(newList))
            {
                return Part1(newList);
            }
        }

        return -1;
    }

    private static bool Validator(List<string> instructions)
    {
        List<int> checkedInstructions = new List<int>();
        int i = 0;

        while (true)
        {
            checkedInstructions.Add(i);
            string instruction = instructions[i];
            string operation = instruction.Substring(0,3);
            int argument = Int32.Parse(instruction.Substring(3));

            if (operation == "jmp")
            {
                i += argument;
            }
            else
            {
                i++;
            }


            if (i == instructions.Count())
            {
                return true;
            }
            else if (checkedInstructions.Contains(i))
            {
                return false;
            }

        }
    }
}