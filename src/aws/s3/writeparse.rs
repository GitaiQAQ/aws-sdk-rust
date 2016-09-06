//
// Copyright 2016 LambdaStack All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Portions borrowed from the rusoto project. See README.md

//! Library Documentation
//!
//! This file is a WIP file. It contains a lot of technical debt that is in the process of being
//! cleaned up and organized. It will most likely go away in the near future.

#![allow(unused_variables)]
#![allow(unused_mut)]
use std::collections::HashMap;
use std::str::FromStr;
use std::str;

use aws::common::params::{Params, ServiceParams};
use aws::common::xmlutil::*;
use aws::common::common::*;
use aws::errors::http::*;
use aws::s3::bucket::*;
use aws::s3::object::*;
use aws::s3::policy::*;
use aws::s3::acl::*;
use aws::s3::grant::*;
use aws::s3::header::*;

pub type DeleteMarkerVersionId = String;
/// Parse `DeleteMarkerVersionId` from XML
pub struct DeleteMarkerVersionIdParser;

impl DeleteMarkerVersionIdParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T)
        -> Result<DeleteMarkerVersionId, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `DeleteMarkerVersionId` contents to a `SignedRequest`
pub struct DeleteMarkerVersionIdWriter;

impl DeleteMarkerVersionIdWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeleteMarkerVersionId) {
        params.put(name, obj);
    }
}

/// Parse `DeletedObject` from XML
pub struct DeletedObjectParser;

impl DeletedObjectParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeletedObject, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeletedObject::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "VersionId" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("VersionId", stack));
                continue;
            }
            if current_name == "DeleteMarkerVersionId" {
                obj.delete_marker_version_id = try!(DeleteMarkerVersionIdParser::parse_xml("DeleteMarkerVersionId", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "DeleteMarker" {
                obj.delete_marker = try!(DeleteMarkerParser::parse_xml("DeleteMarker", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `DeletedObject` contents to a `SignedRequest`
pub struct DeletedObjectWriter;

impl DeletedObjectWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeletedObject) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "VersionId"), &obj.version_id);
        DeleteMarkerVersionIdWriter::write_params(params, &(prefix.to_string() + "DeleteMarkerVersionId"), &obj.delete_marker_version_id);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        DeleteMarkerWriter::write_params(params, &(prefix.to_string() + "DeleteMarker"), &obj.delete_marker);
    }
}

pub type DeletedObjects = Vec<DeletedObject>;
/// Parse `DeletedObjects` from XML
pub struct DeletedObjectsParser;

impl DeletedObjectsParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeletedObjects, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "DeletedObject" {
            obj.push(try!(DeletedObjectParser::parse_xml("DeletedObject", stack)));
        }
        Ok(obj)
    }
}

/// Write `DeletedObjects` contents to a `SignedRequest`
pub struct DeletedObjectsWriter;

impl DeletedObjectsWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeletedObjects) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            DeletedObjectWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

/// Parse `DeleteObjectOutput` from XML
pub struct DeleteObjectOutputParser;

impl DeleteObjectOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteObjectOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-version-id" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("x-amz-version-id", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            if current_name == "x-amz-delete-marker" {
                obj.delete_marker = try!(DeleteMarkerParser::parse_xml("x-amz-delete-marker", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `DeleteObjectOutput` contents to a `SignedRequest`
pub struct DeleteObjectOutputWriter;

impl DeleteObjectOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeleteObjectOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "x-amz-version-id"), &obj.version_id);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        DeleteMarkerWriter::write_params(params, &(prefix.to_string() + "x-amz-delete-marker"), &obj.delete_marker);
    }
}

/// Parse `DeleteObjectRequest` from XML
pub struct DeleteObjectRequestParser;

impl DeleteObjectRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteObjectRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteObjectRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-mfa" {
                obj.mfa = Some(try!(MFAParser::parse_xml("x-amz-mfa", stack)));
                continue;
            }
            if current_name == "versionId" {
                obj.version_id = Some(try!(ObjectVersionIdParser::parse_xml("versionId", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `DeleteObjectRequest` contents to a `SignedRequest`
pub struct DeleteObjectRequestWriter;

impl DeleteObjectRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeleteObjectRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.mfa {
            MFAWriter::write_params(params, &(prefix.to_string() + "x-amz-mfa"), obj);
        }
        if let Some(ref obj) = obj.version_id {
            ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "versionId"), obj);
        }
        //BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        //ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
    }
}

/// Parse `RestoreRequest` from XML
pub struct RestoreRequestParser;

impl RestoreRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RestoreRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = RestoreRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Days" {
                obj.days = try!(DaysParser::parse_xml("Days", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `RestoreRequest` contents to a `SignedRequest`
pub struct RestoreRequestWriter;

impl RestoreRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &RestoreRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        DaysWriter::write_params(params, &(prefix.to_string() + "Days"), &obj.days);
    }
}

/// Parse `RestoreObjectRequest` from XML
pub struct RestoreObjectRequestParser;

impl RestoreObjectRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RestoreObjectRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = RestoreObjectRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "versionId" {
                obj.version_id = Some(try!(ObjectVersionIdParser::parse_xml("versionId", stack)));
                continue;
            }
            if current_name == "RestoreRequest" {
                obj.restore_request = Some(try!(RestoreRequestParser::parse_xml("RestoreRequest", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `RestoreObjectRequest` contents to a `SignedRequest`
pub struct RestoreObjectRequestWriter;

impl RestoreObjectRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &RestoreObjectRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.version_id {
            ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "versionId"), obj);
        }
        if let Some(ref obj) = obj.restore_request {
            RestoreRequestWriter::write_params(params, &(prefix.to_string() + "RestoreRequest"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
    }
}

/// Parse `RestoreObjectOutput` from XML
pub struct RestoreObjectOutputParser;

impl RestoreObjectOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RestoreObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = RestoreObjectOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `RestoreObjectOutput` contents to a `SignedRequest`
pub struct RestoreObjectOutputWriter;

impl RestoreObjectOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &RestoreObjectOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
    }
}

pub type Errors = Vec<String>;
/// Parse `Errors` from XML
pub struct ErrorsParser;

impl ErrorsParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Errors, XmlParseError> {
        let mut obj : Vec<String> = Vec::new();
        while try!(peek_at_name(stack)) == "Error" {
            obj.push(try!(ErrorsParser::parse_single_error(stack)));
        }
        Ok(obj)
    }
    // hand crafted:
    pub fn parse_single_error<T: Peek + Next>(stack: &mut T) -> Result<String, XmlParseError> {
        // TODO: go back to try!

        match characters(stack) {
            Err(why) => Err(why),
            Ok(val) => Ok(val),
        }
    }
}

/// Write `Errors` contents to a `SignedRequest`
pub struct ErrorsWriter;

impl ErrorsWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Errors) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            ErrorsWriter::write_param(params, key, element);
            index += 1;
        }
    }
    // hand crafted:
    fn write_param(params: &mut Params, key: &str, value: &str) {
        params.put(key, value);
    }
}

pub type IfUnmodifiedSince = String;

/// Parse `IfUnmodifiedSince` from XML
pub struct IfUnmodifiedSinceParser;

impl IfUnmodifiedSinceParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<IfUnmodifiedSince, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `IfUnmodifiedSince` contents to a `SignedRequest`
pub struct IfUnmodifiedSinceWriter;

impl IfUnmodifiedSinceWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &IfUnmodifiedSince) {
        params.put(name, obj);
    }
}


pub type MFADelete = String;
/// Parse `MFADelete` from XML
pub struct MFADeleteParser;

impl MFADeleteParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MFADelete, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `MFADelete` contents to a `SignedRequest`
pub struct MFADeleteWriter;

impl MFADeleteWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MFADelete) {
        params.put(name, obj);
    }
}



/// Parse `DeleteBucketCorsRequest` from XML
pub struct DeleteBucketCorsRequestParser;

impl DeleteBucketCorsRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteBucketCorsRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteBucketCorsRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `DeleteBucketCorsRequest` contents to a `SignedRequest`
pub struct DeleteBucketCorsRequestWriter;

impl DeleteBucketCorsRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeleteBucketCorsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}



pub type AllowedMethod = String;
/// Parse `AllowedMethod` from XML
pub struct AllowedMethodParser;

impl AllowedMethodParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AllowedMethod, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `AllowedMethod` contents to a `SignedRequest`
pub struct AllowedMethodWriter;

impl AllowedMethodWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &AllowedMethod) {
        params.put(name, obj);
    }
}


pub type AllowedMethods = Vec<AllowedMethod>;
/// Parse `AllowedMethods` from XML
pub struct AllowedMethodsParser;

impl AllowedMethodsParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AllowedMethods, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "AllowedMethod" {
            obj.push(try!(AllowedMethodParser::parse_xml("AllowedMethod", stack)));
        }
        Ok(obj)
    }
}

/// Write `AllowedMethods` contents to a `SignedRequest`
pub struct AllowedMethodsWriter;

impl AllowedMethodsWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &AllowedMethods) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            AllowedMethodWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

pub type MaxAgeSeconds = i32;
/// Parse `MaxAgeSeconds` from XML
pub struct MaxAgeSecondsParser;

impl MaxAgeSecondsParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MaxAgeSeconds, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `MaxAgeSeconds` contents to a `SignedRequest`
pub struct MaxAgeSecondsWriter;

impl MaxAgeSecondsWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MaxAgeSeconds) {
        params.put(name, &obj.to_string());
    }
}

pub type AllowedOrigins = Vec<AllowedOrigin>;
/// Parse `AllowedOrigins` from XML
pub struct AllowedOriginsParser;

impl AllowedOriginsParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AllowedOrigins, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "AllowedOrigin" {
            obj.push(try!(AllowedOriginParser::parse_xml("AllowedOrigin", stack)));
        }
        Ok(obj)
    }
}

/// Write `AllowedOrigins` contents to a `SignedRequest`
pub struct AllowedOriginsWriter;

impl AllowedOriginsWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &AllowedOrigins) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            AllowedOriginWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

pub type MFA = String;
/// Parse `MFA` from XML
pub struct MFAParser;

impl MFAParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MFA, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `MFA` contents to a `SignedRequest`
pub struct MFAWriter;

impl MFAWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MFA) {
        params.put(name, obj);
    }
}

/// Parse `VersioningConfiguration` from XML
pub struct VersioningConfigurationParser;

impl VersioningConfigurationParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<VersioningConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = VersioningConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Status" {
                obj.status = try!(BucketVersioningStatusParser::parse_xml("Status", stack));
                continue;
            }
            if current_name == "MfaDelete" {
                obj.mfa_delete = try!(MFADeleteParser::parse_xml("MfaDelete", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `VersioningConfiguration` contents to a `SignedRequest`
pub struct VersioningConfigurationWriter;

impl VersioningConfigurationWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &VersioningConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketVersioningStatusWriter::write_params(params, &(prefix.to_string() + "Status"), &obj.status);
        MFADeleteWriter::write_params(params, &(prefix.to_string() + "MfaDelete"), &obj.mfa_delete);
    }
}

/// Parse `CORSRule` from XML
pub struct CORSRuleParser;

impl CORSRuleParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CORSRule, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CORSRule::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "AllowedHeader" {
                obj.allowed_headers = try!(AllowedHeadersParser::parse_xml("AllowedHeader", stack));
                continue;
            }
            if current_name == "ExposeHeader" {
                obj.expose_headers = try!(ExposeHeadersParser::parse_xml("ExposeHeader", stack));
                continue;
            }
            if current_name == "AllowedMethod" {
                obj.allowed_methods = try!(AllowedMethodsParser::parse_xml("AllowedMethod", stack));
                continue;
            }
            if current_name == "MaxAgeSeconds" {
                obj.max_age_seconds = try!(MaxAgeSecondsParser::parse_xml("MaxAgeSeconds", stack));
                continue;
            }
            if current_name == "AllowedOrigin" {
                obj.allowed_origins = try!(AllowedOriginsParser::parse_xml("AllowedOrigin", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `CORSRule` contents to a `SignedRequest`
pub struct CORSRuleWriter;

impl CORSRuleWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CORSRule) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        AllowedHeadersWriter::write_params(params, &(prefix.to_string() + "AllowedHeader"), &obj.allowed_headers);
        ExposeHeadersWriter::write_params(params, &(prefix.to_string() + "ExposeHeader"), &obj.expose_headers);
        AllowedMethodsWriter::write_params(params, &(prefix.to_string() + "AllowedMethod"), &obj.allowed_methods);
        MaxAgeSecondsWriter::write_params(params, &(prefix.to_string() + "MaxAgeSeconds"), &obj.max_age_seconds);
        AllowedOriginsWriter::write_params(params, &(prefix.to_string() + "AllowedOrigin"), &obj.allowed_origins);
    }
}

/// Parse `PutBucketVersioningRequest` from XML
pub struct PutBucketVersioningRequestParser;

impl PutBucketVersioningRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutBucketVersioningRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutBucketVersioningRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-mfa" {
                obj.mfa = Some(try!(MFAParser::parse_xml("x-amz-mfa", stack)));
                continue;
            }
            if current_name == "Content-MD5" {
                obj.content_md5 = Some(try!(ContentMD5Parser::parse_xml("Content-MD5", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "VersioningConfiguration" {
                obj.versioning_configuration = try!(VersioningConfigurationParser::parse_xml("VersioningConfiguration", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `PutBucketVersioningRequest` contents to a `SignedRequest`
pub struct PutBucketVersioningRequestWriter;

impl PutBucketVersioningRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &PutBucketVersioningRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.mfa {
            MFAWriter::write_params(params, &(prefix.to_string() + "x-amz-mfa"), obj);
        }
        if let Some(ref obj) = obj.content_md5 {
            ContentMD5Writer::write_params(params, &(prefix.to_string() + "Content-MD5"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        VersioningConfigurationWriter::write_params(params, &(prefix.to_string() + "VersioningConfiguration"), &obj.versioning_configuration);
    }
}

pub type CORSRules = Vec<CORSRule>;
/// Parse `CORSRules` from XML
pub struct CORSRulesParser;

impl CORSRulesParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CORSRules, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "CORSRule" {
            obj.push(try!(CORSRuleParser::parse_xml("CORSRule", stack)));
        }
        Ok(obj)
    }
}

/// Write `CORSRules` contents to a `SignedRequest`
pub struct CORSRulesWriter;

impl CORSRulesWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CORSRules) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            CORSRuleWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

/// Parse `GetBucketCorsOutput` from XML
pub struct GetBucketCorsOutputParser;

impl GetBucketCorsOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketCorsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketCorsOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "CORSRule" {
                obj.cors_rules = try!(CORSRulesParser::parse_xml("CORSRule", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `GetBucketCorsOutput` contents to a `SignedRequest`
pub struct GetBucketCorsOutputWriter;

impl GetBucketCorsOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GetBucketCorsOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        CORSRulesWriter::write_params(params, &(prefix.to_string() + "CORSRule"), &obj.cors_rules);
    }
}

/// Parse `GetBucketCorsRequest` from XML
pub struct GetBucketCorsRequestParser;

impl GetBucketCorsRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketCorsRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketCorsRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `GetBucketCorsRequest` contents to a `SignedRequest`
pub struct GetBucketCorsRequestWriter;

impl GetBucketCorsRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GetBucketCorsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}

/// Parse `LifecycleConfiguration` from XML
pub struct LifecycleConfigurationParser;

impl LifecycleConfigurationParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<LifecycleConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = LifecycleConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Rule" {
                obj.rules = try!(RulesParser::parse_xml("Rule", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `LifecycleConfiguration` contents to a `SignedRequest`
pub struct LifecycleConfigurationWriter;

impl LifecycleConfigurationWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &LifecycleConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        RulesWriter::write_params(params, &(prefix.to_string() + "Rule"), &obj.rules);
    }
}

/// Parse `PutBucketLifecycleRequest` from XML
pub struct PutBucketLifecycleRequestParser;

impl PutBucketLifecycleRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutBucketLifecycleRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutBucketLifecycleRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "LifecycleConfiguration" {
                obj.lifecycle_configuration = Some(try!(LifecycleConfigurationParser::parse_xml("LifecycleConfiguration", stack)));
                continue;
            }
            if current_name == "Content-MD5" {
                obj.content_md5 = Some(try!(ContentMD5Parser::parse_xml("Content-MD5", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `PutBucketLifecycleRequest` contents to a `SignedRequest`
pub struct PutBucketLifecycleRequestWriter;

impl PutBucketLifecycleRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &PutBucketLifecycleRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.lifecycle_configuration {
            LifecycleConfigurationWriter::write_params(params, &(prefix.to_string() + "LifecycleConfiguration"), obj);
        }
        if let Some(ref obj) = obj.content_md5 {
            ContentMD5Writer::write_params(params, &(prefix.to_string() + "Content-MD5"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}



/// Parse `GetBucketLoggingRequest` from XML
pub struct GetBucketLoggingRequestParser;

impl GetBucketLoggingRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketLoggingRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketLoggingRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `GetBucketLoggingRequest` contents to a `SignedRequest`
pub struct GetBucketLoggingRequestWriter;

impl GetBucketLoggingRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GetBucketLoggingRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}

pub type CloudFunctionInvocationRole = String;
/// Parse `CloudFunctionInvocationRole` from XML
pub struct CloudFunctionInvocationRoleParser;

impl CloudFunctionInvocationRoleParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CloudFunctionInvocationRole, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `CloudFunctionInvocationRole` contents to a `SignedRequest`
pub struct CloudFunctionInvocationRoleWriter;

impl CloudFunctionInvocationRoleWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CloudFunctionInvocationRole) {
        params.put(name, obj);
    }
}

pub type CloudFunction = String;
/// Parse `CloudFunction` from XML
pub struct CloudFunctionParser;

impl CloudFunctionParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CloudFunction, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `CloudFunction` contents to a `SignedRequest`
pub struct CloudFunctionWriter;

impl CloudFunctionWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CloudFunction) {
        params.put(name, obj);
    }
}

pub type QueueArn = String;
/// Parse `QueueArn` from XML
pub struct QueueArnParser;

impl QueueArnParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<QueueArn, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `QueueArn` contents to a `SignedRequest`
pub struct QueueArnWriter;

impl QueueArnWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &QueueArn) {
        params.put(name, obj);
    }
}

pub type TopicArn = String;
/// Parse `TopicArn` from XML
pub struct TopicArnParser;

impl TopicArnParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TopicArn, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `TopicArn` contents to a `SignedRequest`
pub struct TopicArnWriter;

impl TopicArnWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &TopicArn) {
        params.put(name, obj);
    }
}

/// Parse `CloudFunctionConfiguration` from XML
pub struct CloudFunctionConfigurationParser;

impl CloudFunctionConfigurationParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CloudFunctionConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CloudFunctionConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "InvocationRole" {
                obj.invocation_role = try!(CloudFunctionInvocationRoleParser::parse_xml("InvocationRole", stack));
                continue;
            }
            if current_name == "CloudFunction" {
                obj.cloud_function = try!(CloudFunctionParser::parse_xml("CloudFunction", stack));
                continue;
            }
            if current_name == "Event" {
                obj.events = try!(EventListParser::parse_xml("Event", stack));
                continue;
            }
            if current_name == "Id" {
                obj.id = try!(NotificationIdParser::parse_xml("Id", stack));
                continue;
            }
            if current_name == "Event" {
                obj.event = try!(EventParser::parse_xml("Event", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `CloudFunctionConfiguration` contents to a `SignedRequest`
pub struct CloudFunctionConfigurationWriter;

impl CloudFunctionConfigurationWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CloudFunctionConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        CloudFunctionInvocationRoleWriter::write_params(params, &(prefix.to_string() + "InvocationRole"), &obj.invocation_role);
        CloudFunctionWriter::write_params(params, &(prefix.to_string() + "CloudFunction"), &obj.cloud_function);
        EventListWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.events);
        NotificationIdWriter::write_params(params, &(prefix.to_string() + "Id"), &obj.id);
        EventWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.event);
    }
}

/// Parse `QueueConfigurationDeprecated` from XML
pub struct QueueConfigurationDeprecatedParser;

impl QueueConfigurationDeprecatedParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<QueueConfigurationDeprecated, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = QueueConfigurationDeprecated::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Queue" {
                obj.queue = try!(QueueArnParser::parse_xml("Queue", stack));
                continue;
            }
            if current_name == "Event" {
                obj.events = try!(EventListParser::parse_xml("Event", stack));
                continue;
            }
            if current_name == "Id" {
                obj.id = try!(NotificationIdParser::parse_xml("Id", stack));
                continue;
            }
            if current_name == "Event" {
                obj.event = try!(EventParser::parse_xml("Event", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `QueueConfigurationDeprecated` contents to a `SignedRequest`
pub struct QueueConfigurationDeprecatedWriter;

impl QueueConfigurationDeprecatedWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &QueueConfigurationDeprecated) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        QueueArnWriter::write_params(params, &(prefix.to_string() + "Queue"), &obj.queue);
        EventListWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.events);
        NotificationIdWriter::write_params(params, &(prefix.to_string() + "Id"), &obj.id);
        EventWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.event);
    }
}

