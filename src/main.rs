
#[derive(Debug)]
enum Rgb { RED, GREEN , BLUE }

enum Chromatic {
    MonoChrome,
    Colour { color : Rgb }
}

use Chromatic::*;
use Rgb::*;

fn doit (s : Chromatic) {
    match s {
        MonoChrome => { println!("Monochrome"); }
        Colour { color : RED }   => { println!("RWoot ed"); }
        Colour { color : GREEN } => { println!("Green"); }
        Colour { color : BLUE }  => { println!("Blue"); }
    }
}

fn main() {
    doit( Colour { color : RED });
    doit( Colour { color : GREEN });
    doit( Colour { color : BLUE });
    doit( MonoChrome );

    let x  = GREEN;
    println!("Woot {:?}", x);

    fn foo( x : u64) {
        println!("Woot : {:?}", x);
    }

    foo(200);

    let x : &'static str = "foo goo bar";

    fn goo( s : &str) {
        println!("bananarama {}", s);
    }
    goo(x);

    fn add<T,F>(x : T, y : T, op : F) -> T
        where F : Fn(T,T) -> T {
        return op(x,y);
    }

    println!( "{}", add( 2.0, 3.0, |a,b| a+b));
    println!( "{}", add( 2, 3, |a,b| a + b ));
}
