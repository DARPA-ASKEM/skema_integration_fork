use std::clone::Clone;
use crate::ast::{Math, MathExpression, MathExpression::{Mfrac, Mi, Mn, Mo, Mrow, Msqrt, Msup, Msubsup, Mover}, Operator};


use petgraph::visit::NodeRef;
use petgraph::{graph::NodeIndex, Graph};

use std::collections::VecDeque;

pub type MathExpressionGraph<'a> = Graph<String, String>;

use std::collections::HashMap;
use std::string::ToString;
use clap::builder::Str;

#[derive(Debug, PartialEq, Clone)]
enum Atom {
    Number(String),
    Identifier(String),
    Operator(Operator),
}

#[derive(Debug, PartialEq, Clone)]
enum Expr {
    Atom(Atom),
    Expression {
        op: Vec<Operator>,
        args: Vec<Expr>,
        name: String,
    },
}

/// Intermediate data structure to support the generation of graphs of mathematical expressions
#[derive(Debug, PartialEq, Clone)]
pub struct PreExp {
    op: Vec<Operator>,
    args: Vec<Expr>,
    name: String,
}

pub fn is_derivative(xs1: &mut Box<MathExpression>, xs2: &mut Box<MathExpression>) -> bool {
    let mut cond1 = false;
    let mut cond2 = false;
    match &**xs1 {
        Mrow(x) => {
            match &x[0] {
                Mi(x) => {
                    if x == "d" {
                        cond1 = true;
                    }
                }
                _ => ()
            }
        }
        _ => ()
    };

    match &**xs2 {
        Mrow(x) => {
            match &x[0] {
                Mi(x) => {
                    if x == "d" {
                        cond2 = true;
                    }
                }
                _ => ()
            }
        }
        _ => ()
    };
    if cond1 && cond2 {
        match &mut **xs1 {
            Mrow(x) => {
                x.remove(0);
            }
            _ => ()
        };

        match &mut **xs2 {
            Mrow(x) => {
                x.remove(0);
            }
            _ => ()
        };

        return true;
    }
    return false;
}

impl MathExpression {
    pub fn to_expr(self, pre: &mut PreExp) {
        match self {
            Mi(x) => {
                if pre.args.len() > 0 {
                    let args_last_idx = pre.args.len() - 1;
                    match &mut pre.args[args_last_idx] {
                        Expr::Atom(y) => {
                            match y {
                                Atom::Number(_) => {}
                                Atom::Identifier(_) => {}
                                Atom::Operator(z) => {
                                    if *z == Operator::Subtract {
                                        let mut neg_identifier = String::from("-");
                                        neg_identifier.push_str(&x.clone());
                                        pre.args[args_last_idx] = Expr::Atom(Atom::Identifier(neg_identifier));
                                        return;
                                    }
                                }
                            }
                        }
                        Expr::Expression { .. } => {}
                    }
                }
                if pre.args.len() >= pre.op.len() {
                    // deal with the invisible multiply operator
                    pre.op.push(Operator::Multiply);
                }
                pre.args.push(Expr::Atom(Atom::Identifier(x.replace(" ", ""))));
            }
            Mn(x) => {
                if pre.args.len() >= pre.op.len() {
                    // deal with the invisible multiply operator
                    pre.op.push(Operator::Multiply);
                }
                pre.args.push(Expr::Atom(Atom::Number(x.replace(" ", ""))));
            }
            Mo(x) => {
                if x == Operator::Subtract && pre.op.len() > pre.args.len() {
                    pre.op.push(x);
                    pre.args.push(Expr::Atom(Atom::Identifier("place_holder".to_string())));
                } else {
                    pre.op.push(x);
                }
            }
            Mrow(xs) => {
                if pre.args.len() >= pre.op.len() {
                    // deal with the invisible multiply operator
                    pre.op.push(Operator::Multiply);
                }
                let mut pre_exp = PreExp {
                    op: Vec::<Operator>::new(),
                    args: Vec::<Expr>::new(),
                    name: "".to_string(),
                };
                pre_exp.op.push(Operator::Other("".to_string()));
                for x in xs {
                    x.to_expr(&mut pre_exp);
                }
                pre.args.push(
                    Expr::Expression {
                        op: pre_exp.op,
                        args: pre_exp.args,
                        name: "".to_string(),
                    }
                    ,
                );
            }
            Msubsup(xs) => {
                if xs.len() != 3 {
                    return;
                }
                if pre.args.len() >= pre.op.len() {
                    // deal with the invisible multiply operator
                    pre.op.push(Operator::Multiply);
                }
                let mut pre_exp = PreExp {
                    op: Vec::<Operator>::new(),
                    args: Vec::<Expr>::new(),
                    name: "".to_string(),
                };
                pre_exp.op.push(Operator::Other("".to_string()));
                let mut idx = 0;
                for x in xs {
                    if idx == 0 {
                        pre_exp.op.push(Operator::Other("_".to_string()));
                    } else if idx == 1 {
                        pre_exp.op.push(Operator::Other("^".to_string()));
                    }
                    idx = idx + 1;
                    x.to_expr(&mut pre_exp);
                }
                pre.args.push(
                    Expr::Expression {
                        op: pre_exp.op,
                        args: pre_exp.args,
                        name: "".to_string(),
                    }
                    ,
                );
            }
            Msqrt(xs) => {
                if pre.args.len() >= pre.op.len() {
                    // deal with the invisible multiply operator
                    pre.op.push(Operator::Multiply);
                }
                let mut pre_exp = PreExp {
                    op: Vec::<Operator>::new(),
                    args: Vec::<Expr>::new(),
                    name: "".to_string(),
                };
                pre_exp.op.push(Operator::Sqrt);
                xs.to_expr(&mut pre_exp);
                pre.args.push(
                    Expr::Expression {
                        op: pre_exp.op,
                        args: pre_exp.args,
                        name: "".to_string(),
                    }
                    ,
                );
            }
            Mfrac(mut xs1, mut xs2) => {
                if pre.args.len() >= pre.op.len() {
                    // deal with the invisible multiply operator
                    pre.op.push(Operator::Multiply);
                }
                let mut pre_exp = PreExp {
                    op: Vec::<Operator>::new(),
                    args: Vec::<Expr>::new(),
                    name: "".to_string(),
                };
                if is_derivative(&mut xs1, &mut xs2) {
                    pre_exp.op.push(Operator::Other("derivative".to_string()));
                } else {
                    pre_exp.op.push(Operator::Other("".to_string()));
                }
                xs1.to_expr(&mut pre_exp);
                pre_exp.op.push(Operator::Divide);
                xs2.to_expr(&mut pre_exp);
                pre.args.push(
                    Expr::Expression {
                        op: pre_exp.op,
                        args: pre_exp.args,
                        name: "".to_string(),
                    }
                    ,
                );
            }
            Msup(xs1, xs2) => {
                if pre.args.len() >= pre.op.len() {
                    // deal with the invisible multiply operator
                    pre.op.push(Operator::Multiply);
                }
                let mut pre_exp = PreExp {
                    op: Vec::<Operator>::new(),
                    args: Vec::<Expr>::new(),
                    name: "".to_string(),
                };
                pre_exp.op.push(Operator::Other("".to_string()));
                xs1.to_expr(&mut pre_exp);
                pre_exp.op.push(Operator::Other("^".to_string()));
                xs2.to_expr(&mut pre_exp);
                pre.args.push(
                    Expr::Expression {
                        op: pre_exp.op,
                        args: pre_exp.args,
                        name: "".to_string(),
                    }
                    ,
                );
            }
            Mover(xs) => {
                if xs.len() != 2 {
                    return;
                }
                if pre.args.len() >= pre.op.len() {
                    // deal with the invisible multiply operator
                    pre.op.push(Operator::Multiply);
                }
                let mut pre_exp = PreExp {
                    op: Vec::<Operator>::new(),
                    args: Vec::<Expr>::new(),
                    name: "".to_string(),
                };
                pre_exp.op.push(Operator::Other("".to_string()));
                for x in xs {
                    x.to_expr(&mut pre_exp);
                }
                pre_exp.op.remove(0);
                pre.args.push(
                    Expr::Expression {
                        op: pre_exp.op,
                        args: pre_exp.args,
                        name: "".to_string(),
                    }
                    ,
                );
            }
            _ => {
                panic!("Unhandled type!");
            }
        }
    }

