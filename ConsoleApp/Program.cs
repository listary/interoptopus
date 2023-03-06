using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ConsoleApp
{
    internal class Program
    {
        static void Main(string[] args)
        {
            SearchResult result = SearchResult.New();
            IntPtr detail = result.GetResultDetail(3);
            return;
        }
    }
}
