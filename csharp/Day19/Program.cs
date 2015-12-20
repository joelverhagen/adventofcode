using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace AdventOfCode.Day19
{
    public class Program
    {
        public void Run()
        {
            var problem = ParseFile(@"Day19\input.txt");

            // Part 1
            var part1Answer = EnumerateReplacedSequences(problem).Count();
            Console.WriteLine($"Part 1 answer: {part1Answer}");
        }

        private IEnumerable<string> EnumerateReplacedSequences(Problem problem)
        {
            var allReplaced = new HashSet<string>();
            for (int i = 0; i < problem.Sequence.Count; i++)
            {
                if (!problem.Sequence[i].Replaceable)
                {
                    continue;
                }

                foreach (var replacement in problem.Replacements[problem.Sequence[i].Value])
                {
                    var replaced = problem.Sequence.Select(s => s.Value).ToArray();
                    replaced[i] = replacement;
                    var replacedString = string.Join(string.Empty, replaced);
                    if (allReplaced.Add(replacedString))
                    {
                        yield return replacedString;
                    }
                }
            }
        }

        private Problem ParseFile(string path)
        {
            // parse replacements
            var replacementList = new List<KeyValuePair<string, string>>();
            var readingReplacements = true;
            var sequenceString = string.Empty;
            foreach (var line in File.ReadAllLines(path))
            {
                if (line.Trim().Length == 0)
                {
                    readingReplacements = false;
                    continue;
                }

                if (readingReplacements)
                {
                    var pieces = line.Split(new[] { "=>" }, 2, StringSplitOptions.None).Select(p => p.Trim()).ToArray();
                    replacementList.Add(new KeyValuePair<string, string>(pieces[0], pieces[1]));
                }
                else
                {
                    sequenceString = line.Trim();
                }
            }

            var replacements = replacementList.ToLookup(p => p.Key, p => p.Value);

            // parse the sequence
            var sequence = new List<SequenceItem>();
            var replacementKeys = replacements.Select(p => p.Key).OrderByDescending(k => k.Length).Distinct().ToArray();
            for (int i = 0; i < sequenceString.Length; i++)
            {
                var matched = false;
                foreach (var replacement in replacementKeys)
                {
                    if (i + replacement.Length <= sequenceString.Length && sequenceString.Substring(i, replacement.Length) == replacement)
                    {
                        sequence.Add(new SequenceItem { Replaceable = true, Value = replacement });
                        i += replacement.Length - 1;
                        matched = true;
                        break;
                    }
                }

                if (!matched)
                {
                    var last = sequence.LastOrDefault();
                    if (last != null && !last.Replaceable)
                    {
                        last.Value += sequenceString[i];
                    }
                    else
                    {
                        sequence.Add(new SequenceItem { Replaceable = false, Value = sequenceString[i].ToString() });
                    }
                }
            }

            return new Problem {Replacements = replacements, Sequence = sequence};
        }
    }

    public class Problem
    {
        public ILookup<string, string> Replacements { get; set; }
        public IList<SequenceItem> Sequence { get; set; }
    }

    public class SequenceItem
    {
        public bool Replaceable { get; set; }
        public string Value { get; set; }

        public override string ToString()
        {
            return Value;
        }
    }
}
