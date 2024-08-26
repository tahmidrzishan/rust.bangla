fn main() {
    for i in 1..11 {
        println!("Amar number holo {}", i);
    }

    let names = vec!["amar", "nam", "mofiz"];

    for (index, i) in names.iter().enumerate() {
        println!("Word er number hoilo {} ar word hoilo {}", index, i);
    }
}
