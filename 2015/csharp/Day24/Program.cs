using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Numerics;

namespace AdventOfCode.Day24
{
    public class Program
    {
        public void Run()
        {
            var enumerator = new Enumerator();
            var packages = new Parser().ParseFile(@"Day24\input.txt").ToArray();

            var part1Result = enumerator.Enumerate(packages, 3).First();
            Console.WriteLine($"Part 1 answer: {part1Result.QuantumEntanglement}");

            var part2Result = enumerator.Enumerate(packages, 4).First();
            Console.WriteLine($"Part 2 answer: {part2Result.QuantumEntanglement}");
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
        public IEnumerable<Groups> Enumerate(IEnumerable<int> packageSequence, int groupCount)
        {
            // sort packages in increasing weight
            var packages = packageSequence.OrderBy(p => p).ToArray();
            var totalWeight = packages.Sum();
            if (totalWeight%groupCount != 0)
            {
                throw new ArgumentException($"The total package weight ({totalWeight}) must be divisible by {groupCount}.");
            }

            var target = totalWeight/groupCount;
            var tooHeavy = packages.Where(p => p > target).ToArray();
            if (tooHeavy.Any())
            {
                throw new ArgumentException($"One or more of packages weights ({string.Join(", ", tooHeavy)}) weighs more than the target weight ({target}).");
            }
            
            foreach (var group1 in EnumerateGroups(target, new int[0], packages))
            {
                // make sure at least one configuration for the other groups exists
                var remaining = new HashSet<int>(packages);
                remaining.ExceptWith(group1);
                
                var otherGroups = new List<int[]>();
                for (int i = 1; i < groupCount; i++)
                {
                    var otherGroup = EnumerateGroups(target, new int[0], remaining.ToArray()).FirstOrDefault();
                    if (otherGroup == null)
                    {
                        break;
                    }

                    otherGroups.Add(otherGroup);
                    remaining.ExceptWith(otherGroup);
                }

                if (otherGroups.Count == groupCount - 1)
                {
                    yield return new Groups { Group1 = group1, OtherGroups = otherGroups.ToArray() };
                }
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
        public int[][] OtherGroups { get; set; }

        public BigInteger QuantumEntanglement => Group1.Aggregate(BigInteger.One, (p, v) => p*v);
    }
}