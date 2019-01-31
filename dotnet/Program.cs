using BenchmarkDotNet.Configs;
using BenchmarkDotNet.Running;
using BenchmarkDotNet.Validators;
using NHapi.Base.Parser;
using NHapi.Base.Util;
using System;
using System.Linq;

namespace ConsoleApp1
{
    internal class Program
    {
       private static void Main(string[] args)
        {
            //RUST
            using (var mh = Native.BuildMessage(NhapiVsRustHL7.ORU_TEXT))
            { //pointer to Message

                using (var fieldValue = Native.GetField(mh.DangerousGetHandle(), "OBR", 7))
                {
                    var fieldValueAsString = fieldValue.AsString();
                    Console.WriteLine($"Rust retrieved value: '{fieldValueAsString}'");
                } //dispose of string handle, freeing up string memeory on the rust side.
            }

            //HL7-DotNetCore
            var hl7Message = new HL7.Dotnetcore.Message(NhapiVsRustHL7.ORU_TEXT);
            hl7Message.ParseMessage();
            var v = hl7Message.GetValue("OBR.7"); //get a rando field from the middle of the thing
            Console.WriteLine($"HL7-DotNetCore retrieved value: '{v}'");

            //NHAPI
            var parser = new PipeParser();
            var hl7Message2 = parser.Parse(NhapiVsRustHL7.ORU_TEXT) as NHapi.Model.V24.Message.ORU_R01;
            var t = new Terser(hl7Message2);
            var field = t.Get("/.OBR-7"); //get a rando field from the middle of the thing
            Console.WriteLine($"NHapi retrieved value: '{field}'");

            //Console.Read();
            var summary = BenchmarkRunner.Run<NhapiVsRustHL7>( new AllowNonOptimized()); //HL7-DotNet has published a debug build :(

        }

        public class AllowNonOptimized : ManualConfig
        {
            public AllowNonOptimized()
            {
                Add(JitOptimizationsValidator.DontFailOnError); // ALLOW NON-OPTIMIZED DLLS

                Add(DefaultConfig.Instance.GetLoggers().ToArray()); // manual config has no loggers by default
                Add(DefaultConfig.Instance.GetExporters().ToArray()); // manual config has no exporters by default
                Add(DefaultConfig.Instance.GetColumnProviders().ToArray()); // manual config has no columns by default
            }
        }
    }
}