using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Runtime.Remoting.Messaging;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace AdventOfCode.Day15
{
    public class Program
    {
        public void Run()
        {
            var ingredients = new IngredientParser().ParseFile(@"Day15\input.txt").ToArray();

            var part1Answer = new IngredientCombiner()
                .Enumerate(ingredients, 100)
                .Select(c => new { Combination = c, Score = c.GetScore()})
                .OrderByDescending(c => c.Score)
                .First();
            Console.WriteLine($"Part 1 answer: {part1Answer.Score} ({string.Join(", ", part1Answer.Combination.Select(p => $"{p.Value} {p.Key}"))})");

            var part2Answer = new IngredientCombiner()
                .Enumerate(ingredients, 100)
                .Where(c => c.GetCalories() == 500)
                .Select(c => new { Combination = c, Score = c.GetScore() })
                .OrderByDescending(c => c.Score)
                .First();
            Console.WriteLine($"Part 2 answer: {part2Answer.Score} ({string.Join(", ", part2Answer.Combination.Select(p => $"{p.Value} {p.Key}"))})");
        }
    }

    public class IngredientCombiner
    {
        public IEnumerable<IngredientCombination> Enumerate(IEnumerable<Ingredient> ingredients, int remaining)
        {
            return Enumerate(new IngredientCombination(), ingredients.ToArray(), remaining);
        } 

        private IEnumerable<IngredientCombination> Enumerate(IngredientCombination combination, Ingredient[] ingredients, int remaining)
        {
            if (ingredients.Length == 1)
            {
                combination = combination.Clone();
                combination[ingredients[0]] = remaining;
                yield return combination;
                yield break;
            }

            for (int fixedIndex = 0; fixedIndex < ingredients.Length; fixedIndex++)
            {
                var fixedIngredient = ingredients[fixedIndex];
                var variableIngredients = ingredients.Except(new[] { ingredients[fixedIndex]}).ToArray();
                for (int i = 0; i <= remaining; i++)
                {
                    combination[fixedIngredient] = i;
                    foreach (var c in Enumerate(combination, variableIngredients, remaining - i))
                    {
                        yield return c;
                    }
                }
            }
        }
    }

    public class IngredientCombination : Dictionary<Ingredient, int>
    {
        public int GetCalories()
        {
            return this.Sum(p => p.Key.Calories*p.Value);
        }

        public int GetScore()
        {
            var getScores = new Func<Ingredient, int>[]
            {
                i => i.Capacity,
                i => i.Durability,
                i => i.Flavor,
                i => i.Texture
            };

            return getScores
                .Select(GetAttributeScore)
                .Select(i => Math.Max(0, i))
                .Aggregate(1, (a, b) => a*b);
        }

        private int GetAttributeScore(Func<Ingredient, int> getValue)
        {
            return this.Sum(pair => getValue(pair.Key)*pair.Value);
        }

        public IngredientCombination Clone()
        {
            var output = new IngredientCombination();
            foreach (var pair in this)
            {
                output[pair.Key] = pair.Value;
            }

            return output;
        }
    }

    public class Ingredient
    {
        public Ingredient(string name)
        {
            Name = name;
        }

        public string Name { get; }
        public int Capacity { get; set; }
        public int Durability { get; set; }
        public int Flavor { get; set; }
        public int Texture { get; set; }
        public int Calories { get; set; }

        public override int GetHashCode()
        {
            return Name.GetHashCode();
        }

        public override string ToString()
        {
            return Name;
        }

        public override bool Equals(object obj)
        {
            var other = obj as Ingredient;
            if (other == null)
            {
                return false;
            }

            return Name == other.Name;
        }
    }

    public class IngredientParser
    {
        public IEnumerable<Ingredient> ParseFile(string path)
        {
            var ingredients = new List<Ingredient>();
            using (var fileStream = new FileStream(path, FileMode.Open, FileAccess.Read))
            using (var reader = new StreamReader(fileStream))
            {
                string line;
                while ((line = reader.ReadLine()) != null)
                {
                    ingredients.Add(ParseLine(line));
                }
            }

            return ingredients;
        } 
        public Ingredient ParseLine(string line)
        {
            var pieces = line.Split(new[] { ":"}, StringSplitOptions.None);
            var ingredient = new Ingredient(pieces[0].Trim());
            var attributes = pieces[1].Split(new[] {","}, StringSplitOptions.RemoveEmptyEntries);
            foreach (var attribute in attributes)
            {
                var attributePieces = attribute.Split(new[] {" "}, StringSplitOptions.RemoveEmptyEntries);
                var name = attributePieces[0].Trim();
                var value = int.Parse(attributePieces[1]);
                switch (name.ToLowerInvariant())
                {
                    case "capacity":
                        ingredient.Capacity = value;
                        break;
                    case "durability":
                        ingredient.Durability = value;
                        break;
                    case "flavor":
                        ingredient.Flavor = value;
                        break;
                    case "texture":
                        ingredient.Texture = value;
                        break;
                    case "calories":
                        ingredient.Calories = value;
                        break;
                    default:
                        throw new FormatException($"There was an unexpected attribute name '{name}' in the line: {line}");
                }
            }

            return ingredient;
        }
    }
}
