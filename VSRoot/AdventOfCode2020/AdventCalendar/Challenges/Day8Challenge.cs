using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace AdventCalendar.Challenges
{
	class Day8Challenge : Challenge
	{
		private class Operation
		{
			public string Code;
			public int Value;
			public bool Visited;
		}

		private List<Operation> _opList = new List<Operation>();
		private List<Operation> _ops = new List<Operation>();

		protected override string Part1()
		{
			string input = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input8.txt");
			string[] entries = input.Split(new string[] { Environment.NewLine }, StringSplitOptions.RemoveEmptyEntries);

			foreach(var entry in entries)
			{
				var splitEntry = entry.Split(' ');

				_ops.Add(new Operation() { Code = splitEntry[0], Value = int.Parse(splitEntry[1]), Visited = false});
			}

			AttemptProgram();

			return _accumulator.ToString(); ;
		}

		protected override string Part2()
		{
			string input = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input8.txt");
			string[] entries = input.Split(new string[] { Environment.NewLine }, StringSplitOptions.RemoveEmptyEntries);

			foreach (var entry in entries)
			{
				var splitEntry = entry.Split(' ');

				_ops.Add(new Operation() { Code = splitEntry[0], Value = int.Parse(splitEntry[1]), Visited = false });
			}

			AttemptProgram(_opList);
			int fixIdx = _opList.Count - 1;

			do
			{
				foreach(var op in _ops)
				{
					op.Visited = false;
				}

				// revert list
				if(fixIdx + 1 < _opList.Count)
				{
					_opList[fixIdx + 1].Code = _opList[fixIdx].Code == "jmp" ? "nop" : "jmp";
				}

				// swap op
				_opList[fixIdx].Code = _opList[fixIdx].Code == "jmp" ? "nop" : "jmp";
				fixIdx--;
			}
			while (!AttemptProgram());

			return _accumulator.ToString(); ;
		}

		private int _accumulator = 0;

		private bool AttemptProgram(List<Operation> opList = null)
		{
			_accumulator = 0;
			int currentOpIdx = 0;
			while (currentOpIdx < _ops.Count && !_ops[currentOpIdx].Visited)
			{
				var currentOp = _ops[currentOpIdx];

				
				currentOp.Visited = true;
				switch (currentOp.Code)
				{
					case "acc":
						_accumulator += currentOp.Value;
						currentOpIdx++;
						break;
					case "jmp":
						currentOpIdx += currentOp.Value;
						if (opList != null)
						{
							opList.Add(currentOp);
						}
						break;
					case "nop":
						currentOpIdx++;
						if (opList != null)
						{
							opList.Add(currentOp);
						}
						break;
					default:
						break;
				}
			}

			return currentOpIdx >= _ops.Count;
		}
	}
}
