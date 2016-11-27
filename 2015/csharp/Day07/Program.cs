using System;
using System.Linq;

namespace AdventOfCode.Day07
{
    public class Program
    {
        public void Run()
        {
            var parser = new WireParser();
            var processor = new WireProcessor();
            var part1Wires = parser.ParseFile(@"Day07\input.txt").ToArray();
            
            // Part 1
            var part1Result = processor.Process(part1Wires);
            if (part1Result.UnresolvedWires.Any())
            {
                Console.WriteLine("Some wires in part 1 could not be resolved:");
                foreach (var wire in part1Result.UnresolvedWires)
                {
                    Console.WriteLine($" - {wire}");
                }

                return;
            }

            Console.WriteLine($"Part 1 answer: {part1Result.Values["a"]}");

            // Part 2
            var part2Wires = part1Wires
                .Select(w => w.Label == "b" ? new Wire {Label = "b", Signal = new ValueSignal {Value = part1Result.Values["a"]}} : w)
                .ToArray();
            var part2Result = processor.Process(part2Wires);
            if (part2Result.UnresolvedWires.Any())
            {
                Console.WriteLine("Some wires in part 2 could not be resolved:");
                foreach (var wire in part2Result.UnresolvedWires)
                {
                    Console.WriteLine($" - {wire}");
                }

                return;
            }

            Console.WriteLine($"Part 2 answer: {part2Result.Values["a"]}");

        }
    }
}
