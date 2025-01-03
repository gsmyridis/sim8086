use crate::code::EffectiveAddr;
use crate::value::Value;

#[derive(Debug, Default)]
pub struct Memory {
    inner: Vec<u8>,
}

impl Memory {
    pub fn set(&mut self, addr: &EffectiveAddr, val: Value) {
        todo!()
    }
    
    pub fn get(&self, addr: &EffectiveAddr) -> Value {
        todo!()
    }
}

