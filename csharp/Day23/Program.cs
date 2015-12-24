using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text.RegularExpressions;

namespace AdventOfCode.Day23
{
    public class Program
    {
        public void Run()
        {
            var instructions = new Parser().ParseFile(@"Day23\input.txt").ToArray();
            var part1Result = new Evalutor().Evaluate(instructions, new Dictionary<string, uint> { { "a", 0 }, { "b", 0 } });
            Console.WriteLine($"Part 1 answer: {part1Result["b"]}");

            var part2Result = new Evalutor().Evaluate(instructions, new Dictionary<string, uint> { { "a", 1 }, { "b", 0 } });
            Console.WriteLine($"Part 2 answer: {part2Result["b"]}");
        }
    }

    public class Evalutor
    {
        public IDictionary<string, uint> Evaluate(IEnumerable<Instruction> instructionSequence, IDictionary<string, uint> initialRegisters)
        {
            var instructions = instructionSequence.ToArray();
            var registers = initialRegisters;
            var i = 0;
            while (i >= 0 && i < instructions.Length)
            {
                var instruction = instructions[i];
                switch (instruction.Type)
                {
                    case InstructionType.Hlf:
                        registers[instruction.Register] /= 2;
                        i += 1;
                        break;

                    case InstructionType.Tpl:
                        registers[instruction.Register] *= 3;
                        i += 1;
                        break;

                    case InstructionType.Inc:
                        registers[instruction.Register] += 1;
                        i += 1;
                        break;

                    case InstructionType.Jmp:
                        i += instruction.Offset;
                        break;

                    case InstructionType.Jie:
                        if (registers[instruction.Register]%2 == 0)
                        {
                            i += instruction.Offset;
                        }
                        else
                        {
                            i += 1;
                        }
                        break;

                    case InstructionType.Jio:
                        if (registers[instruction.Register] == 1)
                        {
                            i += instruction.Offset;
                        }
                        else
                        {
                            i += 1;
                        }
                        break;
                }
            }

            return registers;
        }
    }

    public class Parser
    {
        public IEnumerable<Instruction> ParseFile(string path)
        {
            foreach (var line in File.ReadAllLines(path))
            {
                var pieces = line.Trim().Split(new[] { ' ', ',' }, StringSplitOptions.RemoveEmptyEntries);
                var type = (InstructionType)Enum.Parse(typeof (InstructionType), pieces[0], true);
                var instruction = new Instruction {Type = type};

                switch (type)
                {
                    case InstructionType.Hlf:
                    case InstructionType.Tpl:
                    case InstructionType.Inc:
                        instruction.Register = pieces[1];
                        break;
                    
                    case InstructionType.Jmp:
                        instruction.Offset = int.Parse(pieces[1]);
                        break;
                    
                    case InstructionType.Jie:
                    case InstructionType.Jio:
                        instruction.Register = pieces[1];
                        instruction.Offset = int.Parse(pieces[2]);
                        break;

                    default:
                        throw new NotSupportedException($"The instruction type '{type}' is not supported.");
                }

                yield return instruction;
            }
        }
    }

    public enum InstructionType
    {
        Hlf,
        Tpl,
        Inc,
        Jmp,
        Jie,
        Jio
    };

    public class Instruction
    {
        public InstructionType Type { get; set; }
        public string Register { get; set; }
        public int Offset { get; set; }

        public override string ToString()
        {
            switch (Type)
            {
                case InstructionType.Hlf:
                case InstructionType.Tpl:
                case InstructionType.Inc:
                    return $"{Type.ToString().ToLower()} {Register}";


                case InstructionType.Jmp:
                    return $"{Type.ToString().ToLower()} {Offset:+#;-#}";

                case InstructionType.Jie:
                case InstructionType.Jio:
                    return $"{Type.ToString().ToLower()} {Register}, {Offset:+#;-#}";
            }

            return base.ToString();
        }
    }
}
