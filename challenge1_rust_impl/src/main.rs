use std::collections::HashMap;
use itertools::Itertools;
use rand::{distributions::Uniform, Rng};
use petgraph::Graph;
use petgraph::algo::is_cyclic_directed;




fn main() {
    // 1. creat graph
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, 20);
    let weights: Vec<i32> = (0..4).map(|_| rng.sample(&range)).collect();
    
    let edges = vec![("1", "2"), ("2", "4"), ("1", "11"), ("4", "11")];
    let input_weights: EdgeVariants = EdgeVariants::ArrOfi32(weights);
    let input_edges: EdgeVariants = EdgeVariants::ArrOfTuples(edges);

    // some incorrect inputs for assertion
    // let edges =  vec!["1", "2", "2", "4"]; 
    // let input_edges: EdgeVariants = EdgeVariants::ArrOfStrings(edges);

    // let edges =  vec![1, 2, 2, 4]; 
    // let input_edges: EdgeVariants = EdgeVariants::ArrOfi32(edges);

    // let edges =  String::from("ABC"); 
    // let input_edges: EdgeVariants = EdgeVariants::OnlyString(edges);

    let mut graph = DAG::new();
    graph.edges = input_edges;
    graph.weights = input_weights;
    
    let dict = graph.create_node_paths_dict();
    let edge_weight_dict = graph.create_edge_weight_dict();

    //  Select the root node here
    let node = "1";

    let all_paths = graph.find_all_paths(&dict,&node);
    
    println!("All the paths strating from the given root node: {}", &node);
    all_paths.iter().enumerate().for_each(|(cnt, path)| {
        println!("Path {}: {:?}",cnt+1, path);
    });
    
    /*check if the graph is a directed acyclic graph (its constructed with different types of data
     so not implementing inside the struct--needs more work, but still works)*/
    
     let gr = Graph::<(), i32>::from_edges(&[
        (1,2), (2, 4), (1, 11), (4, 11)]);
    // Create an undirected graph with `i32` nodes and edges with `()` associated data.
    // let gr = UnGraph::<i32, ()>::from_edges(
    //     &[(1, 2), (2, 3), (3, 4),(1, 4)]);
  
    if !(gr.is_directed() && !is_cyclic_directed(&gr)){
        panic!("\n\n*MyErrMessage: MThe passed in graph is not a directed acyclic graph**\n\n")
    }

    // let longest_path given a root node
    graph.get_longest_path_based_on_edge_weight(&dict,&node, &edge_weight_dict);
    

}

    
struct DAG<'a>{
    weights : EdgeVariants<'a>, // Vec<i32>,
    edges : EdgeVariants<'a>, //Vec<(&'a str, &'a str)>,

}

impl<'a> DAG<'a> {

    fn new() -> Self{
        Self{
            weights : EdgeVariants::ArrOfi32(Vec::new()),
            edges:  EdgeVariants::ArrOfTuples(Vec::new()), //Vec::new(),
        }
    }

    fn create_node_paths_dict(&self)->HashMap<&str, Vec<&str>>{
        let mut connections_dict: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut connections: Vec<(&str, &str)> = Vec::new();
        if let EdgeVariants::ArrOfTuples(a) = &self.edges{
            connections.extend(a);
        }
        for (source,target) in connections{
            connections_dict.entry(source).or_insert_with(Vec::new).push(target);
        }
        connections_dict
    }

    fn create_edge_weight_dict(&self)->HashMap<(&str, &str), i32>{
        let _connections_dict: HashMap<&str, Vec<&str>> = HashMap::new();
        
        let mut _connections: Vec<(&str, &str)> = Vec::new();
        if let EdgeVariants::ArrOfTuples(a) = &self.edges{
            _connections.extend(a);
        }

        let mut weights:Vec<i32> = Vec::new();
        if let EdgeVariants::ArrOfi32(a) = &self.weights{
            weights.extend(a);
        }

        let edge_weight_zip = Iterator::zip(_connections.iter(),weights);
        let edge_weight_dict = edge_weight_zip.map(|(key, value)| {return (key.clone(), value);});
        let edge_weight_dict: HashMap<(&str, &str), i32> = edge_weight_dict.into_iter().collect();
    
        edge_weight_dict
    }