/// Parse `TopicConfigurationDeprecated` from XML
pub struct TopicConfigurationDeprecatedParser;

impl TopicConfigurationDeprecatedParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TopicConfigurationDeprecated, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = TopicConfigurationDeprecated::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Topic" {
                obj.topic = try!(TopicArnParser::parse_xml("Topic", stack));
                continue;
            }
            if current_name == "Id" {
                obj.id = try!(NotificationIdParser::parse_xml("Id", stack));
                continue;
            }
            if current_name == "Event" {
                obj.event = try!(EventParser::parse_xml("Event", stack));
                continue;
            }
            if current_name == "Event" {
                obj.events = try!(EventListParser::parse_xml("Event", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `TopicConfigurationDeprecated` contents to a `SignedRequest`
pub struct TopicConfigurationDeprecatedWriter;

impl TopicConfigurationDeprecatedWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &TopicConfigurationDeprecated) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        TopicArnWriter::write_params(params, &(prefix.to_string() + "Topic"), &obj.topic);
        NotificationIdWriter::write_params(params, &(prefix.to_string() + "Id"), &obj.id);
        EventWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.event);
        EventListWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.events);
    }
}


/// Parse `PutBucketNotificationRequest` from XML
pub struct PutBucketNotificationRequestParser;

impl PutBucketNotificationRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutBucketNotificationRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutBucketNotificationRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            match current_name.as_ref() {
                "NotificationConfiguration" => {
                    obj.notification_configuration = try!(NotificationConfigurationDeprecatedParser::parse_xml("NotificationConfiguration", stack));
                    continue;
                },
                "Content-MD5" => {
                    obj.content_md5 = Some(try!(ContentMD5Parser::parse_xml("Content-MD5", stack)));
                    continue;
                },
                "Bucket" => {
                    obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                    continue;
                },
                _ => break,
            }
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `PutBucketNotificationRequest` contents to a `SignedRequest`
pub struct PutBucketNotificationRequestWriter;

impl PutBucketNotificationRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &PutBucketNotificationRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        NotificationConfigurationDeprecatedWriter::write_params(params, &(prefix.to_string() + "NotificationConfiguration"), &obj.notification_configuration);
        if let Some(ref obj) = obj.content_md5 {
            ContentMD5Writer::write_params(params, &(prefix.to_string() + "Content-MD5"), obj);
        }

        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}

#[derive(Debug, Default)]
pub struct NotificationConfigurationDeprecated {
    pub cloud_function_configuration: CloudFunctionConfiguration,
    pub queue_configuration: QueueConfigurationDeprecated,
    pub topic_configuration: TopicConfigurationDeprecated,
}

/// Parse `NotificationConfigurationDeprecated` from XML
pub struct NotificationConfigurationDeprecatedParser;

impl NotificationConfigurationDeprecatedParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NotificationConfigurationDeprecated, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = NotificationConfigurationDeprecated::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "CloudFunctionConfiguration" {
                obj.cloud_function_configuration = try!(CloudFunctionConfigurationParser::parse_xml("CloudFunctionConfiguration", stack));
                continue;
            }
            if current_name == "QueueConfiguration" {
                obj.queue_configuration = try!(QueueConfigurationDeprecatedParser::parse_xml("QueueConfiguration", stack));
                continue;
            }
            if current_name == "TopicConfiguration" {
                obj.topic_configuration = try!(TopicConfigurationDeprecatedParser::parse_xml("TopicConfiguration", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `NotificationConfigurationDeprecated` contents to a `SignedRequest`
pub struct NotificationConfigurationDeprecatedWriter;

impl NotificationConfigurationDeprecatedWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &NotificationConfigurationDeprecated) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        CloudFunctionConfigurationWriter::write_params(params, &(prefix.to_string() + "CloudFunctionConfiguration"), &obj.cloud_function_configuration);
        QueueConfigurationDeprecatedWriter::write_params(params, &(prefix.to_string() + "QueueConfiguration"), &obj.queue_configuration);
        TopicConfigurationDeprecatedWriter::write_params(params, &(prefix.to_string() + "TopicConfiguration"), &obj.topic_configuration);
    }
}

pub type ResponseContentEncoding = String;
/// Parse `ResponseContentEncoding` from XML
pub struct ResponseContentEncodingParser;

impl ResponseContentEncodingParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ResponseContentEncoding, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ResponseContentEncoding` contents to a `SignedRequest`
pub struct ResponseContentEncodingWriter;

impl ResponseContentEncodingWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ResponseContentEncoding) {
        params.put(name, obj);
    }
}

pub type ResponseContentLanguage = String;
/// Parse `ResponseContentLanguage` from XML
pub struct ResponseContentLanguageParser;

impl ResponseContentLanguageParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ResponseContentLanguage, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ResponseContentLanguage` contents to a `SignedRequest`
pub struct ResponseContentLanguageWriter;

impl ResponseContentLanguageWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ResponseContentLanguage) {
        params.put(name, obj);
    }
}

pub type ResponseContentType = String;
/// Parse `ResponseContentType` from XML
pub struct ResponseContentTypeParser;

impl ResponseContentTypeParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ResponseContentType, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ResponseContentType` contents to a `SignedRequest`
pub struct ResponseContentTypeWriter;

impl ResponseContentTypeWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ResponseContentType) {
        params.put(name, obj);
    }
}

pub type IfNoneMatch = String;
/// Parse `IfNoneMatch` from XML
pub struct IfNoneMatchParser;

impl IfNoneMatchParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<IfNoneMatch, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `IfNoneMatch` contents to a `SignedRequest`
pub struct IfNoneMatchWriter;

impl IfNoneMatchWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &IfNoneMatch) {
        params.put(name, obj);
    }
}

pub type ResponseCacheControl = String;
/// Parse `ResponseCacheControl` from XML
pub struct ResponseCacheControlParser;

impl ResponseCacheControlParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ResponseCacheControl, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ResponseCacheControl` contents to a `SignedRequest`
pub struct ResponseCacheControlWriter;

impl ResponseCacheControlWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ResponseCacheControl) {
        params.put(name, obj);
    }
}

pub type ResponseContentDisposition = String;
/// Parse `ResponseContentDisposition` from XML
pub struct ResponseContentDispositionParser;

impl ResponseContentDispositionParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ResponseContentDisposition, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ResponseContentDisposition` contents to a `SignedRequest`
pub struct ResponseContentDispositionWriter;

impl ResponseContentDispositionWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ResponseContentDisposition) {
        params.put(name, obj);
    }
}

pub type IfMatch = String;
/// Parse `IfMatch` from XML
pub struct IfMatchParser;

impl IfMatchParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<IfMatch, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `IfMatch` contents to a `SignedRequest`
pub struct IfMatchWriter;

impl IfMatchWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &IfMatch) {
        params.put(name, obj);
    }
}

pub type ResponseExpires = String;
/// Parse `ResponseExpires` from XML
pub struct ResponseExpiresParser;

impl ResponseExpiresParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ResponseExpires, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ResponseExpires` contents to a `SignedRequest`
pub struct ResponseExpiresWriter;

impl ResponseExpiresWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ResponseExpires) {
        params.put(name, obj);
    }
}

pub type SSECustomerKey = String;
/// Parse `SSECustomerKey` from XML
pub struct SSECustomerKeyParser;

impl SSECustomerKeyParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<SSECustomerKey, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `SSECustomerKey` contents to a `SignedRequest`
pub struct SSECustomerKeyWriter;

impl SSECustomerKeyWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &SSECustomerKey) {
        params.put(name, obj);
    }
}



/// Parse `GetObjectTorrentRequest` from XML
pub struct GetObjectTorrentRequestParser;

impl GetObjectTorrentRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetObjectTorrentRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetObjectTorrentRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `GetObjectTorrentRequest` contents to a `SignedRequest`
pub struct GetObjectTorrentRequestWriter;

impl GetObjectTorrentRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GetObjectTorrentRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
    }
}

/// Parse `PutBucketReplicationRequest` from XML
pub struct PutBucketReplicationRequestParser;

impl PutBucketReplicationRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutBucketReplicationRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutBucketReplicationRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "ReplicationConfiguration" {
                obj.replication_configuration = try!(ReplicationConfigurationParser::parse_xml("ReplicationConfiguration", stack));
                continue;
            }
            if current_name == "Content-MD5" {
                obj.content_md5 = Some(try!(ContentMD5Parser::parse_xml("Content-MD5", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `PutBucketReplicationRequest` contents to a `SignedRequest`
pub struct PutBucketReplicationRequestWriter;

impl PutBucketReplicationRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &PutBucketReplicationRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ReplicationConfigurationWriter::write_params(params, &(prefix.to_string() + "ReplicationConfiguration"), &obj.replication_configuration);
        if let Some(ref obj) = obj.content_md5 {
            ContentMD5Writer::write_params(params, &(prefix.to_string() + "Content-MD5"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}

/// Parse `PutBucketPolicyRequest` from XML
pub struct PutBucketPolicyRequestParser;

impl PutBucketPolicyRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutBucketPolicyRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutBucketPolicyRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Policy" {
                obj.policy = try!(PolicyParser::parse_xml("Policy", stack));
                continue;
            }
            if current_name == "Content-MD5" {
                obj.content_md5 = Some(try!(ContentMD5Parser::parse_xml("Content-MD5", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `PutBucketPolicyRequest` contents to a `SignedRequest`
pub struct PutBucketPolicyRequestWriter;

impl PutBucketPolicyRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &PutBucketPolicyRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        PolicyWriter::write_params(params, &(prefix.to_string() + "Policy"), &obj.policy);
        if let Some(ref obj) = obj.content_md5 {
            ContentMD5Writer::write_params(params, &(prefix.to_string() + "Content-MD5"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}

/// Parse `BucketLoggingStatus` from XML
pub struct BucketLoggingStatusParser;

impl BucketLoggingStatusParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BucketLoggingStatus, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = BucketLoggingStatus::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "LoggingEnabled" {
                obj.logging_enabled = try!(LoggingEnabledParser::parse_xml("LoggingEnabled", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `BucketLoggingStatus` contents to a `SignedRequest`
pub struct BucketLoggingStatusWriter;

impl BucketLoggingStatusWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &BucketLoggingStatus) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LoggingEnabledWriter::write_params(params, &(prefix.to_string() + "LoggingEnabled"), &obj.logging_enabled);
    }
}

/// Parse `LambdaFunctionConfiguration` from XML
pub struct LambdaFunctionConfigurationParser;

impl LambdaFunctionConfigurationParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<LambdaFunctionConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = LambdaFunctionConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "CloudFunction" {
                obj.lambda_function_arn = try!(LambdaFunctionArnParser::parse_xml("CloudFunction", stack));
                continue;
            }
            if current_name == "Id" {
                obj.id = Some(try!(NotificationIdParser::parse_xml("Id", stack)));
                continue;
            }
            if current_name == "Event" {
                obj.events = try!(EventListParser::parse_xml("Event", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `LambdaFunctionConfiguration` contents to a `SignedRequest`
pub struct LambdaFunctionConfigurationWriter;

impl LambdaFunctionConfigurationWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &LambdaFunctionConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LambdaFunctionArnWriter::write_params(params, &(prefix.to_string() + "CloudFunction"), &obj.lambda_function_arn);
        if let Some(ref obj) = obj.id {
            NotificationIdWriter::write_params(params, &(prefix.to_string() + "Id"), obj);
        }
        EventListWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.events);
    }
}

/// Parse `WebsiteConfiguration` from XML
pub struct WebsiteConfigurationParser;

impl WebsiteConfigurationParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<WebsiteConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = WebsiteConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "RedirectAllRequestsTo" {
                obj.redirect_all_requests_to = try!(RedirectAllRequestsToParser::parse_xml("RedirectAllRequestsTo", stack));
                continue;
            }
            if current_name == "IndexDocument" {
                obj.index_document = try!(IndexDocumentParser::parse_xml("IndexDocument", stack));
                continue;
            }
            if current_name == "ErrorDocument" {
                obj.error_document = try!(ErrorDocumentParser::parse_xml("ErrorDocument", stack));
                continue;
            }
            if current_name == "RoutingRule" {
                obj.routing_rules = try!(RoutingRulesParser::parse_xml("RoutingRule", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `WebsiteConfiguration` contents to a `SignedRequest`
pub struct WebsiteConfigurationWriter;

impl WebsiteConfigurationWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &WebsiteConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        RedirectAllRequestsToWriter::write_params(params, &(prefix.to_string() + "RedirectAllRequestsTo"), &obj.redirect_all_requests_to);
        IndexDocumentWriter::write_params(params, &(prefix.to_string() + "IndexDocument"), &obj.index_document);
        ErrorDocumentWriter::write_params(params, &(prefix.to_string() + "ErrorDocument"), &obj.error_document);
        RoutingRulesWriter::write_params(params, &(prefix.to_string() + "RoutingRule"), &obj.routing_rules);
    }
}


pub type NextKeyMarker = String;
/// Parse `NextKeyMarker` from XML
pub struct NextKeyMarkerParser;

impl NextKeyMarkerParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NextKeyMarker, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = NextKeyMarker::default();

        if let Err(why) = characters(stack) {
            return Ok(obj); // swallow error, it's okay to be blank
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `NextKeyMarker` contents to a `SignedRequest`
pub struct NextKeyMarkerWriter;

impl NextKeyMarkerWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &NextKeyMarker) {
        params.put(name, obj);
    }
}

/// Parse `CopyObjectOutput` from XML
pub struct CopyObjectOutputParser;

impl CopyObjectOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopyObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CopyObjectOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack));
                continue;
            }
            if current_name == "x-amz-copy-source-version-id" {
                obj.copy_source_version_id = try!(CopySourceVersionIdParser::parse_xml("x-amz-copy-source-version-id", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_xml("x-amz-server-side-encryption", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            if current_name == "x-amz-expiration" {
                obj.expiration = try!(ExpirationParser::parse_xml("x-amz-expiration", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack));
                continue;
            }
            if current_name == "CopyObjectResult" {
                obj.copy_object_result = try!(CopyObjectResultParser::parse_xml("CopyObjectResult", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_xml("x-amz-server-side-encryption-aws-kms-key-id", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `CopyObjectOutput` contents to a `SignedRequest`
pub struct CopyObjectOutputWriter;

impl CopyObjectOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CopyObjectOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), &obj.sse_customer_algorithm);
        CopySourceVersionIdWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-version-id"), &obj.copy_source_version_id);
        ServerSideEncryptionWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption"), &obj.server_side_encryption);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        ExpirationWriter::write_params(params, &(prefix.to_string() + "x-amz-expiration"), &obj.expiration);
        SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), &obj.sse_customer_key_md5);
        CopyObjectResultWriter::write_params(params, &(prefix.to_string() + "CopyObjectResult"), &obj.copy_object_result);
        SSEKMSKeyIdWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"), &obj.ssekms_key_id);
    }
}




/// Parse `ReplicationRule` from XML
pub struct ReplicationRuleParser;

impl ReplicationRuleParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ReplicationRule, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ReplicationRule::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Status" {
                obj.status = try!(ReplicationRuleStatusParser::parse_xml("Status", stack));
                continue;
            }
            if current_name == "Prefix" {
                obj.prefix = try!(PrefixParser::parse_xml("Prefix", stack));
                continue;
            }
            if current_name == "Destination" {
                obj.destination = try!(DestinationParser::parse_xml("Destination", stack));
                continue;
            }
            if current_name == "ID" {
                obj.id = Some(try!(IDParser::parse_xml("ID", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ReplicationRule` contents to a `SignedRequest`
pub struct ReplicationRuleWriter;

impl ReplicationRuleWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ReplicationRule) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ReplicationRuleStatusWriter::write_params(params, &(prefix.to_string() + "Status"), &obj.status);
        PrefixWriter::write_params(params, &(prefix.to_string() + "Prefix"), &obj.prefix);
        DestinationWriter::write_params(params, &(prefix.to_string() + "Destination"), &obj.destination);
        if let Some(ref obj) = obj.id {
            IDWriter::write_params(params, &(prefix.to_string() + "ID"), obj);
        }
    }
}



pub type MetadataDirective = String;

/// Parse `MetadataDirective` from XML
pub struct MetadataDirectiveParser;

impl MetadataDirectiveParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MetadataDirective, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `MetadataDirective` contents to a `SignedRequest`
pub struct MetadataDirectiveWriter;

impl MetadataDirectiveWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MetadataDirective) {
        params.put(name, obj);
    }
}



/// Parse `PutBucketLoggingRequest` from XML
pub struct PutBucketLoggingRequestParser;

impl PutBucketLoggingRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutBucketLoggingRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutBucketLoggingRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "BucketLoggingStatus" {
                obj.bucket_logging_status = try!(BucketLoggingStatusParser::parse_xml("BucketLoggingStatus", stack));
                continue;
            }
            if current_name == "Content-MD5" {
                obj.content_md5 = Some(try!(ContentMD5Parser::parse_xml("Content-MD5", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `PutBucketLoggingRequest` contents to a `SignedRequest`
pub struct PutBucketLoggingRequestWriter;

impl PutBucketLoggingRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &PutBucketLoggingRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketLoggingStatusWriter::write_params(params, &(prefix.to_string() + "BucketLoggingStatus"), &obj.bucket_logging_status);
        if let Some(ref obj) = obj.content_md5 {
            ContentMD5Writer::write_params(params, &(prefix.to_string() + "Content-MD5"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}

/// If present, indicates that the requester was successfully charged for the
/// request.
pub type RequestCharged = String;

/// Parse `RequestCharged` from XML
pub struct RequestChargedParser;

impl RequestChargedParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RequestCharged, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `RequestCharged` contents to a `SignedRequest`
pub struct RequestChargedWriter;

impl RequestChargedWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &RequestCharged) {
        params.put(name, obj);
    }
}

pub type LambdaFunctionConfigurationList = Vec<LambdaFunctionConfiguration>;
/// Parse `LambdaFunctionConfigurationList` from XML
pub struct LambdaFunctionConfigurationListParser;

impl LambdaFunctionConfigurationListParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<LambdaFunctionConfigurationList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "LambdaFunctionConfiguration" {
            obj.push(try!(LambdaFunctionConfigurationParser::parse_xml("LambdaFunctionConfiguration", stack)));
        }
        Ok(obj)
    }
}

/// Write `LambdaFunctionConfigurationList` contents to a `SignedRequest`
pub struct LambdaFunctionConfigurationListWriter;

impl LambdaFunctionConfigurationListWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &LambdaFunctionConfigurationList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            LambdaFunctionConfigurationWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

pub type ServerSideEncryption = String;
/// Parse `ServerSideEncryption` from XML
pub struct ServerSideEncryptionParser;

impl ServerSideEncryptionParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ServerSideEncryption, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ServerSideEncryption` contents to a `SignedRequest`
pub struct ServerSideEncryptionWriter;

impl ServerSideEncryptionWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ServerSideEncryption) {
        params.put(name, obj);
    }
}

pub type SSEKMSKeyId = String;
/// Parse `SSEKMSKeyId` from XML
pub struct SSEKMSKeyIdParser;

impl SSEKMSKeyIdParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<SSEKMSKeyId, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `SSEKMSKeyId` contents to a `SignedRequest`
pub struct SSEKMSKeyIdWriter;

impl SSEKMSKeyIdWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &SSEKMSKeyId) {
        params.put(name, obj);
    }
}

pub type MultipartUploadId = String;
/// Parse `MultipartUploadId` from XML
pub struct MultipartUploadIdParser;

impl MultipartUploadIdParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MultipartUploadId, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `MultipartUploadId` contents to a `SignedRequest`
pub struct MultipartUploadIdWriter;

impl MultipartUploadIdWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MultipartUploadId) {
        params.put(name, obj);
    }
}

/// Bucket event for which to send notifications.
pub type Event = String;

/// Parse `Event` from XML
pub struct EventParser;

impl EventParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Event, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Event` contents to a `SignedRequest`
pub struct EventWriter;

impl EventWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Event) {
        params.put(name, obj);
    }
}

/// Parse `PutBucketWebsiteRequest` from XML
pub struct PutBucketWebsiteRequestParser;

impl PutBucketWebsiteRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutBucketWebsiteRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutBucketWebsiteRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Content-MD5" {
                obj.content_md5 = Some(try!(ContentMD5Parser::parse_xml("Content-MD5", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "WebsiteConfiguration" {
                obj.website_configuration = try!(WebsiteConfigurationParser::parse_xml("WebsiteConfiguration", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `PutBucketWebsiteRequest` contents to a `SignedRequest`
pub struct PutBucketWebsiteRequestWriter;

impl PutBucketWebsiteRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &PutBucketWebsiteRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.content_md5 {
            ContentMD5Writer::write_params(params, &(prefix.to_string() + "Content-MD5"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        WebsiteConfigurationWriter::write_params(params, &(prefix.to_string() + "WebsiteConfiguration"), &obj.website_configuration);
    }
}


pub type WebsiteRedirectLocation = String;
/// Parse `WebsiteRedirectLocation` from XML
pub struct WebsiteRedirectLocationParser;

impl WebsiteRedirectLocationParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<WebsiteRedirectLocation, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `WebsiteRedirectLocation` contents to a `SignedRequest`
pub struct WebsiteRedirectLocationWriter;

impl WebsiteRedirectLocationWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &WebsiteRedirectLocation) {
        params.put(name, obj);
    }
}

pub type Expiration = String;
/// Parse `Expiration` from XML
pub struct ExpirationParser;

impl ExpirationParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Expiration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Expiration` contents to a `SignedRequest`
pub struct ExpirationWriter;

impl ExpirationWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Expiration) {
        params.put(name, obj);
    }
}

pub type ReplicationRules = Vec<ReplicationRule>;
/// Parse `ReplicationRules` from XML
pub struct ReplicationRulesParser;

impl ReplicationRulesParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ReplicationRules, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "ReplicationRule" {
            obj.push(try!(ReplicationRuleParser::parse_xml("ReplicationRule", stack)));
        }
        Ok(obj)
    }
}

/// Write `ReplicationRules` contents to a `SignedRequest`
pub struct ReplicationRulesWriter;

impl ReplicationRulesWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ReplicationRules) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            ReplicationRuleWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

pub type MetadataKey = String;
/// Parse `MetadataKey` from XML
pub struct MetadataKeyParser;

impl MetadataKeyParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MetadataKey, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `MetadataKey` contents to a `SignedRequest`
pub struct MetadataKeyWriter;

impl MetadataKeyWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MetadataKey) {
        params.put(name, obj);
    }
}

