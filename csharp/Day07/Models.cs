namespace AdventOfCode.Day07
{
    public class Wire
    {
        public string Label { get; set; }
        public Signal Signal { get; set; }

        public override string ToString()
        {
            return $"{Signal} -> {Label}";
        }
    }

    public enum SignalType { Value, Logical, Shift, Not, Label }

    public abstract class Signal
    {
        public abstract SignalType Type { get; }

        public bool IsValue => Type == SignalType.Value;
        public bool IsLabel => Type == SignalType.Label;

        public ValueSignal ValueSignal => (ValueSignal) this;
        public LabelSignal LabelSignal => (LabelSignal)this;
        public LogicalSignal LogicalSignal => (LogicalSignal)this;
        public ShiftSignal ShiftSignal => (ShiftSignal)this;
        public NotSignal NotSignal => (NotSignal)this;
    }

    public class ValueSignal : Signal
    {
        public ushort Value { get; set; }
        public override SignalType Type => SignalType.Value;

        public override string ToString()
        {
            return $"{Value}";
        }
    }

    public class LabelSignal : Signal
    {
        public override SignalType Type => SignalType.Label;
        public string Label { get; set; }

        public override string ToString()
        {
            return $"{Label}";
        }
    }

    public enum LogicalOperator { And, Or }

    public class LogicalSignal : Signal
    {
        public Signal SignalA { get; set; }
        public LogicalOperator Operator { get; set; }
        public Signal SignalB { get; set; }
        public override SignalType Type => SignalType.Logical;

        public override string ToString()
        {
            return $"{SignalA} {Operator.ToString().ToUpper()} {SignalB}";
        }
    }

    public enum ShiftOperator { LShift, RShift }

    public class ShiftSignal : Signal
    {
        public Signal SignalA { get; set; }
        public ShiftOperator Operator { get; set; }
        public Signal SignalB { get; set; }
        public override SignalType Type => SignalType.Shift;

        public override string ToString()
        {
            return $"{SignalA} {Operator.ToString().ToUpper()} {SignalB}";
        }
    }

    public class NotSignal : Signal
    {
        public Signal Signal { get; set; }
        public override SignalType Type => SignalType.Not;

        public override string ToString()
        {
            return $"NOT {Signal}";
        }
    }
}
