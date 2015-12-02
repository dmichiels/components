#![feature(braced_empty_structs)]
extern crate capnp;

#[macro_use]
extern crate rustfbp;

use rustfbp::component::*;


component! {
    Add,
    inputs(a),
    inputs_array(numbers),
    outputs(output),
    outputs_array(a),
    fn run(&mut self) {
        let mut acc = 0;
        for ins in self.ports.get_input_selections("numbers").expect("numbers input port doesn't exist") {
            let m = self.ports.recv_array("numbers".into(), ins).expect("cannot receive");
            let m: number::Reader = m.get_root().expect("not a date reader");
            let n = m.get_number();

            acc += n;
        }

        let mut new_m = super::capnp::message::Builder::new_default();
        {
            let mut number = new_m.init_root::<number::Builder>();
            number.set_number(acc);
        }
        self.ports.send("output".into(), &new_m).expect("cannot send date");

    }

    mod number_capnp {
        include!("number_capnp.rs");
    }
    use self::number_capnp::number;

}
