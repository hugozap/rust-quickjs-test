//import quickjs
use quick_js::{Context, JsValue};

fn main() {
    /* Create a JavaScript environment, provide a rust function that can be called from javascript */
    let ctx = Context::new().unwrap();
    ctx.add_callback("rust_add", |a: i32, b: i32| a + b).unwrap();
    
    /* Evaluate a script */
    let result = ctx.eval("rust_add(1, 2)").unwrap();
    //match result, if is a number, print it
    match result {
        JsValue::Int(int) => println!("result: {}", int),
        _ => println!("result: {:?}", result),
    }

    //Run javascript file
    //load file contents from file in current directory

     
    let contents = std::fs::read_to_string("test.js").unwrap();
    //set js stdout for quickjs
    
    //run js code
    let f = ctx.eval(&contents).unwrap();
    println!("result: {:?}", f);
    
    

    



}
