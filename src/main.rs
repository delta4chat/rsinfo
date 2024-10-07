fn main() {
    let info = rsinfo::all_info();
    eprintln!("{:#?}", info);
    println!("{:#}", info.to_json())
}
