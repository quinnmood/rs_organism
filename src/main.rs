use organism;

fn main() {
    let mut org: organism::Organism =
        organism::import_org_from_json("organism.json", 0, Some("config.json"));
    println!("Flipping row 0");
    org.clone().recognizers[0].print();
    org.recognizers[0].flip_row(0);
    org.clone().recognizers[0].print();

    println!("Flipping row 2");
    org.clone().recognizers[0].print();
    org.recognizers[0].flip_row(2);
    org.clone().recognizers[0].print();

    println!("Flipping col 0");
    org.clone().recognizers[0].print();
    org.recognizers[0].flip_col(0);
    org.clone().recognizers[0].print();

    println!("Flipping col 5");
    org.clone().recognizers[0].print();
    org.recognizers[0].flip_col(5);
    org.clone().recognizers[0].print();

    println!("swapping col 4 and 5");
    org.clone().recognizers[0].print();
    org.recognizers[0].swap_cols(4, 5);
    org.clone().recognizers[0].print();

    println!("shifting left");
    org.clone().recognizers[0].print();
    org.recognizers[0].shift_left();
    org.clone().recognizers[0].print();
}
