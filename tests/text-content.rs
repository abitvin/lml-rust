extern crate lml;
use lml::Lml;

#[test]
fn children()
{
    match Lml::parse("-a- aaa {} -b- -b- -b- bbb {} -c- -c-") {
        Ok(lml) => {
            assert_eq!("-a-", lml.children[0].text);
            assert_eq!("aaa", lml.children[1].tag);
            assert_eq!("-b- -b- -b-", lml.children[2].text);
            assert_eq!("bbb", lml.children[3].tag);
            assert_eq!("-c- -c-", lml.children[4].text);
        },
        Err(_) => {
            panic!("Error parsing LML.");
        },
    }
}

#[test]
fn escaped()
{
    match Lml::parse("---{{---") {
        Ok(lml) => assert_eq!("---{---", lml.children[0].text),
        Err(_) => panic!("Error parsing LML."),
    }

    match Lml::parse("---{{---}}---") {
        Ok(lml) => assert_eq!("---{---}---", lml.children[0].text),
        Err(_) => panic!("Error parsing LML."),
    }
}

#[test]
fn newlines()
{
    let code = "    aaa
    
    
    {}
    
    
    bbb
        ccc
    ddd
    
    
    
    eee{}     "; 

    match Lml::parse(code) {
        Ok(lml) => {
            assert_eq!("aaa", lml.children[0].tag);
            assert_eq!("bbb\n        ccc\n    ddd", lml.children[1].text);
            assert_eq!("eee", lml.children[2].tag);
        },
        Err(_) => {
            panic!("Error parsing LML.");
        },
    }
}

#[test]
fn spaces()
{
    match Lml::parse("   aaa{}    AAA   ccc{}    DDD   EEE    fff{}     ggg{}         HHH      III    ") {
        Ok(lml) => {
            assert_eq!("aaa", lml.children[0].tag);
            assert_eq!("AAA", lml.children[1].text);
            assert_eq!("ccc", lml.children[2].tag);
            assert_eq!("DDD   EEE", lml.children[3].text);
            assert_eq!("fff", lml.children[4].tag);
            assert_eq!("ggg", lml.children[5].tag);
            assert_eq!("HHH      III", lml.children[6].text);
        },
        Err(_) => {
            panic!("Error parsing LML.");
        },
    }
}