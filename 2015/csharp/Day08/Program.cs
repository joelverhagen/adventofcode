using System;

namespace AdventOfCode.Day08
{
    public class Program
    {
        public void Run()
        {
            // Part 1
            {
                var parser = new StringParser();
                var parsedStrings = parser.ParseFile(@"Day08\input.txt");

                int literalCount = 0;
                int parsedCount = 0;
                foreach (var s in parsedStrings)
                {
                    literalCount += s.Input.Length;
                    parsedCount += s.Output.Length;
                }

                Console.WriteLine("Part 1: ");
                Console.WriteLine($"- Literal count: {literalCount}");
                Console.WriteLine($"- Parsed count: {parsedCount}");
                Console.WriteLine($"- Difference: {literalCount - parsedCount}");
            }

            // Part 2
            {
                var escaper = new StringEscaper();
                var escapedStrings = escaper.EscapeFile(@"Day08\input.txt");

                int literalCount = 0;
                int escapedCount = 0;
                foreach (var s in escapedStrings)
                {
                    literalCount += s.Input.Length;
                    escapedCount += s.Output.Length;
                }

                Console.WriteLine();
                Console.WriteLine("Part 2: ");
                Console.WriteLine($"- Literal count: {literalCount}");
                Console.WriteLine($"- Escaped count: {escapedCount}");
                Console.WriteLine($"- Difference: {escapedCount - literalCount}");
            }
        }
    }
}
