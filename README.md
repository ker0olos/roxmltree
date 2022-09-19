
This rev doesn't separate namespaces from attributes, 
nor does it preserve their original locations.

Instead it uses `HashMap` to store attributes.

The result is faster attribute lookups faster and with the namespace included in the key.

```rust
let doc = roxmltree::Document::parse(
    "<e xmlns:ns='http://www.w3.org' a='b' ns:a='c'/>"
).unwrap();

assert!(doc.root_element().has_attribute("a"));
assert!(doc.root_element().has_attribute("ns:a"));

assert_eq!(doc.root_element().attribute("a"), Some("b"));
assert_eq!(doc.root_element().attribute("ns:a"), Some("c"));

// This rev doesn't separate namespaces from attributes, 
// those lookups are not available in this rev
//
// assert!(doc.root_element().has_attribute(("http://www.w3.org", "a")));
// assert!(doc.root_element().attribute(("http://www.w3.org", "a")));
```

![Original README](https://github.com/ker0olos/roxmltree/blob/master/README.og.md)