    pub fn to_graph(self) -> MathExpressionGraph<'static> {
        let mut pre_exp = PreExp {
            op: Vec::<Operator>::new(),
            args: Vec::<Expr>::new(),
            name: "root".to_string(),
        };
        pre_exp.op.push(Operator::Other("root".to_string()));
        self.to_expr(&mut pre_exp);
        pre_exp.group_expr();
        pre_exp.collapse_expr();
        pre_exp.get_names();

        pre_exp.to_graph()
    }
}

impl Expr {
    fn group_expr(&mut self) {
        match self {
            Expr::Atom(_) => {}
            Expr::Expression { op, args, .. } => {
                let mut removed_idx = Vec::new();
                let op_copy = op.clone();
                let args_copy = args.clone();
                if op.len() > 2 {
                    let mut start_idx: i32 = -1;
                    let mut end_idx: i32 = -1;
                    let mut new_exp = Expr::Expression {
                        op: vec![Operator::Other("".to_string())],
                        args: Vec::<Expr>::new(),
                        name: "".to_string(),
                    };
                    for o in 0..=op.len() - 1 {
                        if op[o] == Operator::Multiply || op[o] == Operator::Divide {
                            removed_idx.push(o);
                            if start_idx == -1 {
                                start_idx = o as i32;
                                end_idx = o as i32;
                                match &mut new_exp {
                                    Expr::Atom(_) => {}
                                    Expr::Expression { op, args, .. } => {
                                        op.push(op_copy[o].clone());
                                        args.push(args_copy[o - 1].clone());
                                        args.push(args_copy[o].clone());
                                    }
                                }
                            } else if o as i32 - end_idx == 1 {
                                end_idx = o as i32;
                                match &mut new_exp {
                                    Expr::Atom(_) => {}
                                    Expr::Expression { op, args, .. } => {
                                        op.push(op_copy[o].clone());
                                        args.push(args_copy[o].clone())
                                    }
                                }
                            } else {
                                args[start_idx as usize - 1] = new_exp.clone();
                                new_exp = Expr::Expression {
                                    op: vec![Operator::Other("".to_string())],
                                    args: Vec::<Expr>::new(),
                                    name: "".to_string(),
                                };
                                match &mut new_exp {
                                    Expr::Atom(_) => {}
                                    Expr::Expression { op, args, .. } => {
                                        op.push(op_copy[o].clone());
                                        args.push(args_copy[o - 1].clone());
                                        args.push(args_copy[o].clone());
                                    }
                                }
                                start_idx = o as i32;
                                end_idx = o as i32;
                            }
                        }
                    }

                    if removed_idx.len() == op.len() - 1 {
                        return;
                    }

                    match &mut new_exp {
                        Expr::Atom(_) => {}
                        Expr::Expression { op, .. } => {
                            if !op.is_empty() && start_idx > 0 {
                                args[start_idx as usize - 1] = new_exp.clone();
                            }
                        }
                    }
                    for ri in removed_idx.iter().rev() {
                        op.remove(*ri);
                        args.remove(*ri);
                    }
                }

                for arg in args {
                    match arg {
                        Expr::Atom(_) => {}
                        Expr::Expression { .. } => {
                            arg.group_expr();
                        }
                    }
                }
            }
        }
    }

    fn collapse_expr(&mut self) {
        match self {
            Expr::Atom(_) => {}
            Expr::Expression { op, args, .. } => {
                let mut op_copy = op.clone();
                let mut args_copy = args.clone();

                let mut shift = 0;
                if all_multi_div(op) && op.len() > 1 {
                    let mut changed = true;
                    while changed {
                        for i in 0..args.len() {
                            match &mut args[i] {
                                Expr::Atom(_) => {}
                                Expr::Expression { op, args, name } => {
                                    if op[0] == Operator::Other("".to_string()) && all_multi_div(op) {
                                        args_copy[i] = args[0].clone();
                                        for j in 1..op.len() {
                                            op_copy.insert(i + shift + j, op[j].clone());
                                            args_copy.insert(i + shift + j, args[j].clone());
                                        }
                                        shift = shift + op.len() - 1;
                                    }
                                }
                            }
                        }
                        if op.clone() == op_copy.clone() {
                            changed = false;
                        }
                        *op = op_copy.clone();
                        *args = args_copy.clone();
                    }
                }

                for mut arg in args {
                    arg.collapse_expr();
                }
            }
        }
    }

