using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Numerics;

namespace AdventOfCode.Day25
{
    public class Program
    {
        public void Run()
        {
            var enumerator = new Enumerator();
            foreach (var n in enumerator.Enumerate(new BigInteger(20151125)))
            {
                if (n.Row == 3010 && n.Column == 3019)
                {
                    Console.WriteLine($"Part 1 answer: {n.Value}");
                }
            }
        }
    }

    public class Enumerator
    {
        public IEnumerable<Node> Enumerate(BigInteger start)
        {
            var nextNode = new Node {Row = 1, Column = 1, Value = start};
            do
            {
                var lastNode = new Node {Row = nextNode.Row, Column = nextNode.Column, Value = nextNode.Value};
                yield return lastNode;
                nextNode.Row -= 1;
                nextNode.Column += 1;
                if (nextNode.Row < 1)
                {
                    nextNode.Row = lastNode.Column + 1;
                    nextNode.Column = 1;
                }

                nextNode.Value *= 252533;
                nextNode.Value %= 33554393;
            }
            while (true);
        }
    }

    public class Node
    {
        public int Row { get; set; }
        public int Column { get; set; }
        public BigInteger Value { get; set; }
    }
}
