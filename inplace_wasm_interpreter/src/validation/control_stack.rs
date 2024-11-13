use alloc::vec::Vec;

use super::control_types::ControlType;

#[derive(Debug)]
pub struct ControlStack {
    stack: Vec<ControlType>,
}
impl Default for ControlStack {
    fn default() -> Self {
        Self::new()
    }
}

impl ControlStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }
    pub fn top(&mut self) -> &mut ControlType {
        self.stack.last_mut().expect("No ControlType on stack")
    }
    pub fn reset(&mut self) {
        self.stack.clear();
    }
    pub fn top_is_reachable(&self) -> bool {
        let top = self.stack.last().expect("No ControlType on stack");
        match top {
            ControlType::Block { reachable, .. }
            | ControlType::Func { reachable, .. }
            | ControlType::For { reachable, .. }
            | ControlType::If { reachable, .. } => *reachable,
        }
    }
    pub fn height_of_top(&mut self) -> u32 {
        let top = self.top();
        match top {
            ControlType::Block {
                height_of_value_stack,
                ..
            }
            | ControlType::For {
                height_of_value_stack,
                ..
            }
            | ControlType::If {
                height_of_value_stack,
                ..
            } => *height_of_value_stack,
            _ => 0,
        }
    }
    pub fn set_unreachable(&mut self) {
        let top = self.top();
        match top {
            ControlType::Block { reachable, .. }
            | ControlType::Func { reachable, .. }
            | ControlType::For { reachable, .. }
            | ControlType::If { reachable, .. } => *reachable = false,
        }
    }

    pub fn pop(&mut self) -> ControlType {
        self.stack.pop().expect("No ControlType on stack")
    }
    pub fn get_control_type(&mut self, depth: u32) -> &mut ControlType {
        if depth as usize > self.stack.len().saturating_sub(1) {
            panic!("Depth exceeds control depth.")
        }
        let control_stack_index_size = self.stack.len().saturating_sub(1);
        let start_control = self
            .stack
            .get_mut(control_stack_index_size - depth as usize)
            .expect("Can't index control start");
        start_control
    }
    pub fn push(&mut self, control_type: ControlType) {
        self.stack.push(control_type);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[should_panic]
    #[test]
    fn invalid_depth() {
        let mut control_stack = ControlStack::default();
        control_stack.get_control_type(2);
    }
}
