extern crate bambou;
extern crate vspk;

use std::fs::File;
use std::io::Read;
use std::default::Default;

use bambou::{SessionBuilder, RestEntity, Certificate};
use vspk::{Enterprise, Domain, Me};

fn main() {
    let mut session_builder = SessionBuilder::new(
        "https://mvqa03.mv.nuagenetworks.net:8443",
        "csproot",
        "csproot",
        "csp").unwrap();

    let mut buf = Vec::new();
    let _ = File::open("my-cert.der").unwrap().read_to_end(&mut buf).unwrap();
    let cert = Certificate::from_der(&buf).unwrap();
    session_builder.add_root_certificate(cert).unwrap();
    session_builder.danger_disable_hostname_verification();
    let mut session = session_builder.build().unwrap();
    let mut me: Me = Default::default();
    match session.connect(&mut me) {
        Err(e) => {
            println!("{:?}", e);
        },
        Ok(_) => {
            println!("{:?}", me);
        }
    }
}
