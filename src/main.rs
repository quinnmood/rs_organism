use organism;
use std::time::Instant;

fn main() {
    let mut seq: Vec<char> = Vec::new();
    for _i in 0..50 {
        for b in ['a', 'c', 'g', 't'] {
            seq.push(b);
        }
    }
    let mut org: organism::Organism =
        organism::from_json("organism.json", 0, Some("config.json")).unwrap();
    println!("Flipping row 0");
    org.print();
    println!("");
    //org.rec_at_mut(0).borrow_mut().flip_row(0);
    println!("");
    org.print();
    //org.remove(2);
    println!("");

    for rec in 0..org.num_recs() {
        org.rec_at_mut(rec).borrow_mut().to_pssm();
    }
    org.print();

    let now = Instant::now();
    for _i in 0..100 {
        org.place(&seq[0..200], None);
    }
    let el_time = now.elapsed();
    println!("took {} seconds", el_time.as_secs());
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