pub type MetadataValue = String;
/// Parse `MetadataValue` from XML
pub struct MetadataValueParser;

impl MetadataValueParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MetadataValue, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `MetadataValue` contents to a `SignedRequest`
pub struct MetadataValueWriter;

impl MetadataValueWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MetadataValue) {
        params.put(name, obj);
    }
}

pub type TargetPrefix = String;
/// Parse `TargetPrefix` from XML
pub struct TargetPrefixParser;

impl TargetPrefixParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TargetPrefix, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `TargetPrefix` contents to a `SignedRequest`
pub struct TargetPrefixWriter;

impl TargetPrefixWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &TargetPrefix) {
        params.put(name, obj);
    }
}

pub type TargetBucket = String;
/// Parse `TargetBucket` from XML
pub struct TargetBucketParser;

impl TargetBucketParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TargetBucket, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `TargetBucket` contents to a `SignedRequest`
pub struct TargetBucketWriter;

impl TargetBucketWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &TargetBucket) {
        params.put(name, obj);
    }
}

/// Parse `TargetGrants` from XML
pub struct TargetGrantsParser;

impl TargetGrantsParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TargetGrants, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "Grant" {
            obj.push(try!(TargetGrantParser::parse_xml("Grant", stack)));
        }
        Ok(obj)
    }
}

/// Write `TargetGrants` contents to a `SignedRequest`
pub struct TargetGrantsWriter;

impl TargetGrantsWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &TargetGrants) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            TargetGrantWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

pub type Role = String;
/// Parse `Role` from XML
pub struct RoleParser;

impl RoleParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Role, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Role` contents to a `SignedRequest`
pub struct RoleWriter;

impl RoleWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Role) {
        params.put(name, obj);
    }
}

#[derive(Debug, Default)]
pub struct CommonPrefix {
    pub prefix: Prefix,
}

/// Parse `CommonPrefix` from XML
pub struct CommonPrefixParser;

impl CommonPrefixParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CommonPrefix, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CommonPrefix::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Prefix" {
                obj.prefix = try!(PrefixParser::parse_xml("Prefix", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `CommonPrefix` contents to a `SignedRequest`
pub struct CommonPrefixWriter;

impl CommonPrefixWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CommonPrefix) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        PrefixWriter::write_params(params, &(prefix.to_string() + "Prefix"), &obj.prefix);
    }
}

pub type ContentEncoding = String;
/// Parse `ContentEncoding` from XML
pub struct ContentEncodingParser;

impl ContentEncodingParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ContentEncoding, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ContentEncoding` contents to a `SignedRequest`
pub struct ContentEncodingWriter;

impl ContentEncodingWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ContentEncoding) {
        params.put(name, obj);
    }
}

pub type ReplicationStatus = String;
/// Parse `ReplicationStatus` from XML
pub struct ReplicationStatusParser;

impl ReplicationStatusParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ReplicationStatus, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ReplicationStatus` contents to a `SignedRequest`
pub struct ReplicationStatusWriter;

impl ReplicationStatusWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ReplicationStatus) {
        params.put(name, obj);
    }
}

pub type StorageClass = String;
/// Parse `StorageClass` from XML
pub struct StorageClassParser;

impl StorageClassParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<StorageClass, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `StorageClass` contents to a `SignedRequest`
pub struct StorageClassWriter;

impl StorageClassWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &StorageClass) {
        params.put(name, obj);
    }
}

pub type ContentDisposition = String;
/// Parse `ContentDisposition` from XML
pub struct ContentDispositionParser;

impl ContentDispositionParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ContentDisposition, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ContentDisposition` contents to a `SignedRequest`
pub struct ContentDispositionWriter;

impl ContentDispositionWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ContentDisposition) {
        params.put(name, obj);
    }
}

pub type Metadata = HashMap<MetadataKey,MetadataValue>;
/// Parse `Metadata` from XML
pub struct MetadataParser;

impl MetadataParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Metadata, XmlParseError> {
        let mut obj = HashMap::new();
        while try!(peek_at_name(stack)) == tag_name {
            try!(start_element(tag_name, stack));
            let key = try!(MetadataKeyParser::parse_xml("MetadataKey", stack));
            let value = try!(MetadataValueParser::parse_xml("MetadataValue", stack));
            obj.insert(key, value);
            try!(end_element(tag_name, stack));
        }
        Ok(obj)
    }
}

/// Write `Metadata` contents to a `SignedRequest`
pub struct MetadataWriter;

impl MetadataWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Metadata) {
        let mut index = 1;
        for (key,value) in obj {
            let prefix = &format!("{}.{}", name, index);
            MetadataKeyWriter::write_params(params, &format!("{}.{}", prefix, "MetadataKey"), key);
            MetadataValueWriter::write_params(params, &format!("{}.{}", prefix, "MetadataValue"), value);
            index += 1;
        }
    }
}


pub type NextMarker = String;
/// Parse `NextMarker` from XML
pub struct NextMarkerParser;

impl NextMarkerParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NextMarker, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `NextMarker` contents to a `SignedRequest`
pub struct NextMarkerWriter;

impl NextMarkerWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &NextMarker) {
        params.put(name, obj);
    }
}

pub type DeleteMarker = bool;
/// Parse `DeleteMarker` from XML
pub struct DeleteMarkerParser;

impl DeleteMarkerParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteMarker, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `DeleteMarker` contents to a `SignedRequest`
pub struct DeleteMarkerWriter;

impl DeleteMarkerWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeleteMarker) {
        params.put(name, &obj.to_string());
    }
}

pub type MissingMeta = i32;
/// Parse `MissingMeta` from XML
pub struct MissingMetaParser;

impl MissingMetaParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MissingMeta, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `MissingMeta` contents to a `SignedRequest`
pub struct MissingMetaWriter;

impl MissingMetaWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MissingMeta) {
        params.put(name, &obj.to_string());
    }
}

pub type Restore = String;
/// Parse `Restore` from XML
pub struct RestoreParser;

impl RestoreParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Restore, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Restore` contents to a `SignedRequest`
pub struct RestoreWriter;

impl RestoreWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Restore) {
        params.put(name, obj);
    }
}

pub type ContentLanguage = String;
/// Parse `ContentLanguage` from XML
pub struct ContentLanguageParser;

impl ContentLanguageParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ContentLanguage, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ContentLanguage` contents to a `SignedRequest`
pub struct ContentLanguageWriter;

impl ContentLanguageWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ContentLanguage) {
        params.put(name, obj);
    }
}

pub type KeyMarker = String;
/// Parse `KeyMarker` from XML
pub struct KeyMarkerParser;

impl KeyMarkerParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<KeyMarker, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = KeyMarker::default();

        match characters(stack) {
            Err(why) => return Ok(obj),
            Ok(chars) => obj = chars,
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `KeyMarker` contents to a `SignedRequest`
pub struct KeyMarkerWriter;

impl KeyMarkerWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &KeyMarker) {
        params.put(name, obj);
    }
}

pub type VersionIdMarker = String;
/// Parse `VersionIdMarker` from XML
pub struct VersionIdMarkerParser;

impl VersionIdMarkerParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<VersionIdMarker, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `VersionIdMarker` contents to a `SignedRequest`
pub struct VersionIdMarkerWriter;

impl VersionIdMarkerWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &VersionIdMarker) {
        params.put(name, obj);
    }
}

pub type ExposeHeader = String;
/// Parse `ExposeHeader` from XML
pub struct ExposeHeaderParser;

impl ExposeHeaderParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ExposeHeader, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ExposeHeader` contents to a `SignedRequest`
pub struct ExposeHeaderWriter;

impl ExposeHeaderWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ExposeHeader) {
        params.put(name, obj);
    }
}

/// Parse `LoggingEnabled` from XML
pub struct LoggingEnabledParser;

impl LoggingEnabledParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<LoggingEnabled, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = LoggingEnabled::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "TargetPrefix" {
                obj.target_prefix = try!(TargetPrefixParser::parse_xml("TargetPrefix", stack));
                continue;
            }
            if current_name == "TargetBucket" {
                obj.target_bucket = try!(TargetBucketParser::parse_xml("TargetBucket", stack));
                continue;
            }
            if current_name == "Grant" {
                obj.target_grants = try!(TargetGrantsParser::parse_xml("Grant", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `LoggingEnabled` contents to a `SignedRequest`
pub struct LoggingEnabledWriter;

impl LoggingEnabledWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &LoggingEnabled) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        TargetPrefixWriter::write_params(params, &(prefix.to_string() + "TargetPrefix"), &obj.target_prefix);
        TargetBucketWriter::write_params(params, &(prefix.to_string() + "TargetBucket"), &obj.target_bucket);
        TargetGrantsWriter::write_params(params, &(prefix.to_string() + "Grant"), &obj.target_grants);
    }
}

pub type Marker = String;
/// Parse `Marker` from XML
pub struct MarkerParser;

impl MarkerParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Marker, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Marker` contents to a `SignedRequest`
pub struct MarkerWriter;

impl MarkerWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Marker) {
        params.put(name, obj);
    }
}

/// Requests Amazon S3 to encode the object keys in the response and specifies the
/// encoding method to use. An object key may contain any Unicode character;
/// however, XML 1.0 parser cannot parse some characters, such as characters with
/// an ASCII value from 0 to 10. For characters that are not supported in XML 1.0,
/// you can add this parameter to request that Amazon S3 encode the keys in the
/// response.
pub type EncodingType = String;

/// Parse `EncodingType` from XML
pub struct EncodingTypeParser;

impl EncodingTypeParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<EncodingType, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `EncodingType` contents to a `SignedRequest`
pub struct EncodingTypeWriter;

impl EncodingTypeWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &EncodingType) {
        params.put(name, obj);
    }
}

pub type SSECustomerAlgorithm = String;
/// Parse `SSECustomerAlgorithm` from XML
pub struct SSECustomerAlgorithmParser;

impl SSECustomerAlgorithmParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<SSECustomerAlgorithm, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `SSECustomerAlgorithm` contents to a `SignedRequest`
pub struct SSECustomerAlgorithmWriter;

impl SSECustomerAlgorithmWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &SSECustomerAlgorithm) {
        params.put(name, obj);
    }
}

/// Parse `ReplicationConfiguration` from XML
pub struct ReplicationConfigurationParser;

impl ReplicationConfigurationParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ReplicationConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ReplicationConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "ReplicationRule" {
                obj.rules = try!(ReplicationRulesParser::parse_xml("ReplicationRule", stack));
                continue;
            }
            if current_name == "Role" {
                obj.role = try!(RoleParser::parse_xml("Role", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ReplicationConfiguration` contents to a `SignedRequest`
pub struct ReplicationConfigurationWriter;

impl ReplicationConfigurationWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ReplicationConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ReplicationRulesWriter::write_params(params, &(prefix.to_string() + "ReplicationRule"), &obj.rules);
        RoleWriter::write_params(params, &(prefix.to_string() + "Role"), &obj.role);
    }
}
/// Confirms that the requester knows that she or he will be charged for the
/// request. Bucket owners need not specify this parameter in their requests.
/// Documentation on downloading objects from requester pays buckets can be found
/// [here](http://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html).

pub type CommonPrefixList = Vec<CommonPrefix>;
/// Parse `CommonPrefixList` from XML
pub struct CommonPrefixListParser;

impl CommonPrefixListParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CommonPrefixList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "CommonPrefix" {
            obj.push(try!(CommonPrefixParser::parse_xml("CommonPrefix", stack)));
        }
        Ok(obj)
    }
}

/// Write `CommonPrefixList` contents to a `SignedRequest`
pub struct CommonPrefixListWriter;

impl CommonPrefixListWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CommonPrefixList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            CommonPrefixWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

pub type CopySourceSSECustomerAlgorithm = String;
/// Parse `CopySourceSSECustomerAlgorithm` from XML
pub struct CopySourceSSECustomerAlgorithmParser;

impl CopySourceSSECustomerAlgorithmParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopySourceSSECustomerAlgorithm, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `CopySourceSSECustomerAlgorithm` contents to a `SignedRequest`
pub struct CopySourceSSECustomerAlgorithmWriter;

impl CopySourceSSECustomerAlgorithmWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CopySourceSSECustomerAlgorithm) {
        params.put(name, obj);
    }
}

pub type SSECustomerKeyMD5 = String;
/// Parse `SSECustomerKeyMD`5 from XML
pub struct SSECustomerKeyMD5Parser;

impl SSECustomerKeyMD5Parser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<SSECustomerKeyMD5, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `SSECustomerKeyMD5` contents to a `SignedRequest`
pub struct SSECustomerKeyMD5Writer;

impl SSECustomerKeyMD5Writer {
    pub fn write_params(params: &mut Params, name: &str, obj: &SSECustomerKeyMD5) {
        params.put(name, obj);
    }
}

pub type Delimiter = String;
/// Parse `Delimiter` from XML
pub struct DelimiterParser;

impl DelimiterParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Delimiter, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Delimiter` contents to a `SignedRequest`
pub struct DelimiterWriter;

impl DelimiterWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Delimiter) {
        params.put(name, obj);
    }
}

/// Parse `DeleteBucketRequest` from XML
pub struct DeleteBucketRequestParser;

impl DeleteBucketRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteBucketRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteBucketRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Parse `DeleteBucketPolicyRequest` from XML
pub struct DeleteBucketPolicyRequestParser;

impl DeleteBucketPolicyRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteBucketPolicyRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteBucketPolicyRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `DeleteBucketPolicyRequest` contents to a `SignedRequest`
pub struct DeleteBucketPolicyRequestWriter;

impl DeleteBucketPolicyRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeleteBucketPolicyRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}

