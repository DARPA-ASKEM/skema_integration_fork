use crate::ast::{
    Math, MathExpression,
    MathExpression::{
        Mfrac, Mi, Mn, Mo, MoLine, Mover, Mrow, Mspace, Msqrt, Mstyle, Msub, Msubsup, Msup, Mtext,
        Munder,
    },
};

use petgraph::{graph::NodeIndex, Graph};

pub type MathMLGraph<'a> = Graph<&'a str, u32>;

fn add_node_and_edge<'a>(
    graph: &mut MathMLGraph<'a>,
    parent_index: Option<NodeIndex>,
    x: &'a str,
) -> NodeIndex {
    let node_index = graph.add_node(x);
    if let Some(p) = parent_index {
        graph.add_edge(p, node_index, 1);
    }
    node_index
}

fn add_to_graph_0<'a>(graph: &mut MathMLGraph<'a>, parent_index: Option<NodeIndex>, x: &'a str) {
    add_node_and_edge(graph, parent_index, x);
}

/// Update the parent index
fn update_parent<'a>(
    graph: &mut MathMLGraph<'a>,
    mut parent_index: Option<NodeIndex>,
    x: &'a str,
) -> Option<NodeIndex> {
    let node_index = add_node_and_edge(graph, parent_index, x);
    parent_index = Some(node_index);
    parent_index
}

/// Macro to add elements with fixed numbers of child elements.
macro_rules! add_to_graph_n {
    ($graph: ident, $parent_index: ident, $elem_type: literal, $($x:ident),+ ) => {{
            let parent_index = update_parent($graph, $parent_index, $elem_type);
            $( $x.add_to_graph($graph, parent_index); )+
    }}
}

/// Function to add elements with a variable number of child elements.
fn add_to_graph_many0<'a>(
    graph: &mut MathMLGraph<'a>,
    parent_index: Option<NodeIndex>,
    elem_type: &'a str,
    elements: &Vec<MathExpression<'a>>,
) {
    let parent_index = update_parent(graph, parent_index, elem_type);
    for element in elements {
        element.add_to_graph(graph, parent_index);
    }
}

impl<'a> MathExpression<'a> {
    pub fn add_to_graph(&self, graph: &mut MathMLGraph<'a>, parent_index: Option<NodeIndex>) {
        match self {
            Mi(x) => add_to_graph_0(graph, parent_index, x),
            Mo(x) => add_to_graph_0(graph, parent_index, x),
            Mn(x) => add_to_graph_0(graph, parent_index, x),
            Msqrt(x) => add_to_graph_n!(graph, parent_index, "msqrt", x),
            Msup(x1, x2) => add_to_graph_n!(graph, parent_index, "msup", x1, x2),
            Msub(x1, x2) => add_to_graph_n!(graph, parent_index, "msub", x1, x2),
            Mfrac(x1, x2) => add_to_graph_n!(graph, parent_index, "mfrac", x1, x2),
            Mrow(xs) => add_to_graph_many0(graph, parent_index, "mrow", xs),
            Munder(xs) => add_to_graph_many0(graph, parent_index, "munder", xs),
            Mover(xs) => add_to_graph_many0(graph, parent_index, "mover", xs),
            Msubsup(xs) => add_to_graph_many0(graph, parent_index, "msubsup", xs),
            Mtext(x) => add_to_graph_0(graph, parent_index, x),
            Mstyle(xs) => add_to_graph_many0(graph, parent_index, "mstyle", xs),
            Mspace(x) => add_to_graph_0(graph, parent_index, x),
            MoLine(x) => add_to_graph_0(graph, parent_index, x),
        }
    }
}

impl<'a> Math<'a> {
    pub fn to_graph(&self) -> MathMLGraph {
        let mut g = MathMLGraph::new();
        let root_index = g.add_node("root");
        for element in &self.content {
            element.add_to_graph(&mut g, Some(root_index));
        }
        g
    }
}