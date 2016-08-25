/*
 Copyright 2016 LambdaStack All rights reserved.

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

 http://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
*/

/*
 Portions borrowed from the rusoto project. See README.md
*/

//! AWS API request signatures.
//!
//! Follows [AWS Signature 4](http://docs.aws.amazon.com/general/latest/gr/signature-version-4.html)
//! algorithm.
//!
//! If needed, the request will be re-issued to a temporary redirect endpoint.  This can happen with
//! newly created S3 buckets not in us-standard/us-east-1.

use std::ascii::AsciiExt;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::str;
use std::io::prelude::*;

use openssl::crypto::hash::Type::{SHA1, SHA256};
use openssl::crypto::hash::hash;
use openssl::crypto::hmac::hmac;
use openssl::crypto::hmac::HMAC;
use rustc_serialize::hex::ToHex;
use rustc_serialize::base64::{ToBase64, STANDARD};
use time::Tm;
use time::now_utc;
use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET, QUERY_ENCODE_SET};

use aws::common::credentials::AwsCredentials;
use aws::common::params::Params;
use aws::common::region::Region;

//const HTTP_TEMPORARY_REDIRECT: StatusCode = StatusCode::TemporaryRedirect;

/// A data structure for all the elements of an HTTP request that are involved in
/// the Amazon Signature Version 4 signing process
/// version - represents the Signature version. The default is 4 but it can also be set to 2 for older environments.
#[derive(Debug)]
pub struct SignedRequest<'a> {
    method: String,
    service: String,
    region: Region,
    path: String,
    headers: BTreeMap<String, Vec<Vec<u8>>>,
    params: Params,
    bucket: String,
    hostname: Option<String>,
    payload: Option<&'a [u8]>,
    content_type: Option<String>,
    canonical_query_string: String,
    canonical_uri: String,
    version: String,
    ssl: bool,
}

