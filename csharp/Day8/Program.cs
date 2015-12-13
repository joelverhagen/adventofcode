using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace AdventOfCode.Day8
{
    public class Program
    {
        public void Run()
        {
            var parser = new EscapedStringParser();
            var strings = parser.ParseFile(@"Day8\input.txt");

            int literalCount = 0;
            int parsedCount = 0;
            foreach (var s in strings)
            {
                literalCount += s.Input.Length;
                parsedCount += s.Output.Length;
            }

            Console.WriteLine($"Literal count: {literalCount}");
            Console.WriteLine($"Parsed count: {parsedCount}");
            Console.WriteLine($"Different: {literalCount - parsedCount}");
        }
    }

    public class EscapedStringParser
    {
        public IEnumerable<ParsedString> ParseFile(string path)
        {
            var lines = new List<ParsedString>();
            using (var stream = new FileStream(path, FileMode.Open, FileAccess.Read))
            using(var reader = new StreamReader(stream))
            {
                string line;
                while ((line = reader.ReadLine()) != null)
                {
                    lines.Add(ParseLine(line));
                }
            }

            return lines;
        }

        public ParsedString ParseLine(string line)
        {
            var builder = new StringBuilder();
            for (int i = 1; i < line.Length - 1; i++)
            {
                switch (line[i])
                {
                    case '\\':
                        if (line[i + 1] == '"')
                        {
                            builder.Append('"');
                            i++;
                        }
                        else if (line[i + 1] == '\\')
                        {

                            builder.Append('\\');
                            i++;
                        }
                        else if(line[i + 1] == 'x')
                        {
                            var b = Convert.ToByte(line.Substring(i + 2, 2), 16);
                            builder.Append(Encoding.ASCII.GetString(new[] {b}));
                            i += 3;
                        }
                        else
                        {
                            throw new FormatException($"Invalid escape sequence found at index {i}.");
                        }

                        break;
                    default:
                        builder.Append(line[i]);
                        break;
                }
            }

            return new ParsedString {Input = line, Output = builder.ToString()};
        }
    }

    public class ParsedString
    {
        public string Input { get; set; }
        public string Output { get; set; }
    }
}
