use std::collections::HashMap;
//use uuid::Uuid;
//use std::fmt::Formatter;
use std::fmt;
//use std::fmt::Debug;

//#[derive(Debug)]
pub struct Proteins<'a>
{
    map: HashMap<&'a str, &'a str>,
}

pub fn parse(condon_map: Vec<(&'static str, &'static str)>) -> Proteins
{
    //let mut map = HashMap::new();
    //map = condon_map.into_iter().collect();

    let map = condon_map.into_iter().collect();

    Proteins { map: map }
}

impl <'a>Proteins<'a>
{
    pub fn name_for(&self, key: &'a str) -> Result<&str, &str>
    {
        if self.map.contains_key(key)
        {
            Ok(self.map.get(key).unwrap())
        }
        else
        {
            Err("Protein name not found")
        }
    }

    pub fn of_rna(&self, rna_sequence: &'a str) -> Result<Vec<&str>, &str>
    {
        //info.of_rna("AUGUUUUGG") -> Ok(vec!["methionine", "phenylalanine", "tryptophan"])

        if rna_sequence.len() % 3 != 0
        {
            return Err("Invalid RNA sequence");
        }

        let mut proteins: Vec<&str> = Vec::new();
        let mut s = rna_sequence;

        while !s.is_empty()
        {
            let (condon, rest) = s.split_at(3);
            let name = self.name_for(condon);

            if name.is_err()
            {
                return Err("Invalid RNA sequence");
            }

            let name = self.name_for(condon).unwrap();

            if name == "stop codon"
            {
                s = "";
            }
            else
            {
                proteins.push(name); // .unwrap_or(Err("you are fucked")));
                s = rest;
            }
        }

        if proteins.len() > 0
        {
            Ok(proteins)
        }
        else
        {
            Err("Dude, something's wrong")
        }
    }


}

//impl Debug for Proteins
//{
//    //fn fmt(&self, &mut f: Formatter) -> Result
//    fn fmt(&self, f: &mut Formatter) -> fmt::Result
//    {
//        write!(f.buf, "Something bad happened"); // {}", self.id);
//    }
//}

//impl<T: fmt::Debug> fmt::Debug for Array<T> {
//    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//        self.data[..].fmt(formatter)
//    }
//}

impl <'a>fmt::Debug for Proteins<'a>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "This is a debugger") //Mapped Proteins {}", self.map.iter().)
    }
}


//let name = self.map.get(key).unwrap() as &str;

//.unwrap(); //    .unwrap().to_string();

//        {
//            Some(n) => n,
//            None => ""
//        };
//
//        println!("name: {}", name);

//let value = match map.entry(key)

//let values = match self.map.get(key)
//        let name = match self.map.get(key)
//        {
//            //Ok(protein) => Ok(&protein.to_string()), //  Ok(protein.unwrap().to_string()),  //., // Ok(&protein.to_string()),
//            //Ok(protein) => Ok("fuck you"), //  self.map.get(key), //   .unwrap().to_string()),
//            //Err(e) => Result::e //Err(e) // return Err(e) // "error" // _ // Option(&"fuckyou") // 0 // () //{ Option("cunt", e) }
//
//            Ok(v) => *v = "fuckyou",
//            Err(e) => *e = 0,
//
//            //Some(v) => v,
//            //None => { panic!("impossiburu!"); }
//            //None => "Not found" //Err("impossiburu!")
//            //None => Error("Not found") //Err("impossiburu!")
//        };

//        match self.map.get(key)
//        {
//            //Ok(protein) => Ok(&protein.to_string()), //  Ok(protein.unwrap().to_string()),  //., // Ok(&protein.to_string()),
//            //Ok(protein) => Ok("fuck you"), //  self.map.get(key), //   .unwrap().to_string()),
//            //Err(e) => Result::e //Err(e) // return Err(e) // "error" // _ // Option(&"fuckyou") // 0 // () //{ Option("cunt", e) }
//
//            Ok(v) => *v = "fuckyou",
//            Err(e) => *e = 0,
//
//            //Some(v) => v,
//            //None => { panic!("impossiburu!"); }
//            //None => "Not found" //Err("impossiburu!")
//            //None => Error("Not found") //Err("impossiburu!")
//        }


//name

//        let mut value = match &self.map.entry(key)
//        {
//            //Entry::Occupied(o) => o.into_mut(),
//            Entry::Occupied(v) => v,
//            Entry::Vacant => "impossiburu!"
//            //Entry::Vacant(v) => v, //Err("Not found")
//            //Entry::Occupied(o) => o,
//        };