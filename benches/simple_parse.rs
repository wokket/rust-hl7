use criterion::{criterion_group, criterion_main, Criterion};
use rusthl7::{message::*, segments::Segment};

fn get_sample_message() -> String {
    "MSH|^~\\&|GHH LAB|ELAB-3|GHH OE|BLDG4|200202150930||ORU^R01|CNTRL-3456|P|2.4\rPID|||555-44-4444||EVERYWOMAN^EVE^E^^^^L|JONES|19620320|F|||153 FERNWOOD DR.^^STATESVILLE^OH^35292||(206)3345232|(206)752-121||||AC555444444||67-A4335^OH^20030520\rOBR|1|845439^GHH OE|1045813^GHH LAB|15545^GLUCOSE|||200202150730|||||||||555-55-5555^PRIMARY^PATRICIA P^^^^MD^^|||||||||F||||||444-44-4444^HIPPOCRATES^HOWARD H^^^^MD\rOBX|1|SN|1554-5^GLUCOSE^POST 12H CFST:MCNC:PT:SER/PLAS:QN||^182|mg/dl|70_105|H|||F".to_string()
}

fn message_parse(c: &mut Criterion) {
    let msg = &get_sample_message();
    c.bench_function("oru parse", |b| {
        b.iter(|| {
            let m = Message::from_str(msg).unwrap();
            let seg = m.segments.first();

            if let Some(Segment::MSH(msh)) = seg {
                let _app = msh.msh_3_sending_application.as_ref().unwrap();
                //println!("{}", _app.value());
            }
        })
    });
}

criterion_group!(benches, message_parse);
criterion_main!(benches);
