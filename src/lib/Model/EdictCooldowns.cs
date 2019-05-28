using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

using IncrementalSociety.Json;
using IncrementalSociety.Utilities;

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
			for (int i = 0 ; i < ResourceLength ; ++i)
				Index.Add (EdictNames[i], i);
		}

		public EdictCooldown Create () => new EdictCooldown (this);
		public EdictCooldown.Builder CreateBuilder () => new EdictCooldown.Builder (this);
	}

	public struct EdictItem
	{
		public string Name;
		public double Countdown;

		public EdictItem (string name, double countdown)
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
			Inventory = new int [ResourceLength];
		}

		// This is invoked during json deserialization
		public EdictCooldown (IEnumerable<ResourceItem> values)
		{
			if (SaveLoadConfig == null)
				throw new InvalidOperationException ("EdictCooldown (IEnumerable<double>) ctor was invoked without SaveLoadConfig setup");

			Config = SaveLoadConfig;
			Inventory = new int [ResourceLength];
			Array.Copy (values.Select (x => x.Value).ToArray (), Inventory, EdictLength);
		}

		protected EdictCooldown (EdictCooldownConfig config, int [] inventory)
		{
			Config = config;
			Inventory = new int [ResourceLength];
			Array.Copy (inventory, Inventory, ResourceLength);
		}

		public int this [string key]
		{
			get => Inventory [Index[key]];
		}

		public int this [int index]
		{
			get => Inventory [index];
		}

		public Builder ToBuilder ()
		{
			return new Builder (Config, Inventory);
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

		public IEnumerator<EdictItem> GetEnumerator()
		{
			for (int i = 0 ; i < EdictLength ; ++i)
				yield return new EdictItem (Config.EdictNames[i], Inventory[i]);
		}

		IEnumerator IEnumerable.GetEnumerator()
		{
			return GetEnumerator();
		}

		public class Builder : EdictCooldown
		{
			internal Builder (EdictCooldownConfig config, double [] inventory) : base (config, inventory) {}
			internal Builder (EdictCooldownConfig config) : base (config) {}

			public new int this [string key]
			{
				get => Inventory [Index[key]];
				set => Inventory [Index[key]] = value;
			}

			public new int this [int index]
			{
				get => Inventory [index];
				set => Inventory [index] = value;
			}

			public EdictCooldown ToEdictCooldown ()
			{
				return new EdictCooldown (Config, Inventory);
			}
		}
	}
}
