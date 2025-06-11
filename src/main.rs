use big_brain::{Network, LayerTopology, TTTBoard};

fn main() {
    let brain_pool: Vec<Network> = Vec::new();

    let layers = vec![
    LayerTopology {neurons: 9},
    LayerTopology {neurons: 16},
    LayerTopology {neurons: 10},
    LayerTopology {neurons: 2},
    ];
    let brain = Network::random(&layers);
    
    let mut board = TTTBoard::new();
    
    let output = brain.propagate(
        board.places
        .iter()
        .map(|value| *value as f32)
        .collect()
    );
    
    let output = output
        .iter()
        .map(|value| value.round() as usize)
        .collect::<Vec<_>>();
        
    board.place_sign(1, (output[0], output[1]))
        .unwrap_or_else(|_| {
            //Fail the brain
            //Remove the brain from the brain pool
        });

    println!("{:?} {:?}", output, board.places);
}
