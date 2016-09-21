extern crate lml;
use lml::Lml;

#[test]
fn parsing()
{
    assert!(Lml::parse("Hello, universe! This is your captain speaking.").is_ok());
    assert!(Lml::parse("tag { text content }").is_ok());
    assert!(Lml::parse("text content tag { text }").is_ok());
    
    assert!(Lml::parse(
       "person {
            first-name { John }
            last-name { Doe }
        }
        person {
            first-name { Bert }
            last-name { From Sesame Street }
        }
        person {
            first-name { Dick }
            last-name { Johnson }
        }"
    ).is_ok());

    assert!(Lml::parse("tag { {{ content with escaped brackets }} }").is_ok());

    assert!(Lml::parse(
       "I'm a line of text.
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

        And that's about it."
    ).is_ok());
}