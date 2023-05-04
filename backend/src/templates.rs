use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "../templates/_head.stpl")]
pub struct HeadTemplate {
    pub title: String,
    pub css_file: String,
    pub js_file: String,
}

#[derive(TemplateOnce)]
#[template(path = "../templates/_header.stpl")]
pub struct HeaderTemplate<'a> { pub active_link: &'a str, }

#[derive(TemplateOnce)]
#[template(path = "../templates/_main.stpl")]
pub struct MainTemplate { pub page_source: String, }

#[derive(TemplateOnce)]
#[template(path = "../templates/_footer.stpl")]
pub struct FooterTemplate<'a> { pub active_link: &'a str, }

pub struct FullTemplate<'a> {
    pub head    : HeadTemplate,
    pub header  : HeaderTemplate<'a>,
    pub main    : MainTemplate,
    pub footer  : FooterTemplate<'a>,
}

impl<'a> FullTemplate<'a> {
    pub fn render(self) -> Result<String, sailfish::RenderError> {
        let head_rendered   = self.head.render_once()?;
        let header_rendered = self.header.render_once()?;
        let main_rendered   = self.main.render_once()?;
        let footer_rendered = self.footer.render_once()?;

        Ok(format!(
            "<!DOCTYPE html>\n<html lang=\"en\">\n{}<body>\n{}\n{}\n{}</body></html>",
            head_rendered, header_rendered, main_rendered, footer_rendered
        ))
    }
}

pub fn create_full_template<'a>(
    title       : &'a str,
    css_file    : &'a str,
    js_file     : &'a str,
    active_link : &'a str,
    page_source : String,
) -> FullTemplate<'a> {
    FullTemplate {
        head: HeadTemplate {
            title: title.to_string(),
            css_file: css_file.to_string(),
            js_file: js_file.to_string(),
        },
        header: HeaderTemplate { active_link },
        main: MainTemplate { page_source },
        footer: FooterTemplate { active_link },
    }
}