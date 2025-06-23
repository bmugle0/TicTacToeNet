use big_brain::{Network, LayerTopology, TTTBoard};

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
    let brain_pool: Vec<Network> = (0..100)
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
    
    for pair in paired_brains {
        
    }
    
    //Set up board
    let mut board = TTTBoard::new();
    
    //Get move from brain
    let output = brain.propagate(
        board.places
        .iter()
        .map(|value| *value as f32)
        .collect::<Vec<_>>()
    );
    
    //Format move as usize so the board can read it
    let output = output
        .iter()
        .map(|value| value.round() as usize)
        .collect::<Vec<_>>();
    
    //Check if move is possible
    //If not, fail brain
    
    //Add the symbol to the board
    board.place_sign(1, (output[0], output[1]))
        .unwrap_or_else(|_| {
            //This is occurs when the brain tries to put a 
            //piece somewhere not on the board
            //Fail the brain
        });
        
    //Check if brain won game
    //If so, fail other brain
    //If not, switch turns
    
    //Loop should end here

    //This is my debug area
    //Do not touch! JK
    println!("{:?} {:?}", output.places);
}
