#[macro_use]
extern crate rustfbp;

use rustfbp::component::*;

component! {
    LoadBalancer,
    inputs(input),
    inputs_array(),
    outputs(),
    outputs_array(outputs),
    fn run(&mut self) {
        // Find the good output port
        let mut actual = self.ports.recv_vecu8("acc".into()).unwrap();
        let mut list = self.ports.get_output_selections("outputs".into()).expect("cannot get outputs");
        if (actual[0] as usize) > list.len()-1 { actual[0] = 0; }
        list.sort_by(|a, b| { (a).cmp((&b)) });
        let port = list.get_mut(actual[0] as usize).unwrap();

        // send the IP
        let ip = self.ports.recv_vecu8("input".into()).unwrap();
        self.ports.send_array_vecu8("outputs".into(), port.clone(), &ip).ok().expect("LoadBalancer: cannot send");
        actual[0] = actual[0] + 1;
        self.ports.send_vecu8("acc".into(), &actual).expect("LoadBalancer : cannot send acc");
    }
}
