#![feature(test)]
extern crate stopwatch;
use stopwatch::Stopwatch;

mod pipe_parser;

// This is a dev-only executable for testing functionality, not for general distribution.
fn main() {
    let input = get_sample_message();

    let sw = Stopwatch::start_new();

    for _ in 0..1000000 {
        let message = pipe_parser::message_parser::MessageParser::parse_message(input);
    }
    let duration = sw.elapsed_ms();
    println!("1000,000 messges parsed in {}ms", duration);
    println!(
        "This means we parsed {} msgs/sec",
        (1000000 / (duration / 1000))
    );
}

fn get_sample_message() -> &'static str {
    r#" MSH|^~\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4
 PID|||555-44-4444||EVERYWOMAN^EVE^E^^^^L|JONES|19620320|F|||153 FERNWOOD DR.^^STATESVILLE^OH^35292||(206)3345232|(206)752-121||||AC555444444||67-A4335^OH^20030520\r
 OBR|1|845439^GHH OE|1045813^GHH LAB|15545^GLUCOSE|||200202150730|||||||||555-55-5555^PRIMARY^PATRICIA P^^^^MD^^|||||||||F||||||444-44-4444^HIPPOCRATES^HOWARD H^^^^MD\r
 OBX|1|SN|1554-5^GLUCOSE^POST 12H CFST:MCNC:PT:SER/PLAS:QN||^182|mg/dl|70_105|H|||F"#
}
