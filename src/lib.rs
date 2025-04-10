#![feature(min_specialization)]

// Single trait for all operations
trait Operation<N> {
    fn apply(&self) -> N;
    
    fn discretize(&self) -> Option<Box<dyn Operation<u32>>> {
        None
    }
}

struct DoubleOp<N>(N);

// Base implementation for all types with default methods
impl<N> Operation<N> for DoubleOp<N> where N: std::ops::Add<Output=N> + Copy {
    default fn apply(&self) -> N {
        self.0 + self.0
    }
    
    default fn discretize(&self) -> Option<Box<dyn Operation<u32>>> {
        None
    }
}

// Implementation for Operation<f32> that specializes the discretize method
impl Operation<f32> for DoubleOp<f32> {
    fn discretize(&self) -> Option<Box<dyn Operation<u32>>> {
        Some(Box::new(DoubleOp(self.0.abs() as u32)))
    }
}

struct OpTrace<N> {
    ops: Box<dyn Operation<N>>,
}

impl OpTrace<f32> {
    fn discretize(&self) -> OpTrace<u32> {
        if let Some(discrete_op) = self.ops.discretize() {
            OpTrace { ops: discrete_op }
        } else {
            panic!("Operation cannot be discretized")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let double = DoubleOp(2.5f32);
        let result = double.apply();
        assert_eq!(result, 5.0);
        
        // Test with boxed operation
        let boxed_op: Box<dyn Operation<f32>> = Box::new(DoubleOp(2.5f32));
        let trace = OpTrace { ops: boxed_op };
        let discrete_trace = trace.discretize();
        assert_eq!(discrete_trace.ops.apply(), 4);
    }
}
