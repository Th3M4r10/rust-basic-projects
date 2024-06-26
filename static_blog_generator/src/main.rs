use pulldown_cmark::{html, Parser};
use std::fs;

        
fn markdown_to_html(markdown_input: &str) -> String {
    let parser = Parser::new(markdown_input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}


fn generate_blog_post(title: &str, content: &str) -> String {
    format!("<!DOCTYPE html>
    <html lang='en'>
    <head>
        <meta charset='UTF-8'>
        <meta name='viewport' content='width=device-width, initial-scale=1.0'>
        <title>{}</title>
    </head>
    <body>
        <article>{}</article>
    </body>
    </html>", title, markdown_to_html(content))
}


fn main() {
    let markdown_content = "Your markdown content here...";
    let title = "Your Blog Post Title";
    let html_content = generate_blog_post(title, &markdown_content);
    
    // file path where you want to save the HTML file
    let file_path = r"C:\Users\Murali\OneDrive\Desktop\rust-ptojects\file.html";

    fs::write(file_path, html_content).expect("Failed to write to file");
}