    fn get_names(&mut self) -> String {
        let mut add_paren = false;
        match self {
            Expr::Atom(_) => {
                "".to_string()
            }
            Expr::Expression { op, args, name } => {
                if op[0] == Operator::Other("".to_string()) && !all_multi_div(op) && !redundant_paren(name) {
                    name.push('(');
                    add_paren = true;
                }
                for i in 0..=op.len() - 1 {
                    if i > 0 {
                        if op[i] == Operator::Equals {
                            let mut new_name: String = "".to_string();
                            for n in name.as_bytes().clone(){
                                if *n == 40 as u8{
                                    new_name.push_str("(");
                                }
                                if *n == 41 as u8{
                                    new_name.push_str(")");
                                }
                            }
                            *name = remove_paren(&mut new_name).clone();
                            //
                            // let mut remove_idx = Vec::new();
                            // let mut x: i32 = (name.len() - 1) as i32;
                            // if x > 0 {
                            //     while x >= 0 {
                            //         if name.chars().nth(x as usize) != Some('(') && name.chars().nth(x as usize) != Some(')') {
                            //             remove_idx.push(x as usize);
                            //         }
                            //         x -= 1;
                            //     }
                            // }
                            // for i in remove_idx.iter() {
                            //     name.remove(*i);
                            // }
                            let tmp2 = name.clone();
                            println!();
                        } else {
                            name.push_str(&op[i].to_string().clone());
                        }
                    }
                    match &mut args[i] {
                        Expr::Atom(x) => match x {
                            Atom::Number(x) => {
                                name.push_str(&x.to_string().clone());
                            }
                            Atom::Identifier(x) => {
                                name.push_str(&x.to_string().clone());
                            }
                            Atom::Operator(_) => {}
                        },
                        Expr::Expression { op, .. } => {
                            let mut string;
                            if op[0] != Operator::Other("".to_string()) {
                                string = op[0].to_string();
                                string.push_str("(");
                                string.push_str(args[i].get_names().as_str().clone());
                                string.push_str(")");
                            } else {
                                string = args[i].get_names().as_str().to_string().clone();
                            }
                            // let mut deri_flag = false;
                            // if op[0].clone().to_string() == "derivative"{
                            //     name.push_str("derivative(");
                            //     deri_flag = true;
                            // }
                            // string = args[i].get_names().as_str().to_string().clone();
                            name.push_str(&string.clone());
                            // if deri_flag {
                            //     name.push_str(")");
                            //     deri_flag = false;
                            // }
                            // name.push_str(")");
                        }
                    }
                }
                if add_paren {
                    name.push(')');
                }
                add_paren = false;

                name.to_string()
            }
        }
    }

