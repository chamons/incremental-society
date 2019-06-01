using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;

using IncrementalSociety.Json;
using IncrementalSociety.Utilities;

namespace IncrementalSociety
{
	public class ResourceConfig
	{
		public int ResourceLength;
		public List<string> ResourceNames;
		public Dictionary <string, int> Index;

		public ResourceConfig (IEnumerable<string> resources)
		{
			ResourceNames = resources.ToList ();
			ResourceLength = ResourceNames.Count;
			Index = new Dictionary<string, int> ();
			for (int i = 0 ; i < ResourceLength ; ++i)
				Index.Add (ResourceNames[i], i);
		}

		public Resources Create () => new Resources (this);
		public Resources.Builder CreateBuilder () => new Resources.Builder (this);

		public Resources Create (Yield yield) => CreateBuilder (yield).ToResources ();
		public Resources.Builder CreateBuilder (Yield yield)
		{
			var resources = new Resources.Builder (this);
			resources[yield.Name] = yield.Amount;
			return resources;
		}

		public Resources Create(IEnumerable <Yield> yields) => CreateBuilder (yields).ToResources ();
		public Resources.Builder CreateBuilder (IEnumerable <Yield> yields)
		{
			var resources = new Resources.Builder (this);
			foreach (var yield in yields.AsNotNull ())
				resources[yield.Name] += yield.Amount;
			return resources;
		}
	}

	public struct ResourceItem
	{
		public string ResourceName;
		public double Value;

		public ResourceItem (string resourceName, double value)
		{
			ResourceName = resourceName;
			Value = value;
		}
	}

	public class Resources : IEnumerable<ResourceItem>
	{
		// During saveload we are deserialized without our config
		// Stash it here during inflation
		public static ResourceConfig SaveLoadConfig { get; set; }

		ResourceConfig Config;
		double [] Inventory;

		int ResourceLength => Config.ResourceLength;
		Dictionary <string, int> Index => Config.Index;

		public Resources (ResourceConfig config)
		{
			Config = config;
			Inventory = new double [ResourceLength];
		}

		// This is invoked during json deserialization
		public Resources (IEnumerable<ResourceItem> values)
		{
			if (SaveLoadConfig == null)
				throw new InvalidOperationException ("Resources (IEnumerable<double>) ctor was invoked without SaveLoadConfig setup");

			Config = SaveLoadConfig;
			Inventory = new double[ResourceLength];
			Array.Copy (values.Select (x => x.Value).ToArray (), Inventory, ResourceLength);
		}

		protected Resources (ResourceConfig config, double [] inventory)
		{
			Config = config;
			Inventory = new double [ResourceLength];
			Array.Copy (inventory, Inventory, ResourceLength);
		}

		public double this [string key]
		{
			get => Inventory [Index[key]];
		}

		public double this [int index]
		{
			get => Inventory [index];
		}

		public Builder ToBuilder ()
		{
			return new Builder (Config, Inventory);
		}

		public Resources Add (Resources right)
		{
			var builder = ToBuilder ();
			builder.Add (right);
			return builder.ToResources ();
		}

		public Resources AddWithMultiply (Resources right, double multiply)
		{
			var builder = ToBuilder ();
			builder.AddWithMultiply (right, multiply);
			return builder;
		}

		public Resources Subtract (Resources right)
		{
			var builder = ToBuilder ();
			builder.Subtract (right);
			return builder;
		}

		public Resources Multiply (double right)
		{
			var builder = ToBuilder ();
			builder.Multiply (right);
			return builder;
		}

		public Resources Multiply (Resources right)
		{
			var builder = ToBuilder ();
			builder.Multiply (right);
			return builder;
		}

		public bool HasMoreThan (Resources right)
		{
			for (int i = 0 ; i < ResourceLength ; ++i) {
				if (right.Inventory[i] > Inventory[i])
					return false;
			}
			return true;
		}

		public bool IsEmpty {
			get {
				for (int i = 0 ; i < ResourceLength ; ++i) {
					if (Inventory[i] != 0)
						return false;
				}
				return true;
			}
		}

		public IEnumerator<ResourceItem> GetEnumerator()
		{
			for (int i = 0 ; i < ResourceLength ; ++i)
				yield return new ResourceItem (Config.ResourceNames[i], Inventory[i]);
		}

		IEnumerator IEnumerable.GetEnumerator()
		{
			return GetEnumerator();
		}

		public class Builder : Resources
		{
			internal Builder (ResourceConfig config, double [] inventory) : base (config, inventory) {}
			internal Builder (ResourceConfig config) : base (config) {}

			public new double this [string key]
			{
				get => Inventory [Index[key]];
				set => Inventory [Index[key]] = value;
			}

			public new double this [int index]
			{
				get => Inventory [index];
				set => Inventory [index] = value;
			}

			public Resources ToResources ()
			{
				return new Resources (Config, Inventory);
			}

			public new void Add (Resources right)
			{
				for (int i = 0 ; i < ResourceLength ; ++i)
					Inventory[i] = Inventory[i] + right[i];
			}

			public new void AddWithMultiply (Resources right, double multiply)
			{
				for (int i = 0 ; i < ResourceLength ; ++i)
					Inventory[i] = Inventory[i] + (right[i] * multiply);
			}

			public new void Subtract (Resources right)
			{
				for (int i = 0 ; i < ResourceLength ; ++i)
					Inventory[i] = Inventory[i] - right[i];
			}

			public new void Multiply (double right)
			{
				if (right == 1)
					return;
				for (int i = 0 ; i < ResourceLength ; ++i)
					Inventory[i] = Inventory[i] * right;
			}

			public new void Multiply (Resources right)
			{
				for (int i = 0; i < ResourceLength; ++i) {
					if (right[i] != 0)
						Inventory[i] = Inventory[i] * right[i];
				}					
			}
		}
	}
}
