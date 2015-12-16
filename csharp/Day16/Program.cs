using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Net.Security;
using System.Runtime.CompilerServices;
using System.Runtime.Remoting.Messaging;
using System.Security.Principal;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace AdventOfCode.Day16
{
    public class Program
    {
        public void Run()
        {
            // parse the hints
            var tickerHints = new HashSet<Hint>();
            using (var fileStream = new FileStream(@"Day16\hints.txt", FileMode.Open, FileAccess.Read))
            using(var reader = new StreamReader(fileStream))
            {
                string line;
                while ((line = reader.ReadLine()) != null)
                {
                    var match = Regex.Match(line, @"^(?<Name>.+?): (?<Count>\d+)$");
                    tickerHints.Add(new Hint(match.Groups["Name"].Value.Trim(), int.Parse(match.Groups["Count"].Value)));
                }
            }

            // parse the aunts
            var aunts = new List<Aunt>();
            using (var fileStream = new FileStream(@"Day16\input.txt", FileMode.Open, FileAccess.Read))
            using (var reader = new StreamReader(fileStream))
            {
                string line;
                while ((line = reader.ReadLine()) != null)
                {
                    var pieces = line.Split(new[] {":"}, 2, StringSplitOptions.None);
                    var linePieces = pieces[1].Split(',').Select(p => p.Trim());
                    var auntHints = new HashSet<Hint>();
                    foreach (var hintPiece in linePieces)
                    {
                        var hintPieces = hintPiece.Split(':');
                        auntHints.Add(new Hint(hintPieces[0], int.Parse(hintPieces[1])));
                    }
                    
                    aunts.Add(new Aunt(pieces[0], auntHints));
                }
            }

            // Part 1
            var part1Answer = aunts.First(a => a.Hints.IsSubsetOf(tickerHints));
            Console.WriteLine($"Part 1 answer: {part1Answer.Name}");

            // Part 2
            foreach (var aunt in aunts)
            {
                bool isFound = true;
                foreach (var tickerHint in tickerHints)
                {
                    var auntHint = aunt.Hints.FirstOrDefault(h => h.Name == tickerHint.Name);
                    if (auntHint == null)
                    {
                        continue;
                    }

                    if (tickerHint.Name == "cats" || tickerHint.Name == "trees")
                    {
                        if (auntHint.Count <= tickerHint.Count)
                        {
                            isFound = false;
                            break;
                        }
                    }
                    else if (tickerHint.Name == "pomeranians" || tickerHint.Name == "goldfish")
                    {
                        if (auntHint.Count >= tickerHint.Count)
                        {
                            isFound = false;
                            break;
                        }
                    }
                    else if (tickerHint.Count != auntHint.Count)
                    {
                        isFound = false;
                        break;
                    }
                }

                if (isFound)
                {
                   Console.WriteLine($"Part 2 answer: {aunt.Name}");
                }
            }
            
        }
    }

    public class Aunt
    {
        public Aunt(string name, ISet<Hint> hints)
        {
            Name = name;
            Hints = hints;
        }

        public string Name { get; }
        public ISet<Hint> Hints { get; }
    }

    public class Hint
    {
        public Hint(string name, int count)
        {
            Name = name;
            Count = count;
        }

        public string Name { get; }
        public int Count { get; }

        public override bool Equals(object obj)
        {
            var other = obj as Hint;
            if (other == null)
            {
                return false;
            }

            return Name == other.Name && Count == other.Count;
        }

        public override int GetHashCode()
        {
            return $"{Name}-{Count}".GetHashCode();
        }
    }
}