/// Parse `HeadObjectOutput` from XML
pub struct HeadObjectOutputParser;

impl HeadObjectOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<HeadObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = HeadObjectOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Last-Modified" {
                obj.last_modified = try!(LastModifiedParser::parse_xml("Last-Modified", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            if current_name == "Content-Encoding" {
                obj.content_encoding = try!(ContentEncodingParser::parse_xml("Content-Encoding", stack));
                continue;
            }
            if current_name == "x-amz-replication-status" {
                obj.replication_status = try!(ReplicationStatusParser::parse_xml("x-amz-replication-status", stack));
                continue;
            }
            if current_name == "x-amz-storage-class" {
                obj.storage_class = try!(StorageClassParser::parse_xml("x-amz-storage-class", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_xml("x-amz-server-side-encryption", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_xml("x-amz-server-side-encryption-aws-kms-key-id", stack));
                continue;
            }
            if current_name == "Content-Disposition" {
                obj.content_disposition = try!(ContentDispositionParser::parse_xml("Content-Disposition", stack));
                continue;
            }
            if current_name == "x-amz-meta-" {
                obj.metadata = try!(MetadataParser::parse_xml("x-amz-meta-", stack));
                continue;
            }
            if current_name == "accept-ranges" {
                obj.accept_ranges = try!(AcceptRangesParser::parse_xml("accept-ranges", stack));
                continue;
            }
            if current_name == "x-amz-website-redirect-location" {
                obj.website_redirect_location = try!(WebsiteRedirectLocationParser::parse_xml("x-amz-website-redirect-location", stack));
                continue;
            }
            if current_name == "Expires" {
                obj.expires = try!(ExpiresParser::parse_xml("Expires", stack));
                continue;
            }
            if current_name == "x-amz-delete-marker" {
                obj.delete_marker = try!(DeleteMarkerParser::parse_xml("x-amz-delete-marker", stack));
                continue;
            }
            if current_name == "Cache-Control" {
                obj.cache_control = try!(CacheControlParser::parse_xml("Cache-Control", stack));
                continue;
            }
            if current_name == "Content-Length" {
                obj.content_length = try!(ContentLengthParser::parse_xml("Content-Length", stack));
                continue;
            }
            if current_name == "x-amz-expiration" {
                obj.expiration = try!(ExpirationParser::parse_xml("x-amz-expiration", stack));
                continue;
            }
            if current_name == "x-amz-missing-meta" {
                obj.missing_meta = try!(MissingMetaParser::parse_xml("x-amz-missing-meta", stack));
                continue;
            }
            if current_name == "x-amz-restore" {
                obj.restore = try!(RestoreParser::parse_xml("x-amz-restore", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack));
                continue;
            }
            if current_name == "Content-Type" {
                obj.content_type = try!(ContentTypeParser::parse_xml("Content-Type", stack));
                continue;
            }
            if current_name == "Content-Language" {
                obj.content_language = try!(ContentLanguageParser::parse_xml("Content-Language", stack));
                continue;
            }
            if current_name == "x-amz-version-id" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("x-amz-version-id", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `HeadObjectOutput` contents to a `SignedRequest`
pub struct HeadObjectOutputWriter;

impl HeadObjectOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &HeadObjectOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LastModifiedWriter::write_params(params, &(prefix.to_string() + "Last-Modified"), &obj.last_modified);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        ContentEncodingWriter::write_params(params, &(prefix.to_string() + "Content-Encoding"), &obj.content_encoding);
        ReplicationStatusWriter::write_params(params, &(prefix.to_string() + "x-amz-replication-status"), &obj.replication_status);
        StorageClassWriter::write_params(params, &(prefix.to_string() + "x-amz-storage-class"), &obj.storage_class);
        ServerSideEncryptionWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption"), &obj.server_side_encryption);
        SSEKMSKeyIdWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"), &obj.ssekms_key_id);
        ContentDispositionWriter::write_params(params, &(prefix.to_string() + "Content-Disposition"), &obj.content_disposition);
        MetadataWriter::write_params(params, &(prefix.to_string() + "x-amz-meta-"), &obj.metadata);
        AcceptRangesWriter::write_params(params, &(prefix.to_string() + "accept-ranges"), &obj.accept_ranges);
        WebsiteRedirectLocationWriter::write_params(params, &(prefix.to_string() + "x-amz-website-redirect-location"), &obj.website_redirect_location);
        ExpiresWriter::write_params(params, &(prefix.to_string() + "Expires"), &obj.expires);
        DeleteMarkerWriter::write_params(params, &(prefix.to_string() + "x-amz-delete-marker"), &obj.delete_marker);
        CacheControlWriter::write_params(params, &(prefix.to_string() + "Cache-Control"), &obj.cache_control);
        ContentLengthWriter::write_params(params, &(prefix.to_string() + "Content-Length"), &obj.content_length);
        ExpirationWriter::write_params(params, &(prefix.to_string() + "x-amz-expiration"), &obj.expiration);
        MissingMetaWriter::write_params(params, &(prefix.to_string() + "x-amz-missing-meta"), &obj.missing_meta);
        RestoreWriter::write_params(params, &(prefix.to_string() + "x-amz-restore"), &obj.restore);
        SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), &obj.sse_customer_algorithm);
        ContentTypeWriter::write_params(params, &(prefix.to_string() + "Content-Type"), &obj.content_type);
        ContentLanguageWriter::write_params(params, &(prefix.to_string() + "Content-Language"), &obj.content_language);
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "x-amz-version-id"), &obj.version_id);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
        SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), &obj.sse_customer_key_md5);
    }
}

/// Parse `DeleteBucketReplicationRequest` from XML
pub struct DeleteBucketReplicationRequestParser;

impl DeleteBucketReplicationRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteBucketReplicationRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteBucketReplicationRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `DeleteBucketReplicationRequest` contents to a `SignedRequest`
pub struct DeleteBucketReplicationRequestWriter;

impl DeleteBucketReplicationRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeleteBucketReplicationRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}

pub type NextVersionIdMarker = String;
/// Parse `NextVersionIdMarker` from XML
pub struct NextVersionIdMarkerParser;

impl NextVersionIdMarkerParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NextVersionIdMarker, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `NextVersionIdMarker` contents to a `SignedRequest`
pub struct NextVersionIdMarkerWriter;

impl NextVersionIdMarkerWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &NextVersionIdMarker) {
        params.put(name, obj);
    }
}

/// Parse `ListObjectVersionsOutput` from XML
pub struct ListVersionsResultParser;

//ListObjectVersionsOutput
impl ListVersionsResultParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListVersionsResult, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ListVersionsResult::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Name" {
                obj.name = try!(BucketNameParser::parse_xml("Name", stack));
                continue;
            }
            if current_name == "Version" {
                obj.versions = try!(ObjectVersionListParser::parse_xml("Version", stack));
                continue;
            }
            if current_name == "DeleteMarker" {
                obj.delete_markers = try!(DeleteMarkersParser::parse_xml("DeleteMarker", stack));
                continue;
            }
            if current_name == "NextKeyMarker" {
                obj.next_key_marker = try!(NextKeyMarkerParser::parse_xml("NextKeyMarker", stack));
                continue;
            }
            if current_name == "Delimiter" {
                obj.delimiter = try!(DelimiterParser::parse_xml("Delimiter", stack));
                continue;
            }
            if current_name == "MaxKeys" {
                obj.max_keys = try!(MaxKeysParser::parse_xml("MaxKeys", stack));
                continue;
            }
            if current_name == "Prefix" {
                obj.prefix = try!(PrefixParser::parse_xml("Prefix", stack));
                continue;
            }
            if current_name == "KeyMarker" {
                obj.key_marker = try!(KeyMarkerParser::parse_xml("KeyMarker", stack));
                continue;
            }
            if current_name == "NextVersionIdMarker" {
                obj.next_version_id_marker = try!(NextVersionIdMarkerParser::parse_xml("NextVersionIdMarker", stack));
                continue;
            }
            if current_name == "EncodingType" {
                obj.encoding_type = try!(EncodingTypeParser::parse_xml("EncodingType", stack));
                continue;
            }
            if current_name == "IsTruncated" {
                obj.is_truncated = try!(IsTruncatedParser::parse_xml("IsTruncated", stack));
                continue;
            }
            if current_name == "VersionIdMarker" {
                obj.version_id_marker = try!(VersionIdMarkerParser::parse_xml("VersionIdMarker", stack));
                continue;
            }
            if current_name == "CommonPrefix" {
                obj.common_prefixes = try!(CommonPrefixListParser::parse_xml("CommonPrefix", stack));
                continue;
            }
            break;
        }

        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ListObjectVersionsOutput` contents to a `SignedRequest`
pub struct ListVersionsResultWriter;

impl ListVersionsResultWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ListVersionsResult) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Name"), &obj.name);
        ObjectVersionListWriter::write_params(params, &(prefix.to_string() + "Version"), &obj.versions);
        DeleteMarkersWriter::write_params(params, &(prefix.to_string() + "DeleteMarkerEntry"), &obj.delete_markers);
        NextKeyMarkerWriter::write_params(params, &(prefix.to_string() + "NextKeyMarker"), &obj.next_key_marker);
        DelimiterWriter::write_params(params, &(prefix.to_string() + "Delimiter"), &obj.delimiter);
        MaxKeysWriter::write_params(params, &(prefix.to_string() + "MaxKeys"), &obj.max_keys);
        PrefixWriter::write_params(params, &(prefix.to_string() + "Prefix"), &obj.prefix);
        KeyMarkerWriter::write_params(params, &(prefix.to_string() + "KeyMarker"), &obj.key_marker);
        NextVersionIdMarkerWriter::write_params(params, &(prefix.to_string() + "NextVersionIdMarker"), &obj.next_version_id_marker);
        EncodingTypeWriter::write_params(params, &(prefix.to_string() + "EncodingType"), &obj.encoding_type);
        IsTruncatedWriter::write_params(params, &(prefix.to_string() + "IsTruncated"), &obj.is_truncated);
        VersionIdMarkerWriter::write_params(params, &(prefix.to_string() + "VersionIdMarker"), &obj.version_id_marker);
        CommonPrefixListWriter::write_params(params, &(prefix.to_string() + "CommonPrefix"), &obj.common_prefixes);
    }
}

/// Parse `ListObjectVersionsRequest` from XML
pub struct ListObjectVersionsRequestParser;

impl ListObjectVersionsRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListObjectVersionsRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ListObjectVersionsRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "prefix" {
                obj.prefix = Some(try!(PrefixParser::parse_xml("prefix", stack)));
                continue;
            }
            if current_name == "max-keys" {
                obj.max_keys = Some(try!(MaxKeysParser::parse_xml("max-keys", stack)));
                continue;
            }
            if current_name == "delimiter" {
                obj.delimiter = Some(try!(DelimiterParser::parse_xml("delimiter", stack)));
                continue;
            }
            if current_name == "key-marker" {
                obj.key_marker = Some(try!(KeyMarkerParser::parse_xml("key-marker", stack)));
                continue;
            }
            if current_name == "encoding-type" {
                obj.encoding_type = Some(try!(EncodingTypeParser::parse_xml("encoding-type", stack)));
                continue;
            }
            if current_name == "version-id-marker" {
                obj.version_id_marker = Some(try!(VersionIdMarkerParser::parse_xml("version-id-marker", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ListObjectVersionsRequest` contents to a `SignedRequest`
pub struct ListObjectVersionsRequestWriter;

impl ListObjectVersionsRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ListObjectVersionsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.prefix {
            PrefixWriter::write_params(params, &(prefix.to_string() + "prefix"), obj);
        }
        if let Some(ref obj) = obj.max_keys {
            MaxKeysWriter::write_params(params, &(prefix.to_string() + "max-keys"), obj);
        }
        if let Some(ref obj) = obj.delimiter {
            DelimiterWriter::write_params(params, &(prefix.to_string() + "delimiter"), obj);
        }
        if let Some(ref obj) = obj.key_marker {
            KeyMarkerWriter::write_params(params, &(prefix.to_string() + "key-marker"), obj);
        }
        if let Some(ref obj) = obj.encoding_type {
            EncodingTypeWriter::write_params(params, &(prefix.to_string() + "encoding-type"), obj);
        }
        if let Some(ref obj) = obj.version_id_marker {
            VersionIdMarkerWriter::write_params(params, &(prefix.to_string() + "version-id-marker"), obj);
        }
    }
}

/// Parse `DeleteMarkerEntry` from XML
pub struct DeleteMarkerEntryParser;

impl DeleteMarkerEntryParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteMarkerEntry, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteMarkerEntry::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Owner" {
                obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
                continue;
            }
            if current_name == "IsLatest" {
                obj.is_latest = try!(IsLatestParser::parse_xml("IsLatest", stack));
                continue;
            }
            if current_name == "VersionId" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("VersionId", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "LastModified" {
                obj.last_modified = try!(LastModifiedParser::parse_xml("LastModified", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `DeleteMarkerEntry` contents to a `SignedRequest`
pub struct DeleteMarkerEntryWriter;

impl DeleteMarkerEntryWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeleteMarkerEntry) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
        IsLatestWriter::write_params(params, &(prefix.to_string() + "IsLatest"), &obj.is_latest);
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "VersionId"), &obj.version_id);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        LastModifiedWriter::write_params(params, &(prefix.to_string() + "LastModified"), &obj.last_modified);
    }
}

/// Parse `ObjectVersionList` from XML
pub struct ObjectVersionListParser;

impl ObjectVersionListParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectVersionList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "Version" {
            obj.push(try!(ObjectVersionParser::parse_xml("Version", stack)));
        }
        Ok(obj)
    }
}

/// Write `ObjectVersionList` contents to a `SignedRequest`
pub struct ObjectVersionListWriter;

impl ObjectVersionListWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectVersionList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            ObjectVersionWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

pub type DeleteMarkers = Vec<DeleteMarkerEntry>;
/// Parse `DeleteMarkers` from XML
pub struct DeleteMarkersParser;

impl DeleteMarkersParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteMarkers, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "DeleteMarker" {
            obj.push(try!(DeleteMarkerEntryParser::parse_xml("DeleteMarker", stack)));
        }
        Ok(obj)
    }
}

/// Write `DeleteMarkers` contents to a `SignedRequest`
pub struct DeleteMarkersWriter;

impl DeleteMarkersWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeleteMarkers) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            DeleteMarkerEntryWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

pub type ExposeHeaders = Vec<ExposeHeader>;

/// Parse `ExposeHeaders` from XML
pub struct ExposeHeadersParser;

impl ExposeHeadersParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ExposeHeaders, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "ExposeHeader" {
            obj.push(try!(ExposeHeaderParser::parse_xml("ExposeHeader", stack)));
        }
        Ok(obj)
    }
}

/// Write `ExposeHeaders` contents to a `SignedRequest`
pub struct ExposeHeadersWriter;

impl ExposeHeadersWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ExposeHeaders) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            ExposeHeaderWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

/// Parse `GetBucketLoggingOutput` from XML
pub struct GetBucketLoggingOutputParser;

