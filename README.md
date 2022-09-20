
This rev doesn't separate namespaces from attributes, 
nor does it preserve their original locations.

Instead it uses `HashMap` to return the attributes.

The result is faster, cacheable attribute lookups? and with the namespace included in the key.

```rust
let doc = roxmltree::Document::parse(
    "<e xmlns:ns='http://www.w3.org' a='b' ns:a='c'/>"
).unwrap();

assert!(doc.root_element().has_attribute("a"));
assert!(doc.root_element().has_attribute("ns:a"));

assert_eq!(doc.root_element().attribute("a"), Some("b"));
assert_eq!(doc.root_element().attribute("ns:a"), Some("c"));

// There's also this now. Which is useful if you need the original data back
// after processing it
assert_eq!(doc.root_element().xml(), "<e xmlns:ns='http://www.w3.org' a='b' ns:a='c'/>");
assert_eq!(doc.root_element().attributes_xml(), "a='b' ns:a='c'");

// This rev doesn't separate namespaces from attributes, 
// those lookups are not available in this rev
// assert!(doc.root_element().has_attribute(("http://www.w3.org", "a")));
// assert!(doc.root_element().attribute(("http://www.w3.org", "a")));
```

---

[Original README](https://github.com/ker0olos/roxmltree/blob/master/README.og.md)
