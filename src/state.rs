use crate::{ gpu::Gpu, input::Input };

pub trait State {
    fn draw(&mut self, gpu: &mut Gpu, buffer: &mut [u8]);
    fn update(&mut self);
    fn input(&mut self, input: &mut Input);
}


pub trait DrawableState {
    fn draw(&mut self, gpu: &mut Gpu);
}

pub trait LoopState {
    fn update(&mut self);
}

pub trait InputState {
    fn input(&mut self, input: &mut Input);
}