use reqwest;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    genome_search().await?;


    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct GenomeDataResponse {
    data: Vec<GenomeData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GenomeData {
    id: String,
    attributes: GenomeAttributes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GenomeAttributes {
    accession: String,

    #[serde(rename = "genome-id")]
    genome_id: Option<u128>,

    #[serde(rename = "geographic-range")]
    geographic_range: Option<Vec<String>>,

    #[serde(rename = "geographic-origin")]
    geographic_origin: Option<String>,

    #[serde(rename = "ena-genome-accession")]
    ena_genome_accession: Option<String>,

    #[serde(rename = "ena-sample-accession")]
    ena_sample_accession: Option<String>,

    #[serde(rename = "ena-study-accession")]
    ena_study_accession: Option<String>,

    #[serde(rename = "ncbi-genome-accession")]
    ncbi_genome_accession: Option<String>,

    #[serde(rename = "ncbi-sample-accession")]
    ncbi_sample_accession: Option<String>,

    #[serde(rename = "ncbi-study-accession")]
    ncbi_study_accession: Option<String>,

    #[serde(rename = "img-genome-accession")]
    img_genome_accession: Option<String>,

    #[serde(rename = "patric-genome-accession")]
    patric_genome_accession: Option<String>,

    length: Option<u128>,

    #[serde(rename = "num-contigs")]
    num_contigs: Option<u128>,

    #[serde(rename = "n-50")]
    n_50: Option<f64>,

    #[serde(rename = "gc-content")]
    gc_content: Option<f32>,

    #[serde(rename = "type")]
    genome_type: Option<String>,

    completeness: Option<f32>,
    
    contamination: Option<f32>,
    
    #[serde(rename = "rna-5s")]
    rna_5s: Option<f32>,

    #[serde(rename = "rna-16s")]
    rna_16s: Option<f32>,

    #[serde(rename = "rna-23s")]
    rna_23s: Option<f32>,

    trnas: Option<f32>,

    #[serde(rename = "nc-rnas")]
    nc_rnas: Option<u32>,

    #[serde(rename = "num-proteins")]
    num_proteins: Option<u128>,

    #[serde(rename = "eggnog-coverage")]
    eggnog_coverage: Option<f32>,

    #[serde(rename = "ipr-coverage")]
    ipr_coverage: Option<f32>,

    #[serde(rename = "taxon-lineage")]
    taxon_lineage: Option<String>,

    #[serde(rename = "num-genomes-total")]
    num_genomes_total: Option<u32>,

    #[serde(rename = "pangenome-size")]
    pangenome_size: Option<u32>,

    #[serde(rename = "pangenome-core-size")]
    pangenome_core_size: Option<u128>,

    #[serde(rename = "pangenome-accessory-size")]
    pangenome_accessory_size: Option<u128>,
    
    // Define other attributes you want to access here
}

async fn genome_search() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "https://www.ebi.ac.uk/metagenomics/api/v1/genomes";
    let client = reqwest::Client::new();

    // Make your HTTP request asynchronously
    let response = client.get(base_url).send().await?;

    // Deserialize JSON response using serde_json
    let response_bytes = response.bytes().await?;
    let genome_data: GenomeDataResponse = serde_json::from_slice(&response_bytes)?;

    // Filter genomes based on the geographic location
    let filter_genomes: Vec<GenomeData> = genome_data
    .data.clone()
    .into_iter()
    .filter(|genome| genome.attributes.geographic_origin == Some("Asia".to_string()))
    .collect();

    // Access the array of genomes and print information for each genome
    for genome in filter_genomes {
        println!("Genome ID: {:?}", genome.id);
        println!("Genome Accession: {:?}", genome.attributes.accession);
        println!("Genome Length: {:?}", genome.attributes.length);
        println!("Geographic Location: {:?}", genome.attributes.geographic_origin);
        println!("GC-Content: {:?}", genome.attributes.gc_content);

        // Construct the download URL based on the accession number
        let download_url = format!("{}/{}/downloads/{}{}", base_url, genome.id, genome.id, ".fna");
        println!("Download Link: {}", download_url);

        // Access other attributes as needed
        // Example: println!("Geographic Origin: {:?}", genome.attributes.geographic_origin);

        println!(); // Separate genome entries
    }
    Ok(())
    }