mod error_pages;
mod templates;
use perseus::define_app;

define_app! {
    templates: [
        crate::templates::index::get_template::<G>()
    ],
    error_pages: crate::error_pages::get_error_pages(),
    locales: {
        default: "en-US",
        other: ["fr-FR", "es-ES"]
    },
    static_aliases: {
        "/bulma.min.css" => "assets/bulma.min.css"
         "/test.txt" => "assets/test.txt"
    }
}
