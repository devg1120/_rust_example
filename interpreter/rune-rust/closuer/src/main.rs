use rune::runtime::Function;
use rune::termcolor::{ColorChoice, StandardStream};
use rune::{Diagnostics, Vm};
use rune_modules;

use std::sync::Arc;

fn main() -> rune::support::Result<()> {
    let context = rune_modules::default_context()?;
    let runtime = Arc::new(context.runtime()?);

    let mut sources = rune::sources! {
        entry => {
            fn foo(a, b) {
                let x = a;
                a + b 
            }


            fn foo_add1(i , n) {
                let name = String::from(n);
                let x = [0];
                let f = |a,b | {  

                          println!("name: {:?}", name);
                          println!("a: {:?}", a);
                          println!("b: {:?}", b);
                          name.push_str("ABC");
                          println!("x: {:?}", x[0]);
                          x[0] = x[0] + 9;
                          //let z = x + 2; 
                          //x = z;
                          //println!("z: {:?}", z);
                          println!("x: {:?}", x[0]);
                          println!("name: {:?}", name);
                          a + b  +x[0] 
                       };
                let a = || { println!("--x: {:?}", x[0]);};
                let b = || { x[0]  = x[0]+ 100;};

                (f,a,b )
            }

            fn foo_add2(n, b) {
                let x = n;
                |a,b|  {  a + b  +x }
            }

            pub fn main() {
                foo_add1
            }
        }
    };

    let mut diagnostics = Diagnostics::new();

    let result = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();

    if !diagnostics.is_empty() {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        diagnostics.emit(&mut writer, &sources)?;
    }

    let unit = result?;

    let mut vm = Vm::new(runtime, Arc::new(unit));
    let output1 = vm.call(["main"], ())?;
    let output2: Function = rune::from_value(output1)?;
    //println!("{:?}", output2.call::<(i64, i64), Function>((1, 3)).into_result()?);
    //let closure  = output2.call::<(i64, i64), Function>((2, 3)).into_result()?;
    let (closure, c2, c3 ) = output2.call::<(i64,&str) , (Function, Function, Function)>((3,"GUSA1120") ).into_result()?;
    println!("{:?}", closure);
    println!("{:?}", closure.call::<(i64, i64), i64>((1, 1)).into_result()?);
    println!("{:?}", c2.call::<(),()>(()).into_result()?);
    println!("{:?}", closure.call::<(i64, i64), i64>((1, 1)).into_result()?);
    println!("{:?}", c2.call::<(),()>(()).into_result()?);
    println!("{:?}", c3.call::<(),()>(()).into_result()?);
    println!("{:?}", c2.call::<(),()>(()).into_result()?);
    println!("{:?}", closure.call::<(i64, i64), i64>((1, 1)).into_result()?);
    //println!("{:?}", closure.call::<(i64, i64), i64>((1, 1)).into_result()?);


    //println!("{}", closure.call::<(i64, i64), i64>((1, 3)).into_result()?);

   // println!("{:?}", output2.call::<(i64), i64>((1)).into_result()?);
    //println!("{:?}", output2.call::<(i64, i64), i64>((1, 3)).into_result()?);

    //let output3 = output2.call::<(i64 ), Function>((3)).into_result()?;

    //println!("{}", output3.call::<(i64, i64), i64>((1, 3)).into_result()?);
    //println!("{}", output3.call::<(i64, i64), i64>((2, 6)).into_result()?);
    Ok(())
}
