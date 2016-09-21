extern crate lml;
use lml::Lml;

#[test]
fn filtering_test_1()
{
    let result = Lml::parse("person {
                                name { John }
                            }
                            person {
                                name { Bert } 
                            }
                            person { 
                                name { Dick } 
                            }");
    
    match result {
        Ok(lml) => {
            let filtered = lml.children.iter().filter(|c| c.tag == "person");
            assert_eq!(3, filtered.count())
        },
        Err(_) => {
            panic!("Error parsing LML.");
        }
    }
}

#[test]
fn filtering_test_2()
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