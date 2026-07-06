use crate::application::ports::CodeGenerator;
use rand::{Rng, thread_rng};

pub struct RandCodeGenerator;

impl CodeGenerator for RandCodeGenerator {
    fn verification_code(&self) -> u32 {
        thread_rng().gen_range(100000..999999)
    }
}
