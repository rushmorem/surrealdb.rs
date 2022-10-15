use crate::param::Jwt;
use serde::Serialize;

#[derive(Debug)]
pub struct Signup;

#[derive(Debug)]
pub struct Signin;

pub trait Credentials<Action, Response>: Serialize {}

#[derive(Debug, Serialize)]
pub struct Root<'a> {
    #[serde(rename = "user")]
    pub username: &'a str,
    #[serde(rename = "pass")]
    pub password: &'a str,
}

impl Credentials<Signin, ()> for Root<'_> {}

#[derive(Debug, Serialize)]
pub struct NameSpace<'a> {
    #[serde(rename = "ns")]
    pub namespace: &'a str,
    #[serde(rename = "user")]
    pub username: &'a str,
    #[serde(rename = "pass")]
    pub password: &'a str,
}

impl Credentials<Signin, Jwt> for NameSpace<'_> {}

#[derive(Debug, Serialize)]
pub struct Database<'a> {
    #[serde(rename = "ns")]
    pub namespace: &'a str,
    #[serde(rename = "db")]
    pub database: &'a str,
    #[serde(rename = "user")]
    pub username: &'a str,
    #[serde(rename = "pass")]
    pub password: &'a str,
}

impl Credentials<Signin, Jwt> for Database<'_> {}

#[derive(Debug, Serialize)]
pub struct Scope<'a, P> {
    #[serde(rename = "ns")]
    pub namespace: &'a str,
    #[serde(rename = "db")]
    pub database: &'a str,
    #[serde(rename = "sc")]
    pub scope: &'a str,
    #[serde(flatten)]
    pub params: P,
}

impl<P> Credentials<Signup, Jwt> for Scope<'_, P> where P: Serialize {}
impl<P> Credentials<Signin, Jwt> for Scope<'_, P> where P: Serialize {}