impl <'a> SignedRequest <'a> {
    /// Default constructor
    pub fn new(method: &str, service: &str, region: Region, bucket: &str, path: &str, version: &str) -> SignedRequest<'a> {
        SignedRequest {
            method: method.to_string(),
            service: service.to_string(),
            region: region,
            path: path.to_string(),
            headers: BTreeMap::new(),
            params: Params::new(),
            bucket: bucket.to_string(),
            hostname: None,
            payload: None,
            content_type: None,
            canonical_query_string: String::new(),
            canonical_uri: String::new(),
            version: version.to_string(),
            ssl: true,
         }
    }

    pub fn set_content_type(&mut self, content_type: String) {
        self.content_type = Some(content_type);
    }

    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    pub fn set_bucket(&mut self, bucket: &str) {
        self.bucket = bucket.to_string();
    }

    // NOTE: Use this for adding an actual endpoint such as s3.us-east-1.amazon.com or one of your choice.
    pub fn set_hostname(&mut self, hostname: Option<String>) {
        self.hostname = hostname;
    }

    // NOTE: This pulls from default service types like S3 etc. Only use this if use AWS directly.
    pub fn set_endpoint_prefix(&mut self, endpoint_prefix: String) {
        self.hostname = Some(build_hostname(&endpoint_prefix, self.region));
    }

    pub fn set_payload(&mut self, payload: Option<&'a [u8]>) {
        self.payload = payload;
    }

    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn canonical_uri(&self) -> &str {
        &self.canonical_uri
    }

    pub fn canonical_query_string(&self) -> &str {
        &self.canonical_query_string
    }

    pub fn payload(&self) -> Option<&'a [u8]> {
        self.payload
    }

    pub fn headers(&'a self) -> &'a BTreeMap<String, Vec<Vec<u8>>> {
        &self.headers
    }

    pub fn hostname(&self) -> String {
        match self.hostname {
            Some(ref h) => h.to_string(),
            None => build_hostname(&self.service, self.region)
        }
    }

    // If the key exists in headers, set it to blank/unoccupied:
    pub fn remove_header(&mut self, key: &str) {
        let key_lower = key.to_ascii_lowercase().to_string();
        self.headers.remove(&key_lower);
    }

    /// Add a value to the array of headers for the specified key.
    /// Headers are kept sorted by key name for use at signing (BTreeMap)
    pub fn add_header(&mut self, key: &str, value: &str) {
        let key_lower = key.to_ascii_lowercase().to_string();
        let value_vec = value.as_bytes().to_vec();

        match self.headers.entry(key_lower) {
            Entry::Vacant(entry) => {
                let mut values = Vec::new();
                values.push(value_vec);
                entry.insert(values);
            }
            Entry::Occupied(entry) => {
                entry.into_mut().push(value_vec);
            }
        }
    }

    pub fn update_header(&mut self, key: &str, value: &str) {
        self.remove_header(key);

        let key_lower = key.to_ascii_lowercase().to_string();
        let value_vec = value.as_bytes().to_vec();

        match self.headers.entry(key_lower) {
            Entry::Vacant(entry) => {
                let mut values = Vec::new();
                values.push(value_vec);
                entry.insert(values);
            }
            Entry::Occupied(entry) => {}
        }
    }

    pub fn get_header(&mut self, key: &str) -> String {
        let key_lower = key.to_ascii_lowercase().to_string();

        match self.headers.entry(key_lower) {
            Entry::Vacant(entry) => "".to_string(),
            Entry::Occupied(entry) => {
                canonical_values(entry.get())
            }
        }
    }

    pub fn add_param<S>(&mut self, key: S, value: S)  where S: Into<String> {
        self.params.insert(key.into(), value.into());
    }

    pub fn set_params(&mut self, params: Params){
        self.params = params;
    }

    /// set_ssl(ssl: bool) - Allows you to override the default ssl flag.
    pub fn set_ssl(&mut self, ssl: bool) {
        self.ssl = ssl;
    }

    /// ssl() - Returns the ssl bool flag
    pub fn ssl(&self) -> bool {
        self.ssl
    }

    pub fn sign(&mut self, creds: &AwsCredentials) {
        if self.version == "V2" {
            self.sign_v2(&creds);
        } else {
            self.sign_v4(&creds);
        }
    }

    fn sign_v2(&mut self, creds: &AwsCredentials) {
        debug!("Creating request to send to AWS.");
        let hostname = match self.hostname {
            Some(ref h) => h.to_string(),
            None => build_hostname(&self.service, self.region)
        };

        // Gotta remove and re-add headers since by default they append the value.  If we're following
        // a 307 redirect we end up with Three Stooges in the headers with duplicate values.
        self.update_header("Host", &hostname);

        let ct = match self.content_type {
            Some(ref h) => h.to_string(),
            None => String::from("application/octet-stream")
        };

        self.update_header("Content-Type", &ct);

        if let Some(ref token) = *creds.token() {
            self.update_header("X-Amz-Security-Token", token);
        }

        // V2 uses GMT in long format
        let date = now_utc().rfc822().to_string();
        self.update_header("Date", &date);

        self.canonical_query_string = build_canonical_query_string(&self.params);
        self.canonical_uri = canonical_uri(&self.path);
        let canonical_headers = canonical_headers_v2(&self.headers);

        println!("----sign_v2----------");
        println!("{:?}", self.bucket);
        println!("{:?}", hostname);
        println!("{:?}", self.canonical_query_string);
        println!("{:?}", self.canonical_uri);
        println!("{:?}", canonical_headers);
        println!("**-------------------");
        println!("{:?}", self.headers);
        println!("{:?}", self.path);
        println!("{:?}", self.params);
        println!("---------------------");

        // NOTE: If you set the 'date' header then include it in the string_to_sign w/o the x-amz-date resource. If you do not
        // use the date header but use the x-amz-date then set the date in string_to_sign to "" and include x-amz-date in the resource.
        // It makes it easier to exclude date and set x-amz-date instead.
        //let new_sig = self.auth(&self.method, &date, &self.canonical_uri, "", "", &canonical_headers, creds.aws_access_key_id().to_string(), creds.aws_secret_access_key().to_string());

        let md5 = self.get_header("Content-MD5");
        let content_type = self.get_header("Content-Type");
        let date_str = self.get_header("Date");

        // NOTE: canonical_headers_v2 may should pull back /{bucket}/{key}
        // AWS takes bucket (host) and uses it for calc

        let string_to_sign = format!("{}\n{}\n{}\n{}\n{}{}",
            &self.method,
            md5,
            content_type,
            date_str,
            canonical_headers_v2(&self.headers),
            canonical_resources_v2(&self.bucket, &self.path));

        match self.payload {
            None => {
                self.update_header("Content-Length", &format!("{}", 0));
            },
            Some(payload) => {
                self.update_header("Content-Length", &format!("{}", payload.len()));
                //println!("--------payload---------");
                //println!("{:?}", payload);
            }
        }

        println!("{:?}", self.canonical_query_string);
        println!("{:?}", string_to_sign);
        println!("===================");

        let signature = {
            let mut hmac = HMAC::new(SHA1, creds.aws_secret_access_key().as_bytes());
            let _ = hmac.write_all(string_to_sign.as_bytes());
            hmac.finish().to_base64(STANDARD)
        };

        self.update_header("Authorization", &format!("AWS {}:{}", creds.aws_access_key_id(), signature));
    }

    fn sign_v4(&mut self, creds: &AwsCredentials) {
        debug!("Creating request to send to AWS.");
        let hostname = match self.hostname {
            Some(ref h) => h.to_string(),
            None => build_hostname(&self.service, self.region)
        };

        // Gotta remove and re-add headers since by default they append the value.  If we're following
        // a 307 redirect we end up with Three Stooges in the headers with duplicate values.
        self.remove_header("host");
        self.add_header("host", &hostname);

        if let Some(ref token) = *creds.token() {
            self.remove_header("X-Amz-Security-Token");
            self.add_header("X-Amz-Security-Token", token);
        }

        self.canonical_query_string = build_canonical_query_string(&self.params);

        let date = now_utc();
        self.remove_header("x-amz-date");
        self.add_header("x-amz-date", &date.strftime("%Y%m%dT%H%M%SZ").unwrap().to_string());

        // build the canonical request
        let signed_headers = signed_headers(&self.headers);
        self.canonical_uri = canonical_uri(&self.path);
        let canonical_headers = canonical_headers(&self.headers);

        let canonical_request : String;

        match self.payload {
            None => {
                canonical_request = format!("{}\n{}\n{}\n{}\n{}\n{}",
                    &self.method,
                    self.canonical_uri,
                    self.canonical_query_string,
                    canonical_headers,
                    signed_headers,
                    &to_hexdigest_from_string(""));
                self.remove_header("x-amz-content-sha256");
                self.add_header("x-amz-content-sha256", &to_hexdigest_from_string(""));
            }
            Some(payload) => {
                canonical_request = format!("{}\n{}\n{}\n{}\n{}\n{}",
                    &self.method,
                    self.canonical_uri,
                    self.canonical_query_string,
                    canonical_headers,
                    signed_headers,
                    &to_hexdigest_from_bytes(payload));
                self.remove_header("x-amz-content-sha256");
                self.add_header("x-amz-content-sha256", &to_hexdigest_from_bytes(payload));
                self.remove_header("content-length");
                self.add_header("content-length", &format!("{}", payload.len()));
            }
        }

        println!("----sign_v4----------");
        println!("{:?}", self.canonical_query_string);
        println!("{:?}", self.canonical_uri);
        println!("{:?}", canonical_headers);
        println!("---------------------");
        println!("{:?}", self.headers);
        println!("{:?}", self.path);
        println!("{:?}", self.params);
        println!("{:?}", canonical_request);
        println!("=====================");

        self.remove_header("content-type");
        let ct = match self.content_type {
            Some(ref h) => h.to_string(),
            None => String::from("application/octet-stream")
        };

        self.add_header("content-type", &ct);

        // use the hashed canonical request to build the string to sign
        let hashed_canonical_request = to_hexdigest_from_string(&canonical_request);
        let scope = format!("{}/{}/{}/aws4_request", date.strftime("%Y%m%d").unwrap(), self.region, &self.service);
        let string_to_sign = string_to_sign_v4(date, &hashed_canonical_request, &scope);

        // construct the signing key and sign the string with it
        let signing_key = signing_key(creds.aws_secret_access_key(), date, &self.region.to_string(), &self.service);
        let signature = signature(&string_to_sign, signing_key);

        // build the actual auth header
        let auth_header = format!("AWS4-HMAC-SHA256 Credential={}/{}, SignedHeaders={}, Signature={}",
                   &creds.aws_access_key_id(), scope, signed_headers, signature);
        self.remove_header("authorization");
        self.add_header("authorization", &auth_header);
    }

}

