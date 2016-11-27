using System;
using System.Collections.Generic;
using System.Data;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;

namespace AdventOfCode.Day10
{
    public class Program
    {
        public void Run()
        {
            var lookAndSay = new LookAndSay();

            var at40 = lookAndSay.Start("1321131112").ElementAt(39);
            Console.WriteLine($"Part 1 answer: {at40.Length}");

            var at50 = lookAndSay.Start("1321131112").ElementAt(49);
            Console.WriteLine($"Part 2 answer: {at50.Length}");
        }
    }

    public class LookAndSay
    {
        public IEnumerable<string> Start(string input)
        {
            string currentString = input;
            while (true)
            {
                var builder = new StringBuilder();
                char currentChar = currentString[0];
                int count = 1;
                for (int i = 1; i < currentString.Length; i++)
                {
                    if (currentString[i] == currentChar)
                    {
                        count += 1;
                    }
                    else
                    {
                        builder.Append(count);
                        builder.Append(currentChar);
                        currentChar = currentString[i];
                        count = 1;
                    }
                }

                builder.Append(count);
                builder.Append(currentChar);

                currentString = builder.ToString();
                yield return currentString;
            }
        }
    }
}
