using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AdventOfCode.Day22
{
    public class Character
    {
        public int HitPoints { get; set; }
    }

    public class Boss : Character
    {
        public int Damage { get; set; }
    }

    public class Player : Character
    {
        public int Mana { get; set; }
        public int Armor { get; set; }
    }
}
