use anyhow::{anyhow, Result};
use scraper::{ElementRef, Selector};

pub trait ExtendedElementRef {
    fn select_some_element<'a>(&'a self, selector_str: &str) -> Result<Option<ElementRef<'a>>>;
    fn select_first_element<'a>(&'a self, selector_str: &str) -> Result<ElementRef<'a>>;
    fn select_all_elements<'a>(&'a self, selector_str: &str) -> Result<Vec<ElementRef<'a>>>;
    fn get_attr(&self, attr: &str) -> Result<String>;
    fn get_text(&self) -> Result<String>;
}

impl<'b> ExtendedElementRef for ElementRef<'b> {
    fn select_some_element<'a>(&'a self, selector_str: &str) -> Result<Option<ElementRef<'a>>> {
        let selector = Selector::parse(selector_str)
            .map_err(|_| anyhow!("failed to parse the selector {}", selector_str))?;
        let mut select = self.select(&selector);
        Ok(select.next())
    }
    fn select_first_element<'a>(&'a self, selector_str: &str) -> Result<ElementRef<'a>> {
        self.select_some_element(selector_str)?
            .ok_or(anyhow!("there is no element selected by {}", selector_str))
    }
    fn select_all_elements<'a>(&'a self, selector_str: &str) -> Result<Vec<ElementRef<'a>>> {
        let selector = Selector::parse(selector_str)
            .map_err(|_| anyhow!("failed to parse the selector {}", selector_str))?;
        let select = self.select(&selector);
        Ok(select.collect())
    }
    fn get_attr(&self, attr: &str) -> Result<String> {
        self.value()
            .attr(attr)
            .ok_or(anyhow!("there is no attribute {}", attr))
            .map(|str| str.to_string())
    }
    fn get_text(&self) -> Result<String> {
        self.text()
            .next()
            .ok_or(anyhow!("there is no text"))
            .map(|str| str.to_string())
    }
}
