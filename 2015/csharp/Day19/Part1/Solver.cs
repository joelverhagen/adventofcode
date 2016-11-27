using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode.Day19.Part1
{
    public class Solver
    {
        public int GetAnswer(string path)
        {
            var problem = Problem.ReadFromFile(path);
            var answer = EnumerateReplacedSequences(problem).Count();
            return answer;
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
    }
}