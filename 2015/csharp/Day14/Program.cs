using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace AdventOfCode.Day14
{
    public class Program
    {
        public void Run()
        {
            Part1();
            Part2();
        }

        private void Part1()
        {
            var states = File
                .ReadAllLines(@"Day14\input.txt")
                .Select(ParseLine)
                .Select(r => new ReindeerState { Reindeer = r, IsFlying = true, FlyingRemaining = r.FlyDuration })
                .ToArray();

            for (int i = 0; i < 2503; i++)
            {
                foreach (var state in states)
                {
                    if (state.IsFlying)
                    {
                        state.Distance += state.Reindeer.Speed;
                        state.FlyingRemaining -= 1;

                        if (state.FlyingRemaining == 0)
                        {
                            state.IsFlying = false;
                            state.RestingRemaining = state.Reindeer.RestDuration;
                        }
                    }
                    else
                    {
                        state.RestingRemaining -= 1;
                        if (state.RestingRemaining == 0)
                        {
                            state.IsFlying = true;
                            state.FlyingRemaining = state.Reindeer.FlyDuration;
                        }
                    }
                }
            }

            var first = states.OrderByDescending(r => r.Distance).First();
            Console.WriteLine($"Part 1 answer: {first.Distance}");
        }

        private void Part2()
        {
            var states = File
                .ReadAllLines(@"Day14\input.txt")
                .Select(ParseLine)
                .Select(r => new ReindeerState { Reindeer = r, IsFlying = true, FlyingRemaining = r.FlyDuration })
                .ToArray();

            for (int i = 0; i < 2503; i++)
            {
                foreach (var state in states)
                {
                    if (state.IsFlying)
                    {
                        state.Distance += state.Reindeer.Speed;
                        state.FlyingRemaining -= 1;

                        if (state.FlyingRemaining == 0)
                        {
                            state.IsFlying = false;
                            state.RestingRemaining = state.Reindeer.RestDuration;
                        }
                    }
                    else
                    {
                        state.RestingRemaining -= 1;
                        if (state.RestingRemaining == 0)
                        {
                            state.IsFlying = true;
                            state.FlyingRemaining = state.Reindeer.FlyDuration;
                        }
                    }
                }

                var leaders = states.GroupBy(r => r.Distance).OrderByDescending(g => g.Key).First();
                foreach (var leader in leaders)
                {
                    leader.Points++;
                }
            }

            var first = states.OrderByDescending(r => r.Points).First();
            Console.WriteLine($"Part 2 answer: {first.Points}");
        }

        private Reindeer ParseLine(string line)
        {
            var match = Regex.Match(
                line,
                @"^(?<Name>.+?) can fly (?<Speed>\d+) km/s for (?<FlyDuration>\d+) seconds, but then must rest for (?<RestDuration>\d+) seconds\.$");

            if (!match.Success)
            {
                throw new FormatException($"The reindeer could not be parsed: {line}");
            }

            return new Reindeer
            {
                Name = match.Groups["Name"].Value,
                Speed = int.Parse(match.Groups["Speed"].Value),
                FlyDuration = int.Parse(match.Groups["FlyDuration"].Value),
                RestDuration = int.Parse(match.Groups["RestDuration"].Value)
            };
        }
    }

    public class ReindeerState
    {
        public Reindeer Reindeer { get; set; }
        public bool IsFlying { get; set; }
        public int Distance { get; set; }
        public int FlyingRemaining { get; set; }
        public int RestingRemaining { get; set; }
        public int Points { get; set; }
    }

    public class Reindeer
    {
        public string Name { get; set; }
        public int Speed { get; set; }
        public int FlyDuration { get; set; }
        public int RestDuration { get; set; }
    }
}