    fn to_graph(&mut self, graph: &mut MathExpressionGraph) {
        match self {
            Expr::Atom(_x) => {}
            Expr::Expression { op, args, name } => {
                if name == "place_holder" {
                    return;
                } else if name.contains("place_holder") {
                    *name = name.replace("place_holder", "");
                }

                let mut parent_node_index: NodeIndex = Default::default();
                if op[0].to_string() != "derivative" {
                    parent_node_index = get_node_idx(graph, name)
                }
                // let parent_node_index: NodeIndex = get_node_idx(graph, name);
                let mut eq_loc = 0;
                if op.contains(&Operator::Equals) {
                    eq_loc = op.iter().position(|r| r == &(Operator::Equals)).unwrap();
                    let mut left_eq_name: String = "".to_string();
                    for i in 0..eq_loc {
                        match &mut args[i] {
                            Expr::Atom(x) => {
                                match x {
                                    Atom::Number(y) => { left_eq_name.push_str(y); }
                                    Atom::Identifier(y) => { left_eq_name.push_str(y); }
                                    Atom::Operator(y) => {}
                                }
                            }
                            Expr::Expression { op, args, name } => {
                                if op[0] != Operator::Other("".to_string()) {
                                    let mut unitary_name = op[0].to_string();
                                    let mut name_copy = name.to_string();
                                    remove_paren(&mut name_copy);
                                    unitary_name.push_str("(".clone());
                                    unitary_name.push_str(&name_copy.clone());
                                    unitary_name.push_str(")".clone());
                                    left_eq_name.push_str(unitary_name.as_str());
                                } else {
                                    left_eq_name.push_str(name.as_str());
                                }
                            }
                        }
                    }

                    let node_idx = get_node_idx(graph, &mut left_eq_name);
                    graph.update_edge(
                        node_idx,
                        parent_node_index,
                        "=".to_string(),
                    );
                }
                if op[0] != Operator::Other("".to_string()) {
                    let mut unitary_name = op[0].to_string();
                    let mut name_copy = name.to_string();
                    remove_paren(&mut name_copy);
                    unitary_name.push_str("(".clone());
                    unitary_name.push_str(&name_copy.clone());
                    unitary_name.push_str(")".clone());
                    let node_idx = get_node_idx(graph, &mut unitary_name);
                    if op[0].to_string() == "derivative" {
                        return;
                    } else {
                        graph.update_edge(parent_node_index, node_idx, op[0].to_string());
                    }
                }
                let op_copy = op.clone();
                for i in eq_loc..=op_copy.len() - 1 {
                    match &mut args[i] {
                        Expr::Atom(x) => match x {
                            Atom::Number(x) => {
                                if x == "place_holder" {
                                    continue;
                                } else if x.contains("place_holder") {
                                    *x = x.replace("place_holder", "");
                                }
                                let node_idx = get_node_idx(graph, x);
                                if i == 0 {
                                    if op_copy.len() > 1 {
                                        if (op_copy[i + 1].to_string() == "+" || op_copy[i + 1].to_string() == "-") && x.chars().nth(0).unwrap() != '-' {
                                            graph.update_edge(
                                                node_idx,
                                                parent_node_index,
                                                "+".to_string(),
                                            );
                                        } else if op_copy[i + 1].to_string() == "*" || op_copy[i + 1].to_string() == "/" {
                                            graph.update_edge(
                                                node_idx,
                                                parent_node_index,
                                                "*".to_string(),
                                            );
                                        } else {
                                            graph.update_edge(
                                                node_idx,
                                                parent_node_index,
                                                op_copy[i + 1].to_string(),
                                            );
                                        }
                                    }
                                } else if op_copy[i] == Operator::Equals {
                                    if i <= op_copy.len() - 2 {
                                        if (op_copy[i + 1].to_string() == "+" || op_copy[i + 1].to_string() == "-") && x.chars().nth(0).unwrap() != '-' {
                                            graph.update_edge(
                                                node_idx,
                                                parent_node_index,
                                                "+".to_string(),
                                            );
                                        } else if op_copy[i + 1].to_string() == "*" || op_copy[i + 1].to_string() == "/" {
                                            graph.update_edge(
                                                node_idx,
                                                parent_node_index,
                                                "*".to_string(),
                                            );
                                        } else {
                                            graph.update_edge(
                                                node_idx,
                                                parent_node_index,
                                                op_copy[i + 1].to_string(),
                                            );
                                        }
                                    }
                                } else {
                                    graph.update_edge(
                                        node_idx,
                                        parent_node_index,
                                        op_copy[i].to_string(),
                                    );
                                }
                            }
                            Atom::Identifier(x) => {
                                if x == "place_holder" {
                                    continue;
                                } else if x.contains("place_holder") {
                                    *x = x.replace("place_holder", "");
                                }
                                let node_idx = get_node_idx(graph, x);
                                if i == 0 {
                                    if op_copy.len() > 1 {
                                        if (op_copy[i + 1].to_string() == "+" || op_copy[i + 1].to_string() == "-") && x.chars().nth(0).unwrap() != '-' {
                                            graph.update_edge(
                                                node_idx,
                                                parent_node_index,
                                                "+".to_string(),
                                            );
                                        } else if op_copy[i + 1].to_string() == "*" || op_copy[i + 1].to_string() == "/" {
                                            graph.update_edge(
                                                node_idx,
                                                parent_node_index,
                                                "*".to_string(),
                                            );
                                        } else {
                                            graph.update_edge(
                                                node_idx,
                                                parent_node_index,
                                                op_copy[i + 1].to_string(),
                                            );
                                        }
                                    }
                                } else if op_copy[i] == Operator::Equals {
                                    if i <= op_copy.len() - 2 {
                                        if (op_copy[i + 1].to_string() == "+" || op_copy[i + 1].to_string() == "-") && x.chars().nth(0).unwrap() != '-' {
                                            graph.update_edge(
                                                node_idx,
                                                parent_node_index,
                                                "+".to_string(),
                                            );
                                        } else if op_copy[i + 1].to_string() == "*" || op_copy[i + 1].to_string() == "/" {
                                            graph.update_edge(
                                                node_idx,
                                                parent_node_index,
                                                "*".to_string(),
                                            );
                                        } else {
                                            graph.update_edge(
                                                node_idx,
                                                parent_node_index,
                                                op_copy[i + 1].to_string(),
                                            );
                                        }
                                    }
                                } else {
                                    graph.update_edge(
                                        node_idx,
                                        parent_node_index,
                                        op_copy[i].to_string(),
                                    );
                                }
                            }
                            Atom::Operator(_x) => {}
                        },
                        Expr::Expression { op, name, .. } => {
                            if name == "place_holder" {
                                continue;
                            } else if name.contains("place_holder") {
                                *name = name.replace("place_holder", "");
                            }
                            if op[0] == Operator::Other("".to_string()) {
                                let node_idx = get_node_idx(graph, name);
                                if i == 0 {
                                    if op_copy.len() > 1 {
                                        if (op_copy[i + 1].to_string() == "+" || op_copy[i + 1].to_string() == "-") && name.chars().nth(0).unwrap() != '-' {
                                            graph.update_edge(
                                                node_idx,
                                                parent_node_index,
                                                "+".to_string(),
                                            );
                                        } else if op_copy[i + 1].to_string() == "*" || op_copy[i + 1].to_string() == "/" {
                                            graph.update_edge(
                                                node_idx,
                                                parent_node_index,
                                                "*".to_string(),
                                            );
                                        } else {
                                            graph.update_edge(
                                                node_idx,
                                                parent_node_index,
                                                op_copy[i + 1].to_string(),
                                            );
                                        }
                                    }
                                } else if op_copy[i] == Operator::Equals {
                                    if i <= op_copy.len() - 2 {
                                        if op_copy.len() > 1 {
                                            if (op_copy[i + 1].to_string() == "+" || op_copy[i + 1].to_string() == "-") && name.chars().nth(0).unwrap() != '-' {
                                                graph.update_edge(
                                                    node_idx,
                                                    parent_node_index,
                                                    "+".to_string(),
                                                );
                                            } else if op_copy[i + 1].to_string() == "*" || op_copy[i + 1].to_string() == "/" {
                                                graph.update_edge(
                                                    node_idx,
                                                    parent_node_index,
                                                    "*".to_string(),
                                                );
                                            } else {
                                                graph.update_edge(
                                                    node_idx,
                                                    parent_node_index,
                                                    op_copy[i + 1].to_string(),
                                                );
                                            }
                                        }
                                    }
                                } else {
                                    graph.update_edge(
                                        node_idx,
                                        parent_node_index,
                                        op_copy[i].to_string(),
                                    );
                                }
                            } else {
                                let mut unitary_name = op[0].to_string();
                                let mut name_copy = name.to_string().clone();
                                remove_paren(&mut name_copy);
                                unitary_name.push_str("(".clone());
                                unitary_name.push_str(&name_copy.clone());
                                unitary_name.push_str(")".clone());
                                let node_idx = get_node_idx(graph, &mut unitary_name);
                                if i == 0 {
                                    if op_copy.len() > 1 {
                                        if (op_copy[i + 1].to_string() == "+" || op_copy[i + 1].to_string() == "-") && unitary_name.chars().nth(0).unwrap() != '-' {
                                            graph.update_edge(
                                                node_idx,
                                                parent_node_index,
                                                "+".to_string(),
                                            );
                                        } else if op_copy[i + 1].to_string() == "*" || op_copy[i + 1].to_string() == "/" {
                                            graph.update_edge(
                                                node_idx,
                                                parent_node_index,
                                                "*".to_string(),
                                            );
                                        } else {
                                            graph.update_edge(
                                                node_idx,
                                                parent_node_index,
                                                op_copy[i + 1].to_string(),
                                            );
                                        }
                                    }
                                } else if op_copy[i] == Operator::Equals {
                                    if i <= op_copy.len() - 2 {
                                        if (op_copy[i + 1].to_string() == "+" || op_copy[i + 1].to_string() == "-") && unitary_name.chars().nth(0).unwrap() != '-' {
                                            graph.update_edge(
                                                node_idx,
                                                parent_node_index,
                                                "+".to_string(),
                                            );
                                        } else if op_copy[i + 1].to_string() == "*" || op_copy[i + 1].to_string() == "/" {
                                            graph.update_edge(
                                                node_idx,
                                                parent_node_index,
                                                "*".to_string(),
                                            );
                                        } else {
                                            graph.update_edge(
                                                node_idx,
                                                parent_node_index,
                                                op_copy[i + 1].to_string(),
                                            );
                                        }
                                    }
                                } else {
                                    graph.update_edge(
                                        node_idx,
                                        parent_node_index,
                                        op_copy[i].to_string(),
                                    );
                                }
                            }
                            args[i].to_graph(graph);
                        }
                    }
                }
            }
        }
    }
}

pub fn redundant_paren(string: &String) -> bool {
    let str_len = string.chars().count();
    if !string.starts_with('(') || string.chars().nth(str_len - 1) != Some(')') {
        return false;
    }
    let mut par_stack = VecDeque::new();
    par_stack.push_back("left_par");
    for i in 1..=str_len - 2 {
        if string.chars().nth(i) == Some('(') {
            par_stack.push_back("par");
        } else if string.chars().nth(i) == Some(')') {
            par_stack.pop_back();
        }
    }
    if !par_stack.is_empty() && par_stack[0] == "left_par" {
        return true;
    }
    false
}

