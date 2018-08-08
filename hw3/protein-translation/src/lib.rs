

//parse(make_pairs())    Result<Vec<String>, String>
//name_for("AUG")
//info.name_for("").is_err()
//of_rna("UGG UGU UAU")

pub struct Proteins
{
    codon_map: Vec<(&'static str, &'static str)>
}



impl Proteins
{

    // Vec<(&'static str, &'static str)>
    pub fn parse(condon_map: Vec<(&'static str, &'static str)>) -> Result<Vec<String>, String>
    {

        unimplemented!("Not implemented!")
    }

    pub fn name_for(codon: &str) -> Result<String, String>
    {
        unimplemented!("Not implemented!")
    }

    pub fn of_rna(rna_sequence: &str) -> Result<Vec<String>, String>
    {
        unimplemented!("Not implemented!")
    }

}