impl GetBucketLoggingOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketLoggingOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketLoggingOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "LoggingEnabled" {
                obj.logging_enabled = try!(LoggingEnabledParser::parse_xml("LoggingEnabled", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `GetBucketLoggingOutput` contents to a `SignedRequest`
pub struct GetBucketLoggingOutputWriter;

impl GetBucketLoggingOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GetBucketLoggingOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LoggingEnabledWriter::write_params(params, &(prefix.to_string() + "LoggingEnabled"), &obj.logging_enabled);
    }
}

/// Parse `ListObjectsRequest` from XML
pub struct ListObjectsRequestParser;

impl ListObjectsRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListObjectsRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ListObjectsRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "prefix" {
                obj.prefix = Some(try!(PrefixParser::parse_xml("prefix", stack)));
                continue;
            }
            if current_name == "max-keys" {
                obj.max_keys = Some(try!(MaxKeysParser::parse_xml("max-keys", stack)));
                continue;
            }
            if current_name == "delimiter" {
                obj.delimiter = Some(try!(DelimiterParser::parse_xml("delimiter", stack)));
                continue;
            }
            if current_name == "marker" {
                obj.marker = Some(try!(MarkerParser::parse_xml("marker", stack)));
                continue;
            }
            if current_name == "encoding-type" {
                obj.encoding_type = Some(try!(EncodingTypeParser::parse_xml("encoding-type", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ListObjectsRequest` contents to a `SignedRequest`
pub struct ListObjectsRequestWriter;

impl ListObjectsRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ListObjectsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.prefix {
            PrefixWriter::write_params(params, &(prefix.to_string() + "prefix"), obj);
        }
        if let Some(ref obj) = obj.max_keys {
            MaxKeysWriter::write_params(params, &(prefix.to_string() + "max-keys"), obj);
        }
        if let Some(ref obj) = obj.delimiter {
            DelimiterWriter::write_params(params, &(prefix.to_string() + "delimiter"), obj);
        }
        if let Some(ref obj) = obj.marker {
            MarkerWriter::write_params(params, &(prefix.to_string() + "marker"), obj);
        }
        if let Some(ref obj) = obj.encoding_type {
            EncodingTypeWriter::write_params(params, &(prefix.to_string() + "encoding-type"), obj);
        }
    }
}

/// Parse `GetBucketReplicationOutput` from XML
pub struct GetBucketReplicationOutputParser;

impl GetBucketReplicationOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketReplicationOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketReplicationOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "ReplicationConfiguration" {
                obj.replication_configuration = try!(ReplicationConfigurationParser::parse_xml("ReplicationConfiguration", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `GetBucketReplicationOutput` contents to a `SignedRequest`
pub struct GetBucketReplicationOutputWriter;

impl GetBucketReplicationOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GetBucketReplicationOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ReplicationConfigurationWriter::write_params(params, &(prefix.to_string() + "ReplicationConfiguration"), &obj.replication_configuration);
    }
}

/// Parse `CreateMultipartUploadOutput` from XML
pub struct CreateMultipartUploadOutputParser;

impl CreateMultipartUploadOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CreateMultipartUploadOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CreateMultipartUploadOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "UploadId" {
                obj.upload_id = try!(MultipartUploadIdParser::parse_xml("UploadId", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_xml("x-amz-server-side-encryption", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_xml("x-amz-server-side-encryption-aws-kms-key-id", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `CreateMultipartUploadOutput` contents to a `SignedRequest`
pub struct CreateMultipartUploadOutputWriter;

impl CreateMultipartUploadOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CreateMultipartUploadOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), &obj.sse_customer_algorithm);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        MultipartUploadIdWriter::write_params(params, &(prefix.to_string() + "UploadId"), &obj.upload_id);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        ServerSideEncryptionWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption"), &obj.server_side_encryption);
        SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), &obj.sse_customer_key_md5);
        SSEKMSKeyIdWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"), &obj.ssekms_key_id);
    }
}

pub type EventList = Vec<Event>;
/// Parse `EventList` from XML
pub struct EventListParser;

impl EventListParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<EventList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "Event" {
            obj.push(try!(EventParser::parse_xml("Event", stack)));
        }
        Ok(obj)
    }
}

/// Write `EventList` contents to a `SignedRequest`
pub struct EventListWriter;

impl EventListWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &EventList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            EventWriter::write_params(params, key, element);
            index += 1;
        }
    }
}


pub type ObjectStorageClass = String;

/// Parse `ObjectStorageClass` from XML
pub struct ObjectStorageClassParser;

impl ObjectStorageClassParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectStorageClass, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ObjectStorageClass` contents to a `SignedRequest`
pub struct ObjectStorageClassWriter;

impl ObjectStorageClassWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectStorageClass) {
        params.put(name, obj);
    }
}


#[derive(Debug, Default)]
pub struct RequestPaymentConfiguration {
    /// Specifies who pays for the download and request fees.
    pub payer: Payer,
}

/// Parse `RequestPaymentConfiguration` from XML
pub struct RequestPaymentConfigurationParser;

impl RequestPaymentConfigurationParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RequestPaymentConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = RequestPaymentConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Payer" {
                obj.payer = try!(PayerParser::parse_xml("Payer", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `RequestPaymentConfiguration` contents to a `SignedRequest`
pub struct RequestPaymentConfigurationWriter;

impl RequestPaymentConfigurationWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &RequestPaymentConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        PayerWriter::write_params(params, &(prefix.to_string() + "Payer"), &obj.payer);
    }
}

/// Confirms that the requester knows that she or he will be charged for the
/// request. Bucket owners need not specify this parameter in their requests.
/// Documentation on downloading objects from requester pays buckets can be found
/// [here](http://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html).
pub type RequestPayer = String;

/// Parse `RequestPayer` from XML
pub struct RequestPayerParser;

impl RequestPayerParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RequestPayer, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `RequestPayer` contents to a `SignedRequest`
pub struct RequestPayerWriter;

impl RequestPayerWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &RequestPayer) {
        params.put(name, obj);
    }
}

#[derive(Debug, Default)]
pub struct GetBucketWebsiteRequest {
    pub bucket: BucketName,
}

/// Parse `GetBucketWebsiteRequest` from XML
pub struct GetBucketWebsiteRequestParser;

impl GetBucketWebsiteRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketWebsiteRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketWebsiteRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `GetBucketWebsiteRequest` contents to a `SignedRequest`
pub struct GetBucketWebsiteRequestWriter;

impl GetBucketWebsiteRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GetBucketWebsiteRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}

pub type Rules = Vec<Rule>;
/// Parse `Rules` from XML
pub struct RulesParser;

impl RulesParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Rules, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "Rule" {
            obj.push(try!(RuleParser::parse_xml("Rule", stack)));
        }
        Ok(obj)
    }
}

/// Write `Rules` contents to a `SignedRequest`
pub struct RulesWriter;

impl RulesWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Rules) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            RuleWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

#[derive(Debug, Default)]
pub struct Rule {
    /// If 'Enabled', the rule is currently being applied. If 'Disabled', the rule is
    /// not currently being applied.
    pub status: ExpirationStatus,
    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
    pub transition: Option<Transition>,
    /// Prefix identifying one or more objects to which the rule applies.
    pub prefix: Prefix,
    pub expiration: Option<LifecycleExpiration>,
    pub noncurrent_version_transition: Option<NoncurrentVersionTransition>,
    /// Unique identifier for the rule. The value cannot be longer than 255
    /// characters.
    pub id: Option<ID>,
}

/// Parse `Rule` from XML
struct RuleParser;

impl RuleParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Rule, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Rule::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Status" {
                obj.status = try!(ExpirationStatusParser::parse_xml("Status", stack));
                continue;
            }
            if current_name == "NoncurrentVersionExpiration" {
                obj.noncurrent_version_expiration = Some(try!(NoncurrentVersionExpirationParser::parse_xml("NoncurrentVersionExpiration", stack)));
                continue;
            }
            if current_name == "Transition" {
                obj.transition = Some(try!(TransitionParser::parse_xml("Transition", stack)));
                continue;
            }
            if current_name == "Prefix" {
                obj.prefix = try!(PrefixParser::parse_xml("Prefix", stack));
                continue;
            }
            if current_name == "Expiration" {
                obj.expiration = Some(try!(LifecycleExpirationParser::parse_xml("Expiration", stack)));
                continue;
            }
            if current_name == "NoncurrentVersionTransition" {
                obj.noncurrent_version_transition = Some(try!(NoncurrentVersionTransitionParser::parse_xml("NoncurrentVersionTransition", stack)));
                continue;
            }
            if current_name == "ID" {
                obj.id = Some(try!(IDParser::parse_xml("ID", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Rule` contents to a `SignedRequest`
pub struct RuleWriter;

impl RuleWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Rule) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ExpirationStatusWriter::write_params(params, &(prefix.to_string() + "Status"), &obj.status);
        if let Some(ref obj) = obj.noncurrent_version_expiration {
            NoncurrentVersionExpirationWriter::write_params(params, &(prefix.to_string() + "NoncurrentVersionExpiration"), obj);
        }
        if let Some(ref obj) = obj.transition {
            TransitionWriter::write_params(params, &(prefix.to_string() + "Transition"), obj);
        }
        PrefixWriter::write_params(params, &(prefix.to_string() + "Prefix"), &obj.prefix);
        if let Some(ref obj) = obj.expiration {
            LifecycleExpirationWriter::write_params(params, &(prefix.to_string() + "Expiration"), obj);
        }
        if let Some(ref obj) = obj.noncurrent_version_transition {
            NoncurrentVersionTransitionWriter::write_params(params, &(prefix.to_string() + "NoncurrentVersionTransition"), obj);
        }
        if let Some(ref obj) = obj.id {
            IDWriter::write_params(params, &(prefix.to_string() + "ID"), obj);
        }
    }
}

/// Parse `GetBucketVersioningRequest` from XML
pub struct GetBucketVersioningRequestParser;

impl GetBucketVersioningRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketVersioningRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketVersioningRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `GetBucketVersioningRequest` contents to a `SignedRequest`
pub struct GetBucketVersioningRequestWriter;
impl GetBucketVersioningRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GetBucketVersioningRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}

/// Parse `GetBucketVersioningOutput` from XML
pub struct GetBucketVersioningOutputParser;

impl GetBucketVersioningOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketVersioningOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketVersioningOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Status" {
                obj.status = try!(BucketVersioningStatusParser::parse_xml("Status", stack));
                continue;
            }
            if current_name == "MfaDelete" {
                obj.mfa_delete = try!(MFADeleteStatusParser::parse_xml("MfaDelete", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `GetBucketVersioningOutput` contents to a `SignedRequest`
pub struct GetBucketVersioningOutputWriter;

impl GetBucketVersioningOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GetBucketVersioningOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketVersioningStatusWriter::write_params(params, &(prefix.to_string() + "Status"), &obj.status);
        MFADeleteStatusWriter::write_params(params, &(prefix.to_string() + "MfaDelete"), &obj.mfa_delete);
    }
}

pub type BucketVersioningStatus = String;
/// Parse `BucketVersioningStatus` from XML
pub struct BucketVersioningStatusParser;

impl BucketVersioningStatusParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BucketVersioningStatus, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `BucketVersioningStatus` contents to a `SignedRequest`
pub struct BucketVersioningStatusWriter;

impl BucketVersioningStatusWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &BucketVersioningStatus) {
        params.put(name, obj);
    }
}

#[derive(Debug, Default)]
pub struct LifecycleExpiration {
    /// Indicates at what date the object is to be moved or deleted. Should be in GMT
    /// ISO 8601 Format.
    pub date: Date,
    /// Indicates the lifetime, in days, of the objects that are subject to the rule.
    /// The value must be a non-zero positive integer.
    pub days: Days,
}

/// Parse `LifecycleExpiration` from XML
pub struct LifecycleExpirationParser;

impl LifecycleExpirationParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<LifecycleExpiration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = LifecycleExpiration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            match current_name.as_ref() {
                "Date" => {
                    obj.date = try!(DateParser::parse_xml("Date", stack));
                    continue;
                },
                "Days" => {
                    obj.days = try!(DaysParser::parse_xml("Days", stack));
                    continue;
                },
                _ => break,
            }
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `LifecycleExpiration` contents to a `SignedRequest`
pub struct LifecycleExpirationWriter;

impl LifecycleExpirationWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &LifecycleExpiration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        DateWriter::write_params(params, &(prefix.to_string() + "Date"), &obj.date);
        DaysWriter::write_params(params, &(prefix.to_string() + "Days"), &obj.days);
    }
}

pub type Prefix = String;
/// Parse `Prefix` from XML
pub struct PrefixParser;

impl PrefixParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Prefix, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Prefix` contents to a `SignedRequest`
pub struct PrefixWriter;

impl PrefixWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Prefix) {
        params.put(name, obj);
    }
}

/// Parse `NoncurrentVersionExpiration` from XML
pub struct NoncurrentVersionExpirationParser;

impl NoncurrentVersionExpirationParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NoncurrentVersionExpiration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = NoncurrentVersionExpiration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "NoncurrentDays" {
                obj.noncurrent_days = try!(DaysParser::parse_xml("NoncurrentDays", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `NoncurrentVersionExpiration` contents to a `SignedRequest`
pub struct NoncurrentVersionExpirationWriter;

impl NoncurrentVersionExpirationWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &NoncurrentVersionExpiration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        DaysWriter::write_params(params, &(prefix.to_string() + "NoncurrentDays"), &obj.noncurrent_days);
    }
}

/// Container for the transition rule that describes when noncurrent objects
/// transition to the GLACIER storage class. If your bucket is versioning-enabled
/// (or versioning is suspended), you can set this action to request that Amazon
/// S3 transition noncurrent object versions to the GLACIER storage class at a
/// specific period in the object's lifetime.
#[derive(Debug, Default)]
pub struct NoncurrentVersionTransition {
    /// Specifies the number of days an object is noncurrent before Amazon S3 can
    /// perform the associated action. For information about the noncurrent days
    /// calculations, see [How Amazon S3 Calculates When an Object Became
    /// Noncurrent](/AmazonS3/latest/dev/s3-access-control.html) in the Amazon Simple
    /// Storage Service Developer Guide.
    pub noncurrent_days: Days,
    /// The class of storage used to store the object.
    pub storage_class: TransitionStorageClass,
}

/// Parse `NoncurrentVersionTransition` from XML
pub struct NoncurrentVersionTransitionParser;

impl NoncurrentVersionTransitionParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NoncurrentVersionTransition, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = NoncurrentVersionTransition::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "NoncurrentDays" {
                obj.noncurrent_days = try!(DaysParser::parse_xml("NoncurrentDays", stack));
                continue;
            }
            if current_name == "StorageClass" {
                obj.storage_class = try!(TransitionStorageClassParser::parse_xml("StorageClass", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `NoncurrentVersionTransition` contents to a `SignedRequest`
pub struct NoncurrentVersionTransitionWriter;

impl NoncurrentVersionTransitionWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &NoncurrentVersionTransition) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        DaysWriter::write_params(params, &(prefix.to_string() + "NoncurrentDays"), &obj.noncurrent_days);
        TransitionStorageClassWriter::write_params(params, &(prefix.to_string() + "StorageClass"), &obj.storage_class);
    }
}

pub type ExpirationStatus = String;
/// Parse `ExpirationStatus` from XML
pub struct ExpirationStatusParser;

impl ExpirationStatusParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ExpirationStatus, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ExpirationStatus` contents to a `SignedRequest`
pub struct ExpirationStatusWriter;

impl ExpirationStatusWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ExpirationStatus) {
        params.put(name, obj);
    }
}

pub type TransitionStorageClass = String;
/// Parse `TransitionStorageClass` from XML
pub struct TransitionStorageClassParser;

impl TransitionStorageClassParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TransitionStorageClass, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `TransitionStorageClass` contents to a `SignedRequest`
pub struct TransitionStorageClassWriter;

impl TransitionStorageClassWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &TransitionStorageClass) {
        params.put(name, obj);
    }
}

pub type ContentLength = i32;
/// Parse `ContentLength` from XML
pub struct ContentLengthParser;

impl ContentLengthParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ContentLength, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ContentLength` contents to a `SignedRequest`
pub struct ContentLengthWriter;

impl ContentLengthWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ContentLength) {
        params.put(name, &obj.to_string());
    }
}

pub type Days = i32;
/// Parse `Days` from XML
pub struct DaysParser;

impl DaysParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Days, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Days` contents to a `SignedRequest`
pub struct DaysWriter;

impl DaysWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Days) {
        params.put(name, &obj.to_string());
    }
}

pub type Date = String;
/// Parse `Date` from XML
pub struct DateParser;

impl DateParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Date, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Date` contents to a `SignedRequest`
pub struct DateWriter;

impl DateWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Date) {
        params.put(name, obj);
    }
}

pub type KeyPrefixEquals = String;
/// Parse `KeyPrefixEquals` from XML
pub struct KeyPrefixEqualsParser;

impl KeyPrefixEqualsParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<KeyPrefixEquals, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `KeyPrefixEquals` contents to a `SignedRequest`
pub struct KeyPrefixEqualsWriter;

impl KeyPrefixEqualsWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &KeyPrefixEquals) {
        params.put(name, obj);
    }
}



pub type CacheControl = String;
/// Parse `CacheControl` from XML
pub struct CacheControlParser;

impl CacheControlParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CacheControl, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `CacheControl` contents to a `SignedRequest`
pub struct CacheControlWriter;

impl CacheControlWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CacheControl) {
        params.put(name, obj);
    }
}

pub type AllowedOrigin = String;
/// Parse `AllowedOrigin` from XML
pub struct AllowedOriginParser;

impl AllowedOriginParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<AllowedOrigin, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `AllowedOrigin` contents to a `SignedRequest`
pub struct AllowedOriginWriter;

impl AllowedOriginWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &AllowedOrigin) {
        params.put(name, obj);
    }
}

pub type IfModifiedSince = String;
/// Parse `IfModifiedSince` from XML
pub struct IfModifiedSinceParser;

impl IfModifiedSinceParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<IfModifiedSince, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `IfModifiedSince` contents to a `SignedRequest`
pub struct IfModifiedSinceWriter;

impl IfModifiedSinceWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &IfModifiedSince) {
        params.put(name, obj);
    }
}

#[derive(Debug, Default)]
pub struct Condition {
    /// The HTTP error code when the redirect is applied. In the event of an error, if
    /// the error code equals this value, then the specified redirect is applied.
    /// Required when parent element Condition is specified and sibling
    /// KeyPrefixEquals is not specified. If both are specified, then both must be
    /// true for the redirect to be applied.
    pub http_error_code_returned_equals: HttpErrorCodeReturnedEquals,
    /// The object key name prefix when the redirect is applied. For example, to
    /// redirect requests for ExamplePage.html, the key prefix will be
    /// ExamplePage.html. To redirect request for all pages with the prefix docs/, the
    /// key prefix will be /docs, which identifies all objects in the docs/ folder.
    /// Required when the parent element Condition is specified and sibling
    /// HttpErrorCodeReturnedEquals is not specified. If both conditions are
    /// specified, both must be true for the redirect to be applied.
    pub key_prefix_equals: KeyPrefixEquals,
}

/// Parse `Condition` from XML
pub struct ConditionParser;

impl ConditionParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Condition, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Condition::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "HttpErrorCodeReturnedEquals" {
                obj.http_error_code_returned_equals = try!(HttpErrorCodeReturnedEqualsParser::parse_xml("HttpErrorCodeReturnedEquals", stack));
                continue;
            }
            if current_name == "KeyPrefixEquals" {
                obj.key_prefix_equals = try!(KeyPrefixEqualsParser::parse_xml("KeyPrefixEquals", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Condition` contents to a `SignedRequest`
pub struct ConditionWriter;

impl ConditionWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Condition) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        HttpErrorCodeReturnedEqualsWriter::write_params(params, &(prefix.to_string() + "HttpErrorCodeReturnedEquals"), &obj.http_error_code_returned_equals);
        KeyPrefixEqualsWriter::write_params(params, &(prefix.to_string() + "KeyPrefixEquals"), &obj.key_prefix_equals);
    }
}

#[derive(Debug, Default)]
pub struct Transition {
    /// Indicates at what date the object is to be moved or deleted. Should be in GMT
    /// ISO 8601 Format.
    pub date: Date,
    /// Indicates the lifetime, in days, of the objects that are subject to the rule.
    /// The value must be a non-zero positive integer.
    pub days: Days,
    /// The class of storage used to store the object.
    pub storage_class: TransitionStorageClass,
}

/// Parse `Transition` from XML
pub struct TransitionParser;

impl TransitionParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Transition, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Transition::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Date" {
                obj.date = try!(DateParser::parse_xml("Date", stack));
                continue;
            }
            if current_name == "Days" {
                obj.days = try!(DaysParser::parse_xml("Days", stack));
                continue;
            }
            if current_name == "StorageClass" {
                obj.storage_class = try!(TransitionStorageClassParser::parse_xml("StorageClass", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Transition` contents to a `SignedRequest`
pub struct TransitionWriter;

impl TransitionWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Transition) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        DateWriter::write_params(params, &(prefix.to_string() + "Date"), &obj.date);
        DaysWriter::write_params(params, &(prefix.to_string() + "Days"), &obj.days);
        TransitionStorageClassWriter::write_params(params, &(prefix.to_string() + "StorageClass"), &obj.storage_class);
    }
}

pub type Payer = String;
/// Parse `Payer` from XML
pub struct PayerParser;

