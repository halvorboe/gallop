use crate::protos::coordinator::Node;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct NodeData {
    id: String,
}

impl NodeData {
    fn from_string(node_string: &String) -> Self {
        serde_json::from_str(node_string).unwrap()
    }

    fn from_node(node: &Node) -> Self {
        Self {
            id: node.get_id().to_string(),
        }
    }

    fn to_node(&self) -> Node {
        let mut node = Node::new();
        node.set_id(self.id.clone());
        node
    }

    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub fn encode_node(node: &Node) -> String {
    let node_data = NodeData::from_node(&node);
    node_data.to_string()
}

pub fn decode_node(node_string: &String) -> Node {
    let node_data = NodeData::from_string(node_string);
    node_data.to_node()
}

#[allow(unused_variables, unused_imports)]
mod tests {

    use super::{decode_node, encode_node};
    use crate::protos::coordinator::Node;

    #[test]
    fn test_basic() {
        let mut node = Node::new();
        node.set_id("123".to_string());
        assert_eq!(node, decode_node(&encode_node(&node)));
        // TODO: Test all fields.
    }
}
