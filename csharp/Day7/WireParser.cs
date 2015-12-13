using System;
using System.Collections.Generic;
using System.IO;
using System.Text;
using System.Text.RegularExpressions;

namespace AdventOfCode.Day7
{
    public class WireParser
    {
        public IEnumerable<Wire> ParseFile(string path)
        {
            using (var stream = new FileStream(path, FileMode.Open, FileAccess.Read))
            {
                return ParseStream(stream);
            }
        }

        public IEnumerable<Wire> ParseStream(Stream stream)
        {
            using (var reader = new StreamReader(stream, Encoding.UTF8, false, 4096, true))
            {
                return ParseTextReader(reader);
            }
        }

        public IEnumerable<Wire> ParseTextReader(TextReader reader)
        {
            var wires = new List<Wire>();
            string line;
            while ((line = reader.ReadLine()) != null)
            {
                line = line.Trim();
                if (line.Length > 0)
                {
                    wires.Add(ParseLine(line));
                }
            }

            return wires;
        }

        public IEnumerable<Wire> ParseLines(string lines)
        {
            return ParseTextReader(new StringReader(lines));
        }

        public Wire ParseLine(string line)
        {
            var pieces = line.Split(new[] { "->" }, StringSplitOptions.None);

            var label = pieces[1].Trim();
            var signalString = pieces[0].Trim();
            var signal = ParseSignal(signalString);

            if (signal == null)
            {
                throw new FormatException($"The wire could not be parsed: '{line}'.");
            }

            return new Wire
            {
                Label = label,
                Signal = signal
            };
        }

        private Signal ParseSignal(string signalString)
        {
            var match = Regex.Match(
                signalString,
                @"^(" +
                @"(?<Value>(?<ValueValue>\d+))" + @"|" +
                @"(?<Label>(?<LabelLabel>[A-Za-z]+))" + "|" +
                @"(?<Logical>((?<LogicalLabelA>[A-Za-z]+)|(?<LogicalValueA>\d+))\s+(?<LogicalOperator>AND|OR)\s+((?<LogicalLabelB>[A-Za-z]+)|(?<LogicalValueB>\d+)))" + @"|" +
                @"(?<Shift>((?<ShiftLabelA>[A-Za-z]+)|(?<ShiftValueA>\d+))\s+(?<ShiftOperator>LSHIFT|RSHIFT)\s+((?<ShiftLabelB>[A-Za-z]+)|(?<ShiftValueB>\d+)))" + @"|" +
                @"(?<Not>NOT\s+((?<NotLabel>[A-Za-z]+)|(?<NotValue>\d+)))" +
                @")$");

            var valueGroup = match.Groups["Value"];
            if (valueGroup.Success)
            {
                return new ValueSignal
                {
                    Value = ushort.Parse(valueGroup.Value)
                };
            }

            var labelGroup = match.Groups["Label"];
            if (labelGroup.Success)
            {
                return new LabelSignal
                {
                    Label = match.Groups["LabelLabel"].Value
                };
            }

            var logicalGroup = match.Groups["Logical"];
            if (logicalGroup.Success)
            {
                Signal signalA;
                if (match.Groups["LogicalLabelA"].Success)
                {
                    signalA = new LabelSignal {Label = match.Groups["LogicalLabelA"].Value};
                }
                else
                {
                    signalA = new ValueSignal {Value = ushort.Parse(match.Groups["LogicalValueA"].Value)};
                }

                var @operator = (LogicalOperator) Enum.Parse(typeof (LogicalOperator), match.Groups["LogicalOperator"].Value, true);

                Signal signalB;
                if (match.Groups["LogicalLabelB"].Success)
                {
                    signalB = new LabelSignal { Label = match.Groups["LogicalLabelB"].Value };
                }
                else
                {
                    signalB = new ValueSignal { Value = ushort.Parse(match.Groups["LogicalValueB"].Value) };
                }

                return new LogicalSignal
                {
                    SignalA = signalA,
                    Operator = @operator,
                    SignalB = signalB
                };
            }

            var shiftGroup = match.Groups["Shift"];
            if (shiftGroup.Success)
            {
                Signal signalA;
                if (match.Groups["ShiftLabelA"].Success)
                {
                    signalA = new LabelSignal { Label = match.Groups["ShiftLabelA"].Value };
                }
                else
                {
                    signalA = new ValueSignal { Value = ushort.Parse(match.Groups["ShiftValueA"].Value) };
                }

                var @operator = (ShiftOperator)Enum.Parse(typeof(ShiftOperator), match.Groups["ShiftOperator"].Value, true);

                Signal signalB;
                if (match.Groups["ShiftLabelB"].Success)
                {
                    signalB = new LabelSignal { Label = match.Groups["ShiftLabelB"].Value };
                }
                else
                {
                    signalB = new ValueSignal { Value = ushort.Parse(match.Groups["ShiftValueB"].Value) };
                }

                return new ShiftSignal
                {
                    SignalA = signalA,
                    Operator = @operator,
                    SignalB = signalB
                };
            }

            var notGroup = match.Groups["Not"];
            if (notGroup.Success)
            {
                Signal signal;
                if (match.Groups["NotLabel"].Success)
                {
                    signal = new LabelSignal { Label = match.Groups["NotLabel"].Value };
                }
                else
                {
                    signal = new ValueSignal { Value = ushort.Parse(match.Groups["NotValue"].Value) };
                }

                return new NotSignal
                {
                    Signal = signal
                };
            }
            return null;
        }
    }
}