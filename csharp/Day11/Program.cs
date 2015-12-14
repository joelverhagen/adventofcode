using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using Microsoft.SqlServer.Server;

namespace AdventOfCode.Day11
{
    public class Program
    {
        public void Run()
        {
            var generator = new PasswordGenerator();
            foreach (var password in generator.Start("hxbxwxba").Take(1))
            {
                Console.WriteLine(password);
            }
        }
    }

    public class PasswordGenerator
    {
        public IEnumerable<string> Start(string input)
        {
            string next = input;
            while (true)
            {
                next = Increment(next);
                if (next == null)
                {
                    yield break;
                }

                if (MeetsCriteria(next))
                {
                    yield return next;
                }
            }
        }

        private string Increment(string input)
        {
            // convert from "base 26"
            var oldTotal = (ulong) input
                .Select(c => Encoding.ASCII.GetBytes(new[] { c })[0] - 97)
                .Reverse()
                .Select((value, place) => value * Math.Pow(26, place))
                .Sum();

            // increment
            var newTotal = oldTotal + 1;

            // convert to "base 26"
            var output = Enumerable.Repeat('a', input.Length).ToArray();
            int position = input.Length - 1;
            do
            {
                if (position < 0)
                {
                    return null;
                }

                var value = newTotal % 26;
                output[position] = Encoding.ASCII.GetString(new[] {(byte) (value + 97)})[0];
                position--;
                newTotal /= 26;
            }
            while (newTotal > 0);

            return new string(output.ToArray());
        }

        private bool MeetsCriteria(string input)
        {
            return ContainsStraight(input) && HasNoConfusingLetters(input) && HasTwoPairs(input);
        }

        private bool ContainsStraight(string input)
        {
            int length = 1;
            for (int i = 1; i < input.Length; i++)
            {
                if (input[i] - input[i - 1] == 1)
                {
                    length++;
                }
                else
                {
                    length = 1;
                }

                if (length >= 3)
                {
                    return true;
                }
            }

            return false;
        }

        private bool HasNoConfusingLetters(string input)
        {
            return !input.Contains("i") && !input.Contains("o") && !input.Contains("l");
        }

        private bool HasTwoPairs(string input)
        {
            int pairCount = 0;
            for (int i = 1; i < input.Length; i++)
            {
                if (input[i] == input[i - 1])
                {
                    pairCount++;
                    i++;
                }

                if (pairCount >= 2)
                {
                    return true;
                }
            }

            return false;
        }
    }
}
