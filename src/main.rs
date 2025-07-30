use rand::{Rng, seq::SliceRandom};
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
    
    //Set up Brain Pool to train from/to
    let mut brain_pool: Vec<Network> = (0..100)
        .into_iter()
        .map(|_| Network::random(&layers))
        .collect();


    loop {
        //Paired brains to compete against each other
        let pool_copy = brain_pool.clone();
        let paired_brains: Vec<(&Network, &Network)> = pool_copy
            .as_slice()
            .chunks(2)
            .map(|chunk| (&chunk[0], &chunk[1]))
            .collect();
        
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
        pool_queue = pool_queue[0..(pool_queue.len()/2)].to_vec();
        println!("{:?}", pool_queue.iter().map(|x| &x.1 ).collect::<Vec<_>>());
        
        //Set up a new brain_pool for the next round of training
        brain_pool = pool_queue.clone().into_iter().map(|(network, _)| network.clone() ).collect();
        //Adding new networks based on the best 25 of the last round
        let mut rng = rand::thread_rng();
        for _ in 0..65 {
            let (brain1, _) = pool_queue[rng.gen_range(0..pool_queue.len())];
            let (brain2, _) = pool_queue[rng.gen_range(0..pool_queue.len())];
            brain_pool.push(brain1.recombine(brain2));
        }
        //Adding 25 random networks for variety
        for _ in 0..10 {
            brain_pool.push(Network::random(&layers));
        }
        //Shuffle the pool around
        brain_pool.shuffle(&mut rng)
    }


    //This is my debug area
    //Do not touch! JK
}
