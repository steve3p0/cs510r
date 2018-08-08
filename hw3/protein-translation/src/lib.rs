use std::collections::HashMap;

#[derive(Debug)]
pub struct Proteins<'a>
{
    map: HashMap<&'a str, &'a str>,
}

pub fn parse(condon_map: Vec<(&'static str, &'static str)>) -> Proteins
{
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