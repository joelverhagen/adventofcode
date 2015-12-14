using System.Collections.Generic;

namespace AdventOfCode.Day09
{
    public class Location
    {
        public string Name { get; set; }
        public IDictionary<string, Distance> Destinations { get; set; }
    }

    public class Distance
    {
        public int Value { get; set; }
        public Location Destination { get; set; }
    }
}