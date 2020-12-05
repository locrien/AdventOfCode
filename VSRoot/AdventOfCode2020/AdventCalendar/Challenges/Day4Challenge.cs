using System;
using System.Collections.Generic;
using System.Text.RegularExpressions;

namespace AdventCalendar.Challenges
{

	/*	
	 	byr (Birth Year)
		iyr (Issue Year)
		eyr (Expiration Year)
		hgt (Height)
		hcl (Hair Color)
		ecl (Eye Color)
		pid (Passport ID)
		cid (Country ID)
	 	
	 	ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
		byr:1937 iyr:2017 cid:147 hgt:183cm

		iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
		hcl:#cfa07d byr:1929 


	*/
	class Day4Challenge : Challenge
	{
		private string[] fields = new string[] { "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" };
		private string[] optionalFields = new string[] {"cid" };

		protected override string Part1()
		{
			string entryText = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input4.txt");
			string[] passports = entryText.Split(new string[] {Environment.NewLine + Environment.NewLine}, StringSplitOptions.RemoveEmptyEntries);

			var validPassports = 0;
			var fieldsToFind = new List<string>();
			foreach (var passport in passports)
			{
				fieldsToFind.Clear();
				fieldsToFind.AddRange(fields);

				var entries = passport.Split(new string[] { " ", "\r\n" }, StringSplitOptions.RemoveEmptyEntries);
				foreach(var entry in entries)
				{
					var entrySplit = entry.Split(':');
					var key = entrySplit[0];
					var value = entrySplit[1];

					fieldsToFind.Remove(key);
				}

				if(fieldsToFind.Count == 0)
				{
					validPassports++;
				}
			}

			
			return validPassports.ToString();
		}

		protected override string Part2()
		{
			string entryText = Utils.ReadEmbededResourceTextFile("AdventCalendar.Data.input4.txt");
			string[] passports = entryText.Split(new string[] { Environment.NewLine + Environment.NewLine }, StringSplitOptions.RemoveEmptyEntries);

			var validPassports = 0;
			var fieldsToFind = new List<string>();
			foreach (var passport in passports)
			{
				fieldsToFind.Clear();
				fieldsToFind.AddRange(fields);

				var entries = passport.Split(new string[] { " ", "\r\n" }, StringSplitOptions.RemoveEmptyEntries);
				foreach (var entry in entries)
				{
					var entrySplit = entry.Split(':');
					var key = entrySplit[0];
					var value = entrySplit[1];

					if(ValidateField(key,value))
					{
						fieldsToFind.Remove(key);
					}
					else
					{
						continue;
					}
				}

				if (fieldsToFind.Count == 0)
				{
					validPassports++;
				}
			}


			return validPassports.ToString();
		}

		private bool ValidateField(string key, string value)
		{
			var parseResult = false;

			switch (key)
			{
				case "byr":
					parseResult = int.TryParse(value, out var birthYear);
					return parseResult && birthYear >= 1920 && birthYear <= 2002;
				case "iyr":
					parseResult = int.TryParse(value, out var issuesYear);
					return parseResult && issuesYear >= 2010 && issuesYear <= 2020;
				case "eyr":
					parseResult = int.TryParse(value, out var expYear);
					return parseResult && expYear >= 2020 && expYear <= 2030;
				case "hgt":
					{
						var cmIdx = value.IndexOf("cm");
						if (cmIdx >= 0)
						{
							var subStr = value.Substring(0, cmIdx);
							parseResult = int.TryParse(subStr, out var cm);
							return parseResult && cm >= 150 && cm <= 193;
						}
					}
					{
						var inchIdx = value.IndexOf("in");
						if (inchIdx >= 0)
						{
							var subStr = value.Substring(0, inchIdx);
							parseResult = int.TryParse(subStr, out var inch);
							return parseResult && inch >= 59 && inch <= 76;
						}
					}
					return false;
				case "hcl":
					Regex expr = new Regex("^[#][0-9a-f]{6}$");
					return expr.IsMatch(value);
				case "ecl":
					switch(value)
					{
						case "amb":
						case "blu":
						case "brn":
						case "gry":
						case "grn":
						case "hzl":
						case "oth":
							return true;
						default:
							return false;
					}
				case "pid":
					Regex exprPid = new Regex("^[0]*[0-9]{9}$");
					return exprPid.IsMatch(value);
				case "cid":
					return true;
				default:
					return false;
			}
		}
	}
}
