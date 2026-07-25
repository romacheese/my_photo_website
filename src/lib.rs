use wasm_bindgen::prelude::*;
use web_sys::window;

// The #[wasm_bindgen(start)] attribute tells the browser to run this function 
// automatically as soon as the WASM module is instantiated.
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // 1. Get the global window and document objects
    let window = window().expect("No global `window` exists");
    let document = window.document().expect("Should have a document on window");
    let body = document.body().expect("Document should have a body");

    // 2. Create a title for your website
    let h1 = document.create_element("h1")?;
    h1.set_inner_html("My Photography Gallery");
    body.append_child(&h1)?;

    // 3. Define the images (we will put dummy filenames here for now)
    let images = vec![
        "image1.jpg",
        "image2.jpg",
        "image3.jpg"
    ];

    // 4. Loop through the images, create an <img> tag for each, and append to the body
    for src in images {
        let img = document.create_element("img")?;
        img.set_attribute("src", src)?;
        // Set a basic width so they don't blow up the screen
        img.set_attribute("width", "400")?; 
        img.set_attribute("style", "margin: 10px; border-radius: 8px;")?;
        
        body.append_child(&img)?;
    }

    Ok(())
}
