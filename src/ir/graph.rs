use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

/// Pipeline configuration for operations
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    pub enable: bool,
    pub initiation_interval: usize, // II - cycles between new inputs
    pub pipeline_depth: usize,      // Number of pipeline stages
    pub unroll_factor: usize,       // Loop unrolling factor
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            enable: false,
            initiation_interval: 1,
            pipeline_depth: 1,
            unroll_factor: 1,
        }
    }
}

/// Pipeline stage information for scheduling
#[derive(Debug, Clone)]
pub struct PipelineStage {
    pub stage: usize,
    pub cycle: usize,
    pub operations: Vec<NodeId>,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Add(ValueId, ValueId),
    Sub(ValueId, ValueId),
    Mul(ValueId, ValueId),
    Div(ValueId, ValueId),
    And(ValueId, ValueId),
    Or(ValueId, ValueId),
    Not(ValueId),
    CmpLt(ValueId, ValueId),
    CmpEq(ValueId, ValueId),
    Load(String),             
    Store(String, ValueId),   
    Const(i64),
    Mux(ValueId, ValueId, ValueId), 
    // Pipeline-specific operations
    PipelineRegister(ValueId),     // Insert pipeline register
    PipelineBarrier,               // Pipeline synchronization point
    Nop,
}

/// An IR node in the graph
#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub op: Operation,
    pub output: Option<ValueId>,
}

/// Main IR container
#[derive(Debug)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub next_value: usize,
    pub next_node: usize,
    pub value_map: HashMap<ValueId, NodeId>, // who produces what
    pub pipeline_config: PipelineConfig,     // Pipeline configuration
    pub pipeline_stages: Vec<PipelineStage>, // Scheduled pipeline stages
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            next_value: 0,
            next_node: 0,
            value_map: HashMap::new(),
            pipeline_config: PipelineConfig::default(),
            pipeline_stages: Vec::new(),
        }
    }

    /// Create a new value ID
    pub fn new_value(&mut self) -> ValueId {
        let id = ValueId(self.next_value);
        self.next_value += 1;
        id
    }

    /// Add a node with output value
    pub fn add_node_with_output(&mut self, op: Operation) -> ValueId {
        let output_value = self.new_value();
        let node = Node {
            id: NodeId(self.next_node),
            op,
            output: Some(output_value),
        };
        
        self.next_node += 1;
        self.value_map.insert(output_value, node.id);
        self.nodes.push(node);
        
        output_value
    }

    /// Add a node without output value
    pub fn add_node(&mut self, op: Operation) -> NodeId {
        let node = Node {
            id: NodeId(self.next_node),
            op,
            output: None,
        };
        
        let node_id = node.id;
        self.next_node += 1;
        self.nodes.push(node);
        
        node_id
    }

    /// Enable pipelining with specified configuration
    pub fn enable_pipeline(&mut self, ii: usize, depth: usize, unroll: usize) {
        self.pipeline_config = PipelineConfig {
            enable: true,
            initiation_interval: ii,
            pipeline_depth: depth,
            unroll_factor: unroll,
        };
    }

    /// Insert a pipeline register for the given value
    pub fn insert_pipeline_register(&mut self, value: ValueId) -> ValueId {
        let reg_value = self.new_value();
        let reg_node = Node {
            id: NodeId(self.next_node),
            op: Operation::PipelineRegister(value),
            output: Some(reg_value),
        };
        
        self.next_node += 1;
        self.value_map.insert(reg_value, reg_node.id);
        self.nodes.push(reg_node);
        
        reg_value
    }

    /// Get operation latency for scheduling
    pub fn get_operation_latency(&self, op: &Operation) -> usize {
        match op {
            Operation::Add(_, _) | Operation::Sub(_, _) => 1,
            Operation::Mul(_, _) => 3, // DSP48 multiplier latency
            Operation::Div(_, _) => 18, // Division latency
            Operation::And(_, _) | Operation::Or(_, _) | Operation::Not(_) => 1,
            Operation::CmpLt(_, _) | Operation::CmpEq(_, _) => 1,
            Operation::Load(_) => 2, // Memory access latency
            Operation::Store(_, _) => 1,
            Operation::Const(_) => 0,
            Operation::Mux(_, _, _) => 1,
            Operation::PipelineRegister(_) => 1,
            Operation::PipelineBarrier => 0,
            Operation::Nop => 0,
        }
    }
}
