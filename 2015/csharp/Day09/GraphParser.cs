using System;
using System.Collections.Generic;
using System.IO;
using System.Text.RegularExpressions;

namespace AdventOfCode.Day09
{
    public class GraphParser
    {
        public IDictionary<string, Location> ParseFile(string path)
        {
            var graph = new Dictionary<string, Location>();
            using (var fileStream = new FileStream(path, FileMode.Open, FileAccess.Read))
            using (var reader = new StreamReader(fileStream))
            {
                string line;
                while ((line = reader.ReadLine()) != null)
                {
                    ParseLine(graph, line);
                }
            }

            return graph;
        }

        private void ParseLine(IDictionary<string, Location> graph, string line)
        {
            var match = Regex.Match(line, @"^\s*(?<From>.+?)\s+to\s+(?<To>.+?)\s*=\s*(?<Distance>\d+)$");
            if (!match.Success)
            {
                throw new FormatException($"The following graph line could not be parsed: {line}");
            }

            Location from = GetLocation(graph, match.Groups["From"].Value);
            Location to = GetLocation(graph, match.Groups["To"].Value);
            int distance = int.Parse(match.Groups["Distance"].Value);

            // this is a digraph
            from.Destinations[to.Name] = new Distance {Destination = to, Value = distance};
            to.Destinations[from.Name] = new Distance {Destination = from, Value = distance};
        }

        private Location GetLocation(IDictionary<string, Location> graph, string name)
        {
            Location location;
            if (!graph.TryGetValue(name, out location))
            {
                location = new Location {Name = name, Destinations = new Dictionary<string, Distance>()};
                graph[name] = location;
            }

            return location;
        }
    }
}