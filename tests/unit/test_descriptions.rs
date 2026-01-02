use std::path::Path;
use ofml_interpreter::oap::ocd::OcdReader;

#[test]
fn test_abalon_descriptions() {
    let path = Path::new("/workspace/ofmldata/vitra/abalon/DE/1/db/pdata.ebase");
    if !path.exists() {
        return;
    }
    
    let reader = OcdReader::from_ebase(path).expect("Should load");
    
    println!("Articles: {}", reader.articles.len());
    println!("Short texts: {}", reader.short_texts.len());
    
    // List all short_texts keys
    println!("\nShort text keys:");
    for key in reader.short_texts.keys().take(10) {
        println!("  '{}'", key);
    }
    
    for article in &reader.articles {
        let desc = reader.get_short_description(&article.short_textnr, "DE");
        println!("\nArticle {} (textnr='{}')", article.article_nr, article.short_textnr);
        println!("  Description: {:?}", desc);
    }
}
