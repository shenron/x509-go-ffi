extern "C" {
    fn PrintCerts() -> bool;
}

fn main() {
    unsafe {
        println!("PrintCert: {}", PrintCerts());
    };
}