fn signature(string_to_sign: &str, signing_key: Vec<u8>) -> String {
    hmac(SHA256, &signing_key, string_to_sign.as_bytes()).to_hex().to_string()
}

fn signing_key(secret: &str, date: Tm, region: &str, service: &str) -> Vec<u8> {
    let k_date = hmac(SHA256, format!("AWS4{}", secret).as_bytes(), date.strftime("%Y%m%d").unwrap().to_string().as_bytes());
    let k_region = hmac(SHA256, &k_date, region.as_bytes());
    let k_service = hmac(SHA256, &k_region, service.as_bytes());
    hmac(SHA256, &k_service, b"aws4_request")
}

/// Mark string as AWS4-HMAC-SHA256 hashed
pub fn string_to_sign_v4(date: Tm, hashed_canonical_request: &str, scope: &str) -> String {
    format!("AWS4-HMAC-SHA256\n{}\n{}\n{}",
        date.strftime("%Y%m%dT%H%M%SZ").unwrap(),
        scope,
        hashed_canonical_request)
}

fn signed_headers(headers: &BTreeMap<String, Vec<Vec<u8>>>) -> String {
    let mut signed = String::new();

    for (key,_) in headers.iter() {
        if !signed.is_empty() {
            signed.push(';')
        }

        if skipped_headers(key) {
            continue;
        }
        signed.push_str(&key.to_ascii_lowercase());
    }
    signed
}

