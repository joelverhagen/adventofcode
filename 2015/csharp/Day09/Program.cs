﻿using System;
using System.Collections.Generic;
using System.Data;
using System.Linq;

namespace AdventOfCode.Day09
{
    public class Program
    {
        public void Run()
        {
            var parser = new GraphParser();
            var graph = parser.ParseFile(@"Day09\input.txt");
            var finder = new PathFinder();

            {
                Console.WriteLine("Part 1:");
                var distances = finder.EnumerateAllPaths(graph);
                var shortest = distances.OrderBy(p => p.Distance).First();
                Console.WriteLine($"{string.Join(" -> ", shortest.Locations)} = {shortest.Distance}");
            }

            {
                Console.WriteLine();
                Console.WriteLine("Part 2:");
                var distances = finder.EnumerateAllPaths(graph);
                var longest = distances.OrderByDescending(p => p.Distance).First();
                Console.WriteLine($"{string.Join(" -> ", longest.Locations)} = {longest.Distance}");
            }
        }
    }

    public class PathFinder
    {
        public IEnumerable<Path> EnumerateAllPaths(IDictionary<string, Location> graph)
        {
            foreach (var start in graph.Values.OrderBy(p => p.Name).ToArray())
            {
                var remaining = new Stack<WorkingPath>();
                remaining.Push(new WorkingPath {Distance = 0, Path = new List<Location> {start}, Visited = new HashSet<string> {start.Name} });

                while (remaining.Any())
                {
                    var current = remaining.Pop();
                    foreach (var edge in current.Path.Last().Destinations.Values)
                    {
                        if (current.Visited.Contains(edge.Destination.Name))
                        {
                            continue;
                        }

                        var next = current.Clone();
                        next.Path.Add(edge.Destination);
                        next.Visited.Add(edge.Destination.Name);
                        next.Distance += edge.Value;

                        remaining.Push(next);
                    }

                    if (current.Visited.Count == graph.Count)
                    {
                        yield return new Path
                        {
                            Distance = current.Distance,
                            Locations = current.Path.Select(l => l.Name).ToArray()
                        };
                    }
                }
            }
        }

        private class WorkingPath
        {
            public ISet<string> Visited { get; set; }
            public IList<Location> Path { get; set; }
            public int Distance { get; set; }

            public WorkingPath Clone()
            {
                return new WorkingPath
                {
                    Distance = Distance,
                    Path = Path.ToList(),
                    Visited = new HashSet<string>(Visited)
                };
            }
        }
    }

    public class Path
    {
        public IEnumerable<string> Locations { get; set; }
        public int Distance { get; set; }
    }
}
