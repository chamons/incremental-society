using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace IncrementalSociety.Web.Services
{
	public class ExceptionNotificationService : TextWriter
	{
		public event EventHandler<string> OnException;

		TextWriter ErrorWriter;

		public override Encoding Encoding => Encoding.UTF8;

		public ExceptionNotificationService ()
		{
			ErrorWriter = Console.Error;
			Console.SetError (this);
		}

		public override void WriteLine (string value)
		{
			OnException?.Invoke (this, value);
			ErrorWriter.WriteLine (value);
		}
	}
}
