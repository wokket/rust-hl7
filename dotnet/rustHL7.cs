using BenchmarkDotNet.Attributes;

namespace ConsoleApp1
{

    [CoreJob]
    [MemoryDiagnoser]
    public class rustHL7
    {

        [Benchmark]
        public void Ack()
        {
            var hl7Message = Native.BuildMessage(nhapi.ACK_TEXT);
        }

        //[Benchmark]
        //public void Oru_Parse()
        //{
        //    var hl7Message = _parser.Parse(ORU_TEXT);
        //}

    }




}
