pub mod tree_walker {
    use std::collections::HashMap;

    use crate::ast_parser::ast_parser::{self, *};
    use crate::lexer::tokenizer::*;
    pub fn generate_tree(tokens: &Vec<Tokens>, syntax: &Tree, print: bool) {
        let mut idx = 0;
        let product = parse_node(
            &tokens,
            &syntax,
            &HashMap::new(),
            &mut idx,
            &String::from("entry"),
        );
        if print {
            match product {
                Ok(prd) => {
                    for nod in prd.nodes {
                        println!("{:?}", nod.0);
                        match nod.1 {
                            NodeType::Array(arr) => {
                                for arg in arr {
                                    println!("{arg:?}");
                                }
                            }
                            NodeType::Value(val) => {
                                println!("{val:?}");
                            }
                        }
                    }
                }
                Err(err) => {
                    println!("{err:?}")
                }
            }
        }
    }
    fn prep_nodes(syntax: &Tree, id: &String) -> Option<HashMap<String, NodeType>> {
        let mut map = HashMap::new();
        match &syntax.get(id) {
            Some(node) => {
                for param in &node.parameters {
                    match param {
                        HeadParam::Array(arr) => {
                            map.insert(arr.into(), NodeType::Array(vec![]));
                        }
                        HeadParam::Value(val) => {
                            map.insert(
                                val.into(),
                                NodeType::Value(Node {
                                    name: Tokens::Text(String::from("'none")),
                                    data: None,
                                    nodes: HashMap::new(),
                                }),
                            );
                        }
                    }
                }
            }
            None => {
                return None;
            }
        }
        Some(map)
    }
    /// TODO: make standalone recursive scope function
    pub fn parse_node(
        tokens: &Vec<Tokens>,
        syntax: &Tree,
        params: &NodeParameters,
        idx: &mut usize,
        id: &String,
    ) -> Result<Node, (Err, bool)> {
        let mut result = Node {
            name: Tokens::Text(id.into()),
            data: None,
            nodes: prep_nodes(&syntax, &id).unwrap(),
        };
        match parse_scope(
            &tokens,
            &syntax,
            &params,
            idx,
            &syntax.get(id).unwrap().nodes,
            &mut result.nodes,
            &mut false,
        ) {
            Ok(ok) => {
                return Ok(result);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
    /// returns how many lines should prev. scope go back or Err
    fn parse_scope(
        tokens: &Vec<Tokens>,
        syntax: &Tree,
        params: &NodeParameters,
        idx: &mut usize,
        nodes: &Vec<ast_parser::NodeType>,
        data: &mut HashMap<String, super::tree_walker::NodeType>,
        harderr: &mut bool,
    ) -> Result<Option<(usize, ReturnActions)>, (Err, bool)> {
        let mut node_idx = 0;
        let mut advance_tok;
        let mut advance_node;

        macro_rules! Advance {
            () => {
                if advance_node {
                    node_idx += 1;
                }
                if advance_tok {
                    *idx += 1;
                }
            };
        }
        macro_rules! Back {
            ($num: expr, $freeze: expr) => {
                //println!("{}   {}", $num, node_idx);
                if $num <= node_idx {
                    if $freeze == ReturnActions::Freeze {
                        advance_node = false;
                    }
                    node_idx -= $num;
                } else {
                    //println!("Warpin away!\n\n\n");
                    *idx += 1;
                    return Ok(Some(($num - node_idx - 1, $freeze)));
                }
            };
        }
        macro_rules! ArgsCheck {
            ($args: expr, $node: expr, $token: expr) => {
                if let Some(arg) = $args.get("print") {
                    println!("{:?}", arg);
                }
                if let Some(_) = $args.get("peek") {
                    advance_tok = false;
                }
                if let Some(num) = $args.get("advance") {
                    match num.parse::<usize>() {
                        Ok(num) => {
                            *idx += num;
                        }
                        _ => {
                            *idx += 1;
                        }
                    }
                }
                if let Some(arg) = $args.get("harderr") {
                    *harderr = false;
                    if arg == "true" {
                        *harderr = true;
                    }
                }
                if let Some(arg) = $args.get("set") {
                    set($token, $node, &mut data.get_mut(arg.into()).unwrap());
                    //println!("{:?}", data);
                }
                if let Some(arg) = $args.get("back") {
                    let num = arg.parse::<usize>().unwrap();
                    Back!(num, ReturnActions::Freeze);
                }
                if let Some(str) = $args.get("end") {
                    if str == "true" {
                        *idx += 1;
                    }
                    return Ok(Some((0, ReturnActions::Chain)));
                }
                if let Some(arg) = $args.get("pass") {
                    Error!(Err::Pass(arg.into()), false);
                }
                if let Some(arg) = $args.get("err") {
                    Error!(Err::Msg(arg.into()), false);
                }
            };
        }
        macro_rules! ScopeEnter {
            ($node: expr, $freeze: expr) => {
                match parse_scope(&tokens, &syntax, &params, idx, &$node.nodes, data, harderr) {
                    Ok(back) => match back {
                        Some(back) => match back.1 {
                            ReturnActions::Freeze => {
                                Back!(back.0, back.1);
                            }
                            ReturnActions::Nothing => {
                                Back!(back.0, back.1);
                            }
                            ReturnActions::Chain => return Ok(Some(back)),
                            ReturnActions::Advance => {
                                advance_tok = false;
                            }
                        },
                        _ => {}
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            };
        }
        macro_rules! OpenStruct {
            ($ident: expr, $node: expr, $recoverable: expr) => {
                let start_idx = *idx;
                match parse_node(&tokens, &syntax, &$node.arguments, idx, &$ident.into()) {
                    Ok(nd) => {
                        ScopeEnter!(&$node, true);
                        ArgsCheck!(&$node.arguments, &$node.kind, TokenOrNode::Node(nd));
                        advance_tok = false;
                        Advance!();
                    }
                    Err(err) => match err.0 {
                        Err::Pass(arg) => {
                            let tok = 'look_for_stop: loop {
                                node_idx += 1;
                                if node_idx >= nodes.len() {
                                    Error!(
                                        Err::Msg(String::from(
                                            "Could not find break with matching name"
                                        )),
                                        true
                                    );
                                }
                                if match &nodes[node_idx] {
                                    NodeType::Expect(node) => {
                                        node.arguments.get("stop") == Some(&arg)
                                    }
                                    NodeType::Maybe(node) => {
                                        node.arguments.get("stop") == Some(&arg)
                                    }
                                    _ => false,
                                } {
                                    break 'look_for_stop;
                                }
                            };
                        }
                        Err::SubOverflow(_) => {
                            panic!("Syntax tree is broken, could not compile");
                        }
                        Err::FileEndPeaceful => return Err((Err::FileEndPeaceful, false)),
                        _ => {
                            /*println!(
                                "BYL JSEM TU {} {} {} = {}",
                                err.1,
                                $recoverable,
                                harderr,
                                err.1 || !$recoverable
                            );*/
                            advance_tok = false;
                            if err.1 || !$recoverable {
                                Error!(err.0, !$recoverable);
                            }
                            *idx = start_idx;
                            if !err.1 {
                                advance_node = true;
                            }
                            Advance!();
                        }
                    },
                }
            };
        }
        macro_rules! Error {
            ($error: expr, $reset: expr) => {
                //println!("Erorruju");
                return Err(($error, *harderr));
            };
        }
        while node_idx < nodes.len() {
            advance_node = true;
            advance_tok = true;
            //println!("nodes: {} idx: {node_idx}\ntokens: {} idx: {}", nodes.len(), tokens.len(), *idx);
            if *idx >= tokens.len() {
                if maybes_end(nodes, node_idx) == nodes.len() {
                    //println!("Advancin'");
                    return Ok(Some((0, ReturnActions::Advance)));
                }
                //println!("nodes: {} idx: {node_idx}\ntokens: {} idx: {}", nodes.len(), tokens.len(), *idx);
                if node_idx >= nodes.len() - 1 {
                    //println!("peaceful end");
                    return Err((Err::FileEndPeaceful, true));
                }
                //println!("KONEC SOUBORU PREJ");
                Error!(Err::FileEnd, true);
            }
            use ast_parser::NodeType;
            match &nodes[node_idx] {
                NodeType::Expect(node) => {
                    //println!("{:?}    {:?}", node.kind, tokens[*idx]);
                    match token_cmp(&node.kind, &tokens[*idx]) {
                        CompareResult::Eq => {
                            // match
                            ArgsCheck!(
                                &node.arguments,
                                &node.kind,
                                TokenOrNode::Token(tokens[*idx].clone())
                            );
                            Advance!();
                        }
                        CompareResult::NotEq => {
                            // err
                            Error!(Err::Expected(node.kind.clone(), tokens[*idx].clone()), true);
                        }
                        CompareResult::Ident(ident) => {
                            OpenStruct!(ident, &node, false);
                        }
                    }
                }
                NodeType::Maybe(node) => {
                    //println!("{:?}    {:?}", node.kind, tokens[*idx]);
                    match token_cmp(&node.kind, &tokens[*idx]) {
                        CompareResult::Eq => {
                            // match
                            ArgsCheck!(
                                &node.arguments,
                                &node.kind,
                                TokenOrNode::Token(tokens[*idx].clone())
                            );
                            Advance!();
                            ScopeEnter!(node, false);
                        }
                        CompareResult::NotEq => {
                            // err
                            //if node_idx == maybes_end(nodes, node_idx) - 1 {
                            advance_tok = false;
                            //}
                            Advance!();
                        }
                        CompareResult::Ident(ident) => {
                            OpenStruct!(ident, &node, true);
                        }
                    }
                }
                NodeType::Command(node) => {
                    Advance!();
                }
                NodeType::ArgsCondition(args_con) => {
                    Advance!();
                    *idx -= 1;
                    let mut all_match = true;
                    for arg in &args_con.params {
                        match params.get(arg.0) {
                            Some(param) => {
                                if param != arg.1 {
                                    all_match = false;
                                }
                            }
                            None => {
                                all_match = false;
                            }
                        }
                    }
                    if all_match {
                        ScopeEnter!(args_con, true);
                    }
                }
            }
        }
        //println!("Advancin' niece");
        Ok(Some((0, ReturnActions::Advance)))
    }
    fn set(token_found: TokenOrNode, token_expected: &Tokens, place: &mut NodeType) {
        match place {
            NodeType::Array(arr) => match token_found {
                TokenOrNode::Node(node) => arr.push(node),
                TokenOrNode::Token(token) => arr.push(construct_token(&token, token_expected)),
            },
            NodeType::Value(val) => match token_found {
                TokenOrNode::Node(node) => *val = node,
                TokenOrNode::Token(token) => *val = construct_token(&token, token_expected),
            },
        }
    }
    enum TokenOrNode {
        Token(Tokens),
        Node(Node),
    }
    fn construct_token(token_found: &Tokens, token_expected: &Tokens) -> Node {
        match token_expected {
            Tokens::String(str) => Node {
                name: token_found.clone(),
                data: Some(token_expected.clone()),
                nodes: HashMap::new(),
            },
            _ => Node {
                name: token_found.clone(),
                data: None,
                nodes: HashMap::new(),
            },
        }
    }
    fn token_cmp<'a>(tree_element: &'a Tokens, source_token: &'a Tokens) -> CompareResult<'a> {
        match tree_element {
            Tokens::String(ref txt) => match txt.as_str() {
                "'string" => {
                    if let Tokens::String(_) = source_token {
                        return CompareResult::Eq;
                    }
                    return CompareResult::NotEq;
                }
                "'number" => {
                    if let Tokens::Number(_, _, _) = source_token {
                        return CompareResult::Eq;
                    }
                    return CompareResult::NotEq;
                }
                "'text" => {
                    if let Tokens::Text(_) = source_token {
                        return CompareResult::Eq;
                    }
                    return CompareResult::NotEq;
                }
                "'char" => {
                    if let Tokens::Char(_) = source_token {
                        return CompareResult::Eq;
                    }
                    return CompareResult::NotEq;
                }
                "'any" => CompareResult::Eq,
                _ => {
                    if let Tokens::Text(str) = source_token {
                        if str == txt {
                            return CompareResult::Eq;
                        }
                        return CompareResult::NotEq;
                    }
                    return CompareResult::NotEq;
                }
            },
            Tokens::Text(ident) => {
                return CompareResult::Ident(&ident);
            }
            _ => {
                if *tree_element == *source_token {
                    return CompareResult::Eq;
                }
                return CompareResult::NotEq;
            }
        }
    }
    /// return end index+1 of maybes row
    fn maybes_end(syntax: &Vec<ast_parser::NodeType>, mut idx: usize) -> usize {
        while let ast_parser::NodeType::Maybe(_) = syntax[idx] {
            idx += 1;
            if idx == syntax.len() {
                break;
            }
        }
        return idx;
    }
    #[derive(PartialEq)]
    enum ReturnActions {
        Freeze,
        Nothing,
        Chain,
        Advance,
    }
    #[derive(PartialEq)]
    enum CompareResult<'a> {
        Eq,
        NotEq,
        Ident(&'a str),
    }
    #[derive(Debug)]
    pub enum Err {
        Expected(Tokens, Tokens),
        Msg(String),
        FileEnd,
        FileEndPeaceful,
        Pass(String),
        SubOverflow(usize),
    }
    /// structures defined by user
    #[derive(Debug)]
    pub struct Node {
        name: Tokens,
        data: Option<Tokens>,
        nodes: HashMap<String, NodeType>,
    }
    #[derive(Debug)]
    pub enum NodeType {
        Array(Vec<Node>),
        Value(Node),
    }
}