impl PayerParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Payer, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Payer` contents to a `SignedRequest`
pub struct PayerWriter;

impl PayerWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Payer) {
        params.put(name, obj);
    }
}

/// Parse `HeadBucketRequest` from XML
pub struct HeadBucketRequestParser;

impl HeadBucketRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<HeadBucketRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = HeadBucketRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `HeadBucketRequest` contents to a `SignedRequest`
pub struct HeadBucketRequestWriter;

impl HeadBucketRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &HeadBucketRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}


/// Parse `Policy` from XML
pub struct PolicyParser;

impl PolicyParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Policy, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Policy` contents to a `SignedRequest`
pub struct PolicyWriter;

impl PolicyWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Policy) {
        params.put(name, obj);
    }
}

/// Parse `PutBucketRequestPaymentRequest` from XML
pub struct PutBucketRequestPaymentRequestParser;

impl PutBucketRequestPaymentRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<PutBucketRequestPaymentRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = PutBucketRequestPaymentRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "RequestPaymentConfiguration" {
                obj.request_payment_configuration = try!(RequestPaymentConfigurationParser::parse_xml("RequestPaymentConfiguration", stack));
                continue;
            }
            if current_name == "Content-MD5" {
                obj.content_md5 = Some(try!(ContentMD5Parser::parse_xml("Content-MD5", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `PutBucketRequestPaymentRequest` contents to a `SignedRequest`
pub struct PutBucketRequestPaymentRequestWriter;

impl PutBucketRequestPaymentRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &PutBucketRequestPaymentRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        RequestPaymentConfigurationWriter::write_params(params, &(prefix.to_string() + "RequestPaymentConfiguration"), &obj.request_payment_configuration);
        if let Some(ref obj) = obj.content_md5 {
            ContentMD5Writer::write_params(params, &(prefix.to_string() + "Content-MD5"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}

#[derive(Debug, Default)]
pub struct ListBucketsOutput {
    pub owner: Owner,
    pub buckets: Buckets,
}

/// Parse `ListBucketsOutput` from XML
pub struct ListBucketsOutputParser;

impl ListBucketsOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ListBucketsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ListBucketsOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            match current_name.as_ref() {
                "Owner" => {
                    obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
                    continue;
                },
                "Buckets" => {
                    stack.next(); // skip Buckets start and go to contents
                    // this will parse all buckets:
                    obj.buckets = try!(BucketsParser::parse_xml("Bucket", stack));
                },
                _ => break,
            }
        }
        stack.next(); // skip Buckets end
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ListBucketsOutput` contents to a `SignedRequest`
pub struct ListBucketsOutputWriter;
impl ListBucketsOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ListBucketsOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
        BucketsWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.buckets);
    }
}

#[derive(Debug, Default)]
pub struct CreateBucketOutput {
    pub location: Location,
}

/// Parse `CreateBucketOutput` from XML
pub struct CreateBucketOutputParser;

impl CreateBucketOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CreateBucketOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CreateBucketOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Location" {
                obj.location = try!(LocationParser::parse_xml("Location", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `CreateBucketOutput` contents to a `SignedRequest`
pub struct CreateBucketOutputWriter;

impl CreateBucketOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CreateBucketOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LocationWriter::write_params(params, &(prefix.to_string() + "Location"), &obj.location);
    }
}

pub type BucketName = String;
/// Parse `BucketName` from XML
pub struct BucketNameParser;

impl BucketNameParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BucketName, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Parse `Bucket` from XML
pub struct BucketParser;

impl BucketParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Bucket, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Bucket::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "CreationDate" {
                obj.creation_date = try!(CreationDateParser::parse_xml("CreationDate", stack));
                continue;
            }
            if current_name == "Name" {
                obj.name = try!(BucketNameParser::parse_xml("Name", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `BucketName` contents to a `SignedRequest`
pub struct BucketNameWriter;

impl BucketNameWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &BucketName) {
        params.put(name, obj);
    }
}

/// Write `Bucket` contents to a `SignedRequest`
pub struct BucketWriter;

impl BucketWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Bucket) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        CreationDateWriter::write_params(params, &(prefix.to_string() + "CreationDate"), &obj.creation_date);
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Name"), &obj.name);
    }
}

pub type Buckets = Vec<Bucket>;
/// Parse `Buckets` from XML
pub struct BucketsParser;

impl BucketsParser {
    #[allow(unused_variables)]
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Buckets, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "Bucket" {
            obj.push(try!(BucketParser::parse_xml("Bucket", stack)));
        }
        Ok(obj)
    }
}

/// Write `Buckets` contents to a `SignedRequest`
pub struct BucketsWriter;

impl BucketsWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Buckets) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            BucketWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

#[derive(Debug, Default)]
pub struct CreateBucketConfiguration {
    /// Specifies the region where the bucket will be created. If you don't specify a
    /// region, the bucket will be created in US Standard.
    pub location_constraint: BucketLocationConstraint,
}

/// Parse `CopyObjectRequest` from XML
pub struct CopyObjectRequestParser;

impl CopyObjectRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopyObjectRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CopyObjectRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-if-modified-since" {
                obj.copy_source_if_modified_since = Some(try!(CopySourceIfModifiedSinceParser::parse_xml("x-amz-copy-source-if-modified-since", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-if-unmodified-since" {
                obj.copy_source_if_unmodified_since = Some(try!(CopySourceIfUnmodifiedSinceParser::parse_xml("x-amz-copy-source-if-unmodified-since", stack)));
                continue;
            }
            if current_name == "Content-Encoding" {
                obj.content_encoding = Some(try!(ContentEncodingParser::parse_xml("Content-Encoding", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-server-side-encryption-customer-key" {
                obj.copy_source_sse_customer_key = Some(try!(CopySourceSSECustomerKeyParser::parse_xml("x-amz-copy-source-server-side-encryption-customer-key", stack)));
                continue;
            }
            if current_name == "x-amz-storage-class" {
                obj.storage_class = Some(try!(StorageClassParser::parse_xml("x-amz-storage-class", stack)));
                continue;
            }
            if current_name == "x-amz-grant-read-acp" {
                obj.grant_read_acp = Some(try!(GrantReadACPParser::parse_xml("x-amz-grant-read-acp", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = Some(try!(ServerSideEncryptionParser::parse_xml("x-amz-server-side-encryption", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = Some(try!(SSEKMSKeyIdParser::parse_xml("x-amz-server-side-encryption-aws-kms-key-id", stack)));
                continue;
            }
            if current_name == "Content-Disposition" {
                obj.content_disposition = Some(try!(ContentDispositionParser::parse_xml("Content-Disposition", stack)));
                continue;
            }
            if current_name == "x-amz-meta-" {
                obj.metadata = Some(try!(MetadataParser::parse_xml("x-amz-meta-", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key" {
                obj.sse_customer_key = Some(try!(SSECustomerKeyParser::parse_xml("x-amz-server-side-encryption-customer-key", stack)));
                continue;
            }
            if current_name == "x-amz-website-redirect-location" {
                obj.website_redirect_location = Some(try!(WebsiteRedirectLocationParser::parse_xml("x-amz-website-redirect-location", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source" {
                obj.copy_source = try!(CopySourceParser::parse_xml("x-amz-copy-source", stack));
                continue;
            }
            if current_name == "Expires" {
                obj.expires = Some(try!(ExpiresParser::parse_xml("Expires", stack)));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "Cache-Control" {
                obj.cache_control = Some(try!(CacheControlParser::parse_xml("Cache-Control", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-server-side-encryption-customer-algorithm" {
                obj.copy_source_sse_customer_algorithm = Some(try!(CopySourceSSECustomerAlgorithmParser::parse_xml("x-amz-copy-source-server-side-encryption-customer-algorithm", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-grant-read" {
                obj.grant_read = Some(try!(GrantReadParser::parse_xml("x-amz-grant-read", stack)));
                continue;
            }
            if current_name == "x-amz-grant-write-acp" {
                obj.grant_write_acp = Some(try!(GrantWriteACPParser::parse_xml("x-amz-grant-write-acp", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-server-side-encryption-customer-key-MD5" {
                obj.copy_source_sse_customer_key_md5 = Some(try!(CopySourceSSECustomerKeyMD5Parser::parse_xml("x-amz-copy-source-server-side-encryption-customer-key-MD5", stack)));
                continue;
            }
            if current_name == "x-amz-grant-full-control" {
                obj.grant_full_control = Some(try!(GrantFullControlParser::parse_xml("x-amz-grant-full-control", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-if-match" {
                obj.copy_source_if_match = Some(try!(CopySourceIfMatchParser::parse_xml("x-amz-copy-source-if-match", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = Some(try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack)));
                continue;
            }
            if current_name == "Content-Type" {
                obj.content_type = Some(try!(ContentTypeParser::parse_xml("Content-Type", stack)));
                continue;
            }
            if current_name == "Content-Language" {
                obj.content_language = Some(try!(ContentLanguageParser::parse_xml("Content-Language", stack)));
                continue;
            }
            if current_name == "x-amz-metadata-directive" {
                obj.metadata_directive = Some(try!(MetadataDirectiveParser::parse_xml("x-amz-metadata-directive", stack)));
                continue;
            }
            if current_name == "x-amz-copy-source-if-none-match" {
                obj.copy_source_if_none_match = Some(try!(CopySourceIfNoneMatchParser::parse_xml("x-amz-copy-source-if-none-match", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = Some(try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `CopyObjectRequest` contents to a `SignedRequest`
pub struct CopyObjectRequestWriter;

impl CopyObjectRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CopyObjectRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        if let Some(ref obj) = obj.copy_source_if_modified_since {
            CopySourceIfModifiedSinceWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-if-modified-since"), obj);
        }
        if let Some(ref obj) = obj.copy_source_if_unmodified_since {
            CopySourceIfUnmodifiedSinceWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-if-unmodified-since"), obj);
        }
        if let Some(ref obj) = obj.content_encoding {
            ContentEncodingWriter::write_params(params, &(prefix.to_string() + "Content-Encoding"), obj);
        }
        if let Some(ref obj) = obj.copy_source_sse_customer_key {
            CopySourceSSECustomerKeyWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-server-side-encryption-customer-key"), obj);
        }
        if let Some(ref obj) = obj.storage_class {
            StorageClassWriter::write_params(params, &(prefix.to_string() + "x-amz-storage-class"), obj);
        }
        if let Some(ref obj) = obj.grant_read_acp {
            GrantReadACPWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-read-acp"), obj);
        }
        if let Some(ref obj) = obj.server_side_encryption {
            ServerSideEncryptionWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption"), obj);
        }
        if let Some(ref obj) = obj.ssekms_key_id {
            SSEKMSKeyIdWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"), obj);
        }
        if let Some(ref obj) = obj.content_disposition {
            ContentDispositionWriter::write_params(params, &(prefix.to_string() + "Content-Disposition"), obj);
        }
        if let Some(ref obj) = obj.metadata {
            MetadataWriter::write_params(params, &(prefix.to_string() + "x-amz-meta-"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_key {
            SSECustomerKeyWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key"), obj);
        }
        if let Some(ref obj) = obj.website_redirect_location {
            WebsiteRedirectLocationWriter::write_params(params, &(prefix.to_string() + "x-amz-website-redirect-location"), obj);
        }
        CopySourceWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source"), &obj.copy_source);
        if let Some(ref obj) = obj.expires {
            ExpiresWriter::write_params(params, &(prefix.to_string() + "Expires"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        if let Some(ref obj) = obj.cache_control {
            CacheControlWriter::write_params(params, &(prefix.to_string() + "Cache-Control"), obj);
        }
        if let Some(ref obj) = obj.copy_source_sse_customer_algorithm {
            CopySourceSSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-server-side-encryption-customer-algorithm"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.grant_read {
            GrantReadWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-read"), obj);
        }
        if let Some(ref obj) = obj.grant_write_acp {
            GrantWriteACPWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-write-acp"), obj);
        }
        if let Some(ref obj) = obj.copy_source_sse_customer_key_md5 {
            CopySourceSSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-copy-source-server-side-encryption-customer-key-MD5"), obj);
        }
        if let Some(ref obj) = obj.grant_full_control {
            GrantFullControlWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-full-control"), obj);
        }
        if let Some(ref obj) = obj.copy_source_if_match {
            CopySourceIfMatchWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-if-match"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_algorithm {
            SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), obj);
        }
        if let Some(ref obj) = obj.content_type {
            ContentTypeWriter::write_params(params, &(prefix.to_string() + "Content-Type"), obj);
        }
        if let Some(ref obj) = obj.content_language {
            ContentLanguageWriter::write_params(params, &(prefix.to_string() + "Content-Language"), obj);
        }
        if let Some(ref obj) = obj.metadata_directive {
            MetadataDirectiveWriter::write_params(params, &(prefix.to_string() + "x-amz-metadata-directive"), obj);
        }
        if let Some(ref obj) = obj.copy_source_if_none_match {
            CopySourceIfNoneMatchWriter::write_params(params, &(prefix.to_string() + "x-amz-copy-source-if-none-match"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_key_md5 {
            SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), obj);
        }
    }
}

/// Parse `DeleteObjectsOutput` from XML
pub struct DeleteObjectsOutputParser;

impl DeleteObjectsOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteObjectsOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteObjectsOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "DeletedObject" {
                obj.deleted = try!(DeletedObjectsParser::parse_xml("DeletedObject", stack));
                continue;
            }
            if current_name == "Error" {
                obj.errors = try!(ErrorsParser::parse_xml("Error", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `DeleteObjectsOutput` contents to a `SignedRequest`
pub struct DeleteObjectsOutputWriter;

impl DeleteObjectsOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeleteObjectsOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        DeletedObjectsWriter::write_params(params, &(prefix.to_string() + "DeletedObject"), &obj.deleted);
        ErrorsWriter::write_params(params, &(prefix.to_string() + "Error"), &obj.errors);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
    }
}

/// Parse `GetObjectOutput` from XML
pub struct GetObjectOutputParser;

impl GetObjectOutputParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetObjectOutput, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetObjectOutput::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Last-Modified" {
                obj.last_modified = try!(LastModifiedParser::parse_xml("Last-Modified", stack));
                continue;
            }
            if current_name == "Content-Range" {
                obj.content_range = try!(ContentRangeParser::parse_xml("Content-Range", stack));
                continue;
            }
            if current_name == "x-amz-request-charged" {
                obj.request_charged = try!(RequestChargedParser::parse_xml("x-amz-request-charged", stack));
                continue;
            }
            if current_name == "Content-Encoding" {
                obj.content_encoding = try!(ContentEncodingParser::parse_xml("Content-Encoding", stack));
                continue;
            }
            if current_name == "x-amz-replication-status" {
                obj.replication_status = try!(ReplicationStatusParser::parse_xml("x-amz-replication-status", stack));
                continue;
            }
            if current_name == "x-amz-storage-class" {
                obj.storage_class = try!(StorageClassParser::parse_xml("x-amz-storage-class", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = try!(ServerSideEncryptionParser::parse_xml("x-amz-server-side-encryption", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = try!(SSEKMSKeyIdParser::parse_xml("x-amz-server-side-encryption-aws-kms-key-id", stack));
                continue;
            }
            if current_name == "Content-Disposition" {
                obj.content_disposition = try!(ContentDispositionParser::parse_xml("Content-Disposition", stack));
                continue;
            }
            if current_name == "x-amz-meta-" {
                obj.metadata = try!(MetadataParser::parse_xml("x-amz-meta-", stack));
                continue;
            }
            if current_name == "Body" {
                obj.body = try!(BodyParser::parse_xml("Body", stack));
                continue;
            }
            if current_name == "accept-ranges" {
                obj.accept_ranges = try!(AcceptRangesParser::parse_xml("accept-ranges", stack));
                continue;
            }
            if current_name == "x-amz-website-redirect-location" {
                obj.website_redirect_location = try!(WebsiteRedirectLocationParser::parse_xml("x-amz-website-redirect-location", stack));
                continue;
            }
            if current_name == "Expires" {
                obj.expires = try!(ExpiresParser::parse_xml("Expires", stack));
                continue;
            }
            if current_name == "x-amz-delete-marker" {
                obj.delete_marker = try!(DeleteMarkerParser::parse_xml("x-amz-delete-marker", stack));
                continue;
            }
            if current_name == "Cache-Control" {
                obj.cache_control = try!(CacheControlParser::parse_xml("Cache-Control", stack));
                continue;
            }
            if current_name == "Content-Length" {
                obj.content_length = try!(ContentLengthParser::parse_xml("Content-Length", stack));
                continue;
            }
            if current_name == "x-amz-expiration" {
                obj.expiration = try!(ExpirationParser::parse_xml("x-amz-expiration", stack));
                continue;
            }
            if current_name == "x-amz-missing-meta" {
                obj.missing_meta = try!(MissingMetaParser::parse_xml("x-amz-missing-meta", stack));
                continue;
            }
            if current_name == "x-amz-restore" {
                obj.restore = try!(RestoreParser::parse_xml("x-amz-restore", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack));
                continue;
            }
            if current_name == "Content-Type" {
                obj.content_type = try!(ContentTypeParser::parse_xml("Content-Type", stack));
                continue;
            }
            if current_name == "Content-Language" {
                obj.content_language = try!(ContentLanguageParser::parse_xml("Content-Language", stack));
                continue;
            }
            if current_name == "x-amz-version-id" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("x-amz-version-id", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `GetObjectOutput` contents to a `SignedRequest`
pub struct GetObjectOutputWriter;

impl GetObjectOutputWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GetObjectOutput) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LastModifiedWriter::write_params(params, &(prefix.to_string() + "Last-Modified"), &obj.last_modified);
        ContentRangeWriter::write_params(params, &(prefix.to_string() + "Content-Range"), &obj.content_range);
        RequestChargedWriter::write_params(params, &(prefix.to_string() + "x-amz-request-charged"), &obj.request_charged);
        ContentEncodingWriter::write_params(params, &(prefix.to_string() + "Content-Encoding"), &obj.content_encoding);
        ReplicationStatusWriter::write_params(params, &(prefix.to_string() + "x-amz-replication-status"), &obj.replication_status);
        StorageClassWriter::write_params(params, &(prefix.to_string() + "x-amz-storage-class"), &obj.storage_class);
        ServerSideEncryptionWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption"), &obj.server_side_encryption);
        SSEKMSKeyIdWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"), &obj.ssekms_key_id);
        ContentDispositionWriter::write_params(params, &(prefix.to_string() + "Content-Disposition"), &obj.content_disposition);
        MetadataWriter::write_params(params, &(prefix.to_string() + "x-amz-meta-"), &obj.metadata);
        BodyWriter::write_params(params, &(prefix.to_string() + "Body"), &obj.body);
        AcceptRangesWriter::write_params(params, &(prefix.to_string() + "accept-ranges"), &obj.accept_ranges);
        WebsiteRedirectLocationWriter::write_params(params, &(prefix.to_string() + "x-amz-website-redirect-location"), &obj.website_redirect_location);
        ExpiresWriter::write_params(params, &(prefix.to_string() + "Expires"), &obj.expires);
        DeleteMarkerWriter::write_params(params, &(prefix.to_string() + "x-amz-delete-marker"), &obj.delete_marker);
        CacheControlWriter::write_params(params, &(prefix.to_string() + "Cache-Control"), &obj.cache_control);
        ContentLengthWriter::write_params(params, &(prefix.to_string() + "Content-Length"), &obj.content_length);
        ExpirationWriter::write_params(params, &(prefix.to_string() + "x-amz-expiration"), &obj.expiration);
        MissingMetaWriter::write_params(params, &(prefix.to_string() + "x-amz-missing-meta"), &obj.missing_meta);
        RestoreWriter::write_params(params, &(prefix.to_string() + "x-amz-restore"), &obj.restore);
        SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), &obj.sse_customer_algorithm);
        ContentTypeWriter::write_params(params, &(prefix.to_string() + "Content-Type"), &obj.content_type);
        ContentLanguageWriter::write_params(params, &(prefix.to_string() + "Content-Language"), &obj.content_language);
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "x-amz-version-id"), &obj.version_id);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
        SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), &obj.sse_customer_key_md5);
    }
}

/// Parse `CreateMultipartUploadRequest` from XML
pub struct CreateMultipartUploadRequestParser;

impl CreateMultipartUploadRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CreateMultipartUploadRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CreateMultipartUploadRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "Content-Encoding" {
                obj.content_encoding = Some(try!(ContentEncodingParser::parse_xml("Content-Encoding", stack)));
                continue;
            }
            if current_name == "x-amz-storage-class" {
                obj.storage_class = Some(try!(StorageClassParser::parse_xml("x-amz-storage-class", stack)));
                continue;
            }
            if current_name == "x-amz-grant-read-acp" {
                obj.grant_read_acp = Some(try!(GrantReadACPParser::parse_xml("x-amz-grant-read-acp", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption" {
                obj.server_side_encryption = Some(try!(ServerSideEncryptionParser::parse_xml("x-amz-server-side-encryption", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-aws-kms-key-id" {
                obj.ssekms_key_id = Some(try!(SSEKMSKeyIdParser::parse_xml("x-amz-server-side-encryption-aws-kms-key-id", stack)));
                continue;
            }
            if current_name == "Content-Disposition" {
                obj.content_disposition = Some(try!(ContentDispositionParser::parse_xml("Content-Disposition", stack)));
                continue;
            }
            if current_name == "x-amz-meta-" {
                obj.metadata = Some(try!(MetadataParser::parse_xml("x-amz-meta-", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key" {
                obj.sse_customer_key = Some(try!(SSECustomerKeyParser::parse_xml("x-amz-server-side-encryption-customer-key", stack)));
                continue;
            }
            if current_name == "x-amz-website-redirect-location" {
                obj.website_redirect_location = Some(try!(WebsiteRedirectLocationParser::parse_xml("x-amz-website-redirect-location", stack)));
                continue;
            }
            if current_name == "Expires" {
                obj.expires = Some(try!(ExpiresParser::parse_xml("Expires", stack)));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "Cache-Control" {
                obj.cache_control = Some(try!(CacheControlParser::parse_xml("Cache-Control", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-grant-read" {
                obj.grant_read = Some(try!(GrantReadParser::parse_xml("x-amz-grant-read", stack)));
                continue;
            }
            if current_name == "x-amz-grant-write-acp" {
                obj.grant_write_acp = Some(try!(GrantWriteACPParser::parse_xml("x-amz-grant-write-acp", stack)));
                continue;
            }
            if current_name == "x-amz-acl" {
                obj.acl = Some(try!(ObjectCannedACLParser::parse_xml("x-amz-acl", stack)));
                continue;
            }
            if current_name == "x-amz-grant-full-control" {
                obj.grant_full_control = Some(try!(GrantFullControlParser::parse_xml("x-amz-grant-full-control", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-algorithm" {
                obj.sse_customer_algorithm = Some(try!(SSECustomerAlgorithmParser::parse_xml("x-amz-server-side-encryption-customer-algorithm", stack)));
                continue;
            }
            if current_name == "Content-Type" {
                obj.content_type = Some(try!(ContentTypeParser::parse_xml("Content-Type", stack)));
                continue;
            }
            if current_name == "Content-Language" {
                obj.content_language = Some(try!(ContentLanguageParser::parse_xml("Content-Language", stack)));
                continue;
            }
            if current_name == "x-amz-server-side-encryption-customer-key-MD5" {
                obj.sse_customer_key_md5 = Some(try!(SSECustomerKeyMD5Parser::parse_xml("x-amz-server-side-encryption-customer-key-MD5", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `CreateMultipartUploadRequest` contents to a `SignedRequest`
pub struct CreateMultipartUploadRequestWriter;

impl CreateMultipartUploadRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CreateMultipartUploadRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        if let Some(ref obj) = obj.content_encoding {
            ContentEncodingWriter::write_params(params, &(prefix.to_string() + "Content-Encoding"), obj);
        }
        if let Some(ref obj) = obj.storage_class {
            StorageClassWriter::write_params(params, &(prefix.to_string() + "x-amz-storage-class"), obj);
        }
        if let Some(ref obj) = obj.grant_read_acp {
            GrantReadACPWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-read-acp"), obj);
        }
        if let Some(ref obj) = obj.server_side_encryption {
            ServerSideEncryptionWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption"), obj);
        }
        if let Some(ref obj) = obj.ssekms_key_id {
            SSEKMSKeyIdWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-aws-kms-key-id"), obj);
        }
        if let Some(ref obj) = obj.content_disposition {
            ContentDispositionWriter::write_params(params, &(prefix.to_string() + "Content-Disposition"), obj);
        }
        if let Some(ref obj) = obj.metadata {
            MetadataWriter::write_params(params, &(prefix.to_string() + "x-amz-meta-"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_key {
            SSECustomerKeyWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key"), obj);
        }
        if let Some(ref obj) = obj.website_redirect_location {
            WebsiteRedirectLocationWriter::write_params(params, &(prefix.to_string() + "x-amz-website-redirect-location"), obj);
        }
        if let Some(ref obj) = obj.expires {
            ExpiresWriter::write_params(params, &(prefix.to_string() + "Expires"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        if let Some(ref obj) = obj.cache_control {
            CacheControlWriter::write_params(params, &(prefix.to_string() + "Cache-Control"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.grant_read {
            GrantReadWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-read"), obj);
        }
        if let Some(ref obj) = obj.grant_write_acp {
            GrantWriteACPWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-write-acp"), obj);
        }
        if let Some(ref obj) = obj.acl {
            ObjectCannedACLWriter::write_params(params, &(prefix.to_string() + "x-amz-acl"), obj);
        }
        if let Some(ref obj) = obj.grant_full_control {
            GrantFullControlWriter::write_params(params, &(prefix.to_string() + "x-amz-grant-full-control"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_algorithm {
            SSECustomerAlgorithmWriter::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-algorithm"), obj);
        }
        if let Some(ref obj) = obj.content_type {
            ContentTypeWriter::write_params(params, &(prefix.to_string() + "Content-Type"), obj);
        }
        if let Some(ref obj) = obj.content_language {
            ContentLanguageWriter::write_params(params, &(prefix.to_string() + "Content-Language"), obj);
        }
        if let Some(ref obj) = obj.sse_customer_key_md5 {
            SSECustomerKeyMD5Writer::write_params(params, &(prefix.to_string() + "x-amz-server-side-encryption-customer-key-MD5"), obj);
        }
    }
}

/// Parse `DeleteBucketTaggingRequest` from XML
pub struct DeleteBucketTaggingRequestParser;

impl DeleteBucketTaggingRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteBucketTaggingRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteBucketTaggingRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `DeleteBucketTaggingRequest` contents to a `SignedRequest`
pub struct DeleteBucketTaggingRequestWriter;

impl DeleteBucketTaggingRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeleteBucketTaggingRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}

/// Parse `DeleteBucketWebsiteRequest` from XML
pub struct DeleteBucketWebsiteRequestParser;

impl DeleteBucketWebsiteRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteBucketWebsiteRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteBucketWebsiteRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `DeleteBucketWebsiteRequest` contents to a `SignedRequest`
pub struct DeleteBucketWebsiteRequestWriter;

impl DeleteBucketWebsiteRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeleteBucketWebsiteRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}

/// Parse `GetBucketNotificationConfigurationRequest` from XML
pub struct GetBucketNotificationConfigurationRequestParser;

impl GetBucketNotificationConfigurationRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<GetBucketNotificationConfigurationRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = GetBucketNotificationConfigurationRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `GetBucketNotificationConfigurationRequest` contents to a `SignedRequest`
pub struct GetBucketNotificationConfigurationRequestWriter;

impl GetBucketNotificationConfigurationRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &GetBucketNotificationConfigurationRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}

pub type CopySourceIfModifiedSince = String;
/// Parse `CopySourceIfModifiedSince` from XML
pub struct CopySourceIfModifiedSinceParser;

impl CopySourceIfModifiedSinceParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopySourceIfModifiedSince, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `CopySourceIfModifiedSince` contents to a `SignedRequest`
pub struct CopySourceIfModifiedSinceWriter;

impl CopySourceIfModifiedSinceWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CopySourceIfModifiedSince) {
        params.put(name, obj);
    }
}

pub type CopySourceSSECustomerKey = String;
/// Parse `CopySourceSSECustomerKey` from XML
pub struct CopySourceSSECustomerKeyParser;

impl CopySourceSSECustomerKeyParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopySourceSSECustomerKey, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `CopySourceSSECustomerKey` contents to a `SignedRequest`
pub struct CopySourceSSECustomerKeyWriter;

impl CopySourceSSECustomerKeyWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CopySourceSSECustomerKey) {
        params.put(name, obj);
    }
}

pub type CopySource = String;
/// Parse `CopySource` from XML
pub struct CopySourceParser;

impl CopySourceParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopySource, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `CopySource` contents to a `SignedRequest`
pub struct CopySourceWriter;

impl CopySourceWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CopySource) {
        params.put(name, obj);
    }
}

pub type CopySourceIfUnmodifiedSince = String;
/// Parse `CopySourceIfUnmodifiedSince` from XML
pub struct CopySourceIfUnmodifiedSinceParser;

impl CopySourceIfUnmodifiedSinceParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopySourceIfUnmodifiedSince, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `CopySourceIfUnmodifiedSince` contents to a `SignedRequest`
pub struct CopySourceIfUnmodifiedSinceWriter;

impl CopySourceIfUnmodifiedSinceWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CopySourceIfUnmodifiedSince) {
        params.put(name, obj);
    }
}

pub type CopySourceSSECustomerKeyMD5 = String;
/// Parse `CopySourceSSECustomerKeyMD`5 from XML
pub struct CopySourceSSECustomerKeyMD5Parser;

impl CopySourceSSECustomerKeyMD5Parser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopySourceSSECustomerKeyMD5, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `CopySourceSSECustomerKeyMD5` contents to a `SignedRequest`
pub struct CopySourceSSECustomerKeyMD5Writer;

impl CopySourceSSECustomerKeyMD5Writer {
    pub fn write_params(params: &mut Params, name: &str, obj: &CopySourceSSECustomerKeyMD5) {
        params.put(name, obj);
    }
}

pub type CopySourceIfMatch = String;
/// Parse `CopySourceIfMatch` from XML
pub struct CopySourceIfMatchParser;

impl CopySourceIfMatchParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopySourceIfMatch, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `CopySourceIfMatch` contents to a `SignedRequest`
pub struct CopySourceIfMatchWriter;

impl CopySourceIfMatchWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CopySourceIfMatch) {
        params.put(name, obj);
    }
}

pub type CopySourceIfNoneMatch = String;
/// Parse `CopySourceIfNoneMatch` from XML
pub struct CopySourceIfNoneMatchParser;

impl CopySourceIfNoneMatchParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopySourceIfNoneMatch, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `CopySourceIfNoneMatch` contents to a `SignedRequest`
pub struct CopySourceIfNoneMatchWriter;

impl CopySourceIfNoneMatchWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CopySourceIfNoneMatch) {
        params.put(name, obj);
    }
}

/// Parse `ObjectIdentifier` from XML
pub struct ObjectIdentifierParser;

impl ObjectIdentifierParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectIdentifier, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ObjectIdentifier::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "VersionId" {
                obj.version_id = Some(try!(ObjectVersionIdParser::parse_xml("VersionId", stack)));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ObjectIdentifier` contents to a `SignedRequest`
pub struct ObjectIdentifierWriter;

impl ObjectIdentifierWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectIdentifier) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.version_id {
            ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "VersionId"), obj);
        }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
    }
}

