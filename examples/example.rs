fn main() {
    let doc = roxmltree::Document::parse("<e xmlns:ns='http://www.w3.org' ns:a='50'/>").unwrap();

    println!("{:?}", doc.root_element().attribute("ns:a").unwrap());

    println!("{:?}", doc.root_element().xml());
    println!("{:?}", doc.root_element().attributes_xml());
}
