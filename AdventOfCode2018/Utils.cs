using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Text;
using System.Threading.Tasks;

namespace AdventCalendar
{
    static class Utils
    {
        public static int[][] CreateArray(string value)
        {
            string[] rows = value.Split('\n');

            int[][] result = new int[rows.Length][];

            for (int i = 0; i < rows.Length; ++i)
            {
                string[] columns = rows[i].Split(new char[] { '\t', ' ', '\r' }, StringSplitOptions.RemoveEmptyEntries);
                result[i] = new int[columns.Length];

                for (int j = 0; j < columns.Length; ++j)
                {
                    result[i][j] = int.Parse(columns[j]);
                }
            }

            return result;
        }

        public static string ReadEmbededResourceTextFile(string name)
        {
            var assembly = Assembly.GetExecutingAssembly();

            using (Stream stream = assembly.GetManifestResourceStream(name))
            using (StreamReader reader = new StreamReader(stream))
            {
                return reader.ReadToEnd();
            }
        }
    }
}
