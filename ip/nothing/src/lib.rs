#[macro_use]
extern crate rustfbp;

extern crate capnp;

use rustfbp::component::*;

component! {
    LoadBalancer,
    inputs(input: any),
    inputs_array(),
    outputs(output: any),
    outputs_array(),
    option(),
    acc(),
    fn run(&mut self) {
        // send the IP
        let ip = self.ports.recv("input".into()).unwrap();
        self.ports.send("output".into(), ip).ok().expect("LoadBalancer: cannot send");
    }
}
