#![feature(braced_empty_structs)]
extern crate capnp;

#[macro_use]
extern crate rustfbp;

use rustfbp::component::*;


component! {
    Inc,
    inputs(input),
    inputs_array(a),
    outputs(output),
    outputs_array(a),
    fn run(&mut self) {
        let m = self.inputs.input.recv().expect("cannot receive");
        let m: number::Reader = m.get_root().expect("not a date reader");

        let n = m.get_number();
        
        let mut new_m = super::capnp::message::Builder::new_default();
        {
            let mut number = new_m.init_root::<number::Builder>();
            number.set_number(n+1);
        }
        self.outputs.output.send(&new_m).expect("cannot send date");

    }

    mod number_capnp {
        include!("number_capnp.rs");
    }
    use self::number_capnp::number;

}
