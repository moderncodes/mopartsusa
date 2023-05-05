use chrono::{Utc, Datelike};
use sailfish::TemplateOnce;
use axum::response::Html;
use crate::templates::create_full_template;

#[derive(TemplateOnce)]
#[template(path = "../../www/home.html")]
struct HomePage { pub comp_age: i32, }

#[derive(TemplateOnce)]
#[template(path = "../../www/products.html")]
struct ProductPage {}

#[derive(TemplateOnce)]
#[template(path = "../../www/locations.html")]
struct LocationsPage {}

#[derive(TemplateOnce)]
#[template(path = "../../www/contact.html")]
struct ContactPage {}

enum Page { Home, Products, Locations, Contact, }

pub async fn inject_home()      -> Html<String> { render_page(Page::Home).await }
pub async fn inject_products()  -> Html<String> { render_page(Page::Products).await }
pub async fn inject_locations() -> Html<String> { render_page(Page::Locations).await }
pub async fn inject_contact()   -> Html<String> { render_page(Page::Contact).await }

async fn render_page(page: Page) -> Html<String> {
    let (title, css_file, js_file,active_link, page_source) = match page {
        Page::Home => {
            let page_data = HomePage { comp_age: Utc::now().year() - 2001, };
            ("About Us",    "home.min.css",     "",     "home",     page_data.render_once().unwrap(),)
        }

        Page::Products => {
            let page_data = ProductPage {};
            ("Products",    "products.min.css", "",     "products", page_data.render_once().unwrap(), )
        }

        Page::Locations => {
            let page_data = LocationsPage {};
            ("Locations",   "locations.min.css","Locations",     "locations",page_data.render_once().unwrap(), )
        }

        Page::Contact => {
            let page_data = ContactPage {};
            ("Contact",     "contact.min.css",  "",     "contact",  page_data.render_once().unwrap(), )
        }
    };

    let full_template = create_full_template(title, css_file, js_file, active_link, page_source);
    Html(full_template.render().unwrap())
}