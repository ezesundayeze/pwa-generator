use html5ever::tendril::StrTendril;
use html5ever::LocalName;
use html5ever::Namespace;
use html5ever::QualName;
use kuchiki::traits::*;
use kuchiki::NodeRef;
use std::fs::read_to_string;
use std::io::{Error, ErrorKind};

pub fn html_parser(path: &str, worker_script: &str) -> Result<(), Box<dyn std::error::Error>> {
    let html = read_to_string(path)?;

    let document = kuchiki::parse_html().one(html.as_str());

    // Create a new script element
    let script_tag_name = QualName::new(None, Namespace::default(), LocalName::from("script"));
    let attrs: Vec<(kuchiki::ExpandedName, kuchiki::Attribute)> = vec![(
        kuchiki::ExpandedName::new("", "src"),
        kuchiki::Attribute {
            prefix: None,
            value: StrTendril::from(worker_script).to_string(),
        },
    )]
    .into_iter()
    .collect();
    let script_tag = NodeRef::new_element(script_tag_name, attrs);

    // Select the body tag
    let body = document
        .select_first("body")
        .map_err(|_| Error::new(ErrorKind::InvalidData, "No body tag found in HTML"))?;
    body.as_node().append(script_tag);

    let mut output = Vec::new();
    document.serialize(&mut output)?;

    // Save the HTML to a file
    std::fs::write(path, output)?;

    Ok(())
}
