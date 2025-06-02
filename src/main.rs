use big_brain::{Network, LayerTopology};

fn main() {
    let layers = vec![
    LayerTopology {neurons: 4},
    LayerTopology {neurons: 5},
    LayerTopology {neurons: 6},
    ];
    let output = Network::random(&layers);

    println!("{:?}", output);
}
