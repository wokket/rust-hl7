use criterion::{criterion_group, criterion_main, Criterion};
use rusthl7::{decoder::*, separators::Separators};

// Note that we;re calkling decode on a whole message here, although it would normally be on an individual field...
// this is just to make it work a bit harder on a larger dataset, not because it makes sense in a HL7 sense

fn no_sequences(c: &mut Criterion) {
    c.bench_function("No Escape Sequences", |b| {
        let delims = Separators::default();
        let decoder = EscapeSequence::new(delims);

        b.iter(|| {
            let _ = decoder.decode(get_sample_message_no_sequence());
        })
    });
}

fn no_sequences_but_backslash(c: &mut Criterion) {
    c.bench_function("No Escape Sequences But Backslash", |b| {
        let delims = Separators::default();
        let decoder = EscapeSequence::new(delims);

        b.iter(|| {
            let _ = decoder.decode(get_sample_message_with_backslash());
        })
    });
}

fn has_escape_sequences(c: &mut Criterion) {
    c.bench_function("Has Escape Sequences", |b| {
        let delims = Separators::default();
        let decoder = EscapeSequence::new(delims);

        b.iter(|| {
            let _ = decoder.decode(get_sample_message_with_escape_sequences());
        })
    });
}

fn get_sample_message_no_sequence() -> &'static str {
    // note we've stripped the backslash from the MSH
    "MSH|^~*&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rPID|||555-44-4444||EVERYWOMAN^EVE^E^^^^L|JONES|19620320|F|||153 FERNWOOD DR.^^STATESVILLE^OH^35292||(206)3345232|(206)752-121||||AC555444444||67-A4335^OH^20030520\rOBR|1|845439^GHH OE|1045813^GHH LAB|15545^GLUCOSE|||200202150730|||||||||555-55-5555^PRIMARY^PATRICIA P^^^^MD^^|||||||||F||||||444-44-4444^HIPPOCRATES^HOWARD H^^^^MD\rOBX|1|SN|1554-5^GLUCOSE^POST 12H CFST:MCNC:PT:SER/PLAS:QN||^182|mg/dl|70_105|H|||F"
}

fn get_sample_message_with_backslash() -> &'static str {
    //there's a backslash down at char 487!
    "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rPID|||555-44-4444||EVERYWOMAN^EVE^E^^^^L|JONES|19620320|F|||153 FERNWOOD DR.^^STATESVILLE^OH^35292||(206)3345232|(206)752-121||||AC555444444||67-A4335^OH^20030520\rOBR|1|845439^GHH OE|1045813^GHH LAB|15545^GLUCOSE|||200202150730|||||||||555-55-5555^PRIMARY^PATRICIA P^^^^MD^^|||||||||F||||||444-44-4444^HIPPOCRATES^HOWARD H^^^^MD\rOBX|1|SN|1554-5^GLUCOSE^POST 12H CFST:MCNC:PT:SER/PLAS:QN||^182|mg/dl|\\70_105|H|||F"
}

fn get_sample_message_with_escape_sequences() -> &'static str {
    //there's a backslash down at char 487!
    "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rPID|||\\F\\555-44-4444||EVERYWOMAN^EVE^E^^^^L|JONES|19620320|F|||153 FERNWOOD DR.^^STATESVILLE^OH^35292||(206)3345232|(206)752-121||||AC555444444||67-A4335^OH^20030520\rOBR|1|845439^GHH OE|1045813^GHH LAB|15545^GLUCOSE|||200202150730|||||||||555-55-5555^PRIMARY^PATRICIA P^^^^MD^^|||||||||F||||||444-44-4444^HIPPOCRATES^HOWARD H^^^^MD\rOBX|1|SN|1554-5^GLUCOSE^POST 12H CFST:MCNC:PT:SER/PLAS:QN||^182|mg/dl|\\70_105|H|||F"
}

criterion_group!(
    decoder,
    no_sequences,
    no_sequences_but_backslash,
    has_escape_sequences
);
criterion_main!(decoder);
