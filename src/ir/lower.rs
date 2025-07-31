use crate::dsl::ast::*;
use crate::ir::graph::{Graph, Operation, ValueId};
use std::collections::HashMap;

/// Lower a single expression to IR graph
pub fn lower_expr_to_graph(expr: &Expr) -> Graph {
    let mut graph = Graph::new();
    let mut env: HashMap<String, ValueId> = HashMap::new();
    
    let _result = lower_expr(expr, &mut graph, &mut env);
    graph
}

/// Lower an expression recursively, building the IR graph
fn lower_expr(expr: &Expr, graph: &mut Graph, env: &mut HashMap<String, ValueId>) -> ValueId {
    match expr {
        Expr::Const { value, width: _ } => {
            graph.add_node_with_output(Operation::Const(*value as i64))
        }
        
        Expr::Input { name, width: _ } => {
            // Check if we already have this input in our environment
            if let Some(&existing_val) = env.get(name) {
                existing_val
            } else {
                // Create a new input (load operation)
                let val_id = graph.add_node_with_output(Operation::Load(name.clone()));
                env.insert(name.clone(), val_id);
                val_id
            }
        }
        
        Expr::Add(left, right) => {
            let l = lower_expr(left, graph, env);
            let r = lower_expr(right, graph, env);
            graph.add_node_with_output(Operation::Add(l, r))
        }
        
        Expr::Sub(left, right) => {
            let l = lower_expr(left, graph, env);
            let r = lower_expr(right, graph, env);
            graph.add_node_with_output(Operation::Sub(l, r))
        }
        
        Expr::Mul(left, right) => {
            let l = lower_expr(left, graph, env);
            let r = lower_expr(right, graph, env);
            graph.add_node_with_output(Operation::Mul(l, r))
        }
        
        Expr::Output { name, expr } => {
            let val = lower_expr(expr, graph, env);
            graph.add_node(Operation::Store(name.clone(), val));
            val // Return the value being stored
        }
    }
}
