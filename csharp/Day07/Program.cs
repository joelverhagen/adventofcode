using System;
using System.Linq;

namespace AdventOfCode.Day07
{
    public class Program
    {
        public void Run()
        {
            var parser = new WireParser();
            var wireSequence = parser.ParseFile(@"Day7\input.txt").ToArray();
            // var wireSequence = parser.ParseLines("123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i").ToArray();
            
            Console.WriteLine("Parsed wires:");
            foreach (var wire in wireSequence)
            {
                Console.WriteLine($" - {wire}");
            }

            var processor = new WireProcessor();
            var result = processor.Process(wireSequence);

            Console.WriteLine();
            Console.WriteLine("Values:");
            foreach (var pair in result.Values.OrderBy(p => p.Key))
            {
                Console.WriteLine($" - {pair.Key}: {pair.Value}");
            }

            Console.WriteLine();
            Console.WriteLine("Unresolved wires:");
            foreach (var wire in result.UnresolvedWires)
            {
                Console.WriteLine($" - {wire}");
            }
        }
    }
}
