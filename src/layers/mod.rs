macro_rules! impl_neuron_layer {
    () => (
        fn exact_num_top_blobs(&self) -> usize { 1 }
        fn exact_num_bottom_blobs(&self) -> usize { 1 }
    )
}

pub use self::sigmoid::Sigmoid;

/// Sigmoid Layer
pub mod sigmoid;
