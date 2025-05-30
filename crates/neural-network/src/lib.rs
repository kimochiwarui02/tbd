use std::iter::once;

use rand::Rng;
use rand::RngCore;
#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

#[derive(Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

impl Neuron {
    pub fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();
        // RLU
        let pre_activation = self.bias + output;
        if pre_activation > 0.0 {
            pre_activation
        } else {
            0.01 * pre_activation
        }
    }

    fn random(rng: &mut dyn RngCore, input_size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..input_size).map(|_| rng.gen_range(-1.0..=1.0)).collect();
        Self { bias, weights }
    }

    fn from_weights(input_size: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        let bias = weights.next().expect("got not enough weights");

        let weights = (0..input_size)
            .map(|_| weights.next().expect("got not enough weights"))
            .collect();

        Self { bias, weights }
    }
}

impl Layer {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    fn random(rng: &mut dyn RngCore, input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(rng, input_size))
            .collect();
        Self { neurons }
    }

    fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();

        Self { neurons }
    }
}

impl Network {
    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();
        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .copied()
    }

    pub fn from_weights(layers: &[LayerTopology], weights: impl IntoIterator<Item = f32>) -> Self {
        assert!(layers.len() > 1);

        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|layers| Layer::from_weights(layers[0].neurons, layers[1].neurons, &mut weights))
            .collect();

        if weights.next().is_some() {
            panic!("got too many weights");
        }

        Self { layers }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let network = Network::random(
            &mut rng,
            &[
                LayerTopology { neurons: 3 },
                LayerTopology { neurons: 2 },
                LayerTopology { neurons: 1 },
            ],
        );

        // Test network
        assert_eq!(network.layers.len(), 2);

        // Test layer 1
        assert_eq!(network.layers[0].neurons.len(), 2);
        // Neruron One
        assert_relative_eq!(network.layers[0].neurons[0].bias, -0.6255188);
        assert_relative_eq!(
            network.layers[0].neurons[0].weights.as_slice(),
            &[0.67383933, 0.81812596, 0.26284885].as_ref()
        );
        //Neuron two
        assert_relative_eq!(network.layers[0].neurons[1].bias, 0.5238805);
        assert_relative_eq!(
            network.layers[0].neurons[1].weights.as_slice(),
            &[-0.5351684, 0.069369555, -0.7648182].as_ref()
        );

        // Test Layer 2
        assert_eq!(network.layers[1].neurons.len(), 1);
        assert_relative_eq!(network.layers[1].neurons[0].bias, -0.102499485);
        assert_relative_eq!(
            network.layers[1].neurons[0].weights.as_slice(),
            &[-0.48879623, -0.19277143].as_ref()
        );
    }

    #[test]
    fn propagate() {
        let layers = (
            Layer {
                neurons: vec![
                    Neuron {
                        bias: 0.0,
                        weights: vec![0.1, 0.2, 0.6],
                    },
                    Neuron {
                        bias: 0.0,
                        weights: vec![0.1, 0.2, 0.6],
                    },
                ],
            },
            Layer {
                neurons: vec![Neuron {
                    bias: 0.0,
                    weights: vec![0.1, 0.2],
                }],
            },
        );
        let network = Network {
            layers: vec![layers.0, layers.1],
        };

        assert_relative_eq!(
            network.propagate(vec![0.4, 0.3, 0.8]).as_slice(),
            vec![0.17400002].as_slice()
        );
    }
}
