fn main() {
    eprintln!("{:#?}", rsinfo::ALL_INFO);
    println!("{:#}", rsinfo::ALL_INFO.to_json())
}
