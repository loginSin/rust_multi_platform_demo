use std::collections::HashMap;
use std::time::Duration;
use std::{fs, io};

use crate::core::enums::platform_type::Platform;
use lazy_static::lazy_static;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Certificate, Client, ClientBuilder, Error, Response, StatusCode};
use tokio_stream::StreamExt;

lazy_static! {
    /// 缓存鸿蒙平台专属 HttpClient
    static ref HARMONY_CLIENT: Client = new_harmony_os_client(false);
     /// 缓存鸿蒙平台专属私有云 HttpClient
    static ref HARMONY_PRIVATE_CLIENT: Client = new_harmony_os_client(true);
    /// 缓存其他平台 HttpClient
    static ref OTHER_CLIENT: Client = new_cloud_os_client(false);
    /// 缓存其他平台私有云 HttpClient
    static ref OTHER_PRIVATE_CLIENT: Client = new_cloud_os_client(true);
}

#[derive(Debug)]
pub(crate) enum HttpError {
    /// 请求超时
    RequestTimeout,
    /// 请求失败
    RequestFailed,
}

pub(crate) struct HttpClient {
    client: Client,
    time_out: Duration,
}

impl HttpClient {
    /// 根据传入的参数发送 get 请求，返回结果
    pub async fn get(&self, url: &str) -> Result<(u16, String), HttpError> {
        let request = self.client.get(url).timeout(self.time_out);
        let ret = request.send().await;
        match ret.as_ref() {
            Ok(_) => unwrap_resp(ret, url).await,
            Err(err) => print_error(url, err),
        }
    }

    pub fn new(platform: Platform, time_out: Duration) -> Self {
        let client = if platform == Platform::HarmonyOS {
            HARMONY_CLIENT.clone()
        } else {
            OTHER_CLIENT.clone()
        };
        Self { client, time_out }
    }

    /// 根据传入的参数发送 post 请求，返回结果
    pub async fn post(
        &self,
        url: &str,
        header: HashMap<&'static str, String>,
        body: &str,
    ) -> Result<(u16, String), HttpError> {
        let mut header_map = HeaderMap::new();
        for (k, v) in header {
            let v = HeaderValue::try_from(v).expect("invalid header value!");
            header_map.append(k, v);
        }
        let request = self
            .client
            .post(url)
            .headers(header_map)
            .body(body.to_string())
            .timeout(self.time_out);

        let ret = request.send().await;
        match ret.as_ref() {
            Ok(_) => unwrap_resp(ret, url).await,
            Err(err) => print_error(url, err),
        }
    }
}

async fn unwrap_resp(
    resp: Result<Response, Error>,
    url: &str,
) -> Result<(u16, String), HttpError> {
    match resp {
        Ok(resp) => {
            let code = resp.status();
            let text = resp.text().await.unwrap_or("no response!".to_string());
            if code != StatusCode::OK {
                println!(
                    "http response is not 200(OK), url: {}, code = {}, text = {}",
                    url, code, text
                );
            }
            // TODO: 这里不应该把 code 直接往上抛，而应该转成 HttpError
            Ok((code.as_u16(), text))
        }
        Err(e) => {
            println!("http request failed, url: {}, error = {:?}", url, e);
            Err(HttpError::RequestFailed)
        }
    }
}

fn print_error(url: &str, err: &Error) -> Result<(u16, String), HttpError> {
    if err.is_timeout() {
        println!("http request timeout, url = {}", &url);
        Err(HttpError::RequestTimeout)
    } else {
        println!("http request failed, url = {}, err = {}", &url, err);
        Err(HttpError::RequestFailed)
    }
}

/// 创建鸿蒙平台专属 HttpClient（因需加载证书）
fn new_harmony_os_client(is_private_cloud: bool) -> Client {
    let mut client_builder = ClientBuilder::new();
    let certs_ret = get_harmony_certs();
    if is_private_cloud {
        client_builder = client_builder.danger_accept_invalid_certs(true);
    }
    if let Ok(certs) = certs_ret {
        for cer in certs {
            client_builder = client_builder.add_root_certificate(cer);
        }
    }
    client_builder.build().unwrap_or_else(|e| {
        println!(
            "build harmony http client {}, failed, error = {:?}",
            is_private_cloud, e
        );
        Client::new()
    })
}

fn new_cloud_os_client(is_private_cloud: bool) -> Client {
    let mut client_builder = ClientBuilder::new();
    if is_private_cloud {
        client_builder = client_builder.danger_accept_invalid_certs(true);
    }
    client_builder.build().unwrap_or_else(|e| {
        println!(
            "build http client  {}, failed,  error = {:?}",
            is_private_cloud, e
        );
        Client::new()
    })
}

/// 获取鸿蒙所有证书：包含 CA 证书和特定路径下的全部证书
pub(crate) fn get_harmony_certs() -> io::Result<Vec<Certificate>> {
    let path_vec = get_harmony_cert_path_vec();
    path_vec
        .iter()
        .map(|path| {
            let pem_data = fs::read(path)?;
            Certificate::from_pem(&pem_data)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
        .collect()
}

fn get_harmony_cert_path_vec() -> Vec<String> {
    let mut ret_cert_vec = Vec::new();
    // 加入 CA 证书路径
    ret_cert_vec.push("/etc/ssl/certs/cacert.pem".to_string());

    // 加入指定路径下所有文件
    let dir_path = "/system/etc/security/certificates/";
    // 使用 read_dir 函数获取目录中的文件迭代器
    if let Ok(entries) = fs::read_dir(dir_path) {
        // 对目录中的每个文件进行迭代
        for entry in entries {
            if let Ok(entry) = entry {
                // 获取文件路径
                let file_path = entry.path();
                let path_opt = file_path.to_str();
                if let Some(path) = path_opt {
                    ret_cert_vec.push(path.to_string());
                }
            }
        }
    }
    ret_cert_vec
}
