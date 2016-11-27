using System;
using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode.Day22
{
    public class SpellBook
    {
        private static readonly IDictionary<SpellType, ISpell> Spells;
        private static readonly IDictionary<SpellType, int> SpellMana;

        static SpellBook()
        {
            Spells = new Dictionary<SpellType, ISpell>
            {
                {SpellType.MagicMissle, new MagicMissle()},
                {SpellType.Drain, new Drain()},
                {SpellType.Shield, new Shield()},
                {SpellType.Poison, new Poison()},
                {SpellType.Recharge, new Recharge()},
            };

            SpellMana = Spells.ToDictionary(p => p.Key, p => p.Value.Mana);
        }

        public ISpell GetSpell(SpellType type)
        {
            ISpell spell;
            if (!Spells.TryGetValue(type, out spell))
            {
                throw new ArgumentException($"The spell type '{type}' is not supported.");
            }

            return spell;
        }

        public int GetMana(SpellType type)
        {
            int mana;
            if (!SpellMana.TryGetValue(type, out mana))
            {
                throw new ArgumentException($"The spell type '{type}' is not supported.");
            }

            return mana;
        }

        public IEnumerable<SpellType> GetAllSpellTypes()
        {
            return Spells.Keys;
        } 
    }

    public enum SpellType
    {
        MagicMissle,
        Drain,
        Shield,
        Poison,
        Recharge
    }

    public class CastedSpell
    {
        private readonly ISpell _spell;

        public CastedSpell(ISpell spell)
        {
            _spell = spell;
            RemainingTurns = _spell.Turns;
        }

        public int RemainingTurns { get; private set; }

        public void Cast(CombatState state)
        {
            state.Player.Mana -= _spell.Mana;
            _spell.Cast(state);
        }

        public void ApplyEffect(CombatState state)
        {
            if (RemainingTurns <= 0)
            {
                return;
            }

            RemainingTurns -= 1;
            _spell.ApplyEffect(state, RemainingTurns);
        }
    }

    public interface ISpell
    {
        int Mana { get; }
        int Turns { get; }
        void Cast(CombatState state);
        void ApplyEffect(CombatState state, int remainingTurns);
    }

    public class MagicMissle : ISpell
    {
        public int Mana => 53;
        public int Turns => 0;

        public void Cast(CombatState state)
        {
            state.Boss.HitPoints -= 4;
        }

        public void ApplyEffect(CombatState state, int remainingTurns)
        {
        }
    }

    public class Drain : ISpell
    {
        public int Mana => 73;
        public int Turns => 2;
        public void Cast(CombatState state)
        {
            state.Player.HitPoints += 2;
            state.Boss.HitPoints -= 2;
        }

        public void ApplyEffect(CombatState state, int remainingTurns)
        {
        }
    }

    public class Shield : ISpell
    {
        public int Mana => 113;
        public int Turns => 6;
        public void Cast(CombatState state)
        {
            state.Player.Armor += 7;
        }

        public void ApplyEffect(CombatState state, int remainingTurns)
        {
            if (remainingTurns == 0)
            {
                state.Player.Armor -= 7;
            }
        }
    }

    public class Poison : ISpell
    {
        public int Mana => 173;
        public int Turns => 6;
        public void Cast(CombatState state)
        {
        }

        public void ApplyEffect(CombatState state, int remainingTurns)
        {   
            state.Boss.HitPoints -= 3;
        }
    }

    public class Recharge : ISpell
    {
        public int Mana => 229;
        public int Turns => 5;

        public void Cast(CombatState state)
        {
        }

        public void ApplyEffect(CombatState state, int remainingTurns)
        {
            state.Player.Mana += 101;
        }
    }
}