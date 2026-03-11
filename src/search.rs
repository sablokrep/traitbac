use crate::dataset::datasetpath;
use crate::filestruct::DIVEOUT;
use crate::genus::genuspath;
use crate::species::speciespath;
use prettytable::Table;
use std::error::Error;

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub fn searchid(id: &str) -> Result<String, Box<dyn Error>> {
    let datasets = datasetpath().unwrap();
    let genuspathunwrap = genuspath().unwrap();
    let speciespathunwrap = speciespath().unwrap();
    let mut filepath: Vec<DIVEOUT> = Vec::new();
    let intitialstruct = datasets.0;

    for i in intitialstruct.iter() {
        for data in datasets.1.iter() {
            for val in genuspathunwrap.1.iter() {
                for species in speciespathunwrap.1.iter() {
                    if data.bacdive_id.contains(id)
                        || val.bacdive_id.contains(id)
                        || species.bacdive_id.contains(id)
                    {
                        filepath.push(DIVEOUT {
                            bacdivename: i.bacdive_id.clone(),
                            ncbi_associated_level_name: i.ncbi_associated_level.clone(),
                            ncbi_tax_id_name: i.ncbi_tax_id.clone(),
                            rrdna16s_accession_name: i.rrdna16s_accession.clone(),
                            taxonomy_upd_method_name: i.taxonomy_upd_method.clone(),
                            phylum_name: i.phylum.clone(),
                            class_name: i.class.clone(),
                            order_name: i.order.clone(),
                            family_name: i.family.clone(),
                            genus_name: i.genus.clone(),
                            species_name: i.species.clone(),
                            rest_name: i.rest.clone(),
                            bacdive_id: data.bacdive_id.clone(),
                            ncbi_tax_id: data.ncbi_tax_id.clone(),
                            ncbi_associated_level: data.ncbi_associated_level.clone(),
                            rrdna16s_accession: data.rrdna16s_accession.clone(),
                            taxonomy_upd_method: data.taxonomy_upd_method.clone(),
                            phylum: data.phylum.clone(),
                            class: data.class.clone(),
                            order: data.order.clone(),
                            family: data.family.clone(),
                            genus: data.genus.clone(),
                            species: data.species.clone(),
                            rest: data.rest.clone(),
                            bacdive_id_genus: val.bacdive_id.clone(),
                            ncbi_tax_id_genus: val.ncbi_associated_level.clone(),
                            ncbi_associated_level_genus: val.ncbi_associated_level.clone(),
                            rrdna16s_accession_genus: val.rrdna16s_accession.clone(),
                            taxonomy_upd_method_genus: val.taxonomy_upd_method.clone(),
                            phylum_genus: val.phylum.clone(),
                            class_genus: val.class.clone(),
                            order_genus: val.order.clone(),
                            family_genus: val.family.clone(),
                            genus_genus: val.genus.clone(),
                            species_genus: val.species.clone(),
                            rest_genus: val.rest.clone(),
                            bacdive_id_species: val.bacdive_id.clone(),
                            ncbi_tax_id_species: val.ncbi_associated_level.clone(),
                            ncbi_associated_level_species: val.ncbi_associated_level.clone(),
                            rrdna16s_accession_species: val.rrdna16s_accession.clone(),
                            taxonomy_upd_method_species: val.taxonomy_upd_method.clone(),
                            phylum_species: val.phylum.clone(),
                            class_species: val.class.clone(),
                            order_species: val.order.clone(),
                            family_species: val.family.clone(),
                            genus_species: val.genus.clone(),
                            species_species: val.species.clone(),
                            rest_species: val.rest.clone(),
                        });
                    }
                }
            }
        }
    }

    let mut tableheading: Vec<String> = Vec::new();
    let mut tablefirstrow: Vec<String> = Vec::new();
    let mut tableseconrow: Vec<String> = Vec::new();
    let mut tablethirdrow: Vec<String> = Vec::new();

    for i in filepath.iter() {
        tableheading.push(i.bacdivename.clone());
        tableheading.push(i.ncbi_associated_level_name.clone());
        tableheading.push(i.ncbi_tax_id_name.clone());
        tableheading.push(i.rrdna16s_accession_name.clone());
        tableheading.push(i.taxonomy_upd_method_name.clone());
        tableheading.push(i.phylum_name.clone());
        tableheading.push(i.class_name.clone());
        tableheading.push(i.order_name.clone());
        tableheading.push(i.family_name.clone());
        tableheading.push(i.genus_name.clone());
        tableheading.push(i.species_name.clone());
        tableheading.push(i.rest_name.clone());
        tablefirstrow.push(i.bacdive_id.clone());
        tablefirstrow.push(i.ncbi_associated_level.clone());
        tablefirstrow.push(i.ncbi_tax_id.clone());
        tablefirstrow.push(i.rrdna16s_accession.clone());
        tablefirstrow.push(i.taxonomy_upd_method.clone());
        tablefirstrow.push(i.phylum.clone());
        tablefirstrow.push(i.class.clone());
        tablefirstrow.push(i.order.clone());
        tablefirstrow.push(i.family.clone());
        tablefirstrow.push(i.genus.clone());
        tablefirstrow.push(i.species.clone());
        tablefirstrow.push(i.rest.clone());
        tableseconrow.push(i.bacdive_id_genus.clone());
        tableseconrow.push(i.ncbi_associated_level_genus.clone());
        tableseconrow.push(i.ncbi_tax_id_genus.clone());
        tableseconrow.push(i.rrdna16s_accession_genus.clone());
        tableseconrow.push(i.taxonomy_upd_method_genus.clone());
        tableseconrow.push(i.phylum_genus.clone());
        tableseconrow.push(i.class_genus.clone());
        tableseconrow.push(i.order_genus.clone());
        tableseconrow.push(i.family_genus.clone());
        tableseconrow.push(i.genus_genus.clone());
        tableseconrow.push(i.species_genus.clone());
        tableseconrow.push(i.rest_name.clone());
        tablethirdrow.push(i.bacdive_id_genus.clone());
        tablethirdrow.push(i.ncbi_associated_level_genus.clone());
        tablethirdrow.push(i.ncbi_tax_id_genus.clone());
        tablethirdrow.push(i.rrdna16s_accession_genus.clone());
        tablethirdrow.push(i.taxonomy_upd_method_genus.clone());
        tablethirdrow.push(i.phylum_genus.clone());
        tablethirdrow.push(i.class_genus.clone());
        tablethirdrow.push(i.order_genus.clone());
        tablethirdrow.push(i.family_genus.clone());
        tablethirdrow.push(i.genus_genus.clone());
        tablethirdrow.push(i.species_genus.clone());
        tablethirdrow.push(i.rest_genus.clone());
    }

    let table_heading = tableheading.join(",");
    let first_1 = tablefirstrow.join(",");
    let first_2 = tableseconrow.join(",");
    let first_3 = tablethirdrow.join(",");

    let value = format!("{}\n{}\n{}\n{}", table_heading, first_1, first_2, first_3);

    let table = Table::from_csv_string(value.as_str()).unwrap();
    table.printstd();
    println!("");
    println!(
        "{}",
        String::from_utf8(table.to_csv(Vec::new()).unwrap().into_inner().unwrap()).unwrap()
    );

    Ok("The file path has been written".to_string())
}
