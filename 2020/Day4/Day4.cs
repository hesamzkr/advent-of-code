using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class MainClass
{
    public static void Main(string[] args)
    {
        List<string> lines = File.ReadLines("./Input.txt").ToList();
        

        Console.WriteLine($"Part1 valid passports: {Part1(lines)}");
        Console.WriteLine($"Part2 valid passports: {Part2(lines)}");
    }

    private static int Part1(List<string> lines)
    {
        int validPassports = 0;
        int fieldCount = 0;
        foreach (string line in lines)
        {
            if (line == "")
            {
                if (fieldCount >= 7) {
                    validPassports++;
                }
                fieldCount = 0;
                continue;
            }

            string[] props = line.Split(' ');

            foreach (string prop in props)
            {
                if (prop.IndexOf("cid") == -1)
                {
                    fieldCount++;
                }
            }
        }

        return validPassports;
    }

    private static int Part2(List<string> lines)
    {
        int validPassports = 0;
        int fieldCount = 0;
        foreach (string line in lines)
        {
            if (line == "")
            {
                if (fieldCount >= 7) {
                    validPassports++;
                }
                fieldCount = 0;
                continue;
            }

            string[] props = line.Split(' ');

            foreach (string prop in props)
            {
                if (prop.Length < 3) continue;

                string[] args = prop.Split(':');
                
                string passField = args[0];
                string info = args[1];

                switch (passField)
                {
                    case "byr":
                        int byrNum = Int32.Parse(info);
                        if (byrNum >= 1920 && byrNum <= 2002) {
                            fieldCount++;
                        }
                        break;
                    case "iyr":
                        int iyrNum = Int32.Parse(info);
                        if (iyrNum >= 2010 && iyrNum <= 2020) {
                            fieldCount++;
                        }
                        break;
                    case "eyr":
                        int eyrNum = Int32.Parse(info);
                        if (eyrNum >= 2020 && eyrNum <= 2030) {
                            fieldCount++;
                        }
                        break;
                    case "hgt":
                        string unit = info.Substring(info.Length - 2);
                        if (unit == "cm") {
                            int hgtNum = Int32.Parse(info.Split('c')[0]);
                            if (hgtNum >= 150 && hgtNum <= 193) {
                                fieldCount++;
                            }
                        } else {
                            int hgtNum = Int32.Parse(info.Split('i')[0]);
                            if (hgtNum >= 59 && hgtNum <= 76) {
                                fieldCount++;
                            }
                        }
                        break;
                    case "hcl":
                        char[] validNums = {'0', '1', '2', '3', '4', '5', '6', '7', '8', '9'};
                        char[] validChars = {'a', 'b', 'c', 'd', 'e', 'f'};
                        if (info.Length == 7) {
                            if (info[0] == '#') {
                                int temp = 0;
                                foreach (char i in info) {
                                    if (validNums.Contains(i) || validChars.Contains(i)) {
                                        temp++;
                                    }
                                }
                                if (temp == 6) {
                                    fieldCount++;
                                }
                            }
                        }
                        break;
                    case "ecl":
                        string[] validEcl = {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"};
                        if (validEcl.Contains(info)) {
                            fieldCount++;
                        }
                        break;
                    case "pid":
                        if (info.Length == 9) {
                            fieldCount++;
                        }
                        break;
                }
            }
        }        

        return validPassports;
    }
}