//! Pipeline scheduling and optimization for HLS
//! 
//! This module implements pipeline scheduling algorithms including:
//! - ASAP/ALAP scheduling for pipeline stages
//! - Resource constraint scheduling
//! - Pipeline register insertion
//! - Initiation interval optimization

use crate::ir::graph::{Graph, NodeId, Operation, PipelineStage};
use std::collections::{HashMap, VecDeque};

/// Pipeline scheduler for HLS operations
pub struct PipelineScheduler {
    pub max_stages: usize,
    pub resource_constraints: HashMap<String, usize>, // Resource type -> max count
}

impl PipelineScheduler {
    pub fn new() -> Self {
        let mut resource_constraints = HashMap::new();
        // AMD Alveo U50 resource constraints (approximate)
        resource_constraints.insert("adder".to_string(), 100);
        resource_constraints.insert("multiplier".to_string(), 12); // DSP48E2 slices
        resource_constraints.insert("divider".to_string(), 4);
        resource_constraints.insert("memory".to_string(), 8);
        
        Self {
            max_stages: 16, // Reasonable pipeline depth
            resource_constraints,
        }
    }

    /// Schedule operations into pipeline stages using ASAP scheduling
    pub fn schedule_pipeline(&mut self, graph: &mut Graph) -> Result<(), String> {
        if !graph.pipeline_config.enable {
            return Ok(()); // No pipelining requested
        }

        println!("🔄 Scheduling pipeline with II={}, depth={}", 
                graph.pipeline_config.initiation_interval,
                graph.pipeline_config.pipeline_depth);

        // Step 1: Build dependency graph
        let dependencies = self.build_dependency_graph(graph);
        
        // Step 2: Calculate ASAP (As Soon As Possible) schedule
        let asap_schedule = self.calculate_asap_schedule(graph, &dependencies)?;
        
        // Step 3: Calculate ALAP (As Late As Possible) schedule  
        let alap_schedule = self.calculate_alap_schedule(graph, &dependencies, &asap_schedule)?;
        
        // Step 4: Resource-constrained scheduling
        let final_schedule = self.resource_constrained_schedule(graph, &asap_schedule, &alap_schedule)?;
        
        // Step 5: Insert pipeline registers
        self.insert_pipeline_registers(graph, &final_schedule)?;
        
        // Step 6: Generate pipeline stages
        graph.pipeline_stages = self.generate_pipeline_stages(&final_schedule, graph);
        
        println!("✅ Pipeline scheduled successfully with {} stages", graph.pipeline_stages.len());
        Ok(())
    }

    /// Build dependency graph for scheduling
    fn build_dependency_graph(&self, graph: &Graph) -> HashMap<NodeId, Vec<NodeId>> {
        let mut dependencies = HashMap::new();
        
        for node in &graph.nodes {
            let mut deps = Vec::new();
            
            // Find dependencies based on value usage
            match &node.op {
                Operation::Add(a, b) | Operation::Sub(a, b) | Operation::Mul(a, b) | 
                Operation::Div(a, b) | Operation::And(a, b) | Operation::Or(a, b) |
                Operation::CmpLt(a, b) | Operation::CmpEq(a, b) => {
                    if let Some(producer_a) = graph.value_map.get(a) {
                        deps.push(*producer_a);
                    }
                    if let Some(producer_b) = graph.value_map.get(b) {
                        deps.push(*producer_b);
                    }
                }
                Operation::Not(a) | Operation::PipelineRegister(a) => {
                    if let Some(producer) = graph.value_map.get(a) {
                        deps.push(*producer);
                    }
                }
                Operation::Mux(sel, a, b) => {
                    if let Some(producer_sel) = graph.value_map.get(sel) {
                        deps.push(*producer_sel);
                    }
                    if let Some(producer_a) = graph.value_map.get(a) {
                        deps.push(*producer_a);
                    }
                    if let Some(producer_b) = graph.value_map.get(b) {
                        deps.push(*producer_b);
                    }
                }
                Operation::Store(_, val) => {
                    if let Some(producer) = graph.value_map.get(val) {
                        deps.push(*producer);
                    }
                }
                _ => {} // No dependencies for Load, Const, etc.
            }
            
            dependencies.insert(node.id, deps);
        }
        
        dependencies
    }

    /// Calculate ASAP (As Soon As Possible) schedule
    fn calculate_asap_schedule(&self, graph: &Graph, dependencies: &HashMap<NodeId, Vec<NodeId>>) 
        -> Result<HashMap<NodeId, usize>, String> {
        let mut schedule = HashMap::new();
        let mut ready_queue = VecDeque::new();
        let mut dependency_count = HashMap::new();
        
        // Initialize dependency counts
        for node in &graph.nodes {
            let empty_deps = Vec::new();
            let deps = dependencies.get(&node.id).unwrap_or(&empty_deps);
            dependency_count.insert(node.id, deps.len());
            
            if deps.is_empty() {
                ready_queue.push_back((node.id, 0)); // Start at cycle 0
            }
        }
        
        // Schedule nodes using topological sort
        while let Some((node_id, earliest_cycle)) = ready_queue.pop_front() {
            let node = graph.nodes.iter().find(|n| n.id == node_id).unwrap();
            let latency = graph.get_operation_latency(&node.op);
            let finish_cycle = earliest_cycle + latency;
            
            schedule.insert(node_id, earliest_cycle);
            
            // Update dependent nodes
            for dependent_node in &graph.nodes {
                let empty_deps = Vec::new();
                let deps = dependencies.get(&dependent_node.id).unwrap_or(&empty_deps);
                if deps.contains(&node_id) {
                    let count = dependency_count.get_mut(&dependent_node.id).unwrap();
                    *count -= 1;
                    
                    if *count == 0 {
                        ready_queue.push_back((dependent_node.id, finish_cycle));
                    }
                }
            }
        }
        
        Ok(schedule)
    }

