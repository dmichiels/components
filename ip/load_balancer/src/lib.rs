#[macro_use]
extern crate rustfbp;

use rustfbp::component::*;

component! {
    LoadBalancer,
    inputs(input),
    inputs_array(a),
    outputs(),
    outputs_array(outputs),
    fn run(&mut self) {
        // Find the good output port
        let mut actual = self.inputs.acc.recv_vecu8().unwrap();
        if (actual[0] as usize) > self.outputs_array.outputs.len()-1 { actual[0] = 0; }
        let mut list: Vec<_> = self.outputs_array.outputs.iter_mut().collect();
        list.sort_by(|a, b| { (a.0).cmp((&b.0)) });
        let port = list.get_mut(actual[0] as usize).unwrap();

        // send the IP
        let ip = self.inputs.input.recv_vecu8().unwrap();
        (port.1).send_vecu8(&ip).ok().expect("LoadBalancer: cannot send");
        actual[0] = actual[0] + 1;
        self.outputs.acc.send_vecu8(&actual).expect("LoadBalancer : cannot send acc");
    }
}
