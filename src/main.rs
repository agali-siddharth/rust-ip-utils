use experiments::ip_utils;

fn main() {
    println!("Hello, world!");
    let ip_string = "255.255.224.0";
    println!(
        "subnet {} is {}",
        ip_string,
        if ip_utils::is_subnet_mask_valid(ip_string) {
            "valid"
        } else {
            "invalid"
        },
    );
}
