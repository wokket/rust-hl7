using BenchmarkDotNet.Attributes;
using NHapi.Base.Parser;
using NHapi.Base.Util;
using NHapi.Model.V24.Message;

namespace ConsoleApp1
{
    [CoreJob]
    [MemoryDiagnoser]

    public class NhapiVsRustHL7
    {
        public const string ACK_TEXT = "MSH|^~\\&|SENDING_APPLICATION|SENDING_FACILITY|RECEIVING_APPLICATION|RECEIVING_FACILITY|20110614075841||ACK|1407511|P|2.3||||||\r" +
                                            "MSA|AA|1407511|Success||";

        public const string ORU_TEXT = 
 "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\r" +
 "PID|||555-44-4444||EVERYWOMAN^EVE^E^^^^L|JONES|19620320|F|||153 FERNWOOD DR.^^STATESVILLE^OH^35292||(206)3345232|(206)752-121||||AC555444444||67-A4335^OH^20030520\r" +
 "OBR|1|845439^GHH OE|1045813^GHH LAB|15545^GLUCOSE|||200202150730|||||||||555-55-5555^PRIMARY^PATRICIA P^^^^MD^^|||||||||F||||||444-44-4444^HIPPOCRATES^HOWARD H^^^^MD\r" +
 "OBX|1|SN|1554-5^GLUCOSE^POST 12H CFST:MCNC:PT:SER/PLAS:QN||^182|mg/dl|70_105|H|||F";
       
        [Benchmark]
        public void NHAPI_Build_Ack()
        {
            var parser = new PipeParser();
            var hl7Message = parser.Parse(ACK_TEXT);
        }

        [Benchmark]
        public void Hl7DotNetCore_Build_Ack()
        {
            var hl7Message = new HL7.Dotnetcore.Message(ACK_TEXT);
            hl7Message.ParseMessage();
        }

        [Benchmark]
        public void Rust_Build_Ack()
        {
            var hl7Message = Native.BuildMessage(ACK_TEXT);
        }

        [Benchmark]
        public void NHAPI_Build_Oru()
        {
            var parser = new PipeParser();
            var hl7Message = parser.Parse(ORU_TEXT);
        }

        [Benchmark]
        public void Hl7DotNetCore_Build_Oru()
        {
            var hl7Message = new HL7.Dotnetcore.Message(ORU_TEXT);
            hl7Message.ParseMessage();
        }

        [Benchmark]
        public void Rust_Build_Oru()
        {
            using (var hl7Message = Native.BuildMessage(ORU_TEXT)) { }
        }

        [Benchmark]
        public void NHAPI_Parse_and_retrieve_field()
        {
            var parser = new PipeParser();
            var hl7Message = parser.Parse(ORU_TEXT) as NHapi.Model.V24.Message.ORU_R01;
            var t = new Terser(hl7Message);
            var fieldValueAsString = t.Get("/.OBR-7"); //get a rando field from the middle of the thing

        }

        [Benchmark]
        public void Hl7DotNetCore_Parse_and_retrieve_field()
        {
            var hl7Message = new HL7.Dotnetcore.Message(ORU_TEXT);
            hl7Message.ParseMessage();
            var fieldValueAsString = hl7Message.GetValue("OBR.7"); //get a rando field from the middle of the thing
        }

        [Benchmark]
        public void Rust_Parse_and_retrieve_field()
        {
            using (var mh = Native.BuildMessage(ORU_TEXT))
            { //pointer to Message, cleaned up via dispose

                using (var fieldValue = Native.GetField(mh.DangerousGetHandle(), "OBR", 7))
                {
                    var fieldValueAsString = fieldValue.AsString();
                } //dispose of string handle, freeing up string memeory on the rust side.
            }

        }
    }
}