    /// Calculate ALAP (As Late As Possible) schedule
    fn calculate_alap_schedule(&self, graph: &Graph, _dependencies: &HashMap<NodeId, Vec<NodeId>>, 
                              asap: &HashMap<NodeId, usize>) -> Result<HashMap<NodeId, usize>, String> {
        // Find critical path length
        let max_cycle = asap.values().max().copied().unwrap_or(0);
        let target_cycles = max_cycle.min(graph.pipeline_config.pipeline_depth);
        
        let mut schedule = HashMap::new();
        
        // Work backwards from target
        for node in &graph.nodes {
            let asap_time = asap.get(&node.id).copied().unwrap_or(0);
            let slack = target_cycles.saturating_sub(asap_time);
            let alap_time = asap_time + slack;
            
            schedule.insert(node.id, alap_time);
        }
        
        Ok(schedule)
    }

    /// Resource-constrained scheduling
    fn resource_constrained_schedule(&self, graph: &Graph, asap: &HashMap<NodeId, usize>, 
                                   alap: &HashMap<NodeId, usize>) -> Result<HashMap<NodeId, usize>, String> {
        let mut final_schedule = HashMap::new();
        let mut resource_usage: HashMap<usize, HashMap<String, usize>> = HashMap::new();
        
        // Sort nodes by mobility (ALAP - ASAP)
        let mut nodes_by_mobility: Vec<_> = graph.nodes.iter().collect();
        nodes_by_mobility.sort_by_key(|node| {
            let asap_time = asap.get(&node.id).copied().unwrap_or(0);
            let alap_time = alap.get(&node.id).copied().unwrap_or(0);
            alap_time.saturating_sub(asap_time) // Lower mobility = higher priority
        });
        
        for node in nodes_by_mobility {
            let asap_time = asap.get(&node.id).copied().unwrap_or(0);
            let alap_time = alap.get(&node.id).copied().unwrap_or(0);
            let resource_type = self.get_resource_type(&node.op);
            
            // Find earliest feasible slot within [ASAP, ALAP] window
            let mut scheduled_cycle = asap_time;
            for cycle in asap_time..=alap_time {
                let cycle_usage = resource_usage.entry(cycle).or_insert_with(HashMap::new);
                let current_usage = cycle_usage.get(&resource_type).copied().unwrap_or(0);
                let max_usage = self.resource_constraints.get(&resource_type).copied().unwrap_or(1);
                
                if current_usage < max_usage {
                    scheduled_cycle = cycle;
                    cycle_usage.insert(resource_type.clone(), current_usage + 1);
                    break;
                }
            }
            
            final_schedule.insert(node.id, scheduled_cycle);
        }
        
        Ok(final_schedule)
    }

    /// Get resource type for operation
    fn get_resource_type(&self, op: &Operation) -> String {
        match op {
            Operation::Add(_, _) | Operation::Sub(_, _) => "adder".to_string(),
            Operation::Mul(_, _) => "multiplier".to_string(),
            Operation::Div(_, _) => "divider".to_string(),
            Operation::Load(_) | Operation::Store(_, _) => "memory".to_string(),
            _ => "logic".to_string(),
        }
    }

    /// Insert pipeline registers between stages
    fn insert_pipeline_registers(&self, graph: &mut Graph, schedule: &HashMap<NodeId, usize>) 
        -> Result<(), String> {
        let mut registers_to_insert = Vec::new();
        
        // Find values that cross stage boundaries
        for node in &graph.nodes {
            let node_stage = schedule.get(&node.id).copied().unwrap_or(0);
            
            if let Some(output_val) = node.output {
                // Check all consumers of this value
                for consumer in &graph.nodes {
                    let consumer_stage = schedule.get(&consumer.id).copied().unwrap_or(0);
                    
                    if consumer_stage > node_stage + 1 {
                        // Insert pipeline registers for multi-cycle delays
                        let stages_between = consumer_stage - node_stage - 1;
                        registers_to_insert.push((output_val, stages_between));
                    }
                }
            }
        }
        
        // Insert the pipeline registers
        for (value, stages) in registers_to_insert {
            let mut current_value = value;
            for _ in 0..stages {
                current_value = graph.insert_pipeline_register(current_value);
            }
        }
        
        Ok(())
    }

    /// Generate pipeline stages from schedule
    fn generate_pipeline_stages(&self, schedule: &HashMap<NodeId, usize>, _graph: &Graph) -> Vec<PipelineStage> {
        let mut stages = HashMap::new();
        
        for (node_id, &cycle) in schedule {
            let stage = stages.entry(cycle).or_insert_with(|| PipelineStage {
                stage: cycle,
                cycle,
                operations: Vec::new(),
            });
            stage.operations.push(*node_id);
        }
        
        let mut result: Vec<_> = stages.into_values().collect();
        result.sort_by_key(|stage| stage.stage);
        result
    }
}

/// Public interface to run pipeline scheduling on a graph
pub fn run_pipeline_pass(graph: &mut Graph) -> Result<(), String> {
    let mut scheduler = PipelineScheduler::new();
    scheduler.schedule_pipeline(graph)
}
