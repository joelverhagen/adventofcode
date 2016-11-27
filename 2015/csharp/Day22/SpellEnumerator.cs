using System;
using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode.Day22
{
    public class SpellEnumerator
    {
        private readonly Combat _combat;
        private readonly SpellBook _spellBook;
        private readonly Queue<SpellType[]> _queue;

        public SpellEnumerator(Combat combat, SpellBook spellBook)
        {
            _combat = combat;
            _spellBook = spellBook;
            _queue = new Queue<SpellType[]>();
        }

        public IEnumerable<CombatResult> Enumerate()
        {
            _queue.Clear();

            // seed the queue
            var allSpellTypes = _spellBook.GetAllSpellTypes().ToArray();
            foreach (var spellType in allSpellTypes)
            {
                _queue.Enqueue(new[] { spellType});
            }

            while (_queue.Any())
            {
                var prefix = _queue.Dequeue();
                var result = _combat.GetCombatResult(prefix);

                switch (result.Type)
                {
                    case CombatResultType.CastedSpellKillsBoss:
                    case CombatResultType.ExistingSpellKillsBoss:
                        yield return result;
                        break;
                    
                    case CombatResultType.DuplicateSpell:
                    case CombatResultType.PlayerRunsOutOfMana:
                    case CombatResultType.BossKillsPlayer:
                    case CombatResultType.HardDifficulty:
                        break;

                    case CombatResultType.PlayerRunsOutOfSpells:
                        foreach (var spellType in allSpellTypes)
                        {
                            if (result.State.Player.Mana < _spellBook.GetMana(spellType))
                            {
                                continue;
                            }

                            var newPrefix = prefix.Concat(new[] { spellType }).ToArray();
                            _queue.Enqueue(newPrefix);
                        }
                        break;
                    default:
                        throw new NotSupportedException($"The combat result '{result.Type}' is not supported.");
                }
            }
        }
    }
}