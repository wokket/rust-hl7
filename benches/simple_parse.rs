use criterion::{criterion_group, criterion_main, Criterion};
use rusthl7::{message::*};
use std::convert::TryFrom;

fn get_sample_message() -> &'static str {
    "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rPID|||555-44-4444||EVERYWOMAN^EVE^E^^^^L|JONES|19620320|F|||153 FERNWOOD DR.^^STATESVILLE^OH^35292||(206)3345232|(206)752-121||||AC555444444||67-A4335^OH^20030520\rOBR|1|845439^GHH OE|1045813^GHH LAB|15545^GLUCOSE|||200202150730|||||||||555-55-5555^PRIMARY^PATRICIA P^^^^MD^^|||||||||F||||||444-44-4444^HIPPOCRATES^HOWARD H^^^^MD\rOBX|1|SN|1554-5^GLUCOSE^POST 12H CFST:MCNC:PT:SER/PLAS:QN||^182|mg/dl|70_105|H|||F"
}

fn message_parse(c: &mut Criterion) {
    c.bench_function("ORU parse", |b| {
        b.iter(|| {
            let _ = Message::try_from(get_sample_message()).unwrap();
        })
    });
}

fn get_segments_by_name(c: &mut Criterion) {
    c.bench_function("Get Segment By Name", |b| {
        let m = Message::try_from(get_sample_message()).unwrap();

        b.iter(|| {
            let _segs = m.segments_by_name("OBR").unwrap();
            //assert!(segs.len() == 1);
        })
    });
}

fn get_pid_and_read_field_via_vec(c: &mut Criterion) {
    c.bench_function("Read Field from PID (lookup)", |b| {
        let m = Message::try_from(get_sample_message()).unwrap();

        b.iter(|| {
            let pid = &m.segments[1];
            let _field = pid[3];
            assert_eq!(_field, "555-44-4444"); // lookup from vec
        })
    });
}

fn get_pid_and_read_field_via_query(c: &mut Criterion) {
    c.bench_function("Read Field from PID (query)", |b| {
        let m = Message::try_from(get_sample_message()).unwrap();

        b.iter(|| {
            let _val = m.query("PID.F3"); // query via Message
            assert_eq!(_val, "555-44-4444"); // lookup from vec
        })
    });
}

#[cfg(feature = "string_index")]
fn get_pid_and_read_field_via_index(c: &mut Criterion) {
    c.bench_function("Read Field from PID (index)", |b| {
        let m = Message::try_from(get_sample_message()).unwrap();

        b.iter(|| {
            let _val = m["PID.F3"]; // query via Message
            assert_eq!(_val, "555-44-4444"); // lookup from vec
        })
    });
}

#[cfg(feature = "string_index")]
criterion_group!(
    benches,
    message_parse,
    get_segments_by_name,
    get_pid_and_read_field_via_vec,
    get_pid_and_read_field_via_query,
    get_pid_and_read_field_via_index
);

#[cfg(not(feature = "string_index"))]
criterion_group!(
    benches,
    message_parse,
    get_segments_by_name,
    get_pid_and_read_field_via_vec,
    get_pid_and_read_field_via_query
);
criterion_main!(benches);
