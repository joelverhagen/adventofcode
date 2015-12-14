using System.Collections.Generic;
using System.IO;
using System.Text;

namespace AdventOfCode.Day08
{
    public class EscapedString
    {
        public string Input { get; set; }
        public string Output { get; set; }
    }

    public class StringEscaper
    {
        public IEnumerable<EscapedString> EscapeFile(string path)
        {
            var lines = new List<EscapedString>();
            using (var stream = new FileStream(path, FileMode.Open, FileAccess.Read))
            using (var reader = new StreamReader(stream))
            {
                string line;
                while ((line = reader.ReadLine()) != null)
                {
                    lines.Add(EscapeLine(line));
                }
            }

            return lines;
        }

        public EscapedString EscapeLine(string line)
        {
            var builder = new StringBuilder();
            builder.Append("\"");
            for (int i = 0; i < line.Length; i++)
            {
                switch (line[i])
                {
                    case '"':
                        builder.Append("\\\"");
                        break;
                    case '\\':
                        builder.Append("\\\\");
                        break;
                    default:
                        builder.Append(line[i]);
                        break;
                }
            }

            builder.Append("\"");
            return new EscapedString { Input = line, Output = builder.ToString() };
        }

    }
}