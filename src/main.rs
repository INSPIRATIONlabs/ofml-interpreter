//! OFML Interpreter CLI
//!
//! A command-line interface for OFML (Office Furniture Modeling Language) processing.

use std::fs;
use std::path::Path;
use std::process;

use clap::{Parser, Subcommand};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use ofml_interpreter::alb_loader::{load_manufacturer_with_deps, AlbLoader};
use ofml_interpreter::oap::families::ProductFamily;
use ofml_interpreter::geometry;
use ofml_interpreter::operations::{
    self, export_to_glb, load_geometry_file, validate_geometry, ProductConfig,
};
use ofml_interpreter::parser::Parser as OfmlParser;
use ofml_interpreter::tokenize;
use ofml_interpreter::Interpreter;

/// OFML Interpreter - Process Office Furniture Modeling Language files
#[derive(Parser)]
#[command(name = "ofml")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Increase output verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // =====================
    // OAP Configurator Commands
    // =====================
    /// List available manufacturers from OFML data directory
    Manufacturers {
        /// Path to OFML data directory
        data_path: String,
        /// Output as JSON
        #[arg(short, long)]
        json: bool,
    },

    /// List articles for a specific manufacturer
    Articles {
        /// Path to OFML data directory
        data_path: String,
        /// Manufacturer ID (e.g., "vitra")
        manufacturer: String,
        /// Output as JSON
        #[arg(short, long)]
        json: bool,
        /// Filter by series
        #[arg(short, long)]
        series: Option<String>,
    },

    /// Configure a product and display/export pricing
    Configure {
        /// Path to OFML data directory
        data_path: String,
        /// Manufacturer ID
        manufacturer: String,
        /// Article/class name
        article: String,
        /// Output as JSON
        #[arg(short, long)]
        json: bool,
        /// Export configuration to file
        #[arg(short, long)]
        export: Option<String>,
        /// Price lookup date (YYYY-MM-DD)
        #[arg(short = 'd', long)]
        price_date: Option<String>,
        /// List available properties and exit
        #[arg(short, long)]
        list_properties: bool,
        /// Property values (format: name=value)
        #[arg(last = true)]
        properties: Vec<String>,
    },

    /// Launch interactive Terminal UI for browsing and configuration
    #[cfg(feature = "tui")]
    Tui {
        /// Path to OFML data directory
        data_path: String,
        /// Initial price date (YYYY-MM-DD)
        #[arg(short = 'd', long)]
        price_date: Option<String>,
    },

    /// Browse XCF catalog structure for a manufacturer
    Catalog {
        /// Path to OFML data directory
        data_path: String,
        /// Manufacturer ID (e.g., "bisley", "kn")
        manufacturer: String,
        /// Language code (default: de)
        #[arg(short, long, default_value = "de")]
        language: String,
        /// Output as JSON
        #[arg(short, long)]
        json: bool,
        /// Show full tree structure
        #[arg(short, long)]
        tree: bool,
        /// Search for category/article by name
        #[arg(short, long)]
        search: Option<String>,
        /// List all available catalogs
        #[arg(long)]
        list: bool,
        /// Load a specific catalog by name (e.g., "desks_m_cat")
        #[arg(short = 'c', long)]
        catalog_name: Option<String>,
    },

    // =====================
    // Existing Commands
    // =====================
    /// Parse and display AST (CLS files)
    Parse {
        /// Path to CLS file
        file_path: String,
    },

    /// Check syntax only (CLS files)
    Check {
        /// Path to CLS file
        file_path: String,
    },

    /// Display tokens (CLS files)
    Tokenize {
        /// Path to CLS file
        file_path: String,
    },

    /// Execute the file (CLS files)
    Run {
        /// Path to CLS file
        file_path: String,
    },

    /// Execute CLS and export scene to GLB
    Export {
        /// Path to CLS file
        file_path: String,
    },

    /// Convert 3DS/GEO/OBJ file to GLB
    Convert {
        /// Path to geometry file
        file_path: String,
    },

    /// Merge multiple geometry files into one GLB
    Merge {
        /// Output GLB file path
        output_path: String,
        /// Input geometry files
        input_files: Vec<String>,
    },

    /// Read and dump EBASE database
    Ebase {
        /// Path to EBASE file
        file_path: String,
        /// Table name to dump records
        table_name: Option<String>,
    },

    /// Assemble product from OFML data
    Product {
        /// Path to product directory
        product_path: String,
        /// Article name (optional)
        article: Option<String>,
        /// Output GLB path (optional)
        output: Option<String>,
    },

    /// Validate geometry file and show metrics
    Validate {
        /// Path to geometry file
        file_path: String,
    },

    /// Explore OFML data directory
    Ofml {
        /// Path to OFML data directory
        data_path: String,
        /// Manufacturer name (optional)
        manufacturer: Option<String>,
        /// Product name (optional)
        product: Option<String>,
    },

    /// List and extract CLS files from ALB
    Cls {
        /// Path to ALB file
        alb_path: String,
        /// Filename pattern to extract
        filename: Option<String>,
    },

    /// List or extract files from ALB
    Alb {
        /// Path to ALB file
        alb_path: String,
        /// Pattern to extract
        pattern: Option<String>,
        /// Output directory
        output_dir: Option<String>,
    },

    /// Build product from CLS using scene graph
    Build {
        /// Path to ALB file
        alb_path: String,
        /// Class name to instantiate
        class_name: Option<String>,
    },

    /// Convert product with OBJ files to GLB
    Gsx {
        /// Path to product directory
        product_path: String,
        /// Output GLB path
        output: Option<String>,
    },

    /// Export 2D floor plan from odb2d to SVG
    Svg {
        /// Path to EBASE file with odb2d
        ebase_path: String,
        /// Output SVG path
        output: Option<String>,
    },

    /// Evaluate EBASE expression
    Expr {
        /// Expression to evaluate
        expression: String,
    },

    /// Load manufacturer with multi-ALB support
    Manufacturer {
        /// Path to OFML data directory
        data_dir: String,
        /// Manufacturer name
        manufacturer: Option<String>,
        /// Class name to instantiate
        class_name: Option<String>,
    },

    /// Extract 3DS files from ALB and convert to GLB
    Extract {
        /// Path to ALB file
        alb_path: String,
        /// Pattern to match
        pattern: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    // Configure tracing based on verbosity
    let filter = match cli.verbose {
        0 => "error",
        1 => "warn",
        2 => "info",
        _ => "debug",
    };

    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stderr))
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(filter)))
        .init();

    let result = match cli.command {
        // OAP Commands
        Commands::Manufacturers { data_path, json } => cmd_oap_manufacturers(&data_path, json),
        Commands::Articles {
            data_path,
            manufacturer,
            json,
            series,
        } => cmd_oap_articles(&data_path, &manufacturer, json, series.as_deref()),
        Commands::Configure {
            data_path,
            manufacturer,
            article,
            json,
            export,
            price_date,
            list_properties,
            properties,
        } => cmd_oap_configure(
            &data_path,
            &manufacturer,
            &article,
            json,
            export.as_deref(),
            price_date.as_deref(),
            list_properties,
            &properties,
        ),
        #[cfg(feature = "tui")]
        Commands::Tui {
            data_path,
            price_date,
        } => cmd_oap_tui(&data_path, price_date.as_deref()),
        Commands::Catalog {
            data_path,
            manufacturer,
            language,
            json,
            tree,
            search,
            list,
            catalog_name,
        } => cmd_oap_catalog(&data_path, &manufacturer, &language, json, tree, search.as_deref(), list, catalog_name.as_deref()),

        // Existing commands
        Commands::Parse { file_path } => {
            let source = read_source_file(&file_path);
            cmd_parse(&source, &file_path)
        }
        Commands::Check { file_path } => {
            let source = read_source_file(&file_path);
            cmd_check(&source, &file_path)
        }
        Commands::Tokenize { file_path } => {
            let source = read_source_file(&file_path);
            cmd_tokenize(&source, &file_path)
        }
        Commands::Run { file_path } => {
            let source = read_source_file(&file_path);
            cmd_run(&source, &file_path)
        }
        Commands::Export { file_path } => {
            let source = read_source_file(&file_path);
            cmd_export(&source, &file_path)
        }
        Commands::Convert { file_path } => cmd_convert(&file_path),
        Commands::Merge {
            output_path,
            input_files,
        } => {
            let files: Vec<&str> = input_files.iter().map(|s| s.as_str()).collect();
            cmd_merge(&output_path, &files)
        }
        Commands::Ebase {
            file_path,
            table_name,
        } => cmd_ebase(&file_path, table_name.as_deref()),
        Commands::Product {
            product_path,
            article,
            output,
        } => cmd_product(&product_path, article.as_deref(), output.as_deref()),
        Commands::Validate { file_path } => cmd_validate(&file_path),
        Commands::Ofml {
            data_path,
            manufacturer,
            product,
        } => cmd_ofml(&data_path, manufacturer.as_deref(), product.as_deref()),
        Commands::Cls { alb_path, filename } => cmd_cls(&alb_path, filename.as_deref()),
        Commands::Alb {
            alb_path,
            pattern,
            output_dir,
        } => cmd_alb(&alb_path, pattern.as_deref(), output_dir.as_deref()),
        Commands::Build {
            alb_path,
            class_name,
        } => cmd_build(&alb_path, class_name.as_deref()),
        Commands::Gsx {
            product_path,
            output,
        } => cmd_gsx(&product_path, output.as_deref()),
        Commands::Svg { ebase_path, output } => cmd_svg(&ebase_path, output.as_deref()),
        Commands::Expr { expression } => cmd_expr(&expression),
        Commands::Manufacturer {
            data_dir,
            manufacturer,
            class_name,
        } => cmd_manufacturer(&data_dir, manufacturer.as_deref(), class_name.as_deref()),
        Commands::Extract { alb_path, pattern } => {
            cmd_extract(&alb_path, pattern.as_deref().unwrap_or(""))
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn read_source_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", path, e);
            process::exit(1);
        }
    }
}

type CmdResult = Result<(), String>;

// ============================================================================
// OAP Configurator Commands
// ============================================================================

