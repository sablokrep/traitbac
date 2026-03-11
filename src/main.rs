mod args;
use self::class::class_search;
use self::family::family_search;
use self::genusa::genus_search;
use self::label::label_search;
use self::order::order_search;
use self::phylum::phylum_search;
use self::rest::rest_search;
use self::rrdna::rrdna_search;
use self::search::searchid;
use self::speciesa::species_search;
use self::taxid::taxid;
use self::taxonomy::taxonomy;
use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
mod class;
mod dataset;
mod family;
mod filestruct;
mod genus;
mod genusa;
mod label;
mod order;
mod phylum;
mod rest;
mod rrdna;
mod search;
mod species;
mod speciesa;
mod taxid;
mod taxonomy;
use figlet_rs::FIGfont;

/*
Gaurav Sablok
codeprog@icloud.com
*/

fn main() {
    let fontgenerate = FIGfont::standard().unwrap();
    let repgenerate = fontgenerate.convert("traitbac");
    println!("{}", repgenerate.unwrap());
    let args = CommandParse::parse();
    match &args.command {
        Commands::BactoIDs { bacdive_id } => {
            let n_threads = 4usize;
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
                let output = searchid(bacdive_id).unwrap();
                println!("The qsar modelling has finished: {:?}", output);
            });
        }
        Commands::BactoNCBI { ncbi_tax_id } => {
            let n_threads = 4usize;
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
                let output = taxid(ncbi_tax_id).unwrap();
                println!("The qsar modelling has finished: {:?}", output);
            });
        }
        Commands::BactoNCBIlevel {
            ncbi_associated_level,
        } => {
            let n_threads = 4usize;
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
                let output = label_search(ncbi_associated_level).unwrap();
                println!("The qsar modelling has finished: {:?}", output);
            });
        }
        Commands::BactorDNA { rrdna16s_accession } => {
            let n_threads = 4usize;
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
                let output = rrdna_search(rrdna16s_accession).unwrap();
                println!("The qsar modelling has finished: {:?}", output);
            });
        }
        Commands::Bactotax {
            taxonomy_upd_method,
        } => {
            let n_threads = 4usize;
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
                let output = taxonomy(taxonomy_upd_method).unwrap();
                println!("The qsar modelling has finished: {:?}", output);
            });
        }
        Commands::BactoPhylo { phylum } => {
            let n_threads = 4usize;
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
                let output = phylum_search(phylum).unwrap();
                println!("The qsar modelling has finished: {:?}", output);
            });
        }
        Commands::BactoClass { class } => {
            let n_threads = 4usize;
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
                let output = class_search(class).unwrap();
                println!("The qsar modelling has finished: {:?}", output);
            });
        }
        Commands::BactoOrder { order } => {
            let n_threads = 4usize;
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
                let output = order_search(order).unwrap();
                println!("The qsar modelling has finished: {:?}", output);
            });
        }
        Commands::BactoFamily { family } => {
            let n_threads = 4usize;
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
                let output = family_search(family).unwrap();
                println!("The qsar modelling has finished: {:?}", output);
            });
        }
        Commands::BactoGenus { genus } => {
            let n_threads = 4usize;
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
                let output = genus_search(genus).unwrap();
                println!("The qsar modelling has finished: {:?}", output);
            });
        }
        Commands::BactoSpecies { species } => {
            let n_threads = 4usize;
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
                let output = species_search(species).unwrap();
                println!("The qsar modelling has finished: {:?}", output);
            });
        }
        Commands::Bactorest { rest } => {
            let n_threads = 4usize;
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
                let output = rest_search(rest).unwrap();
                println!("The qsar modelling has finished: {:?}", output);
            });
        }
    }
}
