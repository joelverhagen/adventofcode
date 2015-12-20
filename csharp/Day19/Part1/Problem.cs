using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode.Day19.Part1
{
    public class Problem
    {
        public ILookup<string, string> Replacements { get; set; }
        public IList<SequenceItem> Sequence { get; set; }

        public static Problem ReadFromFile(string path)
        {
            var parsedInput = ParsedInput.ReadFromFile(path);
            var replacements = parsedInput.Replacements.ToLookup(p => p.Key, p => p.Value);

            // parse the sequence
            var sequence = new List<SequenceItem>();
            var replacementKeys = replacements.Select(p => p.Key).OrderByDescending(k => k.Length).Distinct().ToArray();
            for (int i = 0; i < parsedInput.Sequence.Length; i++)
            {
                var matched = false;
                foreach (var replacement in replacementKeys)
                {
                    if (i + replacement.Length <= parsedInput.Sequence.Length && parsedInput.Sequence.Substring(i, replacement.Length) == replacement)
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
                        last.Value += parsedInput.Sequence[i];
                    }
                    else
                    {
                        sequence.Add(new SequenceItem { Replaceable = false, Value = parsedInput.Sequence[i].ToString() });
                    }
                }
            }

            return new Problem { Replacements = replacements, Sequence = sequence };
        }
    }
}