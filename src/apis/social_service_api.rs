/*
 * madana-api
 *
 * <h1>Using the madana-api</h1>        <p>This documentation contains a Quickstart Guide, relating client functionality and information about the available         endpoints and used datamodels.   </p>       <p> The madana-api and its implementations are still in heavy development. This means that there may be problems in our protocols, or there may be mistakes in our implementations. We take security vulnerabilities very seriously. If you discover a security issue, please bring it to our attention right away! If you find a vulnerability that may affect live deployments -- for example, by exposing a remote execution exploit -- please send your report privately to info@madana.io. Please DO NOT file a public issue. If the issue is a protocol weakness that cannot be immediately exploited or something not yet deployed, just discuss it openly   </p>   <br>   <p> Note: Not all functionality might be acessible without having accquired and api-license token. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a> </p>       <br>
 *
 * The version of the OpenAPI document: 0.4.14-master.18
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

pub struct SocialServiceApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> SocialServiceApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> SocialServiceApiClient<C> {
        SocialServiceApiClient {
            configuration,
        }
    }
}

pub trait SocialServiceApi {
    fn get_my_profile(&self, ) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
    fn get_platforms2(&self, ) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
    fn get_ranking(&self, ) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
    fn get_social_platform_feed(&self, platform: &str) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
    fn get_user_profile(&self, username: &str, simple: Option<&str>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
    fn get_user_profile_0(&self, username: &str) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>SocialServiceApi for SocialServiceApiClient<C> {
    fn get_my_profile(&self, ) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/social/profiles/me".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_platforms2(&self, ) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/social".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_ranking(&self, ) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/social/ranking".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_social_platform_feed(&self, platform: &str) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/social/feed/{platform}".to_string())
        ;
        req = req.with_path_param("platform".to_string(), platform.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_user_profile(&self, username: &str, simple: Option<&str>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/social/profiles/{username}".to_string())
        ;
        if let Some(ref s) = simple {
            req = req.with_query_param("simple".to_string(), s.to_string());
        }
        req = req.with_path_param("username".to_string(), username.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_user_profile_0(&self, username: &str) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/social/profiles/{username}/simple".to_string())
        ;
        req = req.with_path_param("username".to_string(), username.to_string());

        req.execute(self.configuration.borrow())
    }

}
