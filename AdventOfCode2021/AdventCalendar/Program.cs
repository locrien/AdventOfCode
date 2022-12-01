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
            Console.WriteLine("Which Day?");
            int selectedDay = -1;
            if(!int.TryParse(Console.ReadLine(), out selectedDay))
            {
                selectedDay = DateTime.Now.Day;
            }

            Challenge selectedChallenge = Challenge.Create(selectedDay);

            Console.WriteLine("Which Part?");
            int selectedPart = -1;
            if(!int.TryParse(Console.ReadLine(),out selectedPart))
            {
                selectedPart = -1;
            }

            string result = selectedChallenge.Execute(selectedPart);

            // make a day challenge factory that returns a day object based on the current day.
            Console.WriteLine(result);
            Console.ReadKey();
        }
    }
}
