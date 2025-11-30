use super::entity_spec::EntitySpec;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputTarget {
    None,
    Team,
    Hp,
    Energy,
    Damage,
}

pub struct EditorInput {
    pub input_target: InputTarget,
    pub input_buffer: String,
}

impl EditorInput {
    pub fn new() -> Self {
        EditorInput {
            input_target: InputTarget::None,
            input_buffer: String::new(),
        }
    }

    pub fn cancel(&mut self) {
        self.input_target = InputTarget::None;
        self.input_buffer.clear();
    }

    pub fn handle_typing(&mut self, c: char) {
        if c.is_digit(10) {
            self.input_buffer.push(c);
        }
    }
    
    pub fn handle_backspace(&mut self) {
        self.input_buffer.pop();
    }

    pub fn confirm(&mut self, spec: &mut EntitySpec) -> Option<String> {
        if self.input_buffer.is_empty() {
            self.cancel();
            return Some("Edit Cancelled (Empty)".to_string());
        }

        let result = if let Ok(val) = self.input_buffer.parse::<u32>() {
            match self.input_target {
                InputTarget::Team => {
                    spec.team = val;
                    spec.update_symbol(); 
                    format!("Set Team to {} ({})", val, spec.symbol)
                },
                InputTarget::Hp => {
                    spec.hp = val;
                    format!("Set HP to {}", val)
                },
                InputTarget::Energy => {
                    spec.energy = val;
                    format!("Set Energy to {}", val)
                },
                InputTarget::Damage => {
                    spec.damage = val;
                    format!("Set DMG to {}", val)
                },
                InputTarget::None => return None,
            }
        } else {
            "Invalid Number!".to_string()
        };
        
        self.cancel();
        Some(result)
    }
}