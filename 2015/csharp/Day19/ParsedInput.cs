using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace AdventOfCode.Day19
{
    public class ParsedInput
    {
        public IList<KeyValuePair<string, string>> Replacements { get; set; }
        public string Sequence { get; set; }

        public static ParsedInput ReadFromFile(string path)
        {
            // parse replacements
            var replacements = new List<KeyValuePair<string, string>>();
            var readingReplacements = true;
            var sequence = string.Empty;
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
                    replacements.Add(new KeyValuePair<string, string>(pieces[0], pieces[1]));
                }
                else
                {
                    sequence = line.Trim();
                }
            }

            return new ParsedInput { Replacements = replacements, Sequence = sequence };
        }
    }
}