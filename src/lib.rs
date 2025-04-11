#![feature(min_specialization)]

use std::{any::Any, fmt::Debug, marker::PhantomData};

/// FRAMEWORK LOGIC TRAIT AND IMPLEMENTATION: 
/// 
// Single trait for all operations
trait Operation<N>: Debug + Any {
    fn apply(&self) -> N;
    
    fn discretize(&self) -> Option<Box<dyn Operation<u32>>> {
        None
    }
}

trait ProvableOperation<Field> : Operation<u32> {
    fn prove(&self);
}

trait IntoProvable {
    fn into_provable<Field>(&self) -> Box<dyn ProvableOperation<Field>>;
}

struct ProvableTranslator<Field> {
    _field: PhantomData<Field>,
}

impl<Field> ProvableTranslator<Field> {
    fn new() -> Self {
        Self { _field: PhantomData }
    }
    fn translate(op: Box<dyn Operation<u32>>) -> Box<dyn ProvableOperation<Field>> {
        let opa = op.as_ref() as &dyn Any;
        match opa.downcast_ref::<DoubleOp<u32>>() {
            Some(i) => i.into_provable(),
            None => panic!("Operation is not a DoubleOp")
        }
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

impl OpTrace<u32> {
    fn to_provable_trace<Field>(self) -> ProveTrace<Field>{
        ProveTrace {  ops: ProvableTranslator::translate(self.ops) }
    }
}

struct ProveTrace<Field> {
    ops: Box<dyn ProvableOperation<Field>>,
}
/// 
/// USER DEFINED IMPLEMENTATION
#[derive(Debug)]
struct DoubleOp<N>(N);

// Base implementation for all types with default methods
impl<N> Operation<N> for DoubleOp<N> 
where 
    N: std::ops::Add<Output = N> + Copy + Debug + 'static
{
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

impl IntoProvable for DoubleOp<u32> {
    fn into_provable<Field>(&self) -> Box<dyn ProvableOperation<Field>> {
        Box::new(DoubleOp(self.0))
    }
}
impl<Field> ProvableOperation<Field> for DoubleOp<u32> {
    fn prove(&self) {
        println!("Proving DoubleOp: {:?}", self.0);
    }
}



//impl<T> IntoProvable<u32> for Box<T> where T: IntoProvable<u32> {
//    fn into_provable(&self) -> Box<dyn ProvableOperation<u32>> {
//        self.as_ref().into_provable()
//    }
//}


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
        let provable_trace: ProveTrace<f32> = discrete_trace.to_provable_trace();
        provable_trace.ops.prove();
    }
}
