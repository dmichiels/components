#[macro_use]
extern crate rustfbp;

extern crate capnp;

use rustfbp::component::*;

component! {
    LoadBalancer,
    inputs(input),
    inputs_array(),
    outputs(),
    outputs_array(outputs),
    fn run(&mut self) {
        // Find the good output port
        let mut ip_acc = self.ports.recv("acc".into()).unwrap();
        let mut actual = ip_acc.get_reader().expect("cannot get reader");
        let m: number::Reader = actual.get_root().expect("not a number reader");
        let mut actual = m.get_number();

        let mut list = self.ports.get_output_selections("outputs".into()).expect("cannot get outputs");
        if (actual as usize) > list.len()-1 { actual = 0; }
        list.sort_by(|a, b| { (a).cmp((&b)) });
        let port = list.get_mut(actual as usize).unwrap();

        // send the IP
        let ip = self.ports.recv("input".into()).unwrap();
        self.ports.send_array("outputs".into(), port.clone(), ip).ok().expect("LoadBalancer: cannot send");

        
        let mut new_m = super::capnp::message::Builder::new_default();
        {
            let mut number = new_m.init_root::<number::Builder>();
            number.set_number(actual+1);
        }
        ip_acc.write_builder(&new_m);
        self.ports.send("acc".into(), ip_acc).expect("cannot send date");
    }

    mod number_capnp {
        include!("number_capnp.rs");
    }
    use self::number_capnp::number;
}
