using System;
using System.Collections.Generic;
using System.Data;
using System.IO;
using System.Linq;
using System.Text;

namespace AdventOfCode.Day18
{
    public class Program
    {
        public void Run()
        {
            var parser = new Parser();
            var processor = new Processor();

            // Part 1
            var grid1 = parser.ReadFile(@"Day18\input.txt");
            for (int i = 1; i <= 100; i++)
            {
                processor.Step(grid1);
            }

            var part1Answer = grid1.Sum(r => r.Sum(c => c ? 1 : 0));
            Console.WriteLine($"Part 1 answer: {part1Answer}");


            // Part 2
            var grid2 = parser.ReadFile(@"Day18\input.txt");
            processor.TurnOnCorners(grid2);
            for (int i = 1; i <= 100; i++)
            {
                processor.Step(grid2);
                processor.TurnOnCorners(grid2);
            }

            var part2Answer = grid2.Sum(r => r.Sum(c => c ? 1 : 0));
            Console.WriteLine($"Part 2 answer: {part2Answer}");
        }
    }

    public class Processor
    {
        public void Step(bool[][] grid)
        {
            var previousState = grid.Select(r => r.ToArray()).ToArray();
            for (int r = 0; r < previousState.Length; r++)
            {
                for (int c = 0; c < previousState[r].Length; c++)
                {
                    grid[r][c] = GetNewState(previousState, r, c);
                }
            }
        }

        public void TurnOnCorners(bool[][] grid)
        {
            int h = grid.Length - 1;
            int w = grid[0].Length - 1;
            grid[0][0] = true;
            grid[0][w] = true;
            grid[h][0] = true;
            grid[h][w] = true;
        }

        private bool GetNewState(bool[][] previousState, int r, int c)
        {
            int trueNeighbors = CountTrueNeighbors(previousState, r, c);
            if (previousState[r][c])
            {
                return trueNeighbors >= 2 && trueNeighbors <= 3;
            }

            return trueNeighbors == 3;
        }

        private int CountTrueNeighbors(bool[][] state, int r, int c)
        {
            return EnumerateNeighbors(state.Length, state[r].Length, r, c)
                .Select(t => state[t.Item1][t.Item2] ? 1 : 0)
                .Sum();
        } 

        private IEnumerable<Tuple<int, int>> EnumerateNeighbors(int height, int width, int r, int c)
        {
            for (int rC = r - 1; rC <= r + 1; rC++)
            {
                if (rC < 0 || rC >= height)
                {
                    continue;
                }

                for (int cC = c - 1; cC <= c + 1; cC++)
                {
                    if (cC < 0 || cC >= width)
                    {
                        continue;
                    }

                    if (rC == r && c == cC)
                    {
                        continue;
                    }

                    yield return new Tuple<int, int>(rC, cC);
                }
            }
        } 
    }

    public class Parser
    {
        public string GetString(bool[][] grid)
        {
            var builder = new StringBuilder();
            foreach (var row in grid)
            {
                foreach (var cell in row)
                {
                    builder.Append(cell ? '#' : '.');
                }

                builder.AppendLine();
            }

            return builder.ToString();
        }

        public bool[][] ReadFile(string path)
        {
            var rows = new List<bool[]>();
            using (var stream = new FileStream(path, FileMode.Open, FileAccess.Read))
            using(var reader = new StreamReader(stream))
            {
                string line;
                while ((line = reader.ReadLine()) != null)
                {
                    var row = line.Select(c => c == '#').ToArray();
                    rows.Add(row);
                }
            }

            return rows.ToArray();
        }
    }
}
