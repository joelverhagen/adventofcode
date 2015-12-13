using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Newtonsoft.Json;

namespace AdventOfCode.Day12
{
    public class Program
    {
        public void Run()
        {
            long sum = 0;
            using (var fileStream = new FileStream(@"Day12\input.txt", FileMode.Open, FileAccess.Read))
            using (var streamReader = new StreamReader(fileStream))
            using (var jsonTextReader = new JsonTextReader(streamReader))
            {
                while (jsonTextReader.Read())
                {
                    if (jsonTextReader.TokenType == JsonToken.Integer)
                    {
                        sum += (long)jsonTextReader.Value;
                    }
                }
            }
               
            Console.WriteLine(sum);
        }
    }
}
