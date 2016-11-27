using System;
using System.Collections.Generic;
using System.IO;
using System.Text.RegularExpressions;

namespace AdventOfCode.Day13
{
    public class HappinessParser
    {
        public IEnumerable<Happiness> ParseFile(string path)
        {
            var happiness = new List<Happiness>();
            using (var fileStream = new FileStream(path, FileMode.Open, FileAccess.Read))
            using (var reader = new StreamReader(fileStream))
            {
                string line;
                while ((line = reader.ReadLine()) != null)
                {
                    happiness.Add(ParseLine(line));
                }
            }

            return happiness;
        }

        public Happiness ParseLine(string line)
        {
            var match = Regex.Match(line, @"(?<Person>.+?) would (?<Sign>gain|lose) (?<Value>\d+) happiness units by sitting next to (?<Neighbor>.+?)\.$");
            if (!match.Success)
            {
                throw new FormatException($"The following line could not be parsed: {line}");
            }

            int sign = match.Groups["Sign"].Value == "gain" ? 1 : -1;
            int delta = sign*int.Parse(match.Groups["Value"].Value);
            return new Happiness
            {
                Person = match.Groups["Person"].Value,
                Delta = delta,
                Neighbor = match.Groups["Neighbor"].Value
            };
        }
    }
}