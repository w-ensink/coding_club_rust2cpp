use crate::Filter;

fn create_lowpass_filter(sample_rate: f64, cutoff: f64) -> Box<Filter> {
    Box::new(Filter::new_lowpass(sample_rate, cutoff))
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type Filter;

        fn create_lowpass_filter(sample_rate: f64, cutoff: f64) -> Box<Filter>;

        fn process(&mut self, input: f32) -> f32;
        fn set_cutoff(&mut self, cutoff: f64);
    }
}
