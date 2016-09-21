LML Parser
==========

About
-----
Lasanja Markup Language is an alternative to JSON, XML or any other text format and has a very minimalistic syntax.

The data structure consists of nodes with the following properties:
* text content
* tagname
* children of nodes 

LML file example
----------------
```
I'm a line of text.
And you can place enters.

Or multiple enters, I don't care.

first-node {
    I'm text content part of the node named 'first-node'.

    nested {
        I'm a child node part of 'first-node' named 'nested'. 
    }

    nested {
        I'm the second child node part of 'first-node' named 'nested'.
    }

    No the only type is string, you must parse the following nodes yourself to their appropriate type. 
     
    boolean { true }
    number { 1234 }

    If you want to write curly brackets in your text content then write them like so:
    {{ and }}
} 

And that's about it.
```

Usage
-----
```Rust
extern crate lml;
use lml::Lml;

fn main()
{
    let result = Lml::parse("name { John }
                             name {} 
                             name { Bert }");
    
    match result {
        Ok(lml) => {
            let names = lml.children
                           .iter()
                           .filter(|c| c.tag == "name" && c.children.len() == 1)
                           .map(|c| c.children[0].text.clone());
            
            for n in names {
                println!("Name: {}", n);
            }
        },
        Err(_) => {
            panic!("Error parsing LML.");
        }
    }
}

```

FAQ
---
### What is Lasanja?
A name I came up a gazillion years ago and finally had something to use it for. Also the curly brackets look like lasagne noodles.  

### So why isn't it spelled like "Lasagne"?
Well because all those popular domain names with "lasagne" in it were taken. And "lasanja" sounds like how you will pronounce it here in the Netherlands.  

License
-------
This project is licensed under the MIT license.