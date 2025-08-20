use serde::{Deserialize, Serialize};

use super::{action_node::ActionNode, block_node::BlockNode, flow_node::FlowNode};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {
    Action(ActionNode),
    Flow(FlowNode),
    Block(BlockNode),
}
