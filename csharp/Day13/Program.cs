using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using AdventOfCode.Day08;
using Newtonsoft.Json;

namespace AdventOfCode.Day13
{
    public class Program
    {
        public void Run()
        {
            var happiness = new HappinessParser().ParseFile(@"Day13\input.txt").ToArray();
            var people = new PeopleBuilder().Build(happiness);
            var seating = new SeatingEnumerator().Enumerate(people.Keys);
            var evaluator = new SeatingEvaluator();
            var arrangements = seating.Select(s => evaluator.EvaluateHappiness(people, s)).OrderBy(a => a.TotalHappiness);

            int lastHappiness = int.MinValue;
            foreach (var seatingArrangement in arrangements)
            {
                if (seatingArrangement.TotalHappiness != lastHappiness)
                {
                    Console.WriteLine($"{string.Join(", ", seatingArrangement.Order)} -> {seatingArrangement.TotalHappiness}");
                    lastHappiness = seatingArrangement.TotalHappiness;
                }
            }
        }
    }

    public class SeatingEvaluator
    {
        public SeatingArrangement EvaluateHappiness(IDictionary<string, Person> people, string[] arrangement)
        {
            int total = 0;
            for (int i = 0; i < arrangement.Length; i++)
            {
                var happiness = people[arrangement[i]].Happiness;

                var left = arrangement[(i + arrangement.Length - 1) % arrangement.Length];
                var right = arrangement[(i + 1)%arrangement.Length];

                total += happiness[left] + happiness[right];
            }

            return new SeatingArrangement
            {
                Order = arrangement,
                TotalHappiness = total
            };
        }
    }

    public class SeatingArrangement
    {
        public string[] Order { get; set; }
        public int TotalHappiness { get; set; }
    }

    public class SeatingEnumerator
    {
        public IEnumerable<string[]> Enumerate(IEnumerable<string> people)
        {
            var enumeratedPeople = people.ToArray();
            return EnumerateInternal(enumeratedPeople.Take(1).ToArray(), enumeratedPeople.Skip(1).ToArray());
        }

        private IEnumerable<string[]> EnumerateInternal(string[] prefix, string[] remaining)
        {
            if (remaining.Length == 0)
            {
                yield return prefix;
            }

            for (int i = 0; i < remaining.Length; i++)
            {
                var inner = EnumerateInternal(
                    Join(prefix, new[] {remaining[i]}),
                    Join(remaining.Take(i).ToArray(), remaining.Skip(i + 1).ToArray()));

                foreach (var result in inner)
                {
                    yield return result;
                }
            }
        }

        private string[] Join(string[] a, string[] b)
        {
            var output = new string[a.Length + b.Length];
            Array.Copy(a, 0, output, 0, a.Length);
            Array.Copy(b, 0, output, a.Length, b.Length);
            return output;
        }
    }

    public class PeopleBuilder
    {
        public IDictionary<string, Person> Build(IEnumerable<Happiness> happiness)
        {
            var people = new Dictionary<string, Person>();
            foreach (var current in happiness)
            {
                Person person;
                if (!people.TryGetValue(current.Person, out person))
                {
                    person = new Person {Name = current.Person, Happiness = new Dictionary<string, int>()};
                    people[person.Name] = person;
                }

                person.Happiness[current.Neighbor] = current.Delta;
            }

            return people;
        }
    }
}
