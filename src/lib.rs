#[macro_use]
extern crate vst;
extern crate rosc;

use vst::buffer::AudioBuffer;
use vst::plugin::{Info, Plugin, Category, CanDo};
use vst::api::{Supported, Events};

use std::{f32};
use std::net::{UdpSocket, SocketAddrV4};
use std::str::FromStr;
use rosc::{OscPacket, OscMessage, OscType};
use rosc::encoder;

#[derive(Default)]
struct CVout {
    vals: [f32; 20]
}


impl Plugin for CVout {
    fn get_info(&self) -> Info {
        Info {
            name: "CVout".to_string(),
            vendor: "Jelle Akkerman".to_string(),
            version: 0001,

            unique_id: 702767394, 

            inputs: 0,
            outputs: 0,

            parameters: 20,

            category: Category::Effect,

            ..Default::default()
        }
    }
    fn get_parameter_text(&self, index: i32) -> String {
        format!("{:.*}V", 3, self.vals[index as usize]* 10.0)
    }

    fn set_parameter(&mut self, index: i32, val: f32) {
        self.vals[index as usize] = val;

        let host_addr = SocketAddrV4::from_str("127.0.0.1:8004").unwrap();
        let to_addr = SocketAddrV4::from_str("127.0.0.1:8005").unwrap();
        let sock = UdpSocket::bind(host_addr).unwrap();

        let msg_buf = encoder::encode(&OscPacket::Message(OscMessage {
            addr: format!("/out/{}/cvuni", index),
            args: Some(vec![OscType::Float(val)]),
        }))
        .unwrap();

        sock.send_to(&msg_buf, to_addr).unwrap();
    }
    fn process_events(&mut self, _events: &Events) {}
    fn process(&mut self, _buffer: &mut AudioBuffer<f32>) {}
    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            _ => Supported::Maybe,
        }
    }
}

plugin_main!(CVout); 
