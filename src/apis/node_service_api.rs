/*
 * madana-api
 *
 * <h1>Using the madana-api</h1>        <p>This documentation contains a Quickstart Guide, relating client functionality and information about the available         endpoints and used datamodels.   </p>       <p> The madana-api and its implementations are still in heavy development. This means that there may be problems in our protocols, or there may be mistakes in our implementations. We take security vulnerabilities very seriously. If you discover a security issue, please bring it to our attention right away! If you find a vulnerability that may affect live deployments -- for example, by exposing a remote execution exploit -- please send your report privately to info@madana.io. Please DO NOT file a public issue. If the issue is a protocol weakness that cannot be immediately exploited or something not yet deployed, just discuss it openly   </p>   <br>   <p> Note: Not all functionality might be acessible without having accquired and api-license token. For more information visit <a href=\"https://www.madana.io\">www.madana.io</a> </p>       <br>
 *
 * The version of the OpenAPI document: 0.4.14-master.21
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

pub struct NodeServiceApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> NodeServiceApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> NodeServiceApiClient<C> {
        NodeServiceApiClient {
            configuration,
        }
    }
}

pub trait NodeServiceApi {
    fn get_bootstrap(&self, ) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
    fn get_nodes2(&self, owner: Option<&str>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
    fn post_node_info(&self, body: Option<crate::models::JsonNodeInfo>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>NodeServiceApi for NodeServiceApiClient<C> {
    fn get_bootstrap(&self, ) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/nodes/bootstrap".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_nodes2(&self, owner: Option<&str>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/nodes".to_string())
        ;
        if let Some(ref s) = owner {
            req = req.with_query_param("owner".to_string(), s.to_string());
        }

        req.execute(self.configuration.borrow())
    }

    fn post_node_info(&self, body: Option<crate::models::JsonNodeInfo>) -> Box<dyn Future<Item = std::path::PathBuf, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/nodes".to_string())
        ;
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

}
