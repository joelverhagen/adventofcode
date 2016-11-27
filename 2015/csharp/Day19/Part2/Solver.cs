using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AdventOfCode.Day19.Part2
{
    public class Solver
    {
        public int GetAnswer(string path)
        {
            var problem = Problem.ReadFromFile(path);
            var replacements = Reduce(problem, new List<ReplacementRecord>(), 0);
            return replacements.Count;
        }

        private IList<ReplacementRecord> Reduce(Problem problem, IList<ReplacementRecord> previousReplacements, int depth)
        {
            if (problem.Sequence == "e")
            {
                Console.WriteLine(previousReplacements.Count);
                return previousReplacements;
            }

            IList<ReplacementRecord> shortest = null;
            foreach (var rule in problem.ReplacementRules)
            {
                var expanded = rule.Key;
                foreach (var reduced in rule)
                {
                    var replacements = GetAllReplacements(problem.Sequence, expanded, reduced).ToArray();
                    foreach (var replacement in replacements)
                    {
                        var newReplacements = previousReplacements.ToList();
                        newReplacements.Add(new ReplacementRecord {Index = replacement.Index, Expanded = expanded, Reduced = reduced});

                        var innerReplacements = Reduce(new Problem { Sequence = replacement.Value, ReplacementRules = problem.ReplacementRules }, newReplacements, depth + 1);
                        if (innerReplacements == null)
                        {
                            continue;
                        }

                        if (shortest == null || innerReplacements.Count < shortest.Count)
                        {
                            shortest = innerReplacements;
                        }
                    }
                }
            }

            return shortest;
        }

        private IEnumerable<ReplacementResult> GetAllReplacements(string input, string from, string to)
        {
            int startIndex = 0;
            int indexOf;
            do
            {
                indexOf = input.IndexOf(from, startIndex, StringComparison.Ordinal);
                startIndex = indexOf + from.Length;
                if (indexOf < 0)
                {
                    break;
                }

                var replacement = input.Substring(0, indexOf) + to + input.Substring(indexOf + from.Length);
                yield return new ReplacementResult { Index = indexOf, Value = replacement };
            }
            while (indexOf >= 0 && startIndex <= input.Length - from.Length);
        }
    }

    public class ReplacementRecord
    {
        public int Index { get; set; }
        public string Reduced { get; set; }
        public string Expanded { get; set; }

        public override string ToString()
        {
            return $"{Index}: {Reduced} -> {Expanded}";
        }
    }

    public class ReplacementResult
    {
        public int Index { get; set; }
        public string Value { get; set; }
    }
}
