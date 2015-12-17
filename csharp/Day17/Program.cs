using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace AdventOfCode.Day17
{
    public class Program
    {
        public void Run()
        {
            var values = ParseFile(@"Day17\input.txt");
            var solutions = Enumerate(values, 150).ToArray();

            Console.WriteLine($"Part 1 answer: {solutions.Length}");

            var minimumContainers = solutions.Min(i => i.Length);
            Console.WriteLine($"Part 2 answer: {solutions.Count(i => i.Length == minimumContainers)}");
        }

        private IEnumerable<int> ParseFile(string path)
        {
            return File.ReadAllLines(path).Select(int.Parse).ToArray();
        }

        private IEnumerable<int[]> Enumerate(IEnumerable<int> values, int sum)
        {
            return Enumerate(new int[0], values.ToArray(), sum);
        }

        private IEnumerable<int[]> Enumerate(int[] prefix, int[] remaining, int sum)
        {
            if (remaining.Length == 1)
            {
                if (remaining[0] == sum)
                {
                    yield return prefix.Concat(remaining).ToArray();
                }

                yield break;
            }

            for (int i = 0; i < remaining.Length; i++)
            {
                var newSum = sum - remaining[i];
                var newPrefix = prefix.Concat(remaining.Skip(i).Take(1)).ToArray();
                var newRemaining = remaining.Skip(i + 1).Where(v => v <= newSum).ToArray();
                if (newSum < 0)
                {
                    yield break;
                }

                if (newSum == 0)
                {
                    yield return newPrefix;
                }
                else
                {
                    foreach (var result in Enumerate(newPrefix, newRemaining, newSum))
                    {
                        yield return result;
                    }
                }
            }
        }
    }
}