    fn find_all_paths<'b>(&'b self, input_dict: &'b HashMap<&str, Vec<&str>>, node:&'b str) -> Vec<Vec<&'b str>>{
   
        let paths = &input_dict.get(&node);
        match  paths{
            Some(_a) =>  print!("\n\nThe key: {} is valid\n\n", &node),
            None => panic!("\n\n*MyErrMessage: The key: {} is invlid\n\n", &node)
        }
        
        fn find_all_paths_recursively<'b>(edges_dict: &'b HashMap<&str,Vec<&str>>, 
                                            node: &'b str,
                                            visited_init: Option<Vec<&'b str>>,
                                            path_init:    Option<Vec<&'b str>> 
                                            )  ->  Vec<Vec<&'b str>>{
    
            let mut visited:  Vec<&'b str> = visited_init.unwrap_or( Vec::new());
            let mut path: Vec<&'b str> = path_init.unwrap_or( Vec::new());
            
            visited.push(node);
            path.push(node);

            let mut all_paths: Vec<Vec<& str>> = Vec::new();
            let all_targets = edges_dict.get(&node); //.unwrap();


            if let Some(targets) = all_targets {
                for target in targets{
                    if !visited.contains(&target) {
                        path.dedup();
                        let mut final_path: Vec<& str> =  path.clone();
                        final_path.push(&target);
                        all_paths.push(final_path.clone());
                        let visited2: Option<Vec<&'b str>> = Some(visited.clone());
                        let final_path2:Option<Vec<&'b str>> = Some(final_path.clone());
                        let res = find_all_paths_recursively(&edges_dict,&target, visited2, final_path2);
                        all_paths.extend(res);
                    }
                }
            }
            all_paths
        }
        
        let results: Vec<Vec<&'b str>> = find_all_paths_recursively(& input_dict ,node,None, None);
        // println!("{:?}",&results);
        results

    }

    fn get_longest_path_based_on_edge_weight(&self,
                                            input_dict: & HashMap<&str,Vec<&str>>,
                                             node:&str,
                                            edge_weight_dict:& HashMap<(&str, &str), i32> ) {
        let all_paths = self.find_all_paths(&input_dict,&node);
        let mut path_scores = HashMap::new();
        for each_path in all_paths{
            let mut each_path_weight = 0;
            let each_path_edges = each_path.iter().permutations(2);          
            for single_edge in each_path_edges{
                let key: (&str, &str) = (&single_edge[0], &single_edge[1]);
                let single_edge_weight = edge_weight_dict.get(&key).unwrap_or(&&0); 
                each_path_weight += *single_edge_weight;}
            path_scores.insert(each_path, each_path_weight);
        }
        let key_with_max_value = path_scores.iter().max_by_key(|entry | entry.1);//.unwrap();
        match key_with_max_value{
            Some(a) => 
                println!("\n The following is the longest path \n {:?} \n \n with a total edges weight =  {:?}\n", a.0, a.1),
            None => 
                match &self.edges {
                    EdgeVariants::ArrOfStrings(_a) =>  panic!("\n\n *MyErrorMessage => You passed an array of strings\n, instead of an array of sting slice tuples*\n\n"),
                    EdgeVariants::ArrOfi32(_a) =>  panic!("\n\n *MyErrorMessage => You passed an array of integers\n instead of an array of sting slice tuples*\n\n"),
                    EdgeVariants::OnlyString(_a) =>  panic!("\n\n *MyErrorMessage => You passed a strings\n instead of an array of sting slice tuples*\n\n"),
                    EdgeVariants::ArrOfTuples(_a) => 
                    println!("\n\n *Edges are passed in correctly*\n\n"),
                }
        }
    }
}

enum EdgeVariants<'e>{ 
    OnlyString(String),
    ArrOfStrings(Vec<&'e str>),
    ArrOfi32(Vec<i32>),
    ArrOfTuples(Vec<(&'e str, &'e str)>),
}


#[cfg(test)]
#[test]
fn  test1_check_input(){
    // tests if an array of strings is passed instead of array of edges
    let weights = vec![1, 2, 4, 11];
    let edges = vec![("1", "2"), ("2", "4"), ("1", "11"), ("4", "11")];
    let input_weights: EdgeVariants = EdgeVariants::ArrOfi32(weights);
    let input_edges: EdgeVariants = EdgeVariants::ArrOfTuples(edges);
    let mut graph = DAG::new();
    graph.edges = input_edges;
    graph.weights = input_weights;
    let res = graph.create_node_paths_dict();


    let weights2 = vec![1, 2, 4, 11];
    let incorrect_edges2 = vec!["1", "2", "2", "4"];
    let input_weights2: EdgeVariants = EdgeVariants::ArrOfi32(weights2);
    let input_edges2: EdgeVariants = EdgeVariants::ArrOfStrings(incorrect_edges2);
    let mut graph2 = DAG::new();
    graph2.edges = input_edges2;
    graph2.weights = input_weights2;
    let correct_res = graph2.create_node_paths_dict();
    assert_ne!(res, correct_res)
}

#[test]
fn  test2_check_input(){
    // tests if an array of i32 values is passed instead of array of edges
    let weights = vec![1, 2, 4, 11];
    let edges = vec![("1", "2"), ("2", "4"), ("1", "11"), ("4", "11")];
    let input_weights: EdgeVariants = EdgeVariants::ArrOfi32(weights);
    let input_edges: EdgeVariants = EdgeVariants::ArrOfTuples(edges);
    let mut graph = DAG::new();
    graph.edges = input_edges;
    graph.weights = input_weights;
    let res = graph.create_node_paths_dict();


    let weights2 = vec![1, 2, 4, 11];
    let incorrect_edges2 = vec![1, 2, 2, 4];
    let input_weights2: EdgeVariants = EdgeVariants::ArrOfi32(weights2);
    let input_edges2: EdgeVariants = EdgeVariants::ArrOfi32(incorrect_edges2);
    let mut graph2 = DAG::new();
    graph2.edges = input_edges2;
    graph2.weights = input_weights2;
    let correct_res = graph2.create_node_paths_dict();
    assert_ne!(res, correct_res, "bad input")
}
#[test]
#[should_panic(expected = "*MyErrMessage: The key: ")]
fn  test3_check_input_key(){
    // tests if a wrong root node is selected
    let weights = vec![1, 2, 4, 11];
    let edges = vec![("1", "2"), ("2", "4"), ("1", "11"), ("4", "11")];
    let input_weights: EdgeVariants = EdgeVariants::ArrOfi32(weights);
    let input_edges: EdgeVariants = EdgeVariants::ArrOfTuples(edges);
    let mut graph = DAG::new();
    graph.edges = input_edges;
    graph.weights = input_weights;
    let dict = graph.create_node_paths_dict();

    let correct_res = graph.find_all_paths(&dict,"1");
    let incorrect_res = graph.find_all_paths(&dict,"21");
    assert_ne!(correct_res, incorrect_res);
   

}