pub fn all_multi_div(op: &mut Vec<Operator>) -> bool {
    for o in 1..=op.len() - 1 {
        if op[o] != Operator::Multiply && op[o] != Operator::Divide {
            return false;
        }
    }
    true
}

impl PreExp {
    fn group_expr(&mut self) {
        for arg in &mut self.args {
            match arg {
                Expr::Atom(_) => {}
                Expr::Expression { .. } => {
                    arg.group_expr();
                }
            }
        }
    }

    fn collapse_expr(&mut self) {
        for arg in &mut self.args {
            match arg {
                Expr::Atom(_) => {}
                Expr::Expression { .. } => {
                    arg.collapse_expr();
                }
            }
        }
    }

    fn get_names(&mut self) {
        for mut arg in &mut self.args {
            match &mut arg {
                Expr::Atom(_) => {}
                Expr::Expression { .. } => {
                    arg.get_names();
                }
            }
        }
    }

    fn to_graph(&mut self) -> MathExpressionGraph {
        let mut g = MathExpressionGraph::new();
        for mut arg in &mut self.args {
            match &mut arg {
                Expr::Atom(_) => {}
                Expr::Expression { .. } => {
                    arg.to_graph(&mut g);
                }
            }
        }
        g
    }
}

pub fn remove_paren(string: &mut String) -> &mut String {
    while redundant_paren(string) {
        string.remove(string.len() - 1);
        string.remove(0);
    }
    *string = string.replace("()", "");
    string
}

/// if exists, return the node index; if no, add a new node and return the node index
pub fn get_node_idx(graph: &mut MathExpressionGraph, name: &mut String) -> NodeIndex {
    remove_paren(name);
    if name.contains("derivative") {
        *name = name.replace("/", ", ");
    }
    if graph.node_count() > 0 {
        for n in 0..=graph.node_count() - 1 {
            match graph.raw_nodes().get(n) {
                None => {}
                Some(x) => {
                    if *name == x.weight {
                        match graph.node_indices().nth(n) {
                            None => {}
                            Some(x) => {
                                return x;
                            }
                        }
                    }
                }
            }
        }
    }

    graph.add_node(name.to_string())
}

pub fn remove_redundant_mrow(mml: String, key_word: String) -> String {
    let mut content = mml.clone();
    let mut key_words_left = "<mrow>".to_string() + &*key_word.clone();
    let mut key_word_right = key_word.clone();
    key_word_right.insert(1, '/');
    let mut key_words_right = key_word_right.clone() + "</mrow>";
    let mut locs: Vec<_> = content.match_indices(&key_words_left).map(|(i, _)| i).collect();
    for loc in locs.iter().rev() {
        if content[loc + 1..].contains(&key_words_right) {
            let l = content[*loc..].find(&key_word_right).map(|i| i + *loc);
            match l {
                None => {}
                Some(x) => {
                    if content.len() > (x + key_words_right.len()) {
                        if content[x..x + key_words_right.len()] == key_words_right {
                            content.replace_range(x..x + key_words_right.len(), key_word_right.as_str());
                            content.replace_range(*loc..*loc + key_words_left.len(), key_word.as_str());
                        }
                    }
                }
            }
        }
    }
    return content;
}

///remove redundant mrow in mathml
pub fn remove_rmrow(mathml_content: String) -> String {
    let mut content = mathml_content.clone();
    content = content.replace("<mrow>", "(");
    content = content.replace("</mrow>", ")");
    let f = |b: &[u8]| -> Vec<u8>{
        let v = (0..).zip(b).scan(vec![], |a, (b, c)| Some(match c {
            40 => {
                a.push(b);
                None
            }
            41 => { Some((a.pop()?, b)) }
            _ => None
        })).flatten().collect::<Vec<_>>();
        for k in &v {
            if k.0 == 0 && k.1 == b.len() - 1 { return b[1..b.len() - 1].to_vec(); }
            for l in &v { if l.0 == k.0 + 1 && l.1 == k.1 - 1 { return [&b[..k.0], &b[l.0..k.1], &b[k.1 + 1..]].concat(); } }
        }
        return b.to_vec();
    };
    let g = |mut b: Vec<u8>| {
        while f(&b) != b { b = f(&b) }
        b
    };
    content = std::str::from_utf8(&g(content.bytes().collect())).unwrap().to_string();
    content = content.replace("(", "<mrow>");
    content = content.replace(")", "</mrow>");
    content = remove_redundant_mrow(content, "<mi>".to_string());
    content = remove_redundant_mrow(content, "<mo>".to_string());
    content = remove_redundant_mrow(content, "<mfrac>".to_string());
    content = remove_redundant_mrow(content, "<mover>".to_string());
    return content;
}

/// preprocess the parsed content
pub fn preprocess_content(content_str: String) -> String {
    let mut pre_string = content_str.clone();
    pre_string = pre_string.replace(" ", "");
    pre_string = pre_string.replace("<mo>,</mo>", "");
    pre_string = pre_string.replace("<mo>(</mo>", "<mrow>");
    pre_string = pre_string.replace("<mo>)</mo>", "</mrow>");
    pre_string = pre_string.replace("\n", "");
    /// Unicode to Symbol
    let mut unicode_locs: Vec<_> = pre_string.match_indices("&#").map(|(i, _)| i).collect();
    for ul in unicode_locs.iter().rev() {
        let loc = pre_string[*ul..].find("<").map(|i| i + ul);
        match loc {
            None => {}
            Some(x) => { pre_string.insert(x, ';') }
        }
    }
    pre_string = html_escape::decode_html_entities(&pre_string).to_string();
    pre_string = pre_string.replace(&html_escape::decode_html_entities("&#x2212;").to_string(), "-");
    pre_string = remove_rmrow(pre_string);
    return pre_string;
}

pub fn wrap_math (math: Math) -> MathExpression{
    let mut math_vec = vec![];
    for con in math.content {
        math_vec.push(con);
    }
    let mut new_math = Mrow(math_vec);
    return new_math;
}

#[test]
fn test_to_expr() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
    ]);
    let mut pre_exp = PreExp {
        op: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };
    pre_exp.op.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { op, args, .. } => {
            assert_eq!(op[0], Operator::Other("".to_string()));
            assert_eq!(op[1], Operator::Add);
            assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
            assert_eq!(args[1], Expr::Atom(Atom::Identifier("b".to_string())));
        }
    }
}

