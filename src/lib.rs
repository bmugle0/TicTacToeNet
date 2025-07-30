use rand::Rng;

#[derive(Debug, Clone)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
    
    pub fn random(layers: &[LayerTopology]) -> Self {
        let layers: Vec<_> = layers
            .windows(2)
            .map(|layers| Layer::random(layers[0].neurons, layers[1].neurons))
            .collect();
            
        //println!("{:?}", layers);
        Self { layers }
    }
    
    //This will take two networks and randomly take parts from each one to make a new one
    pub fn recombine(&self, other: &Self) -> Self {
        let mut rng = rand::thread_rng();
        let mut result: Network = self.clone();
        
        for (layer_a, layer_b) in result.layers.iter_mut().zip(&other.layers) {
            for (neuron_a, neuron_b) in layer_a.neurons.iter_mut().zip(&layer_b.neurons) {
                //Random Chance that result's neuron's bias will be replaced by other's corresponding bias
                if rng.r#gen() {
                    neuron_a.bias = neuron_b.bias;
                }
                
                for (w_a, w_b) in neuron_a.weights.iter_mut().zip(&neuron_b.weights) {
                    //Same thing as above, but instead with each weight
                    if rng.r#gen() {
                        *w_a = *w_b;
                    }
                }
            }
        }
        
        result
    }

    //This will take one network and make a single random change somewhere in the network
    pub fn mutate(&self) -> Self {
        todo!()
    }
}

pub struct LayerTopology {
    pub neurons: usize,
}

#[derive(Debug, Clone)]
struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
    
    fn random(input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(input_size))
            .collect();
            
        Self { neurons }
    }
}

#[derive(Debug, Clone)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());
        let mut result = inputs.iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();
            
        result = result + self.bias;
            
        result.max(0.0)
    }
    
    fn random(input_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let bias = rng.gen_range(-1.0..=1.0);
        
        let weights = (0..input_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();
            
        Self { bias, weights }
    }
}

pub struct TTTBoard {
    pub places: Vec<isize>
}

impl TTTBoard {
    pub fn new() -> Self {
        let places = vec![0; 9];
    
        Self { places }
    }
    
    pub fn place_sign(&mut self, place_value: isize, position: (usize, usize)) -> Result<(), ()> {
        if position.0 > 2  || position.1 > 2 { return Err(()) }
        self.places[position.0 + (3 * position.1)] = place_value;
        Ok(())
    }
    
    pub fn switch_view(&mut self) {
        let tmp_places = self.places
            .iter()
            .map(|value| {
                match value {
                    0 => 0,
                    1 => 2,
                    2 => 1,
                    _ => 0
                }
            })
            .collect();
        *self = Self { places: tmp_places};
    }
    
    pub fn is_done(&self) -> bool {
        let mut check: Vec<bool> = Vec::new();
        for i in 0..=2 {
            //Check horizontally
            let result = self.places[3*i] == 1
                && self.places[3*i+1] == 1
                && self.places[3*i+2] == 1;
            check.push(result);
            //Check Vertically
            let result = self.places[i] == 1
                && self.places[i+3] == 1
                && self.places[i+6] == 1;
            check.push(result);
        }
        check.push(
            self.places[0] == 1
            && self.places[4] == 1
            && self.places[8] == 1
        );
        check.push(
            self.places[2] == 1
            && self.places[4] == 1
            && self.places[6] == 1
        );
        
        check.iter().any(|item| *item)
    }
    
    pub fn run_game<'a>(player1: &'a Network, player2: &'a Network) -> (&'a Network, GameResult) {
        //println!("Starting Check");
        let mut board = Self::new();
        let mut turns: usize = 0;
        let result = loop {
            if (turns & 1) == 0 {
                //println!("Player 1's turn");
                if let Ok(_) = player1.play_move(&mut board) {}
                else {
                    break (player2, GameResult::Error(turns));
                }
            }
            else {
                //println!("Player 2's turn");
                if let Ok(_) = player2.play_move(&mut board) {}
                else {
                    break (player1, GameResult::Error(turns));
                }
            }
            if board.is_done() {
                //println!("Game Won");
                break (match (turns & 1) == 0 {
                    true => player1,
                    false => player2,
                }, GameResult::Win )
            }
            
            turns += 1;
            board.switch_view();
        };
        return result
    }
}

#[derive(Debug)]
pub enum GameResult {
    Win,
    Error(usize),
}
    
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
