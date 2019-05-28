using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace IncrementalSociety
{
	public class EdictCooldownConfig
	{
		public int EdictLength;
		public List<string> EdictNames;
		public Dictionary <string, int> Index;

		public EdictCooldownConfig (IEnumerable<string> edicts)
		{
			EdictNames = edicts.ToList ();
			EdictLength = EdictNames.Count;
			Index = new Dictionary<string, int> ();
			for (int i = 0 ; i < EdictLength; ++i)
				Index.Add (EdictNames[i], i);
		}

		public EdictCooldown Create () => new EdictCooldown (this);
	}

	public struct EdictItem
	{
		public string Name;
		public int Countdown;

		public EdictItem (string name, int countdown)
		{
			Name = name;
			Countdown = countdown;
		}
	}

	public class EdictCooldown : IEnumerable<EdictItem>
	{
		// During saveload we are deserialized without our config
		// Stash it here during inflation
		public static EdictCooldownConfig SaveLoadConfig { get; set; }

		EdictCooldownConfig Config;
		int [] Inventory;

		int EdictLength => Config.EdictLength;
		Dictionary <string, int> Index => Config.Index;

		public EdictCooldown (EdictCooldownConfig config)
		{
			Config = config;
			Inventory = new int [EdictLength];
		}

		// This is invoked during json deserialization
		public EdictCooldown (IEnumerable<EdictItem> values)
		{
			if (SaveLoadConfig == null)
				throw new InvalidOperationException ("EdictCooldown (IEnumerable<double>) ctor was invoked without SaveLoadConfig setup");

			Config = SaveLoadConfig;
			Inventory = new int [EdictLength];
			Array.Copy (values.Select (x => x.Countdown).ToArray (), Inventory, EdictLength);
		}

		protected EdictCooldown (EdictCooldownConfig config, int [] inventory)
		{
			Config = config;
			Inventory = new int [EdictLength];
			Array.Copy (inventory, Inventory, EdictLength);
		}

		public int this [string key]
		{
			get => Inventory [Index[key]];
		}

		public int this [int index]
		{
			get => Inventory [index];
		}

		public bool IsEmpty {
			get {
				for (int i = 0 ; i < EdictLength; ++i) {
					if (Inventory[i] != 0)
						return false;
				}
				return true;
			}
		}

		public EdictCooldown Tick ()
		{
			var newInventory = new int[EdictLength];
			for (int i = 0; i < EdictLength; ++i) {
				int current = Inventory[i];
				if (current > 0)
					newInventory[i] = current - 1;
				else
					newInventory[i] = 0;
			}
			return new EdictCooldown (Config, newInventory);
		}

		public EdictCooldown Add (string name, int length)
		{
			var newInventory = new int[EdictLength];
			Array.Copy (Inventory, newInventory, EdictLength);
#if DEBUG
			if (newInventory[Index[name]] != 0)
				throw new InvalidOperationException ($"Add on EdictCooldown for {name} {length} but existing value of {newInventory[Index[name]]}");
#endif
			newInventory[Index[name]] = length;
			return new EdictCooldown (Config, newInventory);
		}

		public IEnumerator<EdictItem> GetEnumerator ()
		{
			for (int i = 0 ; i<EdictLength ; ++i)
				yield return new EdictItem (Config.EdictNames[i], Inventory[i]);
		}

		IEnumerator IEnumerable.GetEnumerator ()
		{
			return GetEnumerator ();
		}
	}
}
