using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace AdventOfCode.Day12
{
    public class Program
    {
        public void Run()
        {
            // Part 1
            long part1Sum = 0;
            using (var fileStream = new FileStream(@"Day12\input.txt", FileMode.Open, FileAccess.Read))
            using (var streamReader = new StreamReader(fileStream))
            using (var jsonTextReader = new JsonTextReader(streamReader))
            {
                while (jsonTextReader.Read())
                {
                    if (jsonTextReader.TokenType == JsonToken.Integer)
                    {
                        part1Sum += (long)jsonTextReader.Value;
                    }
                }
            }
            Console.WriteLine($"Part 1 answer: {part1Sum}");

            // Part 2
            var token = JsonConvert.DeserializeObject<JToken>(File.ReadAllText(@"Day12\input.txt"));
            var part2Sum = CountWithoutRed(token);
            Console.WriteLine($"Part 2 answer: {part2Sum}");
        }
        
        private long CountWithoutRed(JToken token)
        {
            long sum = 0;
            switch (token.Type)
            {
                case JTokenType.Object:
                {
                    var jobject = (JObject) token;
                    
                    if (!jobject.Properties().Any(p => p.Value.Type == JTokenType.String && (string)p.Value == "red"))
                    {
                        foreach (var value in jobject.PropertyValues())
                        {
                            sum += CountWithoutRed(value);
                        }
                    }
                }
                    break;

                case JTokenType.Array:
                {
                    var jarray = (JArray) token;
                    foreach (var value in jarray)
                    {
                        sum += CountWithoutRed(value);
                    }
                }
                    break;

                case JTokenType.Integer:
                {
                    sum += (long) token;
                }
                    break;
            }

            return sum;
        }
    }
}
