using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AdventOfCode.Day13
{
    public class Person
    {
        public string Name { get; set; }
        public IDictionary<string, int> Happiness { get; set; }
    }

    public class Happiness
    {
        public string Person { get; set; }
        public int Delta { get; set; }
        public string Neighbor { get; set; }

        public override string ToString()
        {
            return $"{Person} would {(Delta > 0 ? "gain" : "lose")} {Math.Abs(Delta)} happiness units by sitting next to {Neighbor}.";
        }
    }
}
