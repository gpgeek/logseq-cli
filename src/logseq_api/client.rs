use chrono::Local;
use reqwest::header::{self, HeaderMap};
use std::collections::HashMap;
use std::env;

pub struct Client {
    client: reqwest::blocking::Client,
    current_journal: Option<String>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::builder()
                .default_headers(Self::client_headers())
                .build()
                .unwrap(),
            current_journal: Option::None,
        }
    }

    fn client_headers() -> HeaderMap {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&env::var("LOGSEQ_API_KEY").unwrap()).unwrap(),
        );
        headers
    }

    pub fn add_journal_note(&mut self, note_text: &String) {
        let journal_id = self.current_journal();

        let body = "{\"method\": \"logseq.Editor.insertBlock\", \"args\": [".to_owned()
        + &journal_id
        + ", \""
        + note_text
        + "\", {\"isPageBlock\": true}]}";

        let res = self
            .client
            .post(api_url())
            .body(body.clone())
            .send();

        match res {
            Ok(_response) => {
                // println!("Task added to journal! {:?}, {:?} \n\n {:?}", response.status(), response.text(), &body);
            }
            Err(error) => {
                println!("Error: {:?},  {:?}", error.status(), error);
            }
        }
    }

    fn current_journal(&mut self) -> String {
        self.current_journal
            .get_or_insert_with(|| {
                get_journal_uuid(&self.client).expect("Could not get journal id")
            })
            .to_string()
    }
}

fn get_journal_uuid(client: &reqwest::blocking::Client) -> Result<String, String> {
    let now = Local::now();

    let journal_res = client
        .post(api_url())
        .body(
            "{
    \"method\": \"logseq.db.datascriptQuery\", \
    \"args\": \
      [\"[:find (pull ?p [*]) \
          :in $ ?today \
          :where [?b :block/page ?p] \
                 [?p :block/journal? true] \
                 [?p :block/journal-day ?d] \
                 [(>= ?d "
                .to_owned()
                + &now.format("%Y%m%d").to_string()
                + " )] \
         ]\", \
         \"[:today]\"]
        }",
        )
        .send();
    match journal_res {
        Ok(response) => {
            let jason: Vec<Vec<HashMap<String, serde_json::Value>>> = response.json().unwrap();
            let journal_id = jason[0][0]["uuid"].to_string();
            return Ok(journal_id);
        }
        Err(error) => {
            return Err(format!("Error: {:?},  {:?}", error.status(), error));
        }
    }
}

fn api_url() -> String {
    env::var("LOGSEQ_API_URL").unwrap_or("http://127.0.0.1:12315/api".to_string())
}