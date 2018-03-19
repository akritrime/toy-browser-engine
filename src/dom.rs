use std::collections::HashMap;

/// Represents a DOM node

struct Node {
    children: Option<Vec<Node>>,
    n_type: NodeType
}

/// Represents the Node types. https://dom.spec.whatwg.org/#dom-node-nodetype
enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String)
}

/// Internal representaion of an element node
struct ElementData {
    tag: String,
    attrs: AttrMap
}

type AttrMap = HashMap<String, String>;

fn text(data: String) -> Node {
    Node {
        children: None,
        n_type: NodeType::Text(data)
    }
}

fn elem(tag: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    let elem_data = ElementData {
        tag,
        attrs
    };
    Node {
       children: Some(children),
       n_type: NodeType::Element(elem_data) 
    }
}

fn comment(data: String) -> Node {
    Node {
        children: None,
        n_type: NodeType::Comment(data)
    }
}

