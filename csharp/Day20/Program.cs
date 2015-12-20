using System;
using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode.Day20
{
    public class Program
    {
        public void Run()
        {
            // Part 1
            {
                var input = 33100000;
                var sum = 0;
                var house = 0;
                while (sum <= input)
                {
                    house++;
                    sum = GetFactors(house).Sum() * 10;
                }

                Console.WriteLine(house);
            }

            // Part 2
            {
                var visitCount = new Dictionary<int, int>();
                var input = 33100000;
                var sum = 0;
                var house = 0;
                while (sum <= input)
                {
                    house++;
                    var factors = GetFactors(house).OrderBy(f => f).ToArray();
                    var currentSum = 0;
                    foreach (var factor in factors)
                    {
                        int count;
                        if (!visitCount.TryGetValue(factor, out count))
                        {
                            count = 1;
                        }
                        else
                        {
                            count++;
                        }
                        visitCount[factor] = count;

                        if (count <= 50)
                        {
                            currentSum += factor;
                        }
                    }

                    sum = currentSum * 11;
                }

                Console.WriteLine(house);
            }
        }

        private IEnumerable<int> GetFactors(int number)
        {
            var max = (int)Math.Sqrt(number);
            for (var factor = 1; factor <= max; factor++)
            {
                if (number % factor == 0)
                {
                    yield return factor;
                    if (factor != number / factor)
                    {
                        yield return number / factor;
                    }
                }
            }
        }
    }
}
