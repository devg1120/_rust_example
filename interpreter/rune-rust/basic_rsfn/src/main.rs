use rune::termcolor::{ColorChoice, StandardStream};
use rune::{ContextError, Diagnostics, Module, Vm};

use std::sync::Arc;
use rune_modules;
use std::sync::Mutex;

//static  count: Mutex<i64> = 1.into();
static mut N: i64 = 0;

#[rune::function(instance)]
fn divide_by_three(value: i64) -> i64 {
    //let c =  count.lock().unwrap();
    unsafe {
        N += 1;
        value / 3  + N
    }
}

fn main() -> rune::support::Result<()> {
    let m = module()?;

    let mut context = rune_modules::default_context()?;
    context.install(m)?;
    let runtime = Arc::new(context.runtime()?);

    let mut sources = rune::sources!(entry => {
        pub fn main(number) {
            number.divide_by_three()
        }
    });

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
    let output = vm.execute(["main"], (33i64,))?.complete().into_result()?;
    let output: i64 = rune::from_value(output)?;

    println!("output: {}", output);

    let output2 = vm.execute(["main"], (33i64,))?.complete().into_result()?;
    let output2: i64 = rune::from_value(output2)?;

    println!("output: {}", output2);
   unsafe{
    println!("N: {}", N);
}


    Ok(())
}

fn module() -> Result<Module, ContextError> {
    let mut m = Module::with_item(["mymodule"])?;
    m.function_meta(divide_by_three)?;
    Ok(m)
}


/*
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
                a + b 
            }

            pub fn main() {
                foo
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
    let output = vm.call(["main"], ())?;
    let output: Function = rune::from_value(output)?;

    println!("{}", output.call::<(i64, i64), i64>((1, 3)).into_result()?);
    println!("{}", output.call::<(i64, i64), i64>((2, 6)).into_result()?);
    Ok(())
}
*/
