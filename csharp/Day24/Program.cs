using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Numerics;
using System.Text;

namespace AdventOfCode.Day24
{
    public class Program
    {
        public void Run()
        {
            var enumerator = new Enumerator();
            var packages = new Parser().ParseFile(@"Day24\input.txt").ToArray();
            var part1Result = enumerator.Enumerate(packages).First();

            Console.WriteLine($"Part 1 answer: {part1Result.QuantumEntanglement}");
        }
    }

    public class Parser
    {
        public IEnumerable<int> ParseFile(string path)
        {
            return File
                .ReadAllLines(path)
                .Where(p => !string.IsNullOrWhiteSpace(p))
                .Select(int.Parse);
        }
    }

    public class Enumerator
    {
        public IEnumerable<Groups> Enumerate(IEnumerable<int> packageSequence)
        {
            // sort packages in increasing weight
            var packages = packageSequence.OrderBy(p => p).ToArray();
            var totalWeight = packages.Sum();
            if (totalWeight%3 != 0)
            {
                throw new ArgumentException($"The total package weight ({totalWeight}) must be divisible by three.");
            }

            var target = totalWeight/3;
            var tooHeavy = packages.Where(p => p > target).ToArray();
            if (tooHeavy.Any())
            {
                throw new ArgumentException($"One or more of packages weights ({string.Join(", ", tooHeavy)}) weighs more than the target weight ({target}).");
            }
            
            foreach (var group1 in EnumerateGroups(target, new int[0], packages))
            {
                var group1Remaining = new HashSet<int>(packages);
                group1Remaining.ExceptWith(group1);

                var group2 = EnumerateGroups(target, new int[0], group1Remaining.ToArray()).FirstOrDefault();
                if (group2 == null)
                {
                    continue;
                }

                var group2Remaining = new HashSet<int>(group1Remaining);
                group2Remaining.ExceptWith(group2);

                var group3 = EnumerateGroups(target, new int[0], group2Remaining.ToArray()).FirstOrDefault();
                if (group3 == null)
                {
                    continue;
                }

                yield return new Groups {Group1 = group1, Group2 = group2, Group3 = group3};
            }
        }

        private IEnumerable<int[]> EnumerateGroups(int target, int[] prefix, int[] remaining)
        {
            var queue = new Queue<Candidate>();
            queue.Enqueue(new Candidate {Prefix = prefix, Remaining = remaining});

            while (queue.Any())
            {
                var candidate = queue.Dequeue();

                if (candidate.Prefix.Sum() > target)
                {
                    continue;
                }

                if (candidate.Prefix.Sum() == target)
                {
                    yield return candidate.Prefix;
                    continue;
                }

                for (var i = 0; i < candidate.Remaining.Length; i++)
                {
                    var newPrefix = candidate.Prefix.Concat(new[] { candidate.Remaining[i] }).ToArray();
                    var newRemaining = candidate.Remaining.Skip(i + 1).ToArray();
                    queue.Enqueue(new Candidate { Prefix = newPrefix, Remaining = newRemaining });
                }
            }
        }

        private class Candidate
        {
            public int[] Prefix { get; set; }
            public int[] Remaining { get; set; }
        }
    }

    public class Groups
    {
        public int[] Group1 { get; set; }
        public int[] Group2 { get; set; }
        public int[] Group3 { get; set; }

        public BigInteger QuantumEntanglement => Group1.Aggregate(BigInteger.One, (p, v) => p*v);
    }
}