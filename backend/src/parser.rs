use anyhow::{anyhow, Result};
use scraper::{ElementRef, Selector};

pub fn select_some_element<'a>(
    element: &'a ElementRef,
    selector_str: &str,
) -> Result<Option<ElementRef<'a>>> {
    let selector = Selector::parse(selector_str)
        .map_err(|_| anyhow!("failed to parse the selector {}", selector_str))?;
    let mut select = element.select(&selector);
    Ok(select.next())
}

pub fn select_first_element<'a>(
    element: &'a ElementRef,
    selector_str: &str,
) -> Result<ElementRef<'a>> {
    select_some_element(element, selector_str)?
        .ok_or(anyhow!("there is no element selected by {}", selector_str))
}

pub fn select_all_elements<'a>(
    element: &'a ElementRef,
    selector_str: &str,
) -> Result<Vec<ElementRef<'a>>> {
    let selector = Selector::parse(selector_str)
        .map_err(|_| anyhow!("failed to parse the selector {}", selector_str))?;
    let select = element.select(&selector);
    Ok(select.collect())
}

pub fn get_attr(element: &ElementRef, attr: &str) -> Result<String> {
    element
        .value()
        .attr(attr)
        .ok_or(anyhow!("there is no attribute {}", attr))
        .map(|str| str.to_string())
}
