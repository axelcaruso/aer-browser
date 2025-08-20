use std::collections::HashMap;

pub struct Node {

    pub children: Vec<Node>,

    pub node_type: NodeType,
}


pub enum NodeType {

    Text(String),

    Element(ElementData),

}


#[derive(PartialEq, Eq, Debug)]
pub struct ElementData {

    pub tag_name: String,

    pub attributes: HashMap<String, String>,
}