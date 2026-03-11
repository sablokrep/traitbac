use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "traitbac",
    version = "1.0",
    about = "Graphical Front end to BactoTraits
       ************************************************
       Gaurav Sablok,
       Email: codeprog@icloud.com
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// search pattern bacdive
    BactoIDs { bacdive_id: String },
    /// search ncbi
    BactoNCBI { ncbi_tax_id: String },
    /// search ncbi associated level
    BactoNCBIlevel { ncbi_associated_level: String },
    /// search rrdna
    BactorDNA { rrdna16s_accession: String },
    ///search taxonomy
    Bactotax { taxonomy_upd_method: String },
    /// search phylum
    BactoPhylo { phylum: String },
    /// search class
    BactoClass { class: String },
    /// search order
    BactoOrder { order: String },
    /// search family
    BactoFamily { family: String },
    /// search genus
    BactoGenus { genus: String },
    /// search species
    BactoSpecies { species: String },
    /// search rest
    Bactorest { rest: String },
}
