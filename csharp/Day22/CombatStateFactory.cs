using System.Collections.Generic;

namespace AdventOfCode.Day22
{
    public class CombatStateFactory
    {
        private readonly Player _player;
        private readonly Boss _boss;

        public CombatStateFactory(Player player, Boss boss)
        {
            _player = player;
            _boss = boss;
        }

        public CombatState Create()
        {
            return new CombatState
            {
                PlayerTurn = true,
                Player = new Player
                {
                    Armor = _player.Armor,
                    Mana = _player.Mana,
                    HitPoints = _player.HitPoints
                },
                Boss = new Boss
                {
                    HitPoints = _boss.HitPoints,
                    Damage = _boss.Damage
                },
                CastedSpells = new Dictionary<SpellType, CastedSpell>()
            };
        }
    }
}