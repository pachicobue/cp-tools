use crate::oj_client::{base_client::BaseClient, OjClient, ProblemInfo, SampleInfo};
use ::{
    anyhow::{anyhow, bail, Context as _, Result},
    reqwest::Url,
    scraper::{Element, Html, Selector},
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
    async fn login(&self) -> Result<()> {
        if self.check_login().await? {
            log::debug!("Already logged in.");
            return Ok(());
        }

        log::debug!("Logging in to {}...", ATCODER_ENDPOINT);
        let url = Self::endpoint()?;
        log::trace!("Access to {}", url);
        let doc = self.get_page(&url).await?;
        let csrf_token = doc
            .select(&Selector::parse("input[name=\"csrf_token\"]").unwrap())
            .next()
            .with_context(|| "cannot find csrf_token")?;
        let csrf_token = csrf_token
            .value()
            .attr("value")
            .with_context(|| "cannot find csrf_token")?;

        let username = rprompt::prompt_reply("Username: ")?;
        let password = rpassword::prompt_password("Password: ")?;
        log::trace!("Access to {}", &url.join("/login")?);
        let res = self
            .client
            .post(
                &url.join("/login")?,
                &[
                    ("username", &username),
                    ("password", &password),
                    ("csrf_token", csrf_token),
                ],
            )
            .await?;

        log::trace!("Parse page content");
        let res = Html::parse_document(&res);
        if let Some(err) = res
            .select(&Selector::parse("div.alert-danger").unwrap())
            .next()
        {
            bail!("Login failed: {}", err.text().collect::<String>());
        }
        if res
            .select(&Selector::parse("div.alert-success").unwrap())
            .next()
            .is_some()
        {
            return Ok(());
        }

        Err(anyhow!("Login failed: Unknown error"))
    }

    async fn get_problem_info(&self, url: &Url) -> Result<ProblemInfo> {
        log::trace!("Access to {}", url);
        let doc = self.get_page(url).await?;

        let mut info = ProblemInfo {
            samples: SampleInfo {
                case_num: 0,
                inputs: vec![],
                outputs: vec![],
            },
        };

        log::debug!("Parsing sample...");
        let sels = [
            Selector::parse("span.lang > span.lang-en > div.part > section > h3")
                .expect("Invalid selector"),
            Selector::parse("span.lang > span.lang-ja > div.part > section > h3")
                .expect("Invalid selector"),
        ];
        let target_text = [("Sample Input", "Sample Output"), ("入力例", "出力例")];

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

    pub async fn get_page(&self, url: &Url) -> Result<Html> {
        let doc = self.client.get(url).await?;
        Ok(Html::parse_document(&doc))
    }

    async fn check_login(&self) -> Result<bool> {
        let doc = self.get_page(&Self::endpoint()?).await?;
        Ok(doc
            .select(&Selector::parse("li a[href^=\"/users/\"]").unwrap())
            .next()
            .is_some())
    }
}