fn cmd_oap_manufacturers(data_path: &str, json_output: bool) -> CmdResult {
    use ofml_interpreter::oap::{manufacturers, strings, Manufacturer};

    let path = Path::new(data_path);
    if !path.exists() {
        return Err(format!(
            "{}: {}",
            strings::MSG_MANUFACTURER_NOT_FOUND,
            data_path
        ));
    }

    // Initialize manufacturer names from Manufacturers.ebase
    manufacturers::init_from_data_path(path);

    // Load installed manufacturers from SQLite (fast)
    let installed = manufacturers::get_installed_manufacturers(path);

    let mfr_list: Vec<Manufacturer> = installed
        .into_iter()
        .map(|m| Manufacturer {
            id: m.id,
            name: m.name,
            path: m.path,
        })
        .collect();

    if json_output {
        let output = serde_json::json!({
            "manufacturers": mfr_list,
            "total": mfr_list.len()
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    } else {
        println!("{} {}:", strings::MSG_MANUFACTURERS_HEADER, data_path);
        println!();
        for m in &mfr_list {
            println!("  {:<12} {}", m.id, m.name);
        }
        println!();
        println!("{}: {} Hersteller", strings::MSG_TOTAL, mfr_list.len());
    }

    Ok(())
}

fn cmd_oap_articles(
    data_path: &str,
    manufacturer: &str,
    json_output: bool,
    series_filter: Option<&str>,
) -> CmdResult {
    use ofml_interpreter::oap::{manufacturers, ocd, strings, Article};

    let path = Path::new(data_path);
    let mfr_path = path.join(manufacturer);

    if !mfr_path.exists() {
        return Err(format!(
            "{}: '{}'",
            strings::MSG_MANUFACTURER_NOT_FOUND,
            manufacturer
        ));
    }

    // Initialize manufacturer names from Manufacturers.ebase
    manufacturers::init_from_data_path(path);

    // Get manufacturer display name
    let mfr_display_name = manufacturers::get_display_name(manufacturer);

    // Load OCD articles from pdata.ebase files
    let articles_with_desc = ocd::load_articles_with_descriptions(&mfr_path, "DE");

    let mut articles: Vec<Article> = articles_with_desc
        .into_iter()
        // Filter out internal/cryptic articles
        .filter(|(ocd_art, _)| {
            // Skip internal articles (starting with @)
            if ocd_art.article_nr.starts_with('@') {
                return false;
            }
            // Skip articles with control characters in series
            if ocd_art.series.chars().any(|c| c.is_control()) {
                return false;
            }
            true
        })
        .map(|(ocd_art, description)| {
            Article {
                id: ocd_art.article_nr.clone(),
                manufacturer_id: manufacturer.to_string(),
                series_id: if ocd_art.series.is_empty() {
                    None
                } else {
                    Some(ocd_art.series.clone())
                },
                short_description: description,
                long_description: None,
                base_article_number: ocd_art.article_nr.clone(),
                has_configuration: true,
            }
        })
        .collect();

    // Apply series filter
    if let Some(series) = series_filter {
        articles.retain(|a| {
            a.series_id
                .as_ref()
                .map(|s| s.to_lowercase().contains(&series.to_lowercase()))
                .unwrap_or(false)
        });
    }

    // Sort by series first, then by article number
    articles.sort_by(|a, b| {
        match (&a.series_id, &b.series_id) {
            (Some(sa), Some(sb)) => sa.cmp(sb).then(a.id.cmp(&b.id)),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.id.cmp(&b.id),
        }
    });

    if json_output {
        let output = serde_json::json!({
            "manufacturer": manufacturer,
            "manufacturer_name": mfr_display_name,
            "articles": articles,
            "total": articles.len()
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    } else {
        println!("{} {} ({}):", strings::MSG_ARTICLES_HEADER, mfr_display_name, manufacturer);
        println!();

        // Group by series
        let mut current_series: Option<String> = None;
        for article in &articles {
            let article_series = article.series_id.clone().unwrap_or_default();
            if current_series.as_ref() != Some(&article_series) {
                if !article_series.is_empty() {
                    println!();
                    println!("  [{}]", article_series);
                }
                current_series = Some(article_series);
            }

            let config_indicator = if article.has_configuration {
                "●"
            } else {
                "○"
            };
            println!(
                "    {} {:<30} {}",
                config_indicator, article.id, article.short_description
            );
        }

        println!();
        println!("{}: {} Artikel", strings::MSG_TOTAL, articles.len());
    }

    Ok(())
}

fn cmd_oap_configure(
    data_path: &str,
    manufacturer: &str,
    article: &str,
    json_output: bool,
    export_path: Option<&str>,
    price_date_str: Option<&str>,
    list_properties: bool,
    property_args: &[String],
) -> CmdResult {
    use chrono::NaiveDate;
    use ofml_interpreter::oap::config::Configuration;
    use ofml_interpreter::oap::price::{PriceLookup, PriceQuery};
    use ofml_interpreter::oap::property::{parse_property_value, validate_property_value};
    use ofml_interpreter::oap::{format_german_price_with_currency, strings};

    let path = Path::new(data_path);

    // Parse price date
    let price_date = match price_date_str {
        Some(s) => NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map_err(|_| format!("Invalid date format: {}. Use YYYY-MM-DD", s))?,
        None => chrono::Local::now().date_naive(),
    };

    // Load manufacturer
    let loader =
        load_manufacturer_with_deps(path, manufacturer, None).map_err(|e| e.to_string())?;

    let mut interp = Interpreter::new();
    loader
        .load_into_interpreter(&mut interp)
        .map_err(|e| e.to_string())?;

    // Find the class
    let class = interp
        .classes
        .get(article)
        .cloned()
        .ok_or_else(|| format!("{}: '{}'", strings::MSG_ARTICLE_NOT_FOUND, article))?;

    // Instantiate the class
    let instance = interp
        .instantiate_class_public(class)
        .map_err(|e| e.to_string())?;

    // Extract properties from instance
    let properties = ofml_interpreter::oap::property::extract_properties_from_cls(&instance);

    // If list-properties flag, just show properties and exit
    if list_properties {
        println!("Properties for {}::{}:", manufacturer, article);
        println!();

        let mut defs: Vec<_> = properties.definitions.iter().collect();
        defs.sort_by_key(|(_, d)| d.sort_order);

        for (name, def) in defs {
            let type_str = match &def.prop_type {
                ofml_interpreter::property::PropertyType::Int { min, max } => {
                    let min_str = min.map(|v| v.to_string()).unwrap_or_default();
                    let max_str = max.map(|v| v.to_string()).unwrap_or_default();
                    format!("int     {}-{}", min_str, max_str)
                }
                ofml_interpreter::property::PropertyType::Float { min, max } => {
                    let min_str = min.map(|v| format!("{:.1}", v)).unwrap_or_default();
                    let max_str = max.map(|v| format!("{:.1}", v)).unwrap_or_default();
                    format!("float   {}-{}", min_str, max_str)
                }
                ofml_interpreter::property::PropertyType::Choice { options } => {
                    format!("choice  {}", options.join(","))
                }
                ofml_interpreter::property::PropertyType::Bool => "bool".to_string(),
                ofml_interpreter::property::PropertyType::String => "string".to_string(),
            };
            let state_str = match def.state {
                ofml_interpreter::property::PropertyState::Enabled => "[enabled]",
                ofml_interpreter::property::PropertyState::Hidden => "[hidden]",
                ofml_interpreter::property::PropertyState::ReadOnly => "[readonly]",
            };
            println!("  {:<16} {} {}", name, type_str, state_str);
        }
        return Ok(());
    }

    // Create configuration
    let mut config = Configuration::with_properties(
        article.to_string(),
        manufacturer.to_string(),
        properties.clone(),
    );

    // Apply property values from command line
    for prop_arg in property_args {
        if let Some((name, value_str)) = prop_arg.split_once('=') {
            if let Some(def) = config.properties.definitions.get(name) {
                match parse_property_value(def, value_str) {
                    Ok(value) => {
                        if let Err(e) = validate_property_value(def, &value) {
                            return Err(format!(
                                "{}: {} - {}",
                                strings::MSG_INVALID_PROPERTY_VALUE,
                                name,
                                e
                            ));
                        }
                        config.properties.values.insert(name.to_string(), value);
                    }
                    Err(e) => {
                        return Err(format!(
                            "{}: {} - {}",
                            strings::MSG_INVALID_PROPERTY_VALUE,
                            name,
                            e
                        ));
                    }
                }
            } else {
                return Err(format!("Property not found: {}", name));
            }
        } else {
            return Err(format!(
                "Invalid property format: {}. Use name=value",
                prop_arg
            ));
        }
    }

    // Generate variant code
    config.update_variant_code();

    // Lookup price
    let price_lookup = PriceLookup::new();
    let price_query = PriceQuery::new(
        manufacturer.to_string(),
        config.article_number.clone().unwrap_or_default(),
        config.variant_code.clone(),
        price_date,
    );

    config.price = price_lookup.lookup(&price_query).ok();

    // Output
    if json_output {
        let export_data = config.to_export_data();
        println!("{}", serde_json::to_string_pretty(&export_data).unwrap());
    } else {
        println!(
            "{}: {}::{}",
            strings::MSG_CONFIGURING,
            manufacturer,
            article
        );
        if let Some(ref art_num) = config.article_number {
            println!("Article: {}", art_num);
        }
        println!();

        println!("{}:", strings::MSG_PROPERTIES);
        let mut defs: Vec<_> = config.properties.definitions.iter().collect();
        defs.sort_by_key(|(_, d)| d.sort_order);

        for (name, def) in defs {
            let value = config
                .properties
                .values
                .get(name)
                .map(|v| format!("{:?}", v))
                .unwrap_or_else(|| "-".to_string());

            let type_info = match &def.prop_type {
                ofml_interpreter::property::PropertyType::Int { min, max } => {
                    let min_str = min.map(|v| v.to_string()).unwrap_or_default();
                    let max_str = max.map(|v| v.to_string()).unwrap_or_default();
                    format!("[{}-{}]", min_str, max_str)
                }
                ofml_interpreter::property::PropertyType::Float { min, max } => {
                    let min_str = min.map(|v| format!("{:.1}", v)).unwrap_or_default();
                    let max_str = max.map(|v| format!("{:.1}", v)).unwrap_or_default();
                    format!("[{}-{}]", min_str, max_str)
                }
                ofml_interpreter::property::PropertyType::Choice { options } => {
                    format!("[{}]", options.join(","))
                }
                ofml_interpreter::property::PropertyType::Bool => "[ja/nein]".to_string(),
                ofml_interpreter::property::PropertyType::String => "".to_string(),
            };

            println!("  {:<16} = {:<16} {}", name, value, type_info);
        }

        println!();
        println!("{}: {}", strings::MSG_VARIANT_CODE, config.variant_code);

        if let Some(ref price) = config.price {
            println!(
                "{}: {}",
                strings::MSG_BASE_PRICE,
                format_german_price_with_currency(price.base_price, &price.currency)
            );
            for surcharge in &price.surcharges {
                let amount_str = if surcharge.is_percentage {
                    format!("{}%", surcharge.amount)
                } else {
                    format_german_price_with_currency(surcharge.amount, &price.currency)
                };
                println!("  + {}: {}", surcharge.name, amount_str);
            }
            println!(
                "{}: {}",
                strings::MSG_TOTAL_PRICE,
                format_german_price_with_currency(price.total_price, &price.currency)
            );
            println!(
                "{}: {}",
                strings::MSG_PRICE_DATE,
                price.price_date.format("%d.%m.%Y")
            );
        } else {
            println!("{}", strings::MSG_PRICE_NOT_AVAILABLE);
        }
    }

    // Export if requested
    if let Some(export_file) = export_path {
        let export_data = config.to_export_data();
        let json = serde_json::to_string_pretty(&export_data).map_err(|e| e.to_string())?;
        fs::write(export_file, json).map_err(|e| e.to_string())?;
        eprintln!("{}: {}", strings::MSG_EXPORT_SUCCESS, export_file);
    }

    Ok(())
}

fn cmd_oap_catalog(
    data_path: &str,
    manufacturer: &str,
    language: &str,
    json_output: bool,
    show_tree: bool,
    search: Option<&str>,
    list_catalogs: bool,
    catalog_name: Option<&str>,
) -> CmdResult {
    use ofml_interpreter::oap::catalog::{
        find_manufacturer_catalogs, CatalogLoader, CatalogNode, NodeType,
    };
    use ofml_interpreter::oap::manufacturers;

    let path = Path::new(data_path);
    let mfr_path = path.join(manufacturer);

    if !mfr_path.exists() {
        return Err(format!("Manufacturer not found: {}", manufacturer));
    }

    // Initialize manufacturer names
    manufacturers::init_from_data_path(path);
    let mfr_name = manufacturers::get_display_name(manufacturer);

    // Find all catalogs
    let catalogs = find_manufacturer_catalogs(&mfr_path);

    if catalogs.is_empty() {
        // List available series as fallback
        println!("No XCF catalog found for {} ({})", mfr_name, manufacturer);
        println!();
        println!("Available series directories:");
        if let Ok(entries) = std::fs::read_dir(&mfr_path) {
            let mut series: Vec<_> = entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .filter_map(|e| e.file_name().to_str().map(|s| s.to_string()))
                .filter(|s| !s.starts_with('.') && s != "catalog")
                .collect();
            series.sort();
            for s in &series {
                println!("  {}", s);
            }
        }
        return Ok(());
    }

    // List catalogs mode
    if list_catalogs {
        if json_output {
            let json_cats: Vec<_> = catalogs.iter().map(|c| {
                serde_json::json!({
                    "name": c.name,
                    "is_master": c.is_master,
                    "path": c.path.display().to_string()
                })
            }).collect();
            println!("{}", serde_json::to_string_pretty(&json_cats).unwrap());
        } else {
            println!("Available catalogs for {} ({}):", mfr_name, manufacturer);
            println!();
            for cat in &catalogs {
                let master_tag = if cat.is_master { " [MASTER]" } else { "" };
                println!("  {}{}", cat.name, master_tag);
                println!("    Path: {}", cat.path.display());
            }
            println!();
            println!("Use --catalog-name to load a specific catalog");
        }
        return Ok(());
    }

    // Load specific or default catalog
    let catalog = if let Some(name) = catalog_name {
        // Find specific catalog by name
        let cat_info = catalogs.iter()
            .find(|c| c.name == name)
            .ok_or_else(|| format!("Catalog '{}' not found. Use --list to see available catalogs.", name))?;
        CatalogLoader::load(&cat_info.path, language)
            .map_err(|e| format!("Failed to load catalog: {}", e))?
    } else {
        // Load first master catalog, or first available
        let cat_info = catalogs.iter()
            .find(|c| c.is_master)
            .or_else(|| catalogs.first())
            .ok_or_else(|| "No catalog available".to_string())?;
        CatalogLoader::load(&cat_info.path, language)
            .map_err(|e| format!("Failed to load catalog: {}", e))?
    };

    let stats = catalog.stats();

    // Handle search
    if let Some(query) = search {
        let query_lower = query.to_lowercase();

        fn search_tree(node: &CatalogNode, query: &str, path: &[String]) -> Vec<(Vec<String>, CatalogNode)> {
            let mut results = Vec::new();
            let current_path: Vec<String> = path.iter().cloned().chain(std::iter::once(node.name.clone())).collect();

            if node.name.to_lowercase().contains(query) || node.id.to_lowercase().contains(query) {
                results.push((current_path.clone(), node.clone()));
            }

            for child in &node.children {
                results.extend(search_tree(child, query, &current_path));
            }
            results
        }

        let results = search_tree(&catalog.root, &query_lower, &[]);

        if json_output {
            let json_results: Vec<_> = results.iter().map(|(path, node)| {
                serde_json::json!({
                    "path": path.join(" > "),
                    "id": node.id,
                    "name": node.name,
                    "type": match node.node_type {
                        NodeType::Folder => "folder",
                        NodeType::Article => "article",
                        NodeType::Root => "root",
                    }
                })
            }).collect();
            println!("{}", serde_json::to_string_pretty(&json_results).unwrap());
        } else {
            println!("Search results for '{}' in {} ({}):", query, mfr_name, manufacturer);
            println!();
            for (path, node) in &results {
                let type_icon = match node.node_type {
                    NodeType::Folder => "📁",
                    NodeType::Article => "📄",
                    NodeType::Root => "🏠",
                };
                println!("{} {} ({})", type_icon, path.join(" > "), node.id);
            }
            println!();
            println!("Found {} results", results.len());
        }
        return Ok(());
    }

    // Output
    if json_output {
        fn node_to_json(node: &CatalogNode) -> serde_json::Value {
            serde_json::json!({
                "id": node.id,
                "name": node.name,
                "type": match node.node_type {
                    NodeType::Folder => "folder",
                    NodeType::Article => "article",
                    NodeType::Root => "root",
                },
                "children": node.children.iter().map(node_to_json).collect::<Vec<_>>()
            })
        }

        let output = serde_json::json!({
            "manufacturer": manufacturer,
            "manufacturer_name": mfr_name,
            "language": language,
            "stats": {
                "total_nodes": stats.total_nodes,
                "folders": stats.folder_count,
                "articles": stats.article_count,
                "languages": stats.languages,
            },
            "catalog": node_to_json(&catalog.root)
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    } else {
        println!("XCF Catalog: {} ({})", mfr_name, manufacturer);
        println!("Language: {}", language);
        println!("Source: {}", catalog.source_path.display());
        println!();
        println!("Statistics:");
        println!("  Categories: {}", stats.folder_count);
        println!("  Articles: {}", stats.article_count);
        println!("  Languages: {}", stats.languages.join(", "));
        println!();

        if show_tree {
            fn print_tree(node: &CatalogNode, indent: usize, max_depth: usize) {
                if indent > max_depth {
                    return;
                }
                let prefix = "  ".repeat(indent);
                let icon = match node.node_type {
                    NodeType::Folder => "📁",
                    NodeType::Article => "📄",
                    NodeType::Root => "🏠",
                };

                if node.node_type != NodeType::Root {
                    println!("{}{} {}", prefix, icon, node.name);
                }

                for child in &node.children {
                    print_tree(child, indent + 1, max_depth);
                }
            }

            println!("Catalog Structure:");
            print_tree(&catalog.root, 0, 10);
        } else {
            // Show top-level categories only
            println!("Top-level Categories:");
            for child in &catalog.root.children {
                let child_count = child.children.len();
                let article_count = child.article_count();
                println!("  📁 {} ({} sub-categories, {} articles)",
                    child.name, child_count, article_count);
            }
            println!();
            println!("Use --tree to see full structure, --search to find items");
        }
    }

    Ok(())
}

#[cfg(feature = "tui")]
fn cmd_oap_tui(data_path: &str, price_date_str: Option<&str>) -> CmdResult {
    use crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::{backend::CrosstermBackend, Terminal};

    use ofml_interpreter::oap::engine::ConfigurationEngine;
    use ofml_interpreter::oap::manufacturers;
    use ofml_interpreter::oap::strings;
    use ofml_interpreter::tui::app::App;

    let path = Path::new(data_path);
    if !path.exists() {
        return Err(format!(
            "{}: {}",
            strings::MSG_MANUFACTURER_NOT_FOUND,
            data_path
        ));
    }

    // Parse price date
    let price_date = match price_date_str {
        Some(s) => chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map_err(|_| format!("Invalid date format: {}. Use YYYY-MM-DD", s))?,
        None => chrono::Local::now().date_naive(),
    };

    // Setup terminal
    enable_raw_mode().map_err(|e| e.to_string())?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).map_err(|e| e.to_string())?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(|e| e.to_string())?;

    // Create app and load manufacturers
    let mut app = App::new(data_path.to_string());
    app.price_date = price_date;

    // Create configuration engine
    let mut engine = ConfigurationEngine::new(path);

    // Initialize manufacturer names from Manufacturers.ebase
    manufacturers::init_from_data_path(path);

    // Load installed manufacturers from SQLite (fast)
    let installed = manufacturers::get_installed_manufacturers(path);
    for mfr in installed {
        app.manufacturers.push(ofml_interpreter::oap::Manufacturer {
            id: mfr.id,
            name: mfr.name,
            path: mfr.path,
        });
    }
    // Already sorted by get_installed_manufacturers

    // Main loop
    let result = run_tui_loop(&mut terminal, &mut app, &mut engine);

    // Restore terminal
    disable_raw_mode().map_err(|e| e.to_string())?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .map_err(|e| e.to_string())?;
    terminal.show_cursor().map_err(|e| e.to_string())?;

    result
}

#[cfg(feature = "tui")]
fn run_tui_loop(
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    app: &mut ofml_interpreter::tui::App,
    engine: &mut ofml_interpreter::oap::engine::ConfigurationEngine,
) -> CmdResult {
    use crossterm::event::{self, Event, KeyCode};
    use ofml_interpreter::oap::catalog::load_smart_catalog;
    use ofml_interpreter::oap::families::FamilyConfiguration;
    use ofml_interpreter::oap::ocd;
    use ofml_interpreter::tui::{ui::render, Message, Screen};

    loop {
        terminal
            .draw(|f| render(f, app))
            .map_err(|e| e.to_string())?;

        if let Event::Key(key) = event::read().map_err(|e| e.to_string())? {
            let msg = match key.code {
                KeyCode::Char('q') if !app.search_active => Some(Message::Quit),
                KeyCode::Char('?') if !app.search_active => Some(Message::ShowHelp),
                KeyCode::Char('/') if !app.search_active => Some(Message::ToggleSearch),
                KeyCode::Char('e') if !app.search_active => Some(Message::Export),
                KeyCode::Char('t') if !app.search_active && app.selected_manufacturer.is_some() => {
                    // Load tables for current manufacturer
                    if let Some(ref mfr) = app.selected_manufacturer {
                        app.status_message = Some("Lade Tabellen...".to_string());
                        let _ = terminal.draw(|f| render(f, app));
                        app.tables = load_manufacturer_tables(&mfr.path);
                        app.table_list_state.select(Some(0));
                        app.status_message = Some(format!("{} Tabellen geladen", app.tables.len()));
                    }
                    Some(Message::ShowTables)
                }
                KeyCode::Up => Some(Message::NavigateUp),
                KeyCode::Down => Some(Message::NavigateDown),
                KeyCode::Left if app.screen == Screen::FamilyConfig => Some(Message::CyclePropertyOption(-1)),
                KeyCode::Right if app.screen == Screen::FamilyConfig => Some(Message::CyclePropertyOption(1)),
                KeyCode::Left if app.screen == Screen::TableView => Some(Message::ScrollTableHorizontal(-1)),
                KeyCode::Right if app.screen == Screen::TableView => Some(Message::ScrollTableHorizontal(1)),
                KeyCode::Enter => {
                    if let Some(idx) = app.get_selected_index() {
                        match app.screen {
                            Screen::Manufacturers => {
                                // Load catalog and product families for the selected manufacturer
                                if idx < app.manufacturers.len() {
                                    // Clone manufacturer data to avoid borrow issues
                                    let manufacturer_id = app.manufacturers[idx].id.clone();
                                    let manufacturer_name = app.manufacturers[idx].name.clone();
                                    let manufacturer_path = app.manufacturers[idx].path.clone();

                                    app.status_message = Some(format!("Lade {}...", manufacturer_name));
                                    let _ = terminal.draw(|f| render(f, app));

                                    // Try to load XCF catalog (master or aggregated from series)
                                    let data_path = Path::new(&app.data_path);
                                    let catalog = load_smart_catalog(
                                        data_path,
                                        &manufacturer_path,
                                        &manufacturer_id,
                                        "de"
                                    );
                                    let has_catalog = catalog.is_some();
                                    app.set_catalog(catalog);

                                    // Load product families using ConfigurationEngine
                                    let families = engine.load_families(&manufacturer_id);

                                    let configurable_count = families.iter()
                                        .filter(|f| f.is_configurable)
                                        .count();
                                    let with_props_count = families.iter()
                                        .filter(|f| !f.prop_classes.is_empty())
                                        .count();

                                    app.families = families.to_vec();

                                    // Set status message
                                    if has_catalog {
                                        let cat_stats = app.catalog.as_ref()
                                            .map(|c| c.stats())
                                            .unwrap_or_else(|| ofml_interpreter::oap::catalog::CatalogStats {
                                                total_nodes: 0,
                                                folder_count: 0,
                                                article_count: 0,
                                                text_entries: 0,
                                                languages: vec![],
                                            });
                                        app.status_message = Some(format!(
                                            "Katalog: {} Kategorien, {} Artikel",
                                            cat_stats.folder_count,
                                            cat_stats.article_count
                                        ));
                                        // Navigate to catalog view
                                        app.screen = Screen::Catalog;
                                    } else {
                                        app.status_message = Some(format!(
                                            "{} Produktfamilien ({} konfigurierbar, {} mit Eigenschaften)",
                                            app.families.len(),
                                            configurable_count,
                                            with_props_count
                                        ));
                                        // Navigate to families view
                                        app.screen = Screen::Families;
                                    }
                                }
                                Some(Message::SelectManufacturer(idx))
                            }
                            Screen::Catalog => {
                                // Handle catalog selection
                                if idx < app.catalog_children.len() {
                                    let node = app.catalog_children[idx].clone();
                                    match node.node_type {
                                        ofml_interpreter::oap::catalog::NodeType::Folder => {
                                            app.enter_catalog_folder(&node);
                                            app.status_message = Some(format!(
                                                "{} - {} Einträge",
                                                node.name,
                                                node.children.len()
                                            ));
                                        }
                                        ofml_interpreter::oap::catalog::NodeType::Article => {
                                            // Find family matching this article and configure it
                                            // Use case-insensitive matching and try multiple strategies
                                            let node_id_upper = node.id.to_uppercase();
                                            let matching_family = app.families.iter()
                                                .find(|f| f.article_nrs.iter().any(|nr| {
                                                    let nr_upper = nr.to_uppercase();
                                                    // Exact match (case-insensitive)
                                                    nr_upper == node_id_upper ||
                                                    // Family article contains catalog node id
                                                    nr_upper.contains(&node_id_upper) ||
                                                    // Catalog node id contains family article
                                                    node_id_upper.contains(&nr_upper)
                                                }));

                                            if let Some(family) = matching_family {
                                                if let Some(ref manufacturer) = app.selected_manufacturer {
                                                    // Load properties for this family
                                                    let properties = engine.get_family_properties(&manufacturer.id, &family.id);
                                                    app.family_properties = properties.clone();
                                                    app.selected_family = Some(family.clone());

                                                    // Create configuration
                                                    let mut config = FamilyConfiguration::new(&family.id, &properties);

                                                    // Apply variant settings if node has a variant code
                                                    if let Some(ref variant_code) = node.variant_code {
                                                        if let Some(ref catalog) = app.catalog {
                                                            if let Some(variant_def) = catalog.get_variant(&node.id, variant_code) {
                                                                // Parse and apply property settings
                                                                // Format: "PROPERTYCLASS.PROPERTY=VALUE"
                                                                for setting in &variant_def.property_settings {
                                                                    if let Some(eq_pos) = setting.rfind('=') {
                                                                        let prop_path = &setting[..eq_pos];
                                                                        let value = &setting[eq_pos + 1..];
                                                                        // Extract property name (after last dot)
                                                                        if let Some(dot_pos) = prop_path.rfind('.') {
                                                                            let prop_key = &prop_path[dot_pos + 1..];
                                                                            config.set(prop_key, value);
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }

                                                    app.family_config = Some(config.clone());

                                                    // Calculate price
                                                    app.family_price = engine.calculate_family_price(
                                                        &manufacturer.id,
                                                        family,
                                                        &config,
                                                        app.price_date,
                                                    );

                                                    app.screen = Screen::FamilyConfig;
                                                    let price_str = app.family_price.as_ref()
                                                        .map(|p| format!("{:.2} {}", p.base_price, p.currency))
                                                        .unwrap_or_else(|| "Preis n/a".to_string());
                                                    app.status_message = Some(format!(
                                                        "{} - {}",
                                                        family.name, price_str
                                                    ));
                                                }
                                            } else {
                                                // No matching family found - try to create a minimal family from the catalog node
                                                // This allows viewing articles that aren't in the family list
                                                if let Some(ref manufacturer) = app.selected_manufacturer {
                                                    // Get series from catalog node or path
                                                    let series = node.series_ref.clone()
                                                        .or_else(|| {
                                                            // Try to extract series from breadcrumb
                                                            app.catalog_path.first().map(|s| s.id.to_lowercase())
                                                        })
                                                        .unwrap_or_else(|| node.id.clone());

                                                    // Create a minimal family for this article
                                                    let minimal_family = ProductFamily {
                                                        id: node.id.clone(),
                                                        name: node.name.clone(),
                                                        description: node.name.clone(),
                                                        long_description: String::new(),
                                                        series: series.clone(),
                                                        base_article_nr: node.id.clone(),
                                                        prop_classes: vec![],
                                                        variant_count: 1,
                                                        is_configurable: false,
                                                        article_nrs: vec![node.id.clone()],
                                                        article_descriptions: vec![node.name.clone()],
                                                        article_long_descriptions: vec![],
                                                    };

                                                    // Try to load properties using the series
                                                    let properties = engine.get_family_properties(&manufacturer.id, &series);
                                                    app.family_properties = properties.clone();
                                                    app.selected_family = Some(minimal_family.clone());

                                                    // Create configuration
                                                    let config = FamilyConfiguration::new(&minimal_family.id, &properties);
                                                    app.family_config = Some(config.clone());

                                                    // Calculate price
                                                    app.family_price = engine.calculate_family_price(
                                                        &manufacturer.id,
                                                        &minimal_family,
                                                        &config,
                                                        app.price_date,
                                                    );

                                                    app.screen = Screen::FamilyConfig;
                                                    let price_str = app.family_price.as_ref()
                                                        .map(|p| format!("{:.2} {}", p.base_price, p.currency))
                                                        .unwrap_or_else(|| "Preis n/a".to_string());
                                                    app.status_message = Some(format!(
                                                        "{} - {}",
                                                        node.name, price_str
                                                    ));
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                Some(Message::SelectCatalogNode(idx))
                            }
                            Screen::Families => {
                                // Load properties for the selected product family
                                if idx < app.families.len() {
                                    let family = &app.families[idx];

                                    if let Some(ref manufacturer) = app.selected_manufacturer {
                                        app.status_message = Some(format!("Lade {}...", family.name));
                                        let _ = terminal.draw(|f| render(f, app));

                                        // Load family properties
                                        let properties = engine.get_family_properties(&manufacturer.id, &family.id);
                                        app.family_properties = properties.clone();

                                        // Create configuration with default values
                                        let config = FamilyConfiguration::new(&family.id, &properties);
                                        app.family_config = Some(config.clone());

                                        // Calculate initial price
                                        app.family_price = engine.calculate_family_price(
                                            &manufacturer.id,
                                            family,
                                            &config,
                                            app.price_date,
                                        );

                                        let price_str = app.family_price.as_ref()
                                            .map(|p| format!("{:.2} {}", p.base_price, p.currency))
                                            .unwrap_or_else(|| "Preis n/a".to_string());

                                        app.status_message = Some(format!(
                                            "{} Eigenschaften, {}",
                                            app.family_properties.len(),
                                            price_str
                                        ));
                                    }
                                }
                                Some(Message::SelectFamily(idx))
                            }
                            Screen::FamilyConfig => {
                                // Handle Enter in FamilyConfig when no properties (select article)
                                if app.family_properties.is_empty() {
                                    // Extract data first to avoid borrow issues
                                    let article_data = app.selected_family.as_ref().and_then(|f| {
                                        let idx = app.focused_article_index;
                                        f.article_nrs.get(idx).map(|nr| {
                                            let mut selected = f.clone();
                                            selected.base_article_nr = nr.clone();
                                            (nr.clone(), selected)
                                        })
                                    });

                                    if let Some((article_nr, selected_family)) = article_data {
                                        if let Some(ref manufacturer) = app.selected_manufacturer {
                                            // Recalculate price for the selected article
                                            if let Some(ref config) = app.family_config {
                                                app.family_price = engine.calculate_family_price(
                                                    &manufacturer.id,
                                                    &selected_family,
                                                    config,
                                                    app.price_date,
                                                );
                                            }

                                            // Update the selected family to reflect the chosen article
                                            app.selected_family = Some(selected_family);

                                            let price_str = app.family_price.as_ref()
                                                .map(|p| format!("{:.2} {}", p.base_price, p.currency))
                                                .unwrap_or_else(|| "Preis n/a".to_string());

                                            app.status_message = Some(format!(
                                                "Artikel {} ausgewählt - {}",
                                                article_nr, price_str
                                            ));
                                        }
                                    }
                                }
                                None
                            }
                            Screen::Articles => {
                                // Load configuration for the selected article (legacy mode)
                                if idx < app.articles.len() {
                                    let article = &app.articles[idx];

                                    if let Some(ref manufacturer) = app.selected_manufacturer {
                                        app.status_message = Some(format!("Lade {}...", article.short_description));
                                        let _ = terminal.draw(|f| render(f, app));

                                        // Always look up base price from OCD first
                                        let pdata_files = ocd::find_pdata_files(&manufacturer.path);
                                        let mut found_price = None;

                                        for pdata_path in &pdata_files {
                                            if let Ok(reader) = ocd::OcdReader::from_ebase(pdata_path) {
                                                if let Some(ocd_price) = reader.get_base_price(&article.id) {
                                                    use rust_decimal::Decimal;
                                                    use chrono::NaiveDate;

                                                    let base_price = Decimal::from_f32_retain(ocd_price.price)
                                                        .unwrap_or(Decimal::ZERO);

                                                    let price_date = NaiveDate::parse_from_str(&ocd_price.date_from, "%Y%m%d")
                                                        .unwrap_or_else(|_| app.price_date);
                                                    let valid_to = NaiveDate::parse_from_str(&ocd_price.date_to, "%Y%m%d").ok();

                                                    found_price = Some(ofml_interpreter::oap::PriceResult::new(
                                                        base_price,
                                                        vec![],
                                                        ocd_price.currency.clone(),
                                                        app.price_date,
                                                        price_date,
                                                        valid_to,
                                                    ));
                                                    break;
                                                }
                                            }
                                        }

                                        // Create base configuration
                                        let mut config = ofml_interpreter::oap::config::Configuration::new(
                                            article.id.clone(),
                                            manufacturer.id.clone(),
                                        );
                                        config.article_number = Some(article.base_article_number.clone());
                                        config.price = found_price;

                                        // Try to load CLS properties if configurable
                                        let status_msg = if article.has_configuration {
                                            match engine.load_configuration(&manufacturer.id, &article.id) {
                                                Ok(loaded_config) => {
                                                    // Merge properties from CLS
                                                    config.properties = loaded_config.properties;
                                                    let prop_count = config.properties.definitions.len();

                                                    let price_str = config.price.as_ref()
                                                        .map(|p| format!("{:.2} {}", p.base_price, p.currency))
                                                        .unwrap_or_else(|| "Preis n/a".to_string());

                                                    if prop_count > 0 {
                                                        format!("{} Eigenschaften, {}", prop_count, price_str)
                                                    } else {
                                                        format!("Keine Eigenschaften, {}", price_str)
                                                    }
                                                }
                                                Err(_) => {
                                                    let price_str = config.price.as_ref()
                                                        .map(|p| format!("{:.2} {}", p.base_price, p.currency))
                                                        .unwrap_or_else(|| "Preis n/a".to_string());
                                                    format!("CLS nicht geladen, {}", price_str)
                                                }
                                            }
                                        } else {
                                            let price_str = config.price.as_ref()
                                                .map(|p| format!("{:.2} {}", p.base_price, p.currency))
                                                .unwrap_or_else(|| "Preis nicht verfügbar".to_string());
                                            format!("Nicht konfigurierbar, {}", price_str)
                                        };

                                        app.status_message = Some(status_msg);
                                        app.configuration = Some(config);
                                    }
                                }
                                Some(Message::SelectArticle(idx))
                            }
                            Screen::Tables => {
                                // Load table contents for selected table
                                if idx < app.tables.len() {
                                    let table_info = app.tables[idx].clone();
                                    app.status_message = Some(format!("Lade Tabelle {}...", table_info.name));
                                    let _ = terminal.draw(|f| render(f, app));

                                    // Load table rows
                                    app.table_rows = load_table_rows(&table_info.source_path, &table_info.name);
                                    app.table_row_list_state.select(Some(0));
                                    app.table_scroll_x = 0;
                                    app.status_message = Some(format!("{} Zeilen geladen", app.table_rows.len()));
                                }
                                Some(Message::SelectTable(idx))
                            }
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                KeyCode::Esc => {
                    if app.search_active {
                        Some(Message::ToggleSearch)
                    } else {
                        Some(Message::GoBack)
                    }
                }
                KeyCode::Char(c) if app.search_active => {
                    let mut query = app.search_query.clone();
                    query.push(c);
                    Some(Message::UpdateSearch(query))
                }
                KeyCode::Backspace if app.search_active => {
                    let mut query = app.search_query.clone();
                    query.pop();
                    Some(Message::UpdateSearch(query))
                }
                _ => None,
            };

            if let Some(m) = msg {
                app.update(m.clone());

                // Handle price recalculation on property change
                if matches!(m, Message::CyclePropertyOption(_)) {
                    if let (Some(ref manufacturer), Some(ref family), Some(ref config)) =
                        (&app.selected_manufacturer, &app.selected_family, &app.family_config)
                    {
                        app.family_price = engine.calculate_family_price(
                            &manufacturer.id,
                            family,
                            config,
                            app.price_date,
                        );
                    }
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

/// Load all tables from pdata.ebase files for a manufacturer
#[cfg(feature = "tui")]
fn load_manufacturer_tables(manufacturer_path: &std::path::Path) -> Vec<ofml_interpreter::tui::app::TableInfo> {
    use ofml_interpreter::ebase::EBaseReader;
    use ofml_interpreter::tui::app::TableInfo;

    let mut all_tables: std::collections::HashMap<String, TableInfo> = std::collections::HashMap::new();

    // Standard OCD tables to identify
    let standard_tables: std::collections::HashSet<&str> = [
        "ocd_article", "ocd_articletext", "ocd_artshorttext", "ocd_artlongtext",
        "ocd_price", "ocd_pricetext", "ocd_property", "ocd_propertyclass",
        "ocd_propertyvalue", "ocd_propertyvaluetext", "ocd_propvaluetext",
        "ocd_variantcondition", "ocd_relation", "ocd_relationobj",
        "ocd_propertygroup", "ocd_article2propgroup", "ocd_composite", "ocd_billofitems",
        "propvalue2varcond",
    ].into_iter().collect();

    // Find all pdata.ebase files
    fn find_ebase_files(path: &std::path::Path, files: &mut Vec<std::path::PathBuf>) {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    find_ebase_files(&p, files);
                } else if p.file_name().map_or(false, |n| n == "pdata.ebase") {
                    files.push(p);
                }
            }
        }
    }

    let mut ebase_files = Vec::new();
    find_ebase_files(manufacturer_path, &mut ebase_files);

    for ebase_path in ebase_files {
        if let Ok(reader) = EBaseReader::open(&ebase_path) {
            for table_name in reader.table_names() {
                // Skip if we already have this table from a previous file
                if all_tables.contains_key(table_name) {
                    continue;
                }

                let is_standard = standard_tables.contains(table_name);
                let row_count = reader.get_table(table_name).map(|t| t.record_count as usize).unwrap_or(0);
                let columns: Vec<String> = reader.get_table(table_name)
                    .map(|t| t.columns.iter().map(|c| c.name.clone()).collect())
                    .unwrap_or_default();

                all_tables.insert(table_name.to_string(), TableInfo {
                    name: table_name.to_string(),
                    row_count,
                    columns,
                    is_standard,
                    source_path: ebase_path.to_string_lossy().to_string(),
                });
            }
        }
    }

    // Sort: custom tables first, then standard tables
    let mut result: Vec<TableInfo> = all_tables.into_values().collect();
    result.sort_by(|a, b| {
        match (a.is_standard, b.is_standard) {
            (false, true) => std::cmp::Ordering::Less,
            (true, false) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        }
    });

    result
}

/// Load rows from a specific table
#[cfg(feature = "tui")]
fn load_table_rows(ebase_path: &str, table_name: &str) -> Vec<ofml_interpreter::tui::app::TableRow> {
    use ofml_interpreter::ebase::EBaseReader;
    use ofml_interpreter::tui::app::TableRow;

    let mut rows = Vec::new();
    let path = std::path::Path::new(ebase_path);

    if let Ok(mut reader) = EBaseReader::open(path) {
        // Get column order from table definition first
        let columns: Vec<String> = reader.get_table(table_name)
            .map(|t| t.columns.iter().map(|c| c.name.clone()).collect())
            .unwrap_or_default();

        // Read records with limit
        if let Ok(records) = reader.read_records(table_name, Some(500)) {
            for record in records.iter() {
                let values: Vec<String> = columns.iter()
                    .map(|col| {
                        record.get(col.as_str())
                            .map(|v| value_to_string(v))
                            .unwrap_or_default()
                    })
                    .collect();
                rows.push(TableRow { values });
            }
        }
    }

    rows
}

/// Convert an EBase Value to a string
#[cfg(feature = "tui")]
fn value_to_string(v: &ofml_interpreter::ebase::Value) -> String {
    use ofml_interpreter::ebase::Value;
    match v {
        Value::Int(i) => i.to_string(),
        Value::UInt(u) => u.to_string(),
        Value::Float(f) => format!("{:.4}", f),
        Value::String(s) => s.clone(),
        Value::Blob(id) => format!("[blob:{}]", id),
        Value::Null => String::new(),
    }
}

// ============================================================================
// Existing Commands (unchanged functionality)
// ============================================================================

fn cmd_tokenize(source: &str, file_path: &str) -> CmdResult {
    let tokens = tokenize(source).map_err(|e| e.to_string())?;

    println!("Tokens from {}:", file_path);
    println!("{:-<60}", "");
    for (i, t) in tokens.iter().enumerate() {
        let preview = &source[t.span.clone()];
        let preview = if preview.len() > 30 {
            format!("{}...", &preview[..27])
        } else {
            preview.to_string()
        };
        println!("{:4}: {:?} @ {:?} = {:?}", i, t.token, t.span, preview);
    }
    println!("{:-<60}", "");
    println!("Total: {} tokens", tokens.len());
    Ok(())
}

fn cmd_parse(source: &str, file_path: &str) -> CmdResult {
    let unit = OfmlParser::new(source)
        .and_then(|mut p| p.parse())
        .map_err(|e| e.to_string())?;

    println!("AST from {}:", file_path);
    println!("{:-<60}", "");

    if let Some(ref pkg) = unit.package {
        println!("Package: {}", pkg);
    }

    if !unit.imports.is_empty() {
        println!("\nImports:");
        for import in &unit.imports {
            let wildcard = if import.wildcard { "::*" } else { "" };
            println!("  {}{}", import.path, wildcard);
        }
    }

    println!("\nStatements: {}", unit.statements.len());
    for (i, stmt) in unit.statements.iter().enumerate() {
        print_stmt(stmt, i, 1);
    }

    println!("{:-<60}", "");
    println!("Parsing successful!");
    Ok(())
}

fn cmd_check(source: &str, file_path: &str) -> CmdResult {
    let unit = OfmlParser::new(source)
        .and_then(|mut p| p.parse())
        .map_err(|e| format!("{}: FAILED\n  Error: {}", file_path, e))?;

    let (class_count, func_count, var_count) = count_elements(&unit);

    println!("{}: OK", file_path);
    println!(
        "  Package: {}",
        unit.package
            .map(|p| p.to_string())
            .unwrap_or_else(|| "(none)".to_string())
    );
    println!("  Imports: {}", unit.imports.len());
    println!("  Classes: {}", class_count);
    println!("  Functions: {}", func_count);
    println!("  Variables: {}", var_count);
    Ok(())
}

fn cmd_run(source: &str, file_path: &str) -> CmdResult {
    let unit = OfmlParser::new(source)
        .and_then(|mut p| p.parse())
        .map_err(|e| e.to_string())?;

    let mut interp = Interpreter::new();

    println!("Executing {}...", file_path);
    println!("{:-<60}", "");

    interp.execute(&unit).map_err(|e| e.to_string())?;

    println!("{:-<60}", "");
    println!("Execution complete!");

    if !interp.output.is_empty() {
        println!("\nOutput:");
        for line in &interp.output {
            println!("  {}", line);
        }
    }

    if !interp.classes.is_empty() {
        println!("\nRegistered classes:");
        for name in interp.classes.keys() {
            println!("  {}", name);
        }
    }
    Ok(())
}

fn cmd_export(source: &str, file_path: &str) -> CmdResult {
    let unit = OfmlParser::new(source)
        .and_then(|mut p| p.parse())
        .map_err(|e| e.to_string())?;

    let mut interp = Interpreter::new();

    println!("Executing {}...", file_path);
    interp.execute(&unit).map_err(|e| e.to_string())?;
    println!("Execution complete!");

    if !interp.output.is_empty() {
        println!("\nOutput:");
        for line in &interp.output {
            println!("  {}", line);
        }
    }

    interp.scene.debug_print();

    if interp.scene.mesh_count() > 0 {
        let scene3ds = interp.scene.to_scene();
        let glb = export_to_glb(&scene3ds).map_err(|e| e.to_string())?;

        let output_path = Path::new(file_path).with_extension("glb");
        fs::write(&output_path, &glb).map_err(|e| e.to_string())?;
        println!(
            "\nWritten: {} ({} bytes, {} meshes)",
            output_path.display(),
            glb.len(),
            scene3ds.meshes.len()
        );
    } else {
        println!("\nNo geometry in scene graph");
    }
    Ok(())
}

fn cmd_convert(file_path: &str) -> CmdResult {
    let path = Path::new(file_path);
    let scene = load_geometry_file(path).map_err(|e| e.to_string())?;

    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_uppercase();

    println!("Parsed {} file: {}", ext, file_path);
    println!("  Meshes: {}", scene.meshes.len());
    println!("  Materials: {}", scene.materials.len());

    for mesh in &scene.meshes {
        println!(
            "  - {} ({} vertices, {} faces)",
            mesh.name,
            mesh.vertices.len(),
            mesh.faces.len()
        );
    }

    let glb = export_to_glb(&scene).map_err(|e| e.to_string())?;
    let output_path = path.with_extension("glb");
    fs::write(&output_path, &glb).map_err(|e| e.to_string())?;
    println!("\nWritten: {} ({} bytes)", output_path.display(), glb.len());
    Ok(())
}

fn cmd_merge(output_path: &str, input_files: &[&str]) -> CmdResult {
    if input_files.is_empty() {
        return Err("Usage: merge <output.glb> <file1.geo> [file2.geo] ...".to_string());
    }

    let paths: Vec<&Path> = input_files.iter().map(|s| Path::new(*s)).collect();
    let scene = operations::load_and_merge_geometry(&paths).map_err(|e| e.to_string())?;

    println!(
        "Combined scene: {} meshes, {} materials",
        scene.meshes.len(),
        scene.materials.len()
    );

    let glb = export_to_glb(&scene).map_err(|e| e.to_string())?;
    fs::write(output_path, &glb).map_err(|e| e.to_string())?;
    println!("Written: {} ({} bytes)", output_path, glb.len());
    Ok(())
}

fn cmd_validate(file_path: &str) -> CmdResult {
    let path = Path::new(file_path);
    let scene = load_geometry_file(path).map_err(|e| e.to_string())?;

    let result = validate_geometry(&scene);
    let bbox = &result.bounding_box;
    let dims = bbox.dimensions();
    let center = bbox.center();

    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_uppercase();

    println!("Geometry Validation: {}", file_path);
    println!("{}", "=".repeat(60));
    println!();
    println!("Format: .{}", ext);
    println!("Meshes: {}", result.mesh_count);
    println!("Total Vertices: {}", result.vertex_count);
    println!("Total Faces: {}", result.face_count);
    println!("Materials: {}", result.material_count);
    println!();
    println!("Bounding Box:");
    println!(
        "  Min: [{:.4}, {:.4}, {:.4}] m",
        bbox.min[0], bbox.min[1], bbox.min[2]
    );
    println!(
        "  Max: [{:.4}, {:.4}, {:.4}] m",
        bbox.max[0], bbox.max[1], bbox.max[2]
    );
    println!();
    println!("Dimensions (W x H x D):");
    println!(
        "  {:.1} x {:.1} x {:.1} mm",
        dims[0] * 1000.0,
        dims[1] * 1000.0,
        dims[2] * 1000.0
    );
    println!(
        "Center: [{:.4}, {:.4}, {:.4}] m",
        center[0], center[1], center[2]
    );
    println!();

    if !result.warnings.is_empty() {
        println!("Warnings:");
        for w in &result.warnings {
            println!("  ⚠ {}", w);
        }
        println!();
    }

    if !result.errors.is_empty() {
        println!("Errors:");
        for e in &result.errors {
            println!("  ✗ {}", e);
        }
        return Err("INVALID".to_string());
    }

    println!("✓ VALID");
    Ok(())
}

fn cmd_product(product_path: &str, article: Option<&str>, output: Option<&str>) -> CmdResult {
    let path = Path::new(product_path);

    if article.is_none() && output.is_none() {
        return list_product_articles(path);
    }

    let config = ProductConfig {
        article: article.map(|s| s.to_string()),
        ..Default::default()
    };

    println!("Loading product from: {}", product_path);

    let result = operations::assemble_product(path, &config).map_err(|e| e.to_string())?;

    if let Some(art) = article {
        println!(
            "Article: {} ({} geometry refs)",
            art, result.geometry_loaded
        );
    } else {
        println!("All articles: {} geometry refs", result.geometry_loaded);
    }

    if !result.geometry_missing.is_empty() && result.geometry_missing.len() <= 5 {
        for name in &result.geometry_missing {
            eprintln!("  Warning: geometry not found: {}", name);
        }
    }

    println!("\nCombined scene: {} meshes", result.scene.meshes.len());

    let glb = export_to_glb(&result.scene).map_err(|e| e.to_string())?;

    let output_path = output.map(|s| s.to_string()).unwrap_or_else(|| {
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("product");
        format!("{}.glb", name)
    });

    fs::write(&output_path, &glb).map_err(|e| e.to_string())?;
    println!("Written: {} ({} bytes)", output_path, glb.len());
    Ok(())
}

fn list_product_articles(path: &Path) -> CmdResult {
    use ofml_interpreter::ebase::{EBaseReader, Odb3dRecord};

    let odb_path = find_odb_path(path)?;

    let mut reader = EBaseReader::open(&odb_path).map_err(|e| e.to_string())?;
    let records = reader
        .read_records("odb3d", None)
        .map_err(|e| e.to_string())?;

    let mut articles: std::collections::HashSet<String> = std::collections::HashSet::new();
    for record in &records {
        if let Some(odb_rec) = Odb3dRecord::from_record(record) {
            if !odb_rec.odb_name.is_empty() {
                articles.insert(odb_rec.odb_name.clone());
            }
        }
    }

    println!("Found {} odb3d records", records.len());
    if !articles.is_empty() {
        println!("\nAvailable articles ({}):", articles.len());
        let mut sorted: Vec<_> = articles.iter().collect();
        sorted.sort();
        for art in sorted.iter().take(30) {
            println!("  {}", art);
        }
        if sorted.len() > 30 {
            println!("  ... and {} more", sorted.len() - 30);
        }
        println!("\nUsage: product {} <article> [output.glb]", path.display());
    }
    Ok(())
}

fn find_odb_path(path: &Path) -> Result<std::path::PathBuf, String> {
    let direct = path.join("odb.ebase");
    if direct.exists() {
        return Ok(direct);
    }

    for version in &["1", "2", "3", "current"] {
        let odb = path.join(version).join("odb.ebase");
        if odb.exists() {
            return Ok(odb);
        }
    }

    Err(format!("odb.ebase not found in {}", path.display()))
}

fn cmd_ebase(ebase_path: &str, table_name: Option<&str>) -> CmdResult {
    use ofml_interpreter::ebase::EBaseReader;

    let mut reader = EBaseReader::open(ebase_path).map_err(|e| e.to_string())?;

    println!("EBase Database: {}", ebase_path);
    println!("Version: {}.{}", reader.major_version, reader.minor_version);
    println!("Tables: {}", reader.tables.len());
    println!();

    for (name, table) in &reader.tables {
        println!("  Table: {}", name);
        println!("    Records: {}", table.record_count);
        println!("    Record Size: {} bytes", table.record_size);
        println!("    Columns ({}):", table.columns.len());
        for col in &table.columns {
            let type_name = match col.type_id {
                1 => "int8",
                2 => "uint8",
                3 => "int16",
                4 => "uint16",
                5 => "int32",
                6 => "uint32",
                7 => "float32",
                8 => "float64",
                9 => "string",
                10 => "string_ref",
                11 => "blob_ref",
                _ => "unknown",
            };
            println!(
                "      - {}: {} (offset={})",
                col.name, type_name, col.offset
            );
        }
        println!();
    }

    if let Some(tbl) = table_name {
        println!("=== Records from {} (limit 20) ===\n", tbl);
        let records = reader
            .read_records(tbl, Some(20))
            .map_err(|e| e.to_string())?;

        for (i, record) in records.iter().enumerate() {
            println!("Record {}:", i);
            for (key, value) in record {
                let val_str = format_ebase_value(value);
                println!("  {}: {}", key, val_str);
            }
            println!();
        }
    }
    Ok(())
}

fn format_ebase_value(value: &ofml_interpreter::ebase::Value) -> String {
    use ofml_interpreter::ebase::Value;
    match value {
        Value::Int(v) => format!("{}", v),
        Value::UInt(v) => format!("{}", v),
        Value::Float(v) => format!("{}", v),
        Value::String(v) => format!("'{}'", v),
        Value::Blob(v) => format!("<blob:{}>", v),
        Value::Null => "NULL".to_string(),
    }
}

fn cmd_ofml(data_path: &str, manufacturer: Option<&str>, product: Option<&str>) -> CmdResult {
    use ofml_interpreter::ofml::OFMLDataReader;

    let reader = OFMLDataReader::new(data_path);

    match (manufacturer, product) {
        (None, _) => {
            let summary = reader.get_summary();
            println!("OFML Data Directory: {}", data_path);
            println!("{}", "=".repeat(60));
            println!();
            println!("Manufacturers: {}", summary.manufacturers.len());
            println!("Total Products: {}", summary.total_products);
            println!("Total ALB Files: {}", summary.total_alb_files);
            println!("Total GEO Files: {}", summary.total_geo_files);
            println!();

            for mfr in &summary.manufacturers {
                let count = summary.product_counts.get(mfr).unwrap_or(&0);
                println!("  {}: {} products", mfr, count);
            }
        }
        (Some(mfr), None) => {
            let products = reader.discover_products(mfr);
            println!("Manufacturer: {}", mfr);
            println!("Products: {}", products.len());
            println!();

            for prod in &products {
                if let Ok(p) = reader.load_product(mfr, prod) {
                    let has_odb = if p.has_odb() { " [ODB]" } else { "" };
                    println!(
                        "  {}: {} ALB, {} GEO, {} MAT{}",
                        prod,
                        p.album_files.len(),
                        p.geo_files.len(),
                        p.mat_files.len(),
                        has_odb
                    );
                }
            }
        }
        (Some(mfr), Some(prod)) => {
            let p = reader.load_product(mfr, prod).map_err(|e| e.to_string())?;
            println!("Product: {}/{}", mfr, prod);
            println!("Path: {}", p.base_path.display());
            println!();
            println!("ALB Files: {}", p.album_files.len());
            println!("GEO Files: {}", p.geo_files.len());
            println!("MAT Files: {}", p.mat_files.len());
            println!("Countries: {:?}", p.get_countries());
        }
    }
    Ok(())
}

fn cmd_cls(alb_path: &str, filename: Option<&str>) -> CmdResult {
    use ofml_interpreter::ofml::AlbArchive;

    let mut archive = AlbArchive::open(alb_path).map_err(|e| e.to_string())?;
    let cls_files = archive.get_cls_files();

    match filename {
        None => {
            println!("ALB Archive: {}", alb_path);
            println!("CLS Files: {}", cls_files.len());
            println!();

            for name in &cls_files {
                let short = Path::new(name)
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| name.clone());
                println!("  {}", short);
            }
        }
        Some(target) => {
            let matching: Vec<_> = cls_files
                .iter()
                .filter(|n| n.to_lowercase().contains(&target.to_lowercase()))
                .collect();

            if matching.is_empty() {
                return Err(format!("No CLS file matching '{}' found", target));
            }

            for name in matching {
                let content = archive.extract_cls(name).map_err(|e| e.to_string())?;
                println!("// === {} ===", name);
                println!("{}", content);
            }
        }
    }
    Ok(())
}

fn cmd_alb(alb_path: &str, pattern: Option<&str>, output_dir: Option<&str>) -> CmdResult {
    use ofml_interpreter::ofml::AlbArchive;

    let mut archive = AlbArchive::open(alb_path).map_err(|e| e.to_string())?;
    let all_files = archive.list_files();

    match pattern {
        None => {
            println!("ALB Archive: {}", alb_path);
            println!("Total files: {}", all_files.len());
            println!();

            let mut by_ext: std::collections::HashMap<String, Vec<&str>> =
                std::collections::HashMap::new();
            for name in &all_files {
                let ext = Path::new(name)
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("(none)")
                    .to_lowercase();
                by_ext.entry(ext).or_default().push(name);
            }

            let mut exts: Vec<_> = by_ext.keys().collect();
            exts.sort();

            for ext in exts {
                println!(".{}: {} files", ext, by_ext[ext].len());
            }

            println!();
            println!("Use: alb <file.alb> <pattern> [outdir] to extract files");
        }
        Some(pat) => {
            let output_path = Path::new(output_dir.unwrap_or("."));
            if !output_path.exists() {
                fs::create_dir_all(output_path).map_err(|e| e.to_string())?;
            }

            let matching: Vec<_> = if pat == "*" {
                all_files.iter().collect()
            } else {
                all_files
                    .iter()
                    .filter(|n| n.to_lowercase().contains(&pat.to_lowercase()))
                    .collect()
            };

            println!(
                "Extracting {} files to {}",
                matching.len(),
                output_path.display()
            );

            let mut extracted = 0;
            for name in matching {
                if let Ok(data) = archive.extract(name) {
                    let filename = Path::new(name)
                        .file_name()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| name.replace('/', "_"));

                    let out_file = output_path.join(&filename);
                    if fs::write(&out_file, &data).is_ok() {
                        println!("  {} ({} bytes)", filename, data.len());
                        extracted += 1;
                    }
                }
            }
            println!("\nExtracted {} files", extracted);
        }
    }
    Ok(())
}

fn cmd_build(alb_path: &str, class_name: Option<&str>) -> CmdResult {
    use std::io::Read;

    let password = b"Gur#Ynzo$Yvrf%Qbja&Ba*Oebnqjnl.";
    let alb = Path::new(alb_path);

    println!("Building product from: {}", alb_path);
    if let Some(name) = class_name {
        println!("Target class: {}", name);
    }
    println!("{:-<60}", "");

    let file = fs::File::open(alb_path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;

    let mut interp = Interpreter::new();
    interp.set_alb_path(alb.to_path_buf());

    let mut cls_sources: Vec<(String, String)> = Vec::new();
    for i in 0..archive.len() {
        let mut file = match archive.by_index_decrypt(i, password) {
            Ok(f) => f,
            Err(_) => continue,
        };

        let name = file.name().to_string();
        if !name.to_lowercase().ends_with(".cls") {
            continue;
        }

        let mut source = String::new();
        if file.read_to_string(&mut source).is_ok() {
            cls_sources.push((name, source));
        }
    }

    println!("Found {} CLS files", cls_sources.len());

    for (name, source) in &cls_sources {
        match OfmlParser::new(source).and_then(|mut p| p.parse()) {
            Ok(unit) => {
                if let Err(e) = interp.execute(&unit) {
                    eprintln!("  {} - runtime error: {}", name, e);
                }
            }
            Err(e) => {
                eprintln!("  {} - parse error: {}", name, e);
            }
        }
    }

    println!("Registered {} classes", interp.classes.len());

    match class_name {
        Some(target_class) => {
            if let Some(class) = interp.classes.get(target_class).cloned() {
                println!("Instantiating: {}", target_class);
                match interp.instantiate_class_public(class) {
                    Ok(_) => println!("  Instance created successfully"),
                    Err(e) => eprintln!("  Error instantiating: {}", e),
                }
            } else {
                println!("Available classes:");
                for name in interp.classes.keys() {
                    println!("  - {}", name);
                }
                return Err(format!("Class '{}' not found", target_class));
            }
        }
        None => {
            println!("Available classes:");
            for name in interp.classes.keys() {
                println!("  - {}", name);
            }
            println!("\nSpecify a class name to instantiate");
            return Ok(());
        }
    }

    interp.scene.debug_print();

    if interp.scene.mesh_count() > 0 {
        let scene3ds = interp.scene.to_scene();
        let glb = export_to_glb(&scene3ds).map_err(|e| e.to_string())?;

        let output_name = format!(
            "{}_scene.glb",
            class_name.unwrap_or("product").to_lowercase()
        );
        let output_path = alb.parent().unwrap_or(Path::new(".")).join(&output_name);

        fs::write(&output_path, &glb).map_err(|e| e.to_string())?;
        println!(
            "\nWritten: {} ({} bytes, {} meshes)",
            output_path.display(),
            glb.len(),
            scene3ds.meshes.len()
        );
    } else {
        println!("\nNo geometry in scene graph");
    }
    Ok(())
}

fn cmd_gsx(product_path: &str, output: Option<&str>) -> CmdResult {
    use ofml_interpreter::ofml::AlbArchive;
    use std::path::PathBuf;

    let product_dir = Path::new(product_path);

    let alb_path = if product_dir.is_file() && product_path.ends_with(".alb") {
        product_dir.to_path_buf()
    } else {
        find_alb_in_versions(product_dir)?
    };

    println!("SEDUS (gsx) Product Conversion");
    println!("ALB: {}", alb_path.display());

    let mut archive = AlbArchive::open(&alb_path).map_err(|e| e.to_string())?;
    let obj_files = archive.get_obj_files();
    println!("Found {} OBJ files in ALB", obj_files.len());

    if obj_files.is_empty() {
        return Err("No OBJ files found in ALB".to_string());
    }

    let mut combined_scene = geometry::Scene3DS::default();
    let mut loaded_count = 0;

    for obj_name in &obj_files {
        if let Ok(mut scene) = archive.extract_obj(obj_name) {
            let stem = Path::new(obj_name)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("mesh");
            for (i, mesh) in scene.meshes.iter_mut().enumerate() {
                mesh.name = format!("{}_{}", stem, i);
            }
            combined_scene.meshes.extend(scene.meshes);
            combined_scene.materials.extend(scene.materials);
            loaded_count += 1;
        }
    }

    println!("Loaded {} of {} OBJ files", loaded_count, obj_files.len());

    if combined_scene.meshes.is_empty() {
        return Err("No valid geometry loaded".to_string());
    }

    let output_path = match output {
        Some(p) => PathBuf::from(p),
        None => {
            let stem = alb_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("gsx_product");
            PathBuf::from(format!("/tmp/{}.glb", stem))
        }
    };

    let glb_data = export_to_glb(&combined_scene).map_err(|e| e.to_string())?;
    fs::write(&output_path, &glb_data).map_err(|e| e.to_string())?;

    println!();
    println!(
        "Output: {} ({} bytes)",
        output_path.display(),
        glb_data.len()
    );
    println!("Meshes: {}", combined_scene.meshes.len());
    Ok(())
}

fn find_alb_in_versions(product_dir: &Path) -> Result<std::path::PathBuf, String> {
    for version in &["1", "2", "3"] {
        let version_dir = product_dir.join(version);
        if version_dir.exists() {
            if let Ok(entries) = fs::read_dir(&version_dir) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path.extension().map(|s| s == "alb").unwrap_or(false) {
                        return Ok(path);
                    }
                }
            }
        }
    }
    Err("No ALB file found in product directory".to_string())
}

fn cmd_svg(ebase_path: &str, output: Option<&str>) -> CmdResult {
    println!("Reading 2D records from: {}", ebase_path);

    let compound =
        operations::export_2d_floorplan(Path::new(ebase_path)).map_err(|e| e.to_string())?;

    let svg = compound.to_svg();

    let output_path = output.map(|s| s.to_string()).unwrap_or_else(|| {
        Path::new(ebase_path)
            .with_extension("svg")
            .to_string_lossy()
            .to_string()
    });

    fs::write(&output_path, &svg).map_err(|e| e.to_string())?;
    println!("Written: {} ({} bytes)", output_path, svg.len());
    Ok(())
}

fn cmd_expr(expression: &str) -> CmdResult {
    use std::collections::HashMap;

    println!("Evaluating: {}", expression);
    println!("{:-<60}", "");

    let props: HashMap<String, f64> = HashMap::new();
    let result = operations::evaluate_expression(expression, &props)?;

    println!("Result: {:?}", result);
    Ok(())
}

fn cmd_manufacturer(
    data_dir: &str,
    manufacturer: Option<&str>,
    class_name: Option<&str>,
) -> CmdResult {
    let data_path = Path::new(data_dir);

    match manufacturer {
        None => {
            println!("OFML Data Directory: {}", data_dir);
            println!("{}", "=".repeat(60));
            println!();

            let mut manufacturers = Vec::new();
            if let Ok(entries) = fs::read_dir(data_path) {
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            if !name.starts_with('.')
                                && !name.starts_with("pCon")
                                && !name.contains("plugin")
                                && !name.contains("setup")
                                && (path.join("basics").exists() || path.join("global").exists())
                            {
                                manufacturers.push(name.to_string());
                            }
                        }
                    }
                }
            }

            manufacturers.sort();
            println!("Available manufacturers ({}):", manufacturers.len());
            for mfr in &manufacturers {
                let mut alb_count = 0;
                let mut cls_count = 0;
                let mfr_path = data_path.join(mfr);
                if let Ok(entries) = fs::read_dir(&mfr_path) {
                    for entry in entries.filter_map(|e| e.ok()) {
                        if entry.path().is_dir() {
                            alb_count += 1;
                        }
                    }
                }

                let mut loader = AlbLoader::new(data_path);
                if loader.load_manufacturer(mfr).is_ok() {
                    cls_count = loader.stats().classes;
                }

                if cls_count > 0 {
                    println!("  {}: {} packages, {} classes", mfr, alb_count, cls_count);
                } else {
                    println!("  {}: {} packages", mfr, alb_count);
                }
            }

            println!();
            println!("Usage: manufacturer <data_dir> <manufacturer> [class]");
        }
        Some(mfr) => {
            println!("Loading manufacturer: {}", mfr);
            println!("{}", "=".repeat(60));

            let loader =
                load_manufacturer_with_deps(data_path, mfr, None).map_err(|e| e.to_string())?;

            let stats = loader.stats();
            println!();
            println!("Loaded:");
            println!("  ALBs: {}", stats.albs);
            println!("  Packages: {}", stats.packages);
            println!("  CLS Files: {}", stats.files);
            println!("  Classes: {}", stats.classes);
            println!();

            println!("Packages:");
            let mut packages: Vec<_> = loader.sources.keys().collect();
            packages.sort();
            for pkg in packages.iter().take(20) {
                let cls_count = loader.sources.get(*pkg).map(|v| v.len()).unwrap_or(0);
                println!("  {} ({} files)", pkg, cls_count);
            }
            if packages.len() > 20 {
                println!("  ... and {} more", packages.len() - 20);
            }
            println!();

            let mut interp = Interpreter::new();
            match loader.load_into_interpreter(&mut interp) {
                Ok(loaded) => {
                    println!("Successfully parsed {} CLS files", loaded);
                    println!("Registered {} classes in interpreter", interp.classes.len());
                }
                Err(e) => {
                    eprintln!("Warning: Some files failed to load: {}", e);
                }
            }

            match class_name {
                None => {
                    println!();
                    println!("Available classes ({})", interp.classes.len());
                    let mut names: Vec<_> = interp.classes.keys().collect();
                    names.sort();
                    for name in names.iter().take(50) {
                        println!("  {}", name);
                    }
                    if names.len() > 50 {
                        println!("  ... and {} more", names.len() - 50);
                    }
                    println!();
                    println!("Usage: manufacturer {} {} <class_name>", data_dir, mfr);
                }
                Some(target_class) => {
                    println!();
                    println!("Instantiating: {}", target_class);

                    if let Some(class) = interp.classes.get(target_class).cloned() {
                        match interp.instantiate_class_public(class) {
                            Ok(instance) => {
                                println!("Instance created successfully!");

                                if let ofml_interpreter::Value::Object(obj_ref) = &instance {
                                    let obj = obj_ref.borrow();
                                    println!("  Class: {}", obj.class.name);
                                    println!("  Properties: {}", obj.properties.len());
                                    for (key, _) in obj.properties.iter().take(10) {
                                        println!("    - {}", key);
                                    }
                                }

                                if interp.scene.mesh_count() > 0 {
                                    let scene3ds = interp.scene.to_scene();
                                    let glb =
                                        export_to_glb(&scene3ds).map_err(|e| e.to_string())?;

                                    let output_path =
                                        format!("/tmp/{}_{}.glb", mfr, target_class.to_lowercase());
                                    fs::write(&output_path, &glb).map_err(|e| e.to_string())?;
                                    println!();
                                    println!(
                                        "Geometry exported: {} ({} bytes, {} meshes)",
                                        output_path,
                                        glb.len(),
                                        scene3ds.meshes.len()
                                    );
                                }
                            }
                            Err(e) => {
                                eprintln!("Error instantiating class: {}", e);
                            }
                        }
                    } else {
                        let target_lower = target_class.to_lowercase();
                        let similar: Vec<_> = interp
                            .classes
                            .keys()
                            .filter(|n| n.to_lowercase().contains(&target_lower))
                            .take(10)
                            .collect();

                        if !similar.is_empty() {
                            println!("Class '{}' not found. Similar classes:", target_class);
                            for name in similar {
                                println!("  {}", name);
                            }
                        } else {
                            return Err(format!("Class '{}' not found", target_class));
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn cmd_extract(alb_path: &str, pattern: &str) -> CmdResult {
    use std::io::Read;

    let password = b"Gur#Ynzo$Yvrf%Qbja&Ba*Oebnqjnl.";
    let file = fs::File::open(alb_path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;

    println!("ALB archive: {}", alb_path);
    println!(
        "Pattern: {}",
        if pattern.is_empty() { "*" } else { pattern }
    );

    let mut converted = 0;
    let output_dir = Path::new(alb_path).parent().unwrap_or(Path::new("."));

    for i in 0..archive.len() {
        let mut file = match archive.by_index_decrypt(i, password) {
            Ok(f) => f,
            Err(_) => continue,
        };

        let name = file.name().to_string();
        if !name.to_lowercase().ends_with(".3ds") {
            continue;
        }
        if !pattern.is_empty() && !name.to_lowercase().contains(&pattern.to_lowercase()) {
            continue;
        }

        let mut data = Vec::new();
        if file.read_to_end(&mut data).is_err() {
            continue;
        }

        let scene = match geometry::parse_3ds(&data) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("  {} - parse error: {}", name, e);
                continue;
            }
        };

        let glb = match export_to_glb(&scene) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("  {} - convert error: {}", name, e);
                continue;
            }
        };

        let base_name = Path::new(&name)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");
        let output_path = output_dir.join(format!("{}.glb", base_name));

        if fs::write(&output_path, &glb).is_ok() {
            let verts: usize = scene.meshes.iter().map(|m| m.vertices.len()).sum();
            let faces: usize = scene.meshes.iter().map(|m| m.faces.len()).sum();
            println!(
                "  {} -> {} ({} verts, {} faces)",
                name,
                output_path.display(),
                verts,
                faces
            );
            converted += 1;
        }
    }

    println!("\nConverted {} files", converted);
    Ok(())
}

// ============================================================================
// Helper Functions
// ============================================================================

fn print_stmt(stmt: &ofml_interpreter::ast::Stmt, idx: usize, indent: usize) {
    let prefix = "  ".repeat(indent);
    use ofml_interpreter::ast::Stmt;

    match stmt {
        Stmt::Class(c) => {
            let parent = c
                .parent
                .as_ref()
                .map(|p| format!(": {}", p))
                .unwrap_or_default();
            println!("{}[{}] class {}{}", prefix, idx, c.name, parent);
            println!("{}     members: {}", prefix, c.members.len());
        }
        Stmt::Func(f) => {
            println!(
                "{}[{}] func {}({})",
                prefix,
                idx,
                f.name,
                f.params.join(", ")
            );
        }
        Stmt::Var(v) => {
            let init = if v.initializer.is_some() {
                " = ..."
            } else {
                ""
            };
            println!("{}[{}] var {}{}", prefix, idx, v.name, init);
        }
        Stmt::If(_) => println!("{}[{}] if statement", prefix, idx),
        Stmt::While(_) => println!("{}[{}] while loop", prefix, idx),
        Stmt::For(_) => println!("{}[{}] for loop", prefix, idx),
        Stmt::Foreach(_) => println!("{}[{}] foreach loop", prefix, idx),
        Stmt::Return(_) => println!("{}[{}] return", prefix, idx),
        Stmt::Expr(_) => println!("{}[{}] expression", prefix, idx),
        Stmt::Block(b) => {
            println!("{}[{}] block ({} stmts)", prefix, idx, b.stmts.len());
        }
        _ => println!("{}[{}] {:?}", prefix, idx, std::mem::discriminant(stmt)),
    }
}

fn count_elements(unit: &ofml_interpreter::ast::TranslationUnit) -> (usize, usize, usize) {
    use ofml_interpreter::ast::{ClassMember, Stmt};

    let mut classes = 0;
    let mut funcs = 0;
    let mut vars = 0;

    fn count_stmt(stmt: &Stmt, classes: &mut usize, funcs: &mut usize, vars: &mut usize) {
        match stmt {
            Stmt::Class(c) => {
                *classes += 1;
                for member in &c.members {
                    match member {
                        ClassMember::Func(_) => *funcs += 1,
                        ClassMember::Var(_) => *vars += 1,
                        ClassMember::Rule(_) => *funcs += 1,
                        ClassMember::Expr(_) => {}
                    }
                }
            }
            Stmt::Func(_) => *funcs += 1,
            Stmt::Var(_) => *vars += 1,
            Stmt::Block(b) => {
                for s in &b.stmts {
                    count_stmt(s, classes, funcs, vars);
                }
            }
            _ => {}
        }
    }

    for stmt in &unit.statements {
        count_stmt(stmt, &mut classes, &mut funcs, &mut vars);
    }

    (classes, funcs, vars)
}
