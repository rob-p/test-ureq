use ureq;
use serde::{Deserialize};


fn main() -> Result<(), ureq::Error> {
    let body: serde_json::Value = ureq::get("https://raw.githubusercontent.com/COMBINE-lab/simpleaf/dev/resources/permit_list_info.json")
        .call()?
        .into_json()?;
    println!("body : {:?}", body);
    Ok(())
}
