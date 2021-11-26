// image specs read from url
struct ImageSpec {
    specs: Vec<Spec>
}

// kinds of processing supported by the server
enum Spec {
    Resize(Resize),
}

// parameters resize concerned about
struct Resize {
    width: u32,
    height: u32
}

fn main() {
    println!("Hello, world!");
}
