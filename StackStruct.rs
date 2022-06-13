//use std::boxed to get ownership of the simple heap allocation 

use std::boxed::Box;

//use std::mem to query the size and alignment of types, initialize and manipulate memory.

use std::mem;


//use enumeration with the generic type parameter S to define the stack state as empty

enum StateOfStack<S>{
    
    Empty,
    
    Value (S,Box<StateOfStack<S>>),
    
}



//the initializer: a stack is always initizlied as empty 

impl <S> Default for Stack<S>{
    
    fn default() -> Stack<S>{
        
        return Stack {
            
            myStack: StateOfStack::Empty,
        }
    }
}





//define a struct called Stack to implement stack data structure  

pub struct Stack<S>{
    
    // The stack of the automaton
    
    myStack: StateOfStack<S>,
}


impl <S> Stack<S>{
    
    //implement push() function to add an element onto the stack
    
    fn push(&mut self, type_parameter_of_next_state: S) -> (){
        
        //passing to replace the memory of myStack to temp stack
        
        let temp_stack = mem::replace(&mut self.myStack, StateOfStack::Empty);
        
        //assigning the newly pushed in values onto myStack here followed by LIFO method
        
        self.myStack = StateOfStack::Value(type_parameter_of_next_state,Box::new(temp_stack));
    }
    
    
    //implement pop() function to remove the most recently added element from the collection and return the element
    
    fn pop(&mut self)->Option<S>{
        
        //match to replace the memory 
        
        match mem::replace(&mut self.myStack, StateOfStack::Empty){
            
            //enure the stack is not an empty stack before popping element 
            
            StateOfStack::Empty => None,
            
            //popping out the top most element within the stack followed by LIFO method 
            
            StateOfStack::Value(current_stack_state, temp_stack) => {
                
                self.myStack =* temp_stack;
                
                //return the current stack state 
                
                Some(current_stack_state)
            },
        }
    }
    
    
    
    //implement peek() function to return the most recently added element (without removing it)
    
    fn peek(&self)->Option<&S>{
    
        
        match self.myStack{
            
            //ensure the stack is not an empty stack before returning the peek value 
            
            StateOfStack::Empty => None,
            
            //use ref to annotate pattern bindings to return the peek value rather than removing it 
            
            StateOfStack::Value(ref current_stack_state,_) => {
                
                //return the top value of the stack 
                
                Some(current_stack_state)
            },
        }
    }
}







//Driver code 

//start with an empty stack st and perform the following functions in order.

fn main() {
    
    let mut st : Stack<i32> = Stack::default();
    // new stack called st
    
    assert!(st.peek().is_none());
    // push integer values onto empty stack 
    st.push(3);

    assert!(st.peek().is_some());
    //println!("{:?}", st.peek()); 
    //the returned value is 3. test passed. 

    st.push(4);
    st.push(5);
    
    assert!(st.peek().is_some());
    
    //enure we have the correct peek 
    
    assert_eq!(5,*st.peek().unwrap());
    
    //popping the most recently added element out of the stack
    
    st.pop();
    
    //println!("{:?}", st.pop()); 
    //the deleted and returned value is 5. test passed. 
    
    assert!(st.peek().is_some());
    
    //ensure we have the correct peek 
    assert_eq!(4,*st.peek().unwrap());
    //println!("{:?}", st.peek()); 
    //the returned value is 4. test passed. 
    st.push(6); 
    
    assert!(st.peek().is_some());
    
    st.pop(); 
    
    //println!("{:?}", st.peek()); 
    //the deleted and returned value is 6. test passed. 
    


}




//Some extra tests

fn ExtraTests(){
    let mut TestingStack : Stack<i32> = Stack::default();
    // new stack called TestingStack
    assert!(TestingStack.peek().is_none());
    
    // push integer values onto empty stack 
    
    TestingStack.push(52);
    assert!(TestingStack.peek().is_some());
    
    //println!("{:?}", TestingStack.peek()); 
    //the returned value is 52. test passed. 
    TestingStack.push(59);
    
    TestingStack.push(88);
    
    assert!(TestingStack.peek().is_some());
    
    //enure we have the correct peek 
    
    assert_eq!(88,*TestingStack.peek().unwrap());
    
    //popping the most recently added element out of the stack
    TestingStack.pop();
    
    //println!("{:?}", TestingStack.pop()); 
    //the deleted and returned value is 88. test passed. 
    
    assert!(TestingStack.peek().is_some());
    
    //ensure we have the correct peek 
    
    assert_eq!(59,*TestingStack.peek().unwrap());
    
    //println!("{:?}", TestingStack.peek()); 
    //the returned value is 59. test passed. 
    TestingStack.push(16); 
    assert!(TestingStack.peek().is_some());
    
    TestingStack.pop(); 
    //println!("{:?}", TestingStack.peek()); 
    //the deleted and returned value is 16. test passed. 

    
    
    
    
}