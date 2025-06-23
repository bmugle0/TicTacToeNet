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
    
    pub fn recombine(&self, other: Self) -> Self {
        //This will take two networks and randomly take parts from each one to make a new one
        todo!()
    }
    
    pub fn mutate(&self) -> Self {
        //This will take one network and make a single random change somewhere in the network
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
}
