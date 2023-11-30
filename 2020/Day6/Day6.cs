using System;
using System.IO;
using System.Linq;
using System.Collections.Generic;

class MainClass
{
    public static void Main(string[] args)
    {
        List<string> answers = File.ReadLines("./Input.txt").ToList();

        Console.WriteLine($"Part1 sum of 'yes' answers: {Part1(answers)}");
        Console.WriteLine($"Part2 sum of answers: {Part2(answers)}");
    }

    private static int Part1(List<string> answers)
    {
        int totalSum = 0;
        string answeredQuestions = "";

        foreach (string line in answers)
        {
            if (line == "")
            {
                answeredQuestions = "";
                continue;
            }


            foreach (char questionId in line)
            {
                if (answeredQuestions.IndexOf(questionId) == -1)
                {
                    totalSum++;
                    answeredQuestions += questionId;
                }
            }
        }

        return totalSum;
    }

    private static int Part2(List<string> answers)
    {
        int totalSum = 0;
        List<char> answeredQuestions = new List<char>();
        List<int> answerCount = new List<int>();
        int groupLength = 0;

        foreach (string line in answers)
        {
            if (line == "")
            {
                foreach (int count in answerCount)
                {
                    if (count == groupLength)
                    {
                        totalSum++;
                    }
                }

                answeredQuestions.Clear();
                answerCount.Clear();
                groupLength = 0;
                continue;
            }

            groupLength++;

            foreach (char questionId in line)
            {
                int index = answeredQuestions.IndexOf(questionId);
                if (index == -1)
                {
                    answeredQuestions.Add(questionId);
                    answerCount.Add(1);
                }
                else
                {
                    answerCount[index]++;
                }
            }
        }

        return totalSum;
    }
}