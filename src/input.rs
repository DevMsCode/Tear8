use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

pub enum MouseButton {
    Left = 0,
    Right = 1,
    Middle = 2,
    Other = 3
}

pub struct Input {
    pub winit_input: WinitInputHelper,
}

impl Input {
    pub fn new() -> Self {
        let winit_input = WinitInputHelper::new();

        Self {
            winit_input
        }
    }

    // Controlla se viene premuto un tasto della tastiera
    pub fn k_prsd(&mut self, key: VirtualKeyCode) -> bool {                
        self.winit_input.key_held(key)
    }

    // Controlla se viene rilasciato un tasto della tastiera
    pub fn k_rlsd(&mut self, key: VirtualKeyCode) -> bool {                
        self.winit_input.key_released(key)
    }

    // Controlla se viene premuto un tasto del mouse
    pub fn m_prsd(&self, button: MouseButton) -> bool {
        self.winit_input.mouse_held(button as usize)
    }

    // Controlla se viene rilasciato un tasto del mouse
    pub fn m_rlsd(&self, button: MouseButton) -> bool {
        self.winit_input.mouse_released(button as usize)
    }
}