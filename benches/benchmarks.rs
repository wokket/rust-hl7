// #![feature(test)]
// extern crate test;

// #[cfg(test)]
// mod benches {
//     use rusthl7::message_parser::MessageParser;
//     use rusthl7::segment_parser::SegmentParser;
//     use rusthl7::*;
//     use test::Bencher;

//     fn get_sample_message() -> String {
//         "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rPID|||555-44-4444||EVERYWOMAN^EVE^E^^^^L|JONES|19620320|F|||153 FERNWOOD DR.^^STATESVILLE^OH^35292||(206)3345232|(206)752-121||||AC555444444||67-A4335^OH^20030520\rOBR|1|845439^GHH OE|1045813^GHH LAB|15545^GLUCOSE|||200202150730|||||||||555-55-5555^PRIMARY^PATRICIA P^^^^MD^^|||||||||F||||||444-44-4444^HIPPOCRATES^HOWARD H^^^^MD\rOBX|1|SN|1554-5^GLUCOSE^POST 12H CFST:MCNC:PT:SER/PLAS:QN||^182|mg/dl|70_105|H|||F".to_string()
//     }
//     #[bench]
//     fn bench_full_message(b: &mut test::Bencher) {
//         b.iter(
//             || MessageParser::parse_message(get_sample_message()), //note the trailing \r\r
//         );
//     }

//     #[bench]
//     fn message_parse_and_retrieve_field(b: &mut test::Bencher) {
//         b.iter(|| {
//             let msg = MessageParser::parse_message(get_sample_message());
//             let _ = msg.get_segments("OBR")[0].fields[16].get_all_as_string();
//         });
//     }

//     #[bench]
//     fn bench_full_segment(b: &mut Bencher) {
//         b.iter(
//             || SegmentParser::parse_segment(get_sample_segment(), &Seperators::default()), //note the trailing \r\r
//         );
//     }

//     #[bench]
//     fn bench_full_segment_alternate(b: &mut Bencher) {
//         //comparitor for a/b testing
//         b.iter(
//             || SegmentParser::parse_segment(get_sample_segment(), &Seperators::default()), //note the trailing \r\r
//         );
//     }

//     fn get_sample_segment() -> &'static str {
//         "PID|||555-44-4444||EVERYWOMAN^EVE^E^^^^L|JONES|19620320|F|||153 FERNWOOD DR.^^STATESVILLE^OH^35292||(206)3345232|(206)752-121||||AC555444444||67-A4335^OH^20030520\r"
//     }
// }
