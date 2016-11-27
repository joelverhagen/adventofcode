using System.Linq;

namespace AdventOfCode.Day19.Part2
{
    public class Problem
    {
        public ILookup<string, string> ReplacementRules { get; set; }
        public string Sequence { get; set; }

        public static Problem ReadFromFile(string path)
        {
            var parsedInput = ParsedInput.ReadFromFile(path);
            return new Problem
            {
                ReplacementRules = parsedInput.Replacements.ToLookup(p => p.Value, p => p.Key),
                Sequence = parsedInput.Sequence
            };
        }
    }
}
