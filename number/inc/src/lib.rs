#![feature(braced_empty_structs)]
extern crate capnp;

#[macro_use]
extern crate rustfbp;

use rustfbp::component::*;

component! {
    Inc,
    inputs(input),
    inputs_array(),
    outputs(output),
    outputs_array(),
    fn run(&mut self) {
        let mut ip = self.ports.recv("input".into()).expect("cannot receive");
        let m = ip.get_reader().expect("cannot get reader");
        let m: number::Reader = m.get_root().expect("not a number reader");


        let n = m.get_number();
        let mut new_m = super::capnp::message::Builder::new_default();
        {
            let mut number = new_m.init_root::<number::Builder>();
            number.set_number(n+1);
        }

        ip.write_builder(&new_m);
        self.ports.send("output".into(), ip).expect("cannot send date");
    }

    mod number_capnp {
        include!("number_capnp.rs");
    }
    use self::number_capnp::number;

}
