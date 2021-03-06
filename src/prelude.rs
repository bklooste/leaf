
pub use core::{ConnectionDestination, Connection,  Block  };


// standard impl for all neuron blocks
//impl<N: Neuron<f32 , Output = f32>> Blockf32<N> for Block<f32,N , Output = f32> {}

//pub use blocks::neuron::Sigmoid;
//pub use blocks::block::{FullMeshBlock , BlockwWeightHardening};


//fixme move this and the use somewhere more appropiate eg prelude or lib
//trait Blockf32Sigmoid  : Blockf32< Sigmoid> {}


// this is much nicer...
// Default Parameters for a Logistic Neural Net.
//
// #[derive(Copy, Clone, RustcEncodable, RustcDecodable)]
// pub struct LogisticNeuralNet;
//
// impl ActivationFunction for LogisticNeuralNet {
//   #[inline(always)] fn activate(x: f64) -> f64 { 1f64 / (1f64 + (-x).exp()) }
//   #[inline(always)] fn derivative(x: f64) -> f64 { x * (1f64 - x) }
// }
//
// impl NeuralNetParameters for LogisticNeuralNet {
//   type ActivationFunction = LogisticNeuralNet;
//   type Neuron = DefaultNeuron;
//   type BiasNeuron = NegativeOneBiasFunction;
// }


//
// /// Collection of parameters for a `NeuralNetTrainer`.
// ///
// pub trait TrainerParameters {
//   #[allow(missing_docs)] type MomentumConstant : MomentumConstant;
//   #[allow(missing_docs)] type LearningRate : LearningRate;
//   #[allow(missing_docs)] type ErrorGradient : ErrorGradient;
// }
//
// impl<S> TrainerParameters for S where S : MomentumConstant + LearningRate {
//   type MomentumConstant = S;
//   type LearningRate = S;
//   type ErrorGradient = ::params::DefaultErrorGradient;
// }
//
//
// /// Collection of parameters for a `NeuralNetTrainer`.
// ///
// pub trait TrainerParametersWithErrorFunction : TrainerParameters {
//   #[allow(missing_docs)] type ErrorFunction : ErrorFunction;
// }
//
// impl<S> TrainerParametersWithErrorFunction for Spub type LogisticBlock = ::blocks::fullmesh::FullMeshBlock<f32,f32,DefaultLogistic>;
//   where S : MomentumConstant + LearningRate + ErrorFunction
// {
//   type ErrorFunction = S;
// }
//
//
// /// Function to calculate error during training.
// ///
// pub trait ErrorFunction {
//   /// Calculates the error between a predicted result and the expected result.
//   ///
//   fn error<'a, I>(predictions: I, expected: I) -> f64
//     where I : Iterator<Item = &'a f64>;
// }
//
//
// /// Collection of parameters for a `NeuralNet`.
// ///

//
//
// /// A trainer for a single-layer neural network.
// ///
// pub trait NeuralNetTrainer : Iterator {>;
//   where S : MomentumConstant + LearningRate + ErrorFunction
// {
//   type ErrorFunction = S;
// }
//
//
// /// Function to calculate error during training.
// ///
//   #[inline(always)] fn train(&mut self) -> Option<Self::Item> { self.last() }
// }
//
//
// /// Layer Type!
// ///
// pub enum Layer {
//   #[allow(missing_docs)] Input,
//   #[allow(missing_docs)] Hidden,
//   #[allow(missing_docs)] Output
// }
//
// // we dont really use this ...
// /// Coordinates for a node in a specified layer.
// ///
// pub enum Node {
//   #[allow(missing_docs)] Input(usize),
//   #[allow(missing_docs)] Hidden(usize),
//   #[allow(missing_docs)] Output(usize),
//   #[allow(missing_docs)] WeightInputHidden(usize, usize),
//   #[allow(missing_docs)] WeightHiddenOutput(usize, usize)
// }
//
// // generic over paramaters...
// eg Block<P>   thats exactly what we did..but paramaters like this should be better
// eg input and output and weights..
//
// note here neuralblock<paramaters>  , block<O> .. not sure if should be input or output.
//
// note we communicate  everything as vectors the shape of what this represent is up to the compiler
//
//
// use a standard graph /node.. as later may need to compile.
//
// /// A single-layer neural network.
// ///
// pub trait NeuralNet<P> where P : NeuralNetParameters {
//   /// Returns the dimensions of the input layer.
//   ///
//   fn dim_input() -> usize;
//
//   /// Returns the dimensions of the output layer.
//   ///
//   fn dim_output() -> usize;
//
//   /// Returns the dimensions of the hidden layer.
//   ///
//   fn dim_hidden() -> usize;
//
//   /// Returns the value of a node in a layer at a specified coordinate.
//   ///
//   fn node(&self, i: Node) -> f64;
//
//   /// Returns a mutable reference to a node in a layer.
//   ///
//   fn node_mut(&mut self, i: Node) -> &mut f64;
//
//   /// Returns the specified layer.
//   ///
//   fn layer(&self, layer: Layer) -> &[f64];
//
//   /// Computes the predicted value for a given input and stores it
//   /// internally. The prediction can be retrieved using `layer`.
//   /// The reason `predict` doesn't return the prediction, is because it
//   /// requires a mutable borrow on `self`.
//   ///
//   fn predict(&mut self, inp: &[f64]);
// }
//
//
// // Learning Rate
// ///
// pub trait LearningRate {
//   #[allow(missing_docs)] fn lrate() -> f64;
// }
//
//
// // Momentum Constant
// ///
// pub trait MomentumConstant {
//   #[allow(missing_docs)] fn momentum() -> f64;
// }
//
//

//
//
// // Error Gradient method
// ///
// pub trait ErrorGradient {
//   #[allow(missing_docs)] fn errhidden<A>(act: f64, sum: f64) -> f64
//     where A : ActivationFunction;
//   #[allow(missing_docs)] fn erroutput<A>(exp: f64, act: f64) -> f64
//     where A : ActivationFunction;
// }
//
//
// /// The weight function to generate the initial weights.
// ///
// pub trait Neuron {
//   #[allow(missing_docs)] fn initw(ins: usize, outs: usize) -> f64;
// }
//
//
// /// The weight function to generate the bias nodes' weights.
// ///
// pub trait BiasNeuron {
//   #[allow(missing_docs)] fn biasw() -> f64;
// }
//
//
// /// A member of a training set.
// ///
// pub trait TrainingSetMember {
//   #[allow(missing_docs)] fn expected(&self) -> &[f64];
//   #[allow(missing_docs)] fn input(&self) -> &[f64];
// }
//
// impl<'a> TrainingSetMember for (&'a [f64], &'a [f64]) {
//   fn expected(&self) -> &[f64] { self.1 }
//   fn input(&self) -> &[f64] { self.0 }
// }
//
// impl TrainingSetMember for (Vec<f64>, Vec<f64>) {
//   fn expected(&self) -> &[f64] { &self.1[..] }
//   fn input(&self) -> &[f64] { &self.0[..] }
// }
