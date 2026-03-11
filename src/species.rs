use crate::filestruct::DIVE;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub fn speciespath() -> Result<(Vec<DIVE>, Vec<DIVE>), Box<dyn Error>> {
    let filepath = Path::new("../files/BACTOTRAITS_DATASET_2026-01-28_SPECIESLVL.csv");
    let newfilepath = filepath.to_str().unwrap().to_string();
    let fileopen = File::open(newfilepath).expect("file not present");
    let fileread = BufReader::new(fileopen);
    let mut vectsruct: Vec<DIVE> = Vec::new();
    let mut introductionline: Vec<DIVE> = Vec::new();

    for i in fileread.lines() {
        let line = i.expect("fline not present");
        if line.starts_with("BacDive_id") {
            let linesplit = line.split(",").collect::<Vec<_>>();
            introductionline.push(DIVE {
                bacdive_id: linesplit[0].to_string(),
                ncbi_tax_id: linesplit[1].to_string(),
                ncbi_associated_level: linesplit[2].to_string(),
                rrdna16s_accession: linesplit[3].to_string(),
                taxonomy_upd_method: linesplit[4].to_string(),
                phylum: linesplit[5].to_string(),
                class: linesplit[6].to_string(),
                order: linesplit[7].to_string(),
                family: linesplit[8].to_string(),
                genus: linesplit[9].to_string(),
                species: linesplit[10].to_string(),
                rest: linesplit[11..linesplit.len()].to_vec().concat().to_string(),
            });
        }

        let linesplit = line.split(",").collect::<Vec<_>>();
        vectsruct.push(DIVE {
            bacdive_id: linesplit[0].to_string(),
            ncbi_tax_id: linesplit[1].to_string(),
            ncbi_associated_level: linesplit[2].to_string(),
            rrdna16s_accession: linesplit[3].to_string(),
            taxonomy_upd_method: linesplit[4].to_string(),
            phylum: linesplit[5].to_string(),
            class: linesplit[6].to_string(),
            order: linesplit[7].to_string(),
            family: linesplit[8].to_string(),
            genus: linesplit[9].to_string(),
            species: linesplit[10].to_string(),
            rest: linesplit[11..linesplit.len()]
                .to_vec()
                .join(",")
                .to_string(),
        })
    }

    Ok((introductionline, vectsruct))
}
