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
    let mut next_brain_pool: Vec<Network> = Vec::with_capacity(50);
    for pair in paired_brains {
        println!("Starting Check");
        let mut board = TTTBoard::new();
        let mut turn: bool = true;
        let result = loop {
            if turn {
                println!("Player 1's turn");
                if let Ok(_) = pair.0.play_move(&mut board) {}
                else {
                    break pair.1;
                }
            }
            else {
                println!("Player 2's turn");
                if let Ok(_) = pair.1.play_move(&mut board) {}
                else {
                    break pair.0;
                }
            }
            if board.is_done() {
                println!("Game Won");
                break match turn {
                    true => pair.0,
                    false => pair.1,
                }
            }
            
            turn = !turn;
            board.switch_view();
        };
        next_brain_pool.push(result.clone());
    }
    
    brain_pool = next_brain_pool;
    
    trait TTTPlayer {
        fn play_move(&self, board: &mut TTTBoard) -> Result<(), ()>;
    }
    
    impl TTTPlayer for Network {
        fn play_move(&self, board: &mut TTTBoard) -> Result<(), ()> {
            let output = self.propagate(
                board.places
                .iter()
                .map(|value| (*value as f32))
                .collect::<Vec<_>>()
            ).iter()
            .map(|value| value.round() as usize)
            .collect::<Vec<_>>();
            
            if (output[0] + (3 * output[1])) < 9 
            && board.places[output[0] + (3 * output[1])] != 0 {
                return Err(())
            }
            
            board.place_sign(1, (output[0], output[1]))
        }
    }

    //This is my debug area
    //Do not touch! JK
}
