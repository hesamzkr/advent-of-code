using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class MainClass
{
    private static int angle = 0;
    private static int horizontallDistance = 0;
    private static int verticallDistance = 0;
    private static int[] wayPoint = {10, 1};

    public static void Main(string[] args)
    {
        List<string> instructions = File.ReadLines("./Input.txt").ToList();

        Console.WriteLine($"Part1 distance: {Part1(instructions)}");
        Console.WriteLine($"Part2 distance: {Part2(instructions)}");
    }

    private static int Part1(List<string> instructions)
    {
        horizontallDistance = verticallDistance = 0;

        foreach (string instruction in instructions)
        {
            char direction = instruction[0];
            int argument = Int32.Parse(instruction.Substring(1));

            switch (direction)
            {
                case 'F':
                    Part1AngleHandler(argument);
                    break;
                case 'R':
                    angle = (angle + (360 - argument)) % 360;
                    break;
                case 'L':
                    angle =  (angle + argument) % 360;
                    break;
                case 'N':
                    verticallDistance += argument;
                    break;
                case 'S':
                    verticallDistance -= argument;
                    break;
                case 'E':
                    horizontallDistance += argument;
                    break;
                case 'W':
                    horizontallDistance -= argument;
                    break;

                default:
                    Console.WriteLine($"err: dir={direction}, arg={argument}");
                    break;
            }
        }

        return Math.Abs(verticallDistance) + Math.Abs(horizontallDistance);
    }

    private static void Part1AngleHandler(int degrees)
    {
        switch (angle)
        {
            case 0:
                horizontallDistance += degrees;
                break;
            case 90:
                verticallDistance += degrees;
                break;
            case 180:
                horizontallDistance -= degrees;
                break;
            case 270:
                verticallDistance -= degrees;
                break;

            default:
                Console.WriteLine($"Wrong angle {angle}");
                break;
        }
    }

    private static int Part2(List<string> instructions)
    {
        horizontallDistance = verticallDistance = 0;

        foreach (string instruction in instructions)
        {
            char direction = instruction[0];
            int argument = Int32.Parse(instruction.Substring(1));

            switch (direction)
            {
                case 'F':
                    horizontallDistance += argument * wayPoint[0];
                    verticallDistance += argument * wayPoint[1];
                    break;
                case 'R':
                    Part2AngleHandler(argument, 'R');
                    break;
                case 'L':
                    Part2AngleHandler(argument, 'L');
                    break;
                case 'N':
                    wayPoint[1] += argument;
                    break;
                case 'S':
                    wayPoint[1] -= argument;
                    break;
                case 'E':
                    wayPoint[0] += argument;
                    break;
                case 'W':
                    wayPoint[0] -= argument;
                    break;

                default:
                    Console.WriteLine($"err: dir={direction}, arg={argument}");
                    break;
            }
        }

        return Math.Abs(horizontallDistance) + Math.Abs(verticallDistance);
    }

    private static void Part2AngleHandler(int degrees, char type)
    {
        double radians = degrees * Math.PI / 180;

        int sin = (int)Math.Sin(radians);
        int cos = (int)Math.Cos(radians);

        if (type == 'L')
        {
            int temp = wayPoint[0] * cos + wayPoint[1] * -sin;
            wayPoint[1] = wayPoint[0] * sin + wayPoint[1] * cos;
            wayPoint[0] = temp;
        }
        else if (type == 'R')
        {
            int temp = wayPoint[0] * cos + wayPoint[1] * sin;
            wayPoint[1] = wayPoint[0] * -sin + wayPoint[1] * cos;
            wayPoint[0] = temp;
        }
    }
}