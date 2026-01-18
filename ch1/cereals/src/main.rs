#[derive(Debug)]

enum Cereal {
    Barley, Millet, Rice,
    Rye, Spelt, Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![];
    grains.push(Cereal::Millet);
    drop(grains);

    println!("{:?}", grains);
}


// NOTE:
// Rust prevents dangling references, but it does not prevent logical memory exhaustion. ex:
fn this_breaks() {
    let mut big: Vec<u8> = Vec::new();

    loop {
        big.push(0); // loop until OS determines it's out of memory due to infinite push
    }
}

// Shows example of how memory clearing a value won't allow that memory reference to be used
// i.e., variable was cleared and printing the unset variable won't work. Examples in C# and PHP:

// Logical dangling:
//
// using System;
// using System.Collections.Generic;

// class Program
// {
//    static void Main()
//    {
//        List<string> grains = new List<string>();
//        grains.Add("Rye");

//        grains = null;

//        Console.WriteLine(grains.Count); // NullReferenceException
//    }
// }


// Actual dangling:
/*
using System;

class Program
{
    unsafe static void Main()
    {
        int* ptr;

        {
            int value = 42;
            ptr = &value;  // pointer to stack variable
        } // value goes out of scope

        Console.WriteLine(*ptr); // Undefined behavior (dangling pointer)
    }
}

## Object disposed

using System;
using System.IO;

class Program
{
    static void Main()
    {
        FileStream stream = new FileStream("test.txt", FileMode.Create);
        stream.Dispose();

        stream.WriteByte(65); // ObjectDisposedException
    }
}

## PHP
## unsetting clears variable from memory

<?php
$grains = [];
$grains[] = "Rye";

unset($grains);

echo count($grains);

## Via reference drop

<?php
$a = ["Rye", "Wheat"];
$b = &$a;   // reference

unset($a); // you unset the value

print_r($b);

## Object destroyed but method still called
<?php
class GrainStore {
    public function list() {
        echo "Listing grains\n";
    }
}

$store = new GrainStore();
unset($store);

$store->list(); // Fatal error

*/

