use big_brain::{Network, LayerTopology, TTTBoard, GameResult,};

fn main() {
    //Tic Tac Toe Network structure
    let layers = vec![
    LayerTopology {neurons: 9},
    LayerTopology {neurons: 16},
    LayerTopology {neurons: 10},
    LayerTopology {neurons: 2},
    ];
    //Test Brain
    let brain = Network::random(&layers);
    
    //Future Brain Pool to train from/to
    let mut brain_pool: Vec<Network> = (0..100)
        .into_iter()
        .map(|_| Network::random(&layers))
        .collect();
    
    //Paired brains to compete against each other
    let paired_brains: Vec<(&Network, &Network)> = brain_pool
        .as_slice()
        .chunks(2)
        .map(|chunk| (&chunk[0], &chunk[1]))
        .collect();
    
    //Loop should start here
    let mut pool_queue: Vec<(&Network, GameResult)> = Vec::with_capacity(50);
    for pair in paired_brains {
        let result = TTTBoard::run_game(pair.0, pair.1);
        pool_queue.push(result);
    }
    
    //Order Game Results by how well the networks did. Win first, followed by Error(n) with n in descending order.
    pool_queue.sort_by_key(|( _, k)| match k {
        GameResult::Win => (0, 0),
        GameResult::Error(n) => (1, usize::MAX - n),
    });
    println!("{:?}", pool_queue.iter().map(|x| &x.1 ).collect::<Vec<_>>());
    
    println!("Brain 1: {:?}", pool_queue[0].0);
    println!("Brain 2: {:?}", pool_queue[1].0);
    println!("Brain Baby: {:?}", pool_queue[0].0.recombine(&pool_queue[1].0));

    //This is my debug area
    //Do not touch! JK
}
