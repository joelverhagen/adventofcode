using System;
using System.Collections.Generic;
using System.IO;
using System.Text;

namespace AdventOfCode.Day08
{
    public class ParsedString
    {
        public string Input { get; set; }
        public string Output { get; set; }
    }

    public class StringParser
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
}