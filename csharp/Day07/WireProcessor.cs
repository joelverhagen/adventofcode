using System;
using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode.Day07
{
    public class WireProcessResult
    {
        public IDictionary<string, ushort> Values { get; set; }
        public IEnumerable<Wire> UnresolvedWires { get; set; }
    }

    public class WireProcessor
    {
        public WireProcessResult Process(IEnumerable<Wire> wireSequence)
        {
            var wires = wireSequence.ToDictionary(w => w.Label, w => w);
            var values = new Dictionary<string, ushort>();
            var lastCount = 0;
            while (lastCount != wires.Count)
            {
                lastCount = wires.Count;

                foreach (var wire in wires.Values.ToArray())
                {
                    var signal = ResolveSignal(values, wire.Signal);
                    if (signal.IsValue)
                    {
                        values[wire.Label] = signal.ValueSignal.Value;
                        wires.Remove(wire.Label);
                    }
                }
            }

            return new WireProcessResult
            {
                Values = values,
                UnresolvedWires = wires.Values
            };
        }

        private Signal ResolveSignal(IDictionary<string, ushort> values, Signal signal)
        {
            switch (signal.Type)
            {
                case SignalType.Value:
                {
                    return signal;
                }

                case SignalType.Label:
                {
                    ushort value;
                    if (values.TryGetValue(signal.LabelSignal.Label, out value))
                    {
                        return new ValueSignal {Value = value};
                    }
                    
                    return signal;
                }
                
                case SignalType.Logical:
                {
                    var logicalSignal = signal.LogicalSignal;
                    var signalA = ResolveSignal(values, logicalSignal.SignalA);
                    if (!signalA.IsValue)
                    {
                        return signal;
                    }

                    var signalB = ResolveSignal(values, logicalSignal.SignalB);
                    if (!signalB.IsValue)
                    {
                        return signal;
                    }

                    var @operator = logicalSignal.Operator;
                    var value = @operator == LogicalOperator.And ? signalA.ValueSignal.Value & signalB.ValueSignal.Value : signalA.ValueSignal.Value | signalB.ValueSignal.Value;
                    return new ValueSignal {Value = (ushort) value};
                }

                case SignalType.Shift:
                {
                    var shiftSignal = signal.ShiftSignal;
                    var signalA = ResolveSignal(values, shiftSignal.SignalA);
                    if (!signalA.IsValue)
                    {
                        return signal;
                    }

                    var signalB = ResolveSignal(values, shiftSignal.SignalB);
                    if (!signalB.IsValue)
                    {
                        return signal;
                    }

                    var @operator = shiftSignal.Operator;
                    var value = @operator == ShiftOperator.LShift ? signalA.ValueSignal.Value << signalB.ValueSignal.Value : signalA.ValueSignal.Value >> signalB.ValueSignal.Value;
                    return new ValueSignal { Value = (ushort)value };
                    }

                case SignalType.Not:
                {
                    var notSignal = signal.NotSignal;
                    var innerSignal = ResolveSignal(values, notSignal.Signal);
                    if (!innerSignal.IsValue)
                    {
                        return signal;
                    }

                    var value = ~innerSignal.ValueSignal.Value;
                    return new ValueSignal { Value = (ushort)value };
                }
            }

            throw new ArgumentException(nameof(signal), $"The provided signal has an unknown signal type: {signal.Type}.");
        }
    }
}