using System;
using System.IO;
using System.Text.RegularExpressions;

namespace AdventOfCode.Day22
{
    public class Program
    {
        public void Run()
        {
            var player = new Player { Armor = 0, HitPoints = 50, Mana = 500 };
            var boss = new Boss { HitPoints = 71, Damage = 10 };
            var factory = new CombatStateFactory(player, boss);
            var spellBook = new SpellBook();
            var combat = new Combat(factory, spellBook);
            var enumerator = new SpellEnumerator(combat, new SpellBook());

            foreach (var result in enumerator.Enumerate())
            {
                Console.WriteLine($"{result.State.ManaSpent} ({string.Join(", ", result.Spells)})");  
            }
        }
    }
}