/// Parse `QueueConfiguration` from XML
pub struct QueueConfigurationParser;

impl QueueConfigurationParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<QueueConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = QueueConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Id" {
                obj.id = Some(try!(NotificationIdParser::parse_xml("Id", stack)));
                continue;
            }
            if current_name == "Queue" {
                obj.queue_arn = try!(QueueArnParser::parse_xml("Queue", stack));
                continue;
            }
            if current_name == "Event" {
                obj.events = try!(EventListParser::parse_xml("Event", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `QueueConfiguration` contents to a `SignedRequest`
pub struct QueueConfigurationWriter;

impl QueueConfigurationWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &QueueConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.id {
            NotificationIdWriter::write_params(params, &(prefix.to_string() + "Id"), obj);
        }
        QueueArnWriter::write_params(params, &(prefix.to_string() + "Queue"), &obj.queue_arn);
        EventListWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.events);
    }
}

/// Parse `TopicConfiguration` from XML
pub struct TopicConfigurationParser;

impl TopicConfigurationParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TopicConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = TopicConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Id" {
                obj.id = Some(try!(NotificationIdParser::parse_xml("Id", stack)));
                continue;
            }
            if current_name == "Topic" {
                obj.topic_arn = try!(TopicArnParser::parse_xml("Topic", stack));
                continue;
            }
            if current_name == "Event" {
                obj.events = try!(EventListParser::parse_xml("Event", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `TopicConfiguration` contents to a `SignedRequest`
pub struct TopicConfigurationWriter;

impl TopicConfigurationWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &TopicConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.id {
            NotificationIdWriter::write_params(params, &(prefix.to_string() + "Id"), obj);
        }
        TopicArnWriter::write_params(params, &(prefix.to_string() + "Topic"), &obj.topic_arn);
        EventListWriter::write_params(params, &(prefix.to_string() + "Event"), &obj.events);
    }
}

pub type ObjectIdentifierList = Vec<ObjectIdentifier>;
/// Parse `ObjectIdentifierList` from XML
pub struct ObjectIdentifierListParser;

impl ObjectIdentifierListParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectIdentifierList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "ObjectIdentifier" {
            obj.push(try!(ObjectIdentifierParser::parse_xml("ObjectIdentifier", stack)));
        }
        Ok(obj)
    }
}

/// Write `ObjectIdentifierList` contents to a `SignedRequest`
pub struct ObjectIdentifierListWriter;

impl ObjectIdentifierListWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectIdentifierList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            ObjectIdentifierWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

pub type Quiet = bool;
/// Parse `Quiet` from XML
pub struct QuietParser;

impl QuietParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Quiet, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Quiet` contents to a `SignedRequest`
pub struct QuietWriter;

impl QuietWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Quiet) {
        params.put(name, &obj.to_string());
    }
}

pub type QueueConfigurationList = Vec<QueueConfiguration>;
/// Parse `QueueConfigurationList` from XML
pub struct QueueConfigurationListParser;

impl QueueConfigurationListParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<QueueConfigurationList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "QueueConfiguration" {
            obj.push(try!(QueueConfigurationParser::parse_xml("QueueConfiguration", stack)));
        }
        Ok(obj)
    }
}

/// Write `QueueConfigurationList` contents to a `SignedRequest`
pub struct QueueConfigurationListWriter;

impl QueueConfigurationListWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &QueueConfigurationList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            QueueConfigurationWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

pub type TopicConfigurationList = Vec<TopicConfiguration>;
/// Parse `TopicConfigurationList` from XML
pub struct TopicConfigurationListParser;

impl TopicConfigurationListParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<TopicConfigurationList, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "TopicConfiguration" {
            obj.push(try!(TopicConfigurationParser::parse_xml("TopicConfiguration", stack)));
        }
        Ok(obj)
    }
}

/// Write `TopicConfigurationList` contents to a `SignedRequest`
pub struct TopicConfigurationListWriter;

impl TopicConfigurationListWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &TopicConfigurationList) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            TopicConfigurationWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

/// Parse `Delete` from XML
pub struct DeleteParser;

impl DeleteParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Delete, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Delete::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "ObjectIdentifier" {
                obj.objects = try!(ObjectIdentifierListParser::parse_xml("ObjectIdentifier", stack));
                continue;
            }
            if current_name == "Quiet" {
                obj.quiet = Some(try!(QuietParser::parse_xml("Quiet", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Delete` contents to a `SignedRequest`
pub struct DeleteWriter;

impl DeleteWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Delete) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ObjectIdentifierListWriter::write_params(params, &(prefix.to_string() + "ObjectIdentifier"), &obj.objects);
        if let Some(ref obj) = obj.quiet {
            QuietWriter::write_params(params, &(prefix.to_string() + "Quiet"), obj);
        }
    }
}


/// Parse `NotificationConfiguration` from XML
pub struct NotificationConfigurationParser;

