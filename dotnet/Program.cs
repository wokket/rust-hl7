using NHapi.Base.Parser;
using System;
using System.Linq;
using System.Runtime.InteropServices;

namespace ConsoleApp1
{
    internal class Program
    {
       private static void Main(string[] args)
        {
            var h = Native.NewTestStructRef(); //seems to work, get a handle to a rust struct
                                        // var test = Marshal.PtrToStructure<TestStruct>(h); //can safely (?!) get the data behind the pointer
            var fifteen = Native.AddToStruct(h, 10); //call a &self method in rust from the pointer
            var twenty = Native.AddToStruct(h, 5);
            var test1 = Marshal.PtrToStructure<Native.TestStruct>(h); //do we need to re-marshal (as it copies from unmanaged to .net heap)...
            var twentyfive = Native.AddToStruct(h, 5);
            test1.count = 7;
            Marshal.StructureToPtr(test1, h, true); 

            Native.FreeStruct(h);

            using (var mh = Native.BuildMessage("ACK|A01"))
            { //pointer to Message

                //using (var fieldValue = Native.GetField(mh.DangerousGetHandle()))
                //{
                //    var fieldValueAsString = fieldValue.AsString();
                //} //dispose of string handle, freeing up string memeory on the rust side.
            }
            //IntPtr ptrToS;
            //Marshal.StructureToPtr(s, )
            //Add(ptrToS, 5);

            //Console.WriteLine(Add(4, 7));

            var result = Native.ping();
            Console.WriteLine(result);

            var _parser = new PipeParser();
            var hl7Message = _parser.Parse(ORU_TEXT) as NHapi.Model.V24.Message.ORU_R01;
            var field = hl7Message.PATIENT_RESULTs.First().ORDER_OBSERVATIONs.First().OBR.GetOrderingProvider(0);

            Console.Read();
            //   var summary = BenchmarkRunner.Run<nhapi>();
        }

       



        const string ORU_TEXT = @"MSH|^~\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4
 PID|||555-44-4444||EVERYWOMAN^EVE^E^^^^L|JONES|19620320|F|||153 FERNWOOD DR.^^STATESVILLE^OH^35292||(206)3345232|(206)752-121||||AC555444444||67-A4335^OH^20030520
 OBR|1|845439^GHH OE|1045813^GHH LAB|15545^GLUCOSE|||200202150730|||||||||555-55-5555^PRIMARY^PATRICIA P^^^^MD^^|||||||||F||||||444-44-4444^HIPPOCRATES^HOWARD H^^^^MD
 OBX|1|SN|1554-5^GLUCOSE^POST 12H CFST:MCNC:PT:SER/PLAS:QN||^182|mg/dl|70_105|H|||F";
    }
}