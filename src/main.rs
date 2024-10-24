use lib::{activation::SIGMOID,network::Network};
mod lib;


fn main() {
    let inputs = vec![
        vec![0.0,0.0],
        vec![0.0,1.0],
        vec![1.0,0.0],
        vec![1.0,1.0]
    ];

    let targets = vec![vec![0.0],vec![1.0],vec![1.0],vec![0.0]];
    let mut network = Network::new(vec![2,3,1], 0.5, SIGMOID);

    println!("{:?}", network.feed_forward(vec![0.0, 0.0]));
	println!("{:?}", network.feed_forward(vec![0.0, 1.0]));
	println!("{:?}", network.feed_forward(vec![1.0, 0.0]));
	println!("{:?}", network.feed_forward(vec![1.0, 1.0]));


    network.train(inputs, targets, 10000);


    println!("{:?}", network.feed_forward(vec![0.0, 0.0]));
	println!("{:?}", network.feed_forward(vec![0.0, 1.0]));
	println!("{:?}", network.feed_forward(vec![1.0, 0.0]));
	println!("{:?}", network.feed_forward(vec![1.0, 1.0]));
}
