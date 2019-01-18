using BenchmarkDotNet.Attributes;
using NHapi.Base.Parser;

namespace ConsoleApp1
{
    [CoreJob]
    
    public class nhapi
    {
        const string ACK_TEXT = "MSH|^~\\&|SENDING_APPLICATION|SENDING_FACILITY|RECEIVING_APPLICATION|RECEIVING_FACILITY|20110614075841||ACK|1407511|P|2.3||||||\r\n" +
                                            "MSA|AA|1407511|Success||";

        const string ORU_TEXT = @"MSH|^~\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4
 PID|||555-44-4444||EVERYWOMAN^EVE^E^^^^L|JONES|19620320|F|||153 FERNWOOD DR.^^STATESVILLE^OH^35292||(206)3345232|(206)752-121||||AC555444444||67-A4335^OH^20030520\r
 OBR|1|845439^GHH OE|1045813^GHH LAB|15545^GLUCOSE|||200202150730|||||||||555-55-5555^PRIMARY^PATRICIA P^^^^MD^^|||||||||F||||||444-44-4444^HIPPOCRATES^HOWARD H^^^^MD\r
 OBX|1|SN|1554-5^GLUCOSE^POST 12H CFST:MCNC:PT:SER/PLAS:QN||^182|mg/dl|70_105|H|||F";

        // instantiate a PipeParser, which handles the "traditional or default encoding"
        private PipeParser _parser;

        [GlobalSetup]
        public void Setup()
        {
            _parser = new PipeParser();
        }

        [Benchmark]
        public void Ack()
        {
            var hl7Message = _parser.Parse(ACK_TEXT);
        }

        [Benchmark]
        public void Oru()
        {
            var hl7Message = _parser.Parse(ORU_TEXT);
        }

        [Benchmark]
        public void Oru_and_retrieve_field()
        {
            var hl7Message = _parser.Parse(ORU_TEXT);
            var field = hl7Message.GetStructure("OBR");

        }
    }
}