fn canonical_headers_v2(headers: &BTreeMap<String, Vec<Vec<u8>>>) -> String {
    let mut canonical = String::new();

    // NOTE: May need to add to vec, sort and then do the following for x-amz-

    for item in headers.iter() {
        if skipped_headers(item.0) {
            continue;
        } else {
            match item.0.to_ascii_lowercase().find("x-amz-") {
                None => {},
                _ => canonical.push_str(format!("{}:{}\n", item.0.to_ascii_lowercase(), canonical_values(item.1)).as_ref())
            };
        }
    }

    canonical
}

fn canonical_headers(headers: &BTreeMap<String, Vec<Vec<u8>>>) -> String {
    let mut canonical = String::new();

    for item in headers.iter() {
        if skipped_headers(item.0) {
            continue;
        }
        canonical.push_str(format!("{}:{}\n", item.0.to_ascii_lowercase(), canonical_values(item.1)).as_ref());
    }
    canonical
}

fn canonical_values(values: &[Vec<u8>]) -> String {
    let mut st = String::new();
    for v in values {
        let s = str::from_utf8(v).unwrap();
        if !st.is_empty() {
            st.push(',')
        }
        if s.starts_with('\"') {
            st.push_str(s);
        } else {
            st.push_str(s.replace("  ", " ").trim());
        }
    }
    st
}

