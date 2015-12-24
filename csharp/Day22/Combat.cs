using System;
using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode.Day22
{
    public class Combat
    {
        private readonly CombatStateFactory _factory;
        private readonly SpellBook _spellBook;

        public Combat(CombatStateFactory factory, SpellBook spellBook)
        {
            _factory = factory;
            _spellBook = spellBook;
        }

        public CombatResult GetCombatResult(IEnumerable<SpellType> spellTypes)
        {
            var spellTypeArray = spellTypes.ToArray();
            var remaining = spellTypeArray.Reverse().ToList();

            var result = (CombatResultType?)null;
            var state = _factory.Create();
            while (!result.HasValue)
            {
                // apply existing spells
                foreach (var type in state.CastedSpells.Keys.ToArray())
                {
                    var castedSpell = state.CastedSpells[type];
                    castedSpell.ApplyEffect(state);
                    if (castedSpell.RemainingTurns == 0)
                    {
                        state.CastedSpells.Remove(type);
                    }
                }

                if (state.Boss.HitPoints <= 0)
                {
                    result = CombatResultType.ExistingSpellKillsBoss;
                    break;
                }

                // take actions for this turn
                if (state.PlayerTurn)
                {
                    if (!remaining.Any())
                    {
                        result = CombatResultType.PlayerRunsOutOfSpells;
                        break;
                    }

                    var nextSpellType = remaining[remaining.Count - 1];
                    remaining.RemoveAt(remaining.Count - 1);

                    if (state.CastedSpells.ContainsKey(nextSpellType))
                    {
                        result = CombatResultType.DuplicateSpell;
                        break;
                    }

                    var nextSpell = _spellBook.GetSpell(nextSpellType);
                    if (nextSpell.Mana > state.Player.Mana)
                    {
                        result = CombatResultType.PlayerRunsOutOfMana;
                        break;
                    }

                    var castedSpell = new CastedSpell(nextSpell);
                    castedSpell.Cast(state);
                    state.ManaSpent += nextSpell.Mana;
                    state.CastedSpells[nextSpellType] = castedSpell;

                    if (state.Boss.HitPoints <= 0)
                    {
                        result = CombatResultType.CastedSpellKillsBoss;
                        break;
                    }
                }
                else
                {
                    var damage = Math.Max(1, state.Boss.Damage - state.Player.Armor);
                    state.Player.HitPoints -= damage;

                    if (state.Player.HitPoints <= 0)
                    {
                        result = CombatResultType.BossKillsPlayer;
                        break;
                    }
                }

                // switch the turn
                state.PlayerTurn = !state.PlayerTurn;
            }

            return new CombatResult { Spells = spellTypeArray, Type = result.Value, State = state };
        }
    }
}