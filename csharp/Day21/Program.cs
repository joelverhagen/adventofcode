using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;

namespace AdventOfCode.Day21
{
    public class Program
    {
        public void Run()
        {
            var parser = new Parser();
            var items = parser.ParseItemsFile(@"Day21\shop.txt").ToArray();
            var boss = parser.ParseCharacterFile(@"Day21\input.txt");
            var hero = new Character {HitPoints = 100, Damage = 0, Armor = 0};

            var enumerator = new Enumerator();
            int part1Answer = enumerator
                .GetSets(hero, boss, true, items)
                .Select(combination => combination.Sum(i => i.Cost))
                .Min();
            Console.WriteLine($"Part 1 answer: {part1Answer}");

            int part2Answer = enumerator
                .GetSets(hero, boss, false, items)
                .Select(combination => combination.Sum(i => i.Cost))
                .Max();
            Console.WriteLine($"Part 2 answer: {part2Answer}");
        }
    }

    public class Enumerator
    {
        public IEnumerable<Item[]> GetSets(Character hero, Character boss, bool heroWins, IEnumerable<Item> items)
        {
            var itemArray = items.ToArray();
            return GetSets(hero, boss, heroWins, new Item[0], itemArray);
        }

        private IEnumerable<Item[]> GetSets(Character hero, Character boss, bool heroWins, Item[] prefix, Item[] remaining)
        {
            for (int i = 0; i < remaining.Length; i++)
            {
                var newPrefix = prefix.Concat(remaining.Skip(i).Take(1)).ToArray();

                if (IsImpossibleSet(newPrefix))
                {
                    yield break;
                }

                if (ValidItems(newPrefix))
                {
                    if (heroWins)
                    {
                        if (hero.WithItems(newPrefix).CanBeat(boss))
                        {
                            yield return newPrefix;
                        }
                    }
                    else if (boss.CanBeat(hero.WithItems(newPrefix)))
                    {
                        yield return newPrefix;
                    }
                }

                var newRemaining = remaining.Take(i).Concat(remaining.Skip(i + 1)).ToArray();
                foreach (var inner in GetSets(hero, boss, heroWins, newPrefix, newRemaining))
                {
                    yield return inner;
                }
            }
        }

        private bool IsImpossibleSet(Item[] items)
        {
            return items.Count(i => i.Type == ItemType.Armor) > 1 ||
                   items.Count(i => i.Type == ItemType.Weapons) > 1 ||
                   items.Count(i => i.Type == ItemType.Rings) > 2;
        }

        private bool ValidItems(Item[] items)
        {
            return items.Count(i => i.Type == ItemType.Armor) <= 1 &&
                   items.Count(i => i.Type == ItemType.Weapons) == 1 &&
                   items.Count(i => i.Type == ItemType.Rings) <= 2;
        }
    }

    public class Parser
    {
        public IEnumerable<Item> ParseItemsFile(string path)
        {
            ItemType? currentType = null;
            foreach (var line in File.ReadAllLines(path))
            {
                var trimmedLine = line.Trim();
                var typeMatch = Regex.Match(trimmedLine, @"^(?<Type>\w+):");
                ItemType parsedType;
                if (typeMatch.Success && Enum.TryParse(typeMatch.Groups["Type"].Value, out parsedType))
                {
                    currentType = parsedType;
                    continue;
                }

                if (trimmedLine.Length > 0 && currentType != null)
                {
                    var match = Regex.Match(trimmedLine, @"^(?<Name>.+?)\s+(?<Cost>\d+)\s+(?<Damage>\d+)\s+(?<Armor>\d+)$");
                    if (match.Success)
                    {
                        yield return new Item
                        {
                            Name = match.Groups["Name"].Value.Trim(),
                            Cost = int.Parse(match.Groups["Cost"].Value),
                            Armor = int.Parse(match.Groups["Armor"].Value),
                            Damage = int.Parse(match.Groups["Damage"].Value),
                            Type = currentType.Value
                        };
                    }
                }
            }
        }

        public Character ParseCharacterFile(string path)
        {
            var output = new Character();
            foreach (var line in File.ReadAllLines(path))
            {
                var pieces = line.Split(':');
                switch (pieces[0].Trim())
                {
                    case "Hit Points":
                        output.HitPoints = int.Parse(pieces[1]);
                        break;
                    case "Damage":
                        output.Damage = int.Parse(pieces[1]);
                        break;
                    case "Armor":
                        output.Armor = int.Parse(pieces[1]);
                        break;
                }
            }

            return output;
        }
    }

    public class Character
    {
        public int HitPoints { get; set; }
        public int Damage { get; set; }
        public int Armor { get; set; }

        public Character Clone()
        {
            return new Character {Armor = Armor, Damage = Damage, HitPoints = HitPoints};
        }

        public Character WithItems(IEnumerable<Item> items)
        {
            var self = Clone();
            foreach (var item in items)
            {
                self.Damage += item.Damage;
                self.Armor += item.Armor;
            }

            return self;
        }

        public bool CanBeat(Character other)
        {
            var self = Clone();
            other = other.Clone();
            
            bool selfTurn = true;
            while (self.HitPoints > 0 && other.HitPoints > 0)
            {
                if (selfTurn)
                {
                    other.HitPoints -= Math.Max(1, self.Damage - other.Armor);
                }
                else
                {
                    self.HitPoints -= Math.Max(1, other.Damage - self.Armor);
                }

                selfTurn = !selfTurn;
            }

            return self.HitPoints > 0;
        }
    }

    public class Item
    {
        public string Name { get; set; }
        public int Cost { get; set; }
        public int Damage { get; set; }
        public int Armor { get; set; }
        public ItemType Type { get; set; }

        public override string ToString()
        {
            return Name;
        }
    }

    public enum ItemType
    {
        Weapons, Armor, Rings
    }
}