impl NotificationConfigurationParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NotificationConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = NotificationConfiguration::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "QueueConfiguration" {
                obj.queue_configurations = try!(QueueConfigurationListParser::parse_xml("QueueConfiguration", stack));
                continue;
            }
            if current_name == "LambdaFunctionConfiguration" {
                obj.lambda_function_configurations = try!(LambdaFunctionConfigurationListParser::parse_xml("LambdaFunctionConfiguration", stack));
                continue;
            }
            if current_name == "TopicConfiguration" {
                obj.topic_configurations = try!(TopicConfigurationListParser::parse_xml("TopicConfiguration", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `NotificationConfiguration` contents to a `SignedRequest`
pub struct NotificationConfigurationWriter;

impl NotificationConfigurationWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &NotificationConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        QueueConfigurationListWriter::write_params(params, &(prefix.to_string() + "QueueConfiguration"), &obj.queue_configurations);
        LambdaFunctionConfigurationListWriter::write_params(params, &(prefix.to_string() + "LambdaFunctionConfiguration"), &obj.lambda_function_configurations);
        TopicConfigurationListWriter::write_params(params, &(prefix.to_string() + "TopicConfiguration"), &obj.topic_configurations);
    }
}

/// Parse `DeleteObjectsRequest` from XML
pub struct DeleteObjectsRequestParser;

impl DeleteObjectsRequestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<DeleteObjectsRequest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = DeleteObjectsRequest::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "x-amz-mfa" {
                obj.mfa = Some(try!(MFAParser::parse_xml("x-amz-mfa", stack)));
                continue;
            }
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            if current_name == "x-amz-request-payer" {
                obj.request_payer = Some(try!(RequestPayerParser::parse_xml("x-amz-request-payer", stack)));
                continue;
            }
            if current_name == "Delete" {
                obj.delete = try!(DeleteParser::parse_xml("Delete", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `DeleteObjectsRequest` contents to a `SignedRequest`
pub struct DeleteObjectsRequestWriter;

impl DeleteObjectsRequestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &DeleteObjectsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        if let Some(ref obj) = obj.mfa {
            MFAWriter::write_params(params, &(prefix.to_string() + "x-amz-mfa"), obj);
        }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
        if let Some(ref obj) = obj.request_payer {
            RequestPayerWriter::write_params(params, &(prefix.to_string() + "x-amz-request-payer"), obj);
        }
        DeleteWriter::write_params(params, &(prefix.to_string() + "Delete"), &obj.delete);
    }
}

/// The requested bucket name is not available. The bucket namespace is shared by
/// all users of the system. Please select a different name and try again.
#[derive(Debug, Default)]
pub struct BucketAlreadyExists;

/// Parse `BucketAlreadyExists` from XML
pub struct BucketAlreadyExistsParser;

impl BucketAlreadyExistsParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BucketAlreadyExists, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = BucketAlreadyExists::default();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `BucketAlreadyExists` contents to a `SignedRequest`
pub struct BucketAlreadyExistsWriter;

impl BucketAlreadyExistsWriter {
    #[allow(unused_variables)]
    pub fn write_params(params: &mut Params, name: &str, obj: &BucketAlreadyExists) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
    }
}

pub type BucketLocationConstraint = String;
/// Parse `BucketLocationConstraint` from XML
pub struct BucketLocationConstraintParser;

impl BucketLocationConstraintParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BucketLocationConstraint, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `BucketLocationConstraint` contents to a `SignedRequest`
pub struct BucketLocationConstraintWriter;

impl BucketLocationConstraintWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &BucketLocationConstraint) {
        params.put(name, obj);
    }
}

pub type MFADeleteStatus = String;
/// Parse `MFADeleteStatus` from XML
pub struct MFADeleteStatusParser;

impl MFADeleteStatusParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<MFADeleteStatus, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `MFADeleteStatus` contents to a `SignedRequest`
pub struct MFADeleteStatusWriter;

impl MFADeleteStatusWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &MFADeleteStatus) {
        params.put(name, obj);
    }
}

pub type Location = String;
/// Parse `Location` from XML
pub struct LocationParser;

impl LocationParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Location, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Location` contents to a `SignedRequest`
pub struct LocationWriter;

impl LocationWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Location) {
        params.put(name, obj);
    }
}

pub type S3ClientMessage = String;
/// Parse `S`3ClientMessage from XML
pub struct S3ClientMessageParser;

impl S3ClientMessageParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<S3ClientMessage, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `S3ClientMessage` contents to a `SignedRequest`
pub struct S3ClientMessageWriter;

impl S3ClientMessageWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &S3ClientMessage) {
        params.put(name, obj);
    }
}






#[derive(Debug, Default)]
pub struct ObjectVersion {
    /// Date and time the object was last modified.
    pub last_modified: LastModified,
    /// Version ID of an object.
    pub version_id: ObjectVersionId,
    pub e_tag: ETag,
    /// The class of storage used to store the object.
    pub storage_class: ObjectVersionStorageClass,
    /// The object key.
    pub key: ObjectKey,
    pub owner: Owner,
    /// Specifies whether the object is (true) or is not (false) the latest version of
    /// an object.
    pub is_latest: IsLatest,
    /// Size in bytes of the object.
    pub size: Size,
}

/// Parse `ObjectVersion` from XML
pub struct ObjectVersionParser;

impl ObjectVersionParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectVersion, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ObjectVersion::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "LastModified" {
                obj.last_modified = try!(LastModifiedParser::parse_xml("LastModified", stack));
                continue;
            }
            if current_name == "VersionId" {
                obj.version_id = try!(ObjectVersionIdParser::parse_xml("VersionId", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            if current_name == "StorageClass" {
                obj.storage_class = try!(ObjectVersionStorageClassParser::parse_xml("StorageClass", stack));
                continue;
            }
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            if current_name == "Owner" {
                obj.owner = try!(OwnerParser::parse_xml("Owner", stack));
                continue;
            }
            if current_name == "IsLatest" {
                obj.is_latest = try!(IsLatestParser::parse_xml("IsLatest", stack));
                continue;
            }
            if current_name == "Size" {
                obj.size = try!(SizeParser::parse_xml("Size", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ObjectVersion` contents to a `SignedRequest`
pub struct ObjectVersionWriter;

impl ObjectVersionWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectVersion) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LastModifiedWriter::write_params(params, &(prefix.to_string() + "LastModified"), &obj.last_modified);
        ObjectVersionIdWriter::write_params(params, &(prefix.to_string() + "VersionId"), &obj.version_id);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
        ObjectVersionStorageClassWriter::write_params(params, &(prefix.to_string() + "StorageClass"), &obj.storage_class);
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
        OwnerWriter::write_params(params, &(prefix.to_string() + "Owner"), &obj.owner);
        IsLatestWriter::write_params(params, &(prefix.to_string() + "IsLatest"), &obj.is_latest);
        SizeWriter::write_params(params, &(prefix.to_string() + "Size"), &obj.size);
    }
}

pub type ObjectKey = String;
/// Parse `ObjectKey` from XML
pub struct ObjectKeyParser;

impl ObjectKeyParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectKey, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ObjectKey` contents to a `SignedRequest`
pub struct ObjectKeyWriter;

impl ObjectKeyWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectKey) {
        params.put(name, obj);
    }
}

pub type ObjectVersionStorageClass = String;
/// Parse `ObjectVersionStorageClass` from XML
pub struct ObjectVersionStorageClassParser;

impl ObjectVersionStorageClassParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectVersionStorageClass, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ObjectVersionStorageClass` contents to a `SignedRequest`
pub struct ObjectVersionStorageClassWriter;

impl ObjectVersionStorageClassWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectVersionStorageClass) {
        params.put(name, obj);
    }
}

pub type Size = i32;
/// Parse `Size` from XML
pub struct SizeParser;

impl SizeParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Size, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i32::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `Size` contents to a `SignedRequest`
pub struct SizeWriter;

impl SizeWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Size) {
        params.put(name, &obj.to_string());
    }
}

pub type IsLatest = bool;
/// Parse `IsLatest` from XML
pub struct IsLatestParser;

impl IsLatestParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<IsLatest, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `IsLatest` contents to a `SignedRequest`
pub struct IsLatestWriter;

impl IsLatestWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &IsLatest) {
        params.put(name, &obj.to_string());
    }
}

pub type LambdaFunctionArn = String;
/// Parse `LambdaFunctionArn` from XML
pub struct LambdaFunctionArnParser;

impl LambdaFunctionArnParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<LambdaFunctionArn, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `LambdaFunctionArn` contents to a `SignedRequest`
pub struct LambdaFunctionArnWriter;

impl LambdaFunctionArnWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &LambdaFunctionArn) {
        params.put(name, obj);
    }
}

/// Optional unique identifier for configurations in a notification configuration.
/// If you don't provide one, Amazon S3 will assign an ID.
pub type NotificationId = String;
/// Parse `NotificationId` from XML
pub struct NotificationIdParser;

impl NotificationIdParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<NotificationId, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `NotificationId` contents to a `SignedRequest`
pub struct NotificationIdWriter;

impl NotificationIdWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &NotificationId) {
        params.put(name, obj);
    }
}

pub type Suffix = String;
/// Parse `Suffix` from XML
pub struct SuffixParser;

impl SuffixParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Suffix, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Suffix` contents to a `SignedRequest`
pub struct SuffixWriter;

impl SuffixWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Suffix) {
        params.put(name, obj);
    }
}

pub type ReplaceKeyWith = String;
/// Parse `ReplaceKeyWith` from XML
pub struct ReplaceKeyWithParser;

impl ReplaceKeyWithParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ReplaceKeyWith, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ReplaceKeyWith` contents to a `SignedRequest`
pub struct ReplaceKeyWithWriter;

impl ReplaceKeyWithWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ReplaceKeyWith) {
        params.put(name, obj);
    }
}


pub type ReplaceKeyPrefixWith = String;
/// Parse `ReplaceKeyPrefixWith` from XML
pub struct ReplaceKeyPrefixWithParser;

impl ReplaceKeyPrefixWithParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ReplaceKeyPrefixWith, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ReplaceKeyPrefixWith` contents to a `SignedRequest`
pub struct ReplaceKeyPrefixWithWriter;

impl ReplaceKeyPrefixWithWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ReplaceKeyPrefixWith) {
        params.put(name, obj);
    }
}

/// Parse `RedirectAllRequestsTo` from XML
pub struct RedirectAllRequestsToParser;

impl RedirectAllRequestsToParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RedirectAllRequestsTo, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = RedirectAllRequestsTo::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "HostName" {
                obj.host_name = try!(HostNameParser::parse_xml("HostName", stack));
                continue;
            }
            if current_name == "Protocol" {
                obj.protocol = Some(try!(ProtocolParser::parse_xml("Protocol", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `RedirectAllRequestsTo` contents to a `SignedRequest`
pub struct RedirectAllRequestsToWriter;

impl RedirectAllRequestsToWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &RedirectAllRequestsTo) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        HostNameWriter::write_params(params, &(prefix.to_string() + "HostName"), &obj.host_name);
        if let Some(ref obj) = obj.protocol {
            ProtocolWriter::write_params(params, &(prefix.to_string() + "Protocol"), obj);
        }
    }
}

/// Parse `IndexDocument` from XML
pub struct IndexDocumentParser;

impl IndexDocumentParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<IndexDocument, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = IndexDocument::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Suffix" {
                obj.suffix = try!(SuffixParser::parse_xml("Suffix", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `IndexDocument` contents to a `SignedRequest`
pub struct IndexDocumentWriter;

impl IndexDocumentWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &IndexDocument) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        SuffixWriter::write_params(params, &(prefix.to_string() + "Suffix"), &obj.suffix);
    }
}

/// Parse `ErrorDocument` from XML
pub struct ErrorDocumentParser;

impl ErrorDocumentParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ErrorDocument, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = ErrorDocument::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Key" {
                obj.key = try!(ObjectKeyParser::parse_xml("Key", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ErrorDocument` contents to a `SignedRequest`
pub struct ErrorDocumentWriter;

impl ErrorDocumentWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ErrorDocument) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ObjectKeyWriter::write_params(params, &(prefix.to_string() + "Key"), &obj.key);
    }
}

/// Parse `Redirect` from XML
pub struct RedirectParser;

impl RedirectParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Redirect, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Redirect::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "ReplaceKeyWith" {
                obj.replace_key_with = try!(ReplaceKeyWithParser::parse_xml("ReplaceKeyWith", stack));
                continue;
            }
            if current_name == "HostName" {
                obj.host_name = try!(HostNameParser::parse_xml("HostName", stack));
                continue;
            }
            if current_name == "Protocol" {
                obj.protocol = try!(ProtocolParser::parse_xml("Protocol", stack));
                continue;
            }
            if current_name == "ReplaceKeyPrefixWith" {
                obj.replace_key_prefix_with = try!(ReplaceKeyPrefixWithParser::parse_xml("ReplaceKeyPrefixWith", stack));
                continue;
            }
            if current_name == "HttpRedirectCode" {
                obj.http_redirect_code = try!(HttpRedirectCodeParser::parse_xml("HttpRedirectCode", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Redirect` contents to a `SignedRequest`
pub struct RedirectWriter;

impl RedirectWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Redirect) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        ReplaceKeyWithWriter::write_params(params, &(prefix.to_string() + "ReplaceKeyWith"), &obj.replace_key_with);
        HostNameWriter::write_params(params, &(prefix.to_string() + "HostName"), &obj.host_name);
        ProtocolWriter::write_params(params, &(prefix.to_string() + "Protocol"), &obj.protocol);
        ReplaceKeyPrefixWithWriter::write_params(params, &(prefix.to_string() + "ReplaceKeyPrefixWith"), &obj.replace_key_prefix_with);
        HttpRedirectCodeWriter::write_params(params, &(prefix.to_string() + "HttpRedirectCode"), &obj.http_redirect_code);
    }
}

/// Parse `Initiator` from XML
pub struct InitiatorParser;

impl InitiatorParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Initiator, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Initiator::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "DisplayName" {
                obj.display_name = try!(DisplayNameParser::parse_xml("DisplayName", stack));
                continue;
            }
            if current_name == "ID" {
                obj.id = try!(IDParser::parse_xml("ID", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Initiator` contents to a `SignedRequest`
pub struct InitiatorWriter;

impl InitiatorWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Initiator) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        DisplayNameWriter::write_params(params, &(prefix.to_string() + "DisplayName"), &obj.display_name);
        IDWriter::write_params(params, &(prefix.to_string() + "ID"), &obj.id);
    }
}

/// Parse `RoutingRule` from XML
pub struct RoutingRuleParser;

impl RoutingRuleParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RoutingRule, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = RoutingRule::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Redirect" {
                obj.redirect = try!(RedirectParser::parse_xml("Redirect", stack));
                continue;
            }
            if current_name == "Condition" {
                obj.condition = Some(try!(ConditionParser::parse_xml("Condition", stack)));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `RoutingRule` contents to a `SignedRequest`
pub struct RoutingRuleWriter;

impl RoutingRuleWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &RoutingRule) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        RedirectWriter::write_params(params, &(prefix.to_string() + "Redirect"), &obj.redirect);
        if let Some(ref obj) = obj.condition {
            ConditionWriter::write_params(params, &(prefix.to_string() + "Condition"), obj);
        }
    }
}

pub type RoutingRules = Vec<RoutingRule>;
/// Parse `RoutingRules` from XML
pub struct RoutingRulesParser;

impl RoutingRulesParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<RoutingRules, XmlParseError> {
        let mut obj = Vec::new();
        while try!(peek_at_name(stack)) == "RoutingRule" {
            obj.push(try!(RoutingRuleParser::parse_xml("RoutingRule", stack)));
        }
        Ok(obj)
    }
}

/// Write `RoutingRules` contents to a `SignedRequest`
pub struct RoutingRulesWriter;

impl RoutingRulesWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &RoutingRules) {
        let mut index = 1;
        for element in obj.iter() {
            let key = &format!("{}.{}", name, index);
            RoutingRuleWriter::write_params(params, key, element);
            index += 1;
        }
    }
}

pub type Initiated = String;
/// Parse `Initiated` from XML
pub struct InitiatedParser;

impl InitiatedParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Initiated, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Initiated` contents to a `SignedRequest`
pub struct InitiatedWriter;

impl InitiatedWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Initiated) {
        params.put(name, obj);
    }
}

pub type CopySourceVersionId = String;
/// Parse `CopySourceVersionId` from XML
pub struct CopySourceVersionIdParser;

impl CopySourceVersionIdParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopySourceVersionId, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `CopySourceVersionId` contents to a `SignedRequest`
pub struct CopySourceVersionIdWriter;

impl CopySourceVersionIdWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CopySourceVersionId) {
        params.put(name, obj);
    }
}

/// Parse `CopyObjectResult` from XML
pub struct CopyObjectResultParser;

impl CopyObjectResultParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<CopyObjectResult, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = CopyObjectResult::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "LastModified" {
                obj.last_modified = try!(LastModifiedParser::parse_xml("LastModified", stack));
                continue;
            }
            if current_name == "ETag" {
                obj.e_tag = try!(ETagParser::parse_xml("ETag", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `CopyObjectResult` contents to a `SignedRequest`
pub struct CopyObjectResultWriter;

impl CopyObjectResultWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &CopyObjectResult) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        LastModifiedWriter::write_params(params, &(prefix.to_string() + "LastModified"), &obj.last_modified);
        ETagWriter::write_params(params, &(prefix.to_string() + "ETag"), &obj.e_tag);
    }
}

pub type ReplicationRuleStatus = String;
/// Parse `ReplicationRuleStatus` from XML
pub struct ReplicationRuleStatusParser;

impl ReplicationRuleStatusParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ReplicationRuleStatus, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `ReplicationRuleStatus` contents to a `SignedRequest`
pub struct ReplicationRuleStatusWriter;

impl ReplicationRuleStatusWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ReplicationRuleStatus) {
        params.put(name, obj);
    }
}

/// Parse `Destination` from XML
pub struct DestinationParser;

impl DestinationParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Destination, XmlParseError> {
        try!(start_element(tag_name, stack));
        let mut obj = Destination::default();
        loop {
            let current_name = try!(peek_at_name(stack));
            if current_name == "Bucket" {
                obj.bucket = try!(BucketNameParser::parse_xml("Bucket", stack));
                continue;
            }
            break;
        }
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `Destination` contents to a `SignedRequest`
pub struct DestinationWriter;

impl DestinationWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &Destination) {
        let mut prefix = name.to_string();
        if prefix != "" { prefix.push_str("."); }
        BucketNameWriter::write_params(params, &(prefix.to_string() + "Bucket"), &obj.bucket);
    }
}

pub type BucketLogsPermission = String;
/// Parse `BucketLogsPermission` from XML
pub struct BucketLogsPermissionParser;

impl BucketLogsPermissionParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<BucketLogsPermission, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}

/// Write `BucketLogsPermission` contents to a `SignedRequest`
pub struct BucketLogsPermissionWriter;

impl BucketLogsPermissionWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &BucketLogsPermission) {
        params.put(name, obj);
    }
}





pub type ObjectVersionId = String;
/// Parse `ObjectVersionId` from XML
pub struct ObjectVersionIdParser;

impl ObjectVersionIdParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<ObjectVersionId, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `ObjectVersionId` contents to a `SignedRequest`
pub struct ObjectVersionIdWriter;

impl ObjectVersionIdWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &ObjectVersionId) {
        params.put(name, obj);
    }
}

pub type LastModified = String;
/// Parse `LastModified` from XML
pub struct LastModifiedParser;

impl LastModifiedParser {
    pub fn parse_xml<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<LastModified, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));
        Ok(obj)
    }
}
/// Write `LastModified` contents to a `SignedRequest`
pub struct LastModifiedWriter;

impl LastModifiedWriter {
    pub fn write_params(params: &mut Params, name: &str, obj: &LastModified) {
        params.put(name, obj);
    }
}