#[test]
fn test_to_expr2() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
        Mo(Operator::Subtract),
        Mrow(vec![
            Mn("4".to_string()),
            Mi("c".to_string()),
            Mi("d".to_string()),
        ]),
    ]);
    let mut pre_exp = PreExp {
        op: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };

    math_expression.to_expr(&mut pre_exp);
    pre_exp.op.push(Operator::Other("root".to_string()));
    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { op, args, .. } => {
            assert_eq!(op[0], Operator::Other("".to_string()));
            assert_eq!(op[1], Operator::Add);
            assert_eq!(op[2], Operator::Subtract);
            assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
            assert_eq!(args[1], Expr::Atom(Atom::Identifier("b".to_string())));
            match &args[2] {
                Expr::Atom(_) => {}
                Expr::Expression { op, args, .. } => {
                    assert_eq!(op[0], Operator::Other("".to_string()));
                    assert_eq!(op[1], Operator::Multiply);
                    assert_eq!(op[2], Operator::Multiply);
                    assert_eq!(args[0], Expr::Atom(Atom::Number("4".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("c".to_string())));
                    assert_eq!(args[2], Expr::Atom(Atom::Identifier("d".to_string())));
                }
            }
        }
    }
}

#[test]
fn test_to_expr3() {
    let math_expression = Msqrt(Box::from(Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
    ])));
    let mut pre_exp = PreExp {
        op: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };
    pre_exp.op.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { op, args, .. } => {
            assert_eq!(op[0], Operator::Sqrt);
            match &args[0] {
                Expr::Atom(_) => {}
                Expr::Expression { op, args, .. } => {
                    assert_eq!(op[0], Operator::Other("".to_string()));
                    assert_eq!(op[1], Operator::Add);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("b".to_string())));
                }
            }
        }
    }
}

#[test]
fn test_to_expr4() {
    let math_expression = Mfrac(
        Box::from(Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Add),
            Mi("b".to_string()),
        ])),
        Box::from(Mi("c".to_string())),
    );
    let mut pre_exp = PreExp {
        op: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };
    pre_exp.op.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { op, args, .. } => {
            assert_eq!(op[0], Operator::Other("".to_string()));
            assert_eq!(op[1], Operator::Divide);
            match &args[0] {
                Expr::Atom(_) => {}
                Expr::Expression { op, args, .. } => {
                    assert_eq!(op[0], Operator::Other("".to_string()));
                    assert_eq!(op[1], Operator::Add);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("b".to_string())));
                }
            }
            match &args[1] {
                Expr::Atom(_x) => {
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("c".to_string())));
                }
                Expr::Expression { .. } => {}
            }
        }
    }
}

#[test]
fn test_to_expr5() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mi("c".to_string()),
    ]);
    let mut pre_exp = PreExp {
        op: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };
    pre_exp.op.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { op, args, .. } => {
            assert_eq!(op[0], Operator::Other("".to_string()));
            assert_eq!(op[1], Operator::Add);
            assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
            match &args[1] {
                Expr::Atom(_) => {}
                Expr::Expression { op, args, .. } => {
                    assert_eq!(op[0], Operator::Other("".to_string()));
                    assert_eq!(op[1], Operator::Multiply);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("b".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("c".to_string())));
                }
            }
        }
    }
}

#[test]
fn test_to_expr6() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mi("c".to_string()),
        Mo(Operator::Multiply),
        Mi("d".to_string()),
        Mo(Operator::Divide),
        Mi("e".to_string()),
        Mo(Operator::Subtract),
        Mi("f".to_string()),
        Mo(Operator::Multiply),
        Mi("g".to_string()),
        Mo(Operator::Subtract),
        Mi("h".to_string()),
    ]);
    let mut pre_exp = PreExp {
        op: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };
    pre_exp.op.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { op, args, .. } => {
            assert_eq!(op[0], Operator::Other("".to_string()));
            assert_eq!(op[1], Operator::Add);
            assert_eq!(op[2], Operator::Subtract);
            assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
            assert_eq!(args[3], Expr::Atom(Atom::Identifier("h".to_string())));
            match &args[1] {
                Expr::Atom(_) => {}
                Expr::Expression { op, args, .. } => {
                    assert_eq!(op[0], Operator::Other("".to_string()));
                    assert_eq!(op[1], Operator::Multiply);
                    assert_eq!(op[2], Operator::Multiply);
                    assert_eq!(op[3], Operator::Divide);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("b".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("c".to_string())));
                    assert_eq!(args[2], Expr::Atom(Atom::Identifier("d".to_string())));
                    assert_eq!(args[3], Expr::Atom(Atom::Identifier("e".to_string())));
                }
            }
            match &args[2] {
                Expr::Atom(_) => {}
                Expr::Expression { op, args, .. } => {
                    assert_eq!(op[0], Operator::Other("".to_string()));
                    assert_eq!(op[1], Operator::Multiply);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("f".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("g".to_string())));
                }
            }
        }
    }
}

#[test]
fn test_to_expr7() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mi("c".to_string()),
    ]);
    let mut pre_exp = PreExp {
        op: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "root".to_string(),
    };
    pre_exp.op.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.get_names();

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { op, args, name } => {
            assert_eq!(op[0], Operator::Other("".to_string()));
            assert_eq!(op[1], Operator::Add);
            assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
            assert_eq!(name, "(a+b*c)");
            match &args[1] {
                Expr::Atom(_) => {}
                Expr::Expression { op, args, name } => {
                    assert_eq!(op[0], Operator::Other("".to_string()));
                    assert_eq!(op[1], Operator::Multiply);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("b".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("c".to_string())));
                    assert_eq!(name, "b*c");
                }
            }
        }
    }
}

#[test]
fn test_to_expr8() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mi("c".to_string()),
        Mo(Operator::Multiply),
        Mi("d".to_string()),
        Mo(Operator::Divide),
        Mi("e".to_string()),
        Mo(Operator::Subtract),
        Mi("f".to_string()),
        Mo(Operator::Multiply),
        Mi("g".to_string()),
        Mo(Operator::Subtract),
        Mi("h".to_string()),
    ]);
    let mut pre_exp = PreExp {
        op: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };
    pre_exp.op.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.get_names();

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { op, args, name } => {
            assert_eq!(op[0], Operator::Other("".to_string()));
            assert_eq!(op[1], Operator::Add);
            assert_eq!(op[2], Operator::Subtract);
            assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
            assert_eq!(args[3], Expr::Atom(Atom::Identifier("h".to_string())));
            assert_eq!(name, "(a+b*c*d/e-f*g-h)");
            match &args[1] {
                Expr::Atom(_) => {}
                Expr::Expression { op, args, name } => {
                    assert_eq!(op[0], Operator::Other("".to_string()));
                    assert_eq!(op[1], Operator::Multiply);
                    assert_eq!(op[2], Operator::Multiply);
                    assert_eq!(op[3], Operator::Divide);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("b".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("c".to_string())));
                    assert_eq!(args[2], Expr::Atom(Atom::Identifier("d".to_string())));
                    assert_eq!(args[3], Expr::Atom(Atom::Identifier("e".to_string())));
                    assert_eq!(name, "b*c*d/e");
                }
            }
            match &args[2] {
                Expr::Atom(_) => {}
                Expr::Expression { op, args, name } => {
                    assert_eq!(op[0], Operator::Other("".to_string()));
                    assert_eq!(op[1], Operator::Multiply);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("f".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("g".to_string())));
                    assert_eq!(name, "f*g");
                }
            }
        }
    }
}

