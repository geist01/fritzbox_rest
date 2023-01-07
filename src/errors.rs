use error_chain::*;

error_chain!{
    foreign_links {
        Network(std::io::Error);
        Reqwest(reqwest::Error);
        SerdeXml(serde_xml_rs::Error);
        SerdeJson(serde_json::Error);
        ParseUrl(reqwest::UrlError);
        ParseResponse(std::num::ParseIntError);
    }

    errors {
        MissingParameter(t: String) {
            description("Missing parameter")
            display("Missing parameter: '{}'", t)
        }
    }
}
