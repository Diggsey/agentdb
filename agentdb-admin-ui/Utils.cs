using Newtonsoft.Json;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AgentdbAdmin
{
    static class Utils
    {
        public static string StringifyBytes(IEnumerable<byte> keyData)
        {
            return string.Concat(keyData.Select(b => (b >= 0x20 && b < 127 && b != '\\') ? ((char)b).ToString() : "\\x" + b.ToString("x2")));
        }

        public static string FormatCsv(IEnumerable<string> fields)
        {
            return string.Join(",", fields.Select(field =>
            {
                var escaped = field.Replace("\"", "\"\"");
                return $"\"{escaped}\"";
            }));
        }

        public static string FormatJson(string json)
        {
            try
            {
                dynamic parsedJson = JsonConvert.DeserializeObject(json);
                return JsonConvert.SerializeObject(parsedJson, Formatting.Indented);
            } catch (JsonException)
            {
                return "<Invalid JSON>";
            }
        }

        public static IEnumerable<IEnumerable<T>> Batch<T>(this IEnumerable<T> source, int size)
        {
            if (size <= 0)
                throw new ArgumentOutOfRangeException("size", "Must be greater than zero.");
            using (var enumerator = source.GetEnumerator())
                while (enumerator.MoveNext())
                {
                    int i = 0;
                    // Batch is a local function closing over `i` and `enumerator` that
                    // executes the inner batch enumeration
                    IEnumerable<T> Batch()
                    {
                        do yield return enumerator.Current;
                        while (++i < size && enumerator.MoveNext());
                    }

                    yield return Batch();
                    while (++i < size && enumerator.MoveNext()) ; // discard skipped items
                }
        }

        public static string FormatBinary(IEnumerable<byte> input)
        {
            return string.Join(
                Environment.NewLine,
                input.Select(b => b.ToString("x2"))
                .Batch(8).Select(g => string.Join(" ", g))
                .Batch(4).Select(g => string.Join("   ", g))
            );
        }

        public const string DateFormat = "yyyy'-'MM'-'dd' 'HH':'mm':'ss";
    }
}