#[test]
fn test_to_expr9() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mrow(vec![
            Mi("c".to_string()),
            Mo(Operator::Subtract),
            Mi("d".to_string()),
        ]),
    ]);
    let mut pre_exp = PreExp {
        op: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "root".to_string(),
    };
    pre_exp.op.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.get_names();

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { op, args, name } => {
            assert_eq!(op[0], Operator::Other("".to_string()));
            assert_eq!(op[1], Operator::Add);
            assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
            assert_eq!(name, "(a+b*(c-d))");
            match &args[1] {
                Expr::Atom(_) => {}
                Expr::Expression { op, args, name } => {
                    assert_eq!(op[0], Operator::Other("".to_string()));
                    assert_eq!(op[1], Operator::Multiply);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("b".to_string())));
                    assert_eq!(name, "b*(c-d)");
                    match &args[1] {
                        Expr::Atom(_) => {}
                        Expr::Expression { op, args, name } => {
                            assert_eq!(op[0], Operator::Other("".to_string()));
                            assert_eq!(op[1], Operator::Subtract);
                            assert_eq!(args[0], Expr::Atom(Atom::Identifier("c".to_string())));
                            assert_eq!(args[1], Expr::Atom(Atom::Identifier("d".to_string())));
                            assert_eq!(name, "(c-d)");
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_to_expr10() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mrow(vec![
            Mi("c".to_string()),
            Mo(Operator::Subtract),
            Mi("a".to_string()),
        ]),
    ]);
    let mut pre_exp = PreExp {
        op: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "root".to_string(),
    };
    pre_exp.op.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.get_names();
    let _g = pre_exp.to_graph();
}

#[test]
fn test_to_expr11() {
    let math_expression = Msqrt(Box::from(Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Subtract),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Subtract),
            Mi("b".to_string()),
        ]),
    ])));

    let mut pre_exp = PreExp {
        op: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "root".to_string(),
    };
    pre_exp.op.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.get_names();
    let _g = pre_exp.to_graph();
}

#[test]
fn test_to_expr12() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mi("c".to_string()),
        Mo(Operator::Multiply),
        Mi("d".to_string()),
        Mo(Operator::Divide),
        Mi("e".to_string()),
        Mo(Operator::Subtract),
        Mi("f".to_string()),
        Mo(Operator::Multiply),
        Mi("g".to_string()),
        Mo(Operator::Subtract),
        Mi("h".to_string()),
    ]);
    let mut pre_exp = PreExp {
        op: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };
    pre_exp.op.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.get_names();
    let _g = pre_exp.to_graph();
}

#[test]
fn test_to_expr13() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mi("c".to_string()),
        Mo(Operator::Multiply),
        Mi("a".to_string()),
        Mo(Operator::Divide),
        Mi("d".to_string()),
        Mo(Operator::Subtract),
        Mi("c".to_string()),
        Mo(Operator::Multiply),
        Mi("a".to_string()),
        Mo(Operator::Subtract),
        Mi("b".to_string()),
    ]);
    let mut pre_exp = PreExp {
        op: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };
    pre_exp.op.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.get_names();
    let _g = pre_exp.to_graph();
}

#[test]
fn test_to_expr14() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mi("c".to_string()),
        Mo(Operator::Multiply),
        Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Subtract),
            Mi("b".to_string()),
        ]),
    ]);
    let mut pre_exp = PreExp {
        op: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };
    pre_exp.op.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.get_names();
    let _g = pre_exp.to_graph();
}

#[test]
fn test_to_expr15() {
    let math_expression = Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mi("c".to_string()),
        Mo(Operator::Subtract),
        Msqrt(Box::from(Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Add),
            Mi("b".to_string()),
        ]))),
    ]);
    let mut pre_exp = PreExp {
        op: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };
    pre_exp.op.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);
    pre_exp.group_expr();
    pre_exp.get_names();
    let _g = pre_exp.to_graph();
}

#[test]
fn test_to_expr16() {
    let math_expression = Msqrt(Box::from(Mrow(vec![
        Mi("a".to_string()),
        Mo(Operator::Subtract),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Subtract),
            Mi("b".to_string()),
        ]),
    ])));
    let _g = math_expression.to_graph();
}

#[test]
fn test_to_expr17() {
    let math_expression = Mrow(vec![
        Mi("s".to_string()),
        Mo(Operator::Equals),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Subtract),
            Mi("b".to_string()),
        ]),
    ]);
    let _g = math_expression.to_graph();
}

#[test]
fn test_to_expr18() {
    let math_expression = Mrow(vec![
        Mi("s".to_string()),
        Mo(Operator::Equals),
        Mi("a".to_string()),
        Mo(Operator::Multiply),
        Mi("b".to_string()),
        Mo(Operator::Subtract),
        Msqrt(Box::from(Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Subtract),
            Mi("b".to_string()),
            Mo(Operator::Multiply),
            Mrow(vec![
                Mi("a".to_string()),
                Mo(Operator::Subtract),
                Mi("b".to_string()),
            ]),
        ]))),
    ]);
    let _g = math_expression.to_graph();
}

use crate::parsing::parse;

#[test]
fn test_to_expr19() {
    let input = "tests/sir.xml";
    let contents =
        std::fs::read_to_string(input).expect(format!("Unable to read file {input}!").as_str());
    let (_, mut math) = parse(&contents).expect(format!("Unable to parse file {input}!").as_str());
    math.normalize();
    let _g = &mut math.content[0].clone().to_graph();
}

#[test]
fn test_to_expr20() {
    let math_expression = Mrow(vec![Mi("s".to_string()),
                                    Mo(Operator::Equals),
                                    Mfrac(
                                        Box::from(Mrow(vec![
                                            Mi("a".to_string()),
                                            Mo(Operator::Add),
                                            Mi("b".to_string()),
                                        ])),
                                        Box::from(Mrow(vec![
                                            Mi("a".to_string()),
                                            Mo(Operator::Multiply),
                                            Mi("c".to_string()),
                                            Mi("d".to_string()),
                                            Msqrt(Box::from(Mrow(vec![
                                                Mi("a".to_string()),
                                                Mo(Operator::Add),
                                                Mi("d".to_string()),
                                            ]))),
                                        ]),
                                        ))]);
    let _g = math_expression.to_graph();
}

