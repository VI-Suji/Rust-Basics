use std::sync::mpsc::{self, Receiver};
use std::time::Duration;


pub fn test_mpsc() {
    let(transmitter, reciever) = mpsc::channel::<u8>();
    // drop(reciever);
    // let send_result = transmitter.send(100);
    // println!("Send status: {}", send_result.is_ok());
    // let recieve_result = reciever.recv();
    // println!("Recieved msg of {}", recieve_result.unwrap());
    
    // let send_result = transmitter.send(152);
    // println!("Send status: {}", send_result.is_ok());
    // let recieve_result = reciever.recv();
    // println!("Recieved msg of {}", recieve_result.unwrap());

    let proccessor_code = move || {
        println!("Starting processor thread...");
        let mut failed_count = 0u8;
        loop{
            println!("Attempting to receive msg from channel");
            let recieve_result = reciever.recv_timeout(Duration::from_millis(800));
            if recieve_result.is_ok() {
                println!("Received result {}", recieve_result.unwrap());
            }else{
                failed_count+=1;
            }
            if failed_count > 10 {
                println!("Error");
                break;
            }
        }
    };

    for x in 1..=6{
        let send_result = transmitter.send(x);
        println!("Send data {}",send_result.is_ok());
        std::thread::sleep(Duration::from_millis(200));
    }

    std::thread::spawn(proccessor_code).join();
}