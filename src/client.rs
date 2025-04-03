use super::error::{Result, ResultExt};
use super::xmlfmt::{from_params, into_params, parse, Call, Fault, Params, Response};
use serde::{Deserialize, Serialize};
use std;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

pub fn call_value<URL, Tkey>(uri: &URL, name: Tkey, params: Params) -> Result<Response>
where
    URL: reqwest::IntoUrl + Clone,
    Tkey: Into<String>,
{
    Client::new()?.call_value(uri, name, params)
}

pub fn call<'a, URL, Tkey, Treq, Tres>(
    uri: &URL,
    name: Tkey,
    req: Treq,
) -> Result<std::result::Result<Tres, Fault>>
where
    URL: reqwest::IntoUrl + Clone,
    Tkey: Into<String>,
    Treq: Serialize,
    Tres: Deserialize<'a>,
{
    Client::new()?.call(uri, name, req)
}

pub struct Client {
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new() -> Result<Client> {
        let client = reqwest::blocking::Client::new();
        Ok(Client { client })
    }

    pub fn call_value<URL, Tkey>(&mut self, uri: &URL, name: Tkey, params: Params) -> Result<Response>
    where
        URL: reqwest::IntoUrl + Clone,
        Tkey: Into<String>,
    {
        use super::xmlfmt::value::ToXml;
        let body = Call {
            name: name.into(),
            params,
        }
        .to_xml();

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/xml"));

        let response = self
            .client
            .post(uri.clone())
            .headers(headers)
            .body(body)
            .send()
            .chain_err(|| "Failed to run the HTTP request within hyper.")?;

        parse::response(response).map_err(Into::into)
    }

    pub fn call<'a, URL, Tkey, Treq, Tres>(
        &mut self,
        uri: &URL,
        name: Tkey,
        req: Treq,
    ) -> Result<std::result::Result<Tres, Fault>>
    where
        URL: reqwest::IntoUrl + Clone,
        Tkey: Into<String>,
        Treq: Serialize,
        Tres: Deserialize<'a>,
    {
        match self.call_value(uri, name, into_params(&req)?) {
            Ok(Ok(v)) => from_params(v).map(Ok).map_err(Into::into),
            Ok(Err(v)) => Ok(Err(v)),
            Err(v) => Err(v),
        }
    }
}