#[test]
fn test_to_expr21() {
    let math_expression = Msup(
        Box::from(Mrow(vec![
            Mi("a".to_string()),
            Mo(Operator::Add),
            Mi("b".to_string()),
        ])),
        Box::from(Mi("c".to_string())),
    );
    let mut pre_exp = PreExp {
        op: Vec::<Operator>::new(),
        args: Vec::<Expr>::new(),
        name: "".to_string(),
    };
    pre_exp.op.push(Operator::Other("root".to_string()));
    math_expression.to_expr(&mut pre_exp);

    match &pre_exp.args[0] {
        Expr::Atom(_) => {}
        Expr::Expression { op, args, .. } => {
            assert_eq!(op[0], Operator::Other("".to_string()));
            assert_eq!(op[1], Operator::Other("^".to_string()));
            match &args[0] {
                Expr::Atom(_) => {}
                Expr::Expression { op, args, .. } => {
                    assert_eq!(op[0], Operator::Other("".to_string()));
                    assert_eq!(op[1], Operator::Add);
                    assert_eq!(args[0], Expr::Atom(Atom::Identifier("a".to_string())));
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("b".to_string())));
                }
            }
            match &args[1] {
                Expr::Atom(_x) => {
                    assert_eq!(args[1], Expr::Atom(Atom::Identifier("c".to_string())));
                }
                Expr::Expression { .. } => {}
            }
        }
    }
}

#[test]
fn test_to_expr22() {
    let math_expression = Mrow(vec![Mi("a".to_string()),
                                    Mo(Operator::Subtract),
                                    Msup(Box::from(Mrow(vec![
                                        Mi("a".to_string()),
                                        Mo(Operator::Add),
                                        Mi("b".to_string()),
                                    ])),
                                         Box::from(Mrow(vec![
                                             Mi("c".to_string()),
                                             Mo(Operator::Add),
                                             Mi("d".to_string()),
                                         ])),
                                    )]);
    let _g = math_expression.to_graph();
}

#[test]
fn test_to_expr23() {
    let math_expression = Mrow(vec![Msubsup(vec![Mrow(vec![Mi("a".to_string()),
                                                           Mo(Operator::Add),
                                                           Mi("b".to_string()), ]),
                                                 Mrow(vec![Mi("c".to_string()),
                                                           Mo(Operator::Subtract),
                                                           Mi("d".to_string()), ]),
                                                 Mi("c".to_string())]),
    ]);
    let _g = math_expression.to_graph();
}

use petgraph::dot::{Config, Dot};


#[test]
fn test_to_expr24() {
    let math_expression = Mrow(vec![
        Mo(Operator::Subtract),
        Mi("a".to_string()),
        Mo(Operator::Multiply),
        Mi("b".to_string()),
        Mo(Operator::Add),
        Mi("c".to_string()),
    ]);
    let _g = math_expression.to_graph();
    println!("{}", Dot::new(&_g));
}

#[test]
fn test_to_expr25() {
    let math_expression = Mrow(vec![
        Mo(Operator::Subtract),
        Mrow(vec![Mi("a".to_string()),
                  Mo(Operator::Add),
                  Mi("b".to_string()), ]),
        Mo(Operator::Multiply),
        Mi("c".to_string()),
    ]);
    let _g = math_expression.to_graph();
    println!("{}", Dot::new(&_g));
}

#[test]
fn test_to_expr26() {
    let math_expression = Mrow(vec![
        Mo(Operator::Subtract),
        Mi("a".to_string()),
        Mo(Operator::Multiply),
        Mi("b".to_string()),
        Mo(Operator::Multiply),
        Mi("c".to_string()),
        Mo(Operator::Add),
        Mi("d".to_string()),
    ]);
    let _g = math_expression.to_graph();
    println!("{}", Dot::new(&_g));
}

#[test]
fn test_to_expr27() {
    let math_expression = Mrow(vec![
        Mo(Operator::Subtract),
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
    ]);
    let _g = math_expression.to_graph();
    println!("{}", Dot::new(&_g));
}

#[test]
fn test_to_expr28() {
    let math_expression = Mrow(vec![
        Mo(Operator::Subtract),
        Mi("a".to_string()),
        Mo(Operator::Add),
        Mi("b".to_string()),
    ]);
    let _g = math_expression.to_graph();
    println!("{}", Dot::new(&_g));
}

#[test]
fn test_to_expr29() {
    let math_expression = Mrow(vec![Mo(Operator::Subtract),
                                    Mi("a".to_string()),
                                    Mo(Operator::Add),
                                    Msup(Box::from(Mrow(vec![
                                        Mo(Operator::Subtract),
                                        Mi("a".to_string()),
                                        Mo(Operator::Add),
                                        Mi("b".to_string()),
                                    ])),
                                         Box::from(Mrow(vec![
                                             Mi("c".to_string()),
                                             Mo(Operator::Add),
                                             Mi("d".to_string()),
                                         ])),
                                    )]);
    let _g = math_expression.to_graph();
    println!("{}", Dot::new(&_g));
}

#[test]
fn test_to_expr30() {
    let input = "tests/SEIR_eq1.xml";
    let mut contents =
        std::fs::read_to_string(input).expect(format!("Unable to read file {input}!").as_str());
    contents = preprocess_content(contents);
    let (_, mut math) = parse(&contents).expect(format!("Unable to parse file {input}!").as_str());
    math.normalize();
    let mut math_vec = vec![];
    for con in math.content {
        math_vec.push(con);
    }
    let mut new_math = Mrow(math_vec);
    let _g = new_math.clone().to_graph();
    println!("{}", Dot::new(&_g));
}

#[test]
fn test_to_expr31() {
    let input = "tests/test02.xml";
    let mut contents =
        std::fs::read_to_string(input).expect(format!("Unable to read file {input}!").as_str());
    contents = preprocess_content(contents);
    let (_, mut math) = parse(&contents).expect(format!("Unable to parse file {input}!").as_str());
    math.normalize();
    let mut math_vec = vec![];
    for con in math.content {
        math_vec.push(con);
    }
    let mut new_math = Mrow(math_vec);
    let _g = new_math.clone().to_graph();
    println!("{}", Dot::new(&_g));
}

#[test]
fn test_to_expr32() {
    let input = "tests/seirdv_eq7.xml";
    let mut contents =
        std::fs::read_to_string(input).expect(format!("Unable to read file {input}!").as_str());
    contents = preprocess_content(contents);
    let (_, mut math) = parse(&contents).expect(format!("Unable to parse file {input}!").as_str());
    math.normalize();
    let mut new_math = wrap_math(math);
    let _g = new_math.clone().to_graph();
    println!("{}", Dot::new(&_g));
}