fn skipped_headers(header: &str) -> bool {
    ["authorization", "content-length", "user-agent"].contains(&header)
}

fn canonical_uri(path: &str) -> String {
    match path {
        "" => "/".to_string(),
        _ => encode_uri(path)
    }
}

fn canonical_resources(path: &str) -> String {
    match path {
        "" => "/".to_string(),
        _ => encode_uri(path)
    }
}

fn canonical_resources_v2(bucket: &str, path: &str) -> String {
    match bucket {
        "" => {
            match path {
                "" => "/".to_string(),
                _ => encode_uri(&format!("{}", path))  // This assumes / as leading char
            }
        }
        _ => {
            match path {
                "" => format!("/{}/", bucket),
                _ => encode_uri(&format!("/{}{}", bucket, path))  // This assumes path with leading / char
            }
        }
    }
}

fn build_canonical_query_string(params: &Params) -> String {
    if params.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    for item in params.iter() {
        if !output.is_empty() {
            output.push_str("&");
        }
        output.push_str(&byte_serialize(item.0));
        output.push_str("=");
        output.push_str(&byte_serialize(item.1));
    }

    output
}

#[inline]
fn encode_uri(uri: &str) -> String {
    utf8_percent_encode(uri, QUERY_ENCODE_SET).collect::<String>()
}

#[inline]
fn byte_serialize(input: &str) -> String {
    utf8_percent_encode(input, DEFAULT_ENCODE_SET).collect::<String>()
}

// TODO: consolidate these functions
fn to_hexdigest_from_string(val: &str) -> String {
    let h = hash(SHA256, val.as_bytes());
    h.to_hex().to_string()
}

fn to_hexdigest_from_bytes(val: &[u8]) -> String {
    let h = hash(SHA256, val);
    h.to_hex().to_string()
}

// NOTE: Used to build a hostname from a set of defaults. Use set_hostname is preferred.
fn build_hostname(service: &str, region: Region) -> String {
    //iam has only 1 endpoint, other services have region-based endpoints
    match service {
        "iam" => {
                match region {
                    Region::CnNorth1 => format!("{}.{}.amazonaws.com.cn", service, region),
                    _ => format!("{}.amazonaws.com", service),
                }
            }
        "s3" => {
                match region {
                    Region::UsEast1 => "s3.amazonaws.com".to_string(),
                    Region::CnNorth1 => format!("s3.{}.amazonaws.com.cn", region),
                    _ => format!("s3-{}.amazonaws.com", region),
                }
            }
        _ => {
                match region {
                    Region::CnNorth1 => format!("{}.{}.amazonaws.com.cn", service, region),
                    _ => format!("{}.{}.amazonaws.com", service, region),
                }
            }
    }
}


#[cfg(test)]
mod tests {
    use region::Region;

    use super::SignedRequest;

    use super::super::ProfileProvider;
    use super::super::credential::ProvideAwsCredentials;

    #[test]
    fn get_hostname_none_present() {
        let request = SignedRequest::new("POST", "sqs", Region::UsEast1, "/");
        assert_eq!("sqs.us-east-1.amazonaws.com", request.hostname());
    }

    #[test]
    fn get_hostname_happy_path() {
        let mut request = SignedRequest::new("POST", "sqs", Region::UsEast1, "/");
        request.set_hostname(Some("test-hostname".to_string()));
        assert_eq!("test-hostname", request.hostname());
    }
    #[test]
    fn path_percent_encoded() {
        let provider = ProfileProvider::with_configuration(
            "tests/sample-data/multiple_profile_credentials",
            "foo",
        );
        let mut request = SignedRequest::new("GET", "s3", Region::UsEast1, "/path with spaces");
        request.sign(provider.credentials().as_ref().unwrap());
        assert_eq!("/path%20with%20spaces", request.canonical_uri());
    }
}
