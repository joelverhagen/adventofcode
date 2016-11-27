namespace AdventOfCode.Day19.Part1
{
    public class SequenceItem
    {
        public bool Replaceable { get; set; }
        public string Value { get; set; }

        public override string ToString()
        {
            return Value;
        }
    }
}