extern crate itertools;
use itertools::Zip;

pub fn hamming_distance(dna1: &str, dna2: &str) -> Result<u32, bool>
{
    if dna1 == dna2
    {
        Ok(0)
    }
    else if dna1.len() != dna2.len()
    {
        Err(true)
    }
    else
    {
        let chars1 = dna1.chars();
        let chars2 = dna2.chars();
        let mut count = 0;
        for (x, y) in Zip::new((chars1, chars2))
        {
            if x != y
            {
                count += 1;
            }
            else
            {
                continue;
            }
        }
        Ok(count)
    }
}
