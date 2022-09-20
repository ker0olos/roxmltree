fn main() {
    let doc = roxmltree::Document::parse("<e xmlns:ns='http://www.w3.org' a:ns='50'/>").unwrap();

    println!("{:?}", doc.root_element().attribute("a:ns").unwrap());
}
