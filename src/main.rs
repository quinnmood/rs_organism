extern crate organism;

fn main() {
    let org: organism::Organism = organism::import_org_from_json("organism.json", "config.json", 0).unwrap();
    org.print();
}
