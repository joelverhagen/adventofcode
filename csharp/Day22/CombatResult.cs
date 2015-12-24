namespace AdventOfCode.Day22
{
    public class CombatResult
    {
        public CombatResultType Type { get; set; }
        public CombatState State { get; set; }
        public SpellType[] Spells { get; set; }
    }
}