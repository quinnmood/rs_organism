use organism;

fn main() {
    let mut org: organism::Organism = organism::from_json("organism.json", 0, Some("config.json")).unwrap();
    println!("Flipping row 0");
    org.print();
    org.rec_at_mut(0).borrow_mut().flip_row(0);
    println!("");
    org.print();
/*
    println!("Flipping row 2");
    org.recognizers[0].print();
    org.recognizers[0].flip_row(2);
    org.recognizers[0].print();

    println!("Flipping col 0");
    org.recognizers[0].print();
    org.recognizers[0].flip_col(0);
    org.recognizers[0].print();

    println!("Flipping col 5");
    org.recognizers[0].print();
    org.recognizers[0].flip_col(5);
    org.recognizers[0].print();

    println!("swapping col 4 and 5");
    org.recognizers[0].print();
    org.recognizers[0].swap_cols(4, 5);
    org.recognizers[0].print();

    println!("shifting left");
    org.recognizers[0].print();
    org.recognizers[0].shift_left();
    org.recognizers[0].print();
*/
}
