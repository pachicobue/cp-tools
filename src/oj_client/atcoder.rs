use crate::oj_client::{base_client::BaseClient, OjClient, ProblemInfo, SampleInfo};
use ::{
    color_eyre::eyre::{ensure, OptionExt, Result},
    scraper::{Element, Html, Selector},
    url::Url,
};

const ATCODER_ENDPOINT: &str = "https://atcoder.jp";
pub(crate) const ATCODER_HOST: &str = "atcoder.jp";

pub(crate) struct AtcoderClient {
    client: BaseClient,
}

impl OjClient for AtcoderClient {
    fn new() -> Result<Self> {
        let client = BaseClient::new()?;
        Ok(Self { client })
    }
    fn login(&self) -> Result<()> {
        if self.check_login()? {
            log::debug!("Already logged in.");
            return Ok(());
        }

        log::debug!("Login to {}", ATCODER_ENDPOINT);
        let url = Self::endpoint()?;
        let username = dialoguer::Input::<String>::new()
            .with_prompt("Username")
            .interact()?;
        let password = dialoguer::Password::new()
            .with_prompt("Password")
            .interact()?;

        log::debug!("Try get CSRF token.");
        let doc = self.get_page(&url)?;
        let csrf_token = doc
            .select(&Selector::parse("input[name=\"csrf_token\"]").unwrap())
            .next()
            .ok_or_eyre("Could not find CSRF token")?
            .value()
            .attr("value")
            .ok_or_eyre("Could not find CSRF token")?;

        log::debug!("Post account info.");
        self.client.post(
            &url.join("/login")?,
            &[
                ("username", &username),
                ("password", &password),
                ("csrf_token", csrf_token),
            ],
        )?;

        log::debug!("Check login status.");
        ensure!(self.check_login()?, "Login failed");
        Ok(())
    }

    fn get_problem_info(&self, url: &Url) -> Result<ProblemInfo> {
        let mut info = ProblemInfo {
            samples: SampleInfo {
                case_num: 0,
                inputs: vec![],
                outputs: vec![],
            },
        };

        let doc = self.get_page(url)?;
        let sels = [
            Selector::parse("span.lang > span.lang-en > div.part > section > h3")
                .expect("Invalid selector"),
            Selector::parse("span.lang > span.lang-ja > div.part > section > h3")
                .expect("Invalid selector"),
        ];
        let target_text = [("Sample Input", "Sample Output"), ("入力例", "出力例")];
        log::debug!("Target element\n{:?}", target_text);

        for (sel, (in_text, out_text)) in sels.iter().zip(target_text.iter()) {
            for header in doc.select(sel) {
                let str = header.text().collect::<String>();
                if str.starts_with(in_text) {
                    log::trace!("Input elem found: \"{}\"", str);
                    if let Some(body) = header.next_sibling_element() {
                        info.samples.inputs.push(body.text().collect::<String>());
                    }
                } else if str.starts_with(out_text) {
                    log::trace!("Output elem found: \"{}\"", str);
                    if let Some(body) = header.next_sibling_element() {
                        info.samples.outputs.push(body.text().collect::<String>());
                    }
                }
            }
            info.samples.case_num = info.samples.inputs.len();
            if info.samples.case_num > 0 {
                break;
            }
        }
        Ok(info)
    }
}

impl AtcoderClient {
    fn endpoint() -> Result<Url> {
        Ok(Url::parse(ATCODER_ENDPOINT)?)
    }

    pub fn get_page(&self, url: &Url) -> Result<Html> {
        let doc = self.client.get(url)?;
        Ok(Html::parse_document(&doc))
    }

    fn check_login(&self) -> Result<bool> {
        log::trace!("Access to {}", &Self::endpoint()?);
        let doc = self.get_page(&Self::endpoint()?)?;
        Ok(doc
            .select(&Selector::parse("li a[href^=\"/users/\"]").unwrap())
            .next()
            .is_some())
    }
}
