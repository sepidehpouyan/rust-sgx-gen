#[macro_use] extern crate lazy_static;

extern crate reactive_net;

mod __authentic_execution;
pub mod __run;

#[allow(unused_imports)] use __authentic_execution::authentic_execution;
#[allow(unused_imports)] use __authentic_execution::authentic_execution::{MODULE_NAME, success, failure, handle_output};
#[allow(unused_imports)] use reactive_net::{ResultCode, ResultMessage};

// Imports and other stuff

//@ sm_output(button_pressed)
pub fn button_pressed(data : &[u8]) {
    debug!("OUTPUT: button_pressed");
	let id : u16 = 0;

    handle_output(id, data);
}

//@ sm_output(output1)
pub fn output1(data : &[u8]) {
    debug!("OUTPUT: output1");
	let id : u16 = 1;

    handle_output(id, data);
}


//@ sm_entry
pub fn press_button(_data : &[u8]) -> ResultMessage {
    debug!("ENTRYPOINT: press_button");

    button_pressed(&[]);

    success(None)
}

//@ sm_input
pub fn input1(data : &[u8]) {
    info!("INPUT: input1");

    output1(data);
}

// User-defined functions and other stuff
