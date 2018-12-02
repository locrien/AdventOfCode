using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AdventCalendar
{
    class Program
    {
        static void Main(string[] args)
        {
            int part = 1;
            int forceDay = 18;

            string result = Challenge.Create(forceDay > -1 ? forceDay : DateTime.Now.Day).Execute(part);

            // make a day challenge factory that returns a day object based on the current day.
            Console.WriteLine(result);
            Console.ReadKey();
        }
    }
}
