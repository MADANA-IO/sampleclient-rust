/*
 * madana-api
 *
 * <h1>Using the madana-api</h1>        <p>This documentation contains a Quickstart Guide, relating client functionality and information about the available         endpoints and used datamodels.   </p>       <p> The madana-api and its implementations are still in heavy development. This means that there may be problems in our protocols, or there may be mistakes in our implementations. We take security vulnerabilities very seriously. If you discover a security issue, please bring it to our attention right away! If you find a vulnerability that may affect live deployments -- for example, by exposing a remote execution exploit -- please send your report privately to info@madana.io. Please DO NOT file a public issue. If the issue is a protocol weakness that cannot be immediately exploited or something not yet deployed, just discuss it openly   </p>   <br>   <p> Note: Not all functionality might be acessible without having accquired and api-license token. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a> </p>       <br>
 *
 * The version of the OpenAPI document: 0.4.14-master.17
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct AuthenticationServiceApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> AuthenticationServiceApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> AuthenticationServiceApiClient<C> {
        AuthenticationServiceApiClient {
            configuration,
        }
    }
}

pub trait AuthenticationServiceApi {
    fn authenticate_application(&self, body: Option<crate::models::JsonMdnCertificate>) -> Box<dyn Future<Item = crate::models::JsonMdnToken, Error = Error<serde_json::Value>>>;
    fn authenticate_ethereum_wallet(&self, wallet: &str, body: Option<crate::models::JsonMdnOAuthToken>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
    fn authenticate_user(&self, body: Option<crate::models::JsonMdnUserCredentials>) -> Box<dyn Future<Item = crate::models::JsonMdnToken, Error = Error<serde_json::Value>>>;
    fn authenticate_with_ethereum_challenge(&self, wallet: &str, body: Option<crate::models::JsonMdnOAuthToken>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
    fn get_fractal_authentication_url(&self, ) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
    fn get_nonce_for_ethereum_wallet(&self, wallet: &str, authorization: Option<&str>) -> Box<dyn Future<Item = crate::models::JsonMdnToken, Error = Error<serde_json::Value>>>;
    fn get_object(&self, ) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn get_twitter_authentication_url(&self, ) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
    fn set_facebook_uid(&self, body: Option<&str>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
    fn set_fractal_uid(&self, body: Option<&str>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
    fn set_twitter_uid(&self, body: Option<crate::models::JsonMdnOAuthToken>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>AuthenticationServiceApi for AuthenticationServiceApiClient<C> {
    fn authenticate_application(&self, body: Option<crate::models::JsonMdnCertificate>) -> Box<dyn Future<Item = crate::models::JsonMdnToken, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/authentication/application".to_string())
        ;
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn authenticate_ethereum_wallet(&self, wallet: &str, body: Option<crate::models::JsonMdnOAuthToken>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/authentication/ethereum/{wallet}".to_string())
        ;
        req = req.with_path_param("wallet".to_string(), wallet.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn authenticate_user(&self, body: Option<crate::models::JsonMdnUserCredentials>) -> Box<dyn Future<Item = crate::models::JsonMdnToken, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/authentication".to_string())
        ;
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn authenticate_with_ethereum_challenge(&self, wallet: &str, body: Option<crate::models::JsonMdnOAuthToken>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/authentication/ethereum/{wallet}/challenge".to_string())
        ;
        req = req.with_path_param("wallet".to_string(), wallet.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn get_fractal_authentication_url(&self, ) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/authentication/fractal".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_nonce_for_ethereum_wallet(&self, wallet: &str, authorization: Option<&str>) -> Box<dyn Future<Item = crate::models::JsonMdnToken, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/authentication/ethereum/{wallet}".to_string())
        ;
        req = req.with_path_param("wallet".to_string(), wallet.to_string());
        if let Some(param_value) = authorization {
            req = req.with_header_param("Authorization".to_string(), param_value.to_string());
        }

        req.execute(self.configuration.borrow())
    }

    fn get_object(&self, ) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/authentication".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_twitter_authentication_url(&self, ) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/authentication/twitter".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn set_facebook_uid(&self, body: Option<&str>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/authentication/facebook".to_string())
        ;
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn set_fractal_uid(&self, body: Option<&str>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/authentication/fractal".to_string())
        ;
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn set_twitter_uid(&self, body: Option<crate::models::JsonMdnOAuthToken>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/authentication/twitter".to_string())
        ;
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

}