using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading.Tasks;

namespace AdventCalendar.Challenges
{
	class Day7Challenge : Challenge
	{
		private class Bag
		{
			public string Id
			{
				get;
				private set;
			}

			public List<Bag> Parents = new List<Bag>();
			public List<(int, Bag)> Contents = new List<(int, Bag)>(); 
			
			public Bag(string id)
			{
				Id = id;
			}
		}

		private Dictionary<string, Bag> _bags = new Dictionary<string, Bag>();

		protected override string Part1()
		{
			string input = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input7.txt");
			string[] entries = input.Split(new string[] { Environment.NewLine }, StringSplitOptions.RemoveEmptyEntries);

			BuildGraph(entries);

			var outermostBags = new List<Bag>();
			var shinyGoldBag = _bags["shiny gold"];

			FindParentBags(shinyGoldBag, outermostBags);

			return (outermostBags.Count - 1).ToString();
		}

		private void BuildGraph(string[] entries)
		{
			foreach (var entry in entries)
			{
				var entrySplit = entry.Split(new string[] { " bags contain " }, StringSplitOptions.RemoveEmptyEntries);
				var bagId = entrySplit[0].Trim(' ');

				var bag = _bags.TryGetValue(bagId, out var existingBag) ? existingBag : new Bag(bagId);
				_bags[bagId] = bag;

				var contents = entrySplit[1].Split(new string[] { ", ", "." }, StringSplitOptions.RemoveEmptyEntries);
				foreach (var content in contents)
				{
					var splitPoint = content.IndexOf(" ");
					if (int.TryParse(content.Substring(0, splitPoint), out var amount))
					{
						var type = content.Substring(splitPoint + 1, content.Length - 2).Replace("bags", "").Replace("bag", "").Trim(' ');

						var innerBag = _bags.TryGetValue(type, out var existingInnerBag) ? existingInnerBag : new Bag(type);
						_bags[type] = innerBag;

						bag.Contents.Add((amount, innerBag));
						innerBag.Parents.Add(bag);
					}
				}
			}
		}

		private void FindParentBags(Bag bag, List<Bag> results)
		{
			if (!results.Contains(bag))
			{
				results.Add(bag);
			}

			foreach (var parent in bag.Parents)
			{
				FindParentBags(parent, results);
			}
		}

		protected override string Part2()
		{
			string input = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input7.txt");
			string[] entries = input.Split(new string[] { Environment.NewLine }, StringSplitOptions.RemoveEmptyEntries);

			BuildGraph(entries);
			
			var shinyGoldBag = _bags["shiny gold"];

			var amount = FindChildBagAmount(shinyGoldBag);

			return amount.ToString();
		}

		private int FindChildBagAmount(Bag bag)
		{
			var total = 0;
			foreach(var child in bag.Contents)
			{
				total += child.Item1 + (FindChildBagAmount(child.Item2) * child.Item1);
			}

			return total;
		}
	}
}
