using System.Collections.Generic;

namespace AdventOfCode.Day22
{
    public class CombatState
    {
        public bool PlayerTurn { get; set; }
        public Boss Boss { get; set; }
        public Player Player { get; set; }
        public int ManaSpent { get; set; }
        public IDictionary<SpellType, CastedSpell> CastedSpells { get; set; }
    }
}