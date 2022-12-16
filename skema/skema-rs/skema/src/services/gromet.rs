//! REST API endpoints related to CRUD operations and other queries on GroMEt objects.

use crate::database::{execute_query, parse_gromet_queries};
use crate::Gromet;
use crate::config::Config;
use rsmgclient::{ConnectParams, Connection, MgError, Value};

use actix_web::{HttpResponse, get, post, web, delete};
use utoipa;

pub fn push_model_to_db(gromet: Gromet, host: &str) -> Result<i64, MgError> {

    // parse gromet into vec of queries
    let queries = parse_gromet_queries(gromet);

    // need to make the whole query list one line, individual executions are treated as different graphs for each execution.
    let mut full_query = queries[0].clone();
    for i in 1..(queries.len()) {
        full_query.push_str("\n");
        let temp_str = &queries[i].clone();
        full_query.push_str(&temp_str);
    }
    execute_query(&full_query, host);
    let model_ids = module_query(host)?;
    let last_model_id = model_ids[model_ids.len() - 1];
    Ok(last_model_id)
}

pub fn delete_module(module_id: i64, host: &str) -> Result<(),MgError> {
    // construct the query that will delete the module with a given unique identifier

    let query = format!("MATCH (n)-[r:Contains|Port_Of|Wire*1..5]->(m) WHERE id(n) = {}\nDETACH DELETE n,m", module_id);
    execute_query(&query, host);
    Ok(())
}

pub fn named_opi_query(module_id: i64, host: &str) -> Result<Vec<String>,MgError> {
    // construct the query that will delete the module with a given unique identifier

    // Connect to Memgraph.
    let connect_params = ConnectParams {
        host: Some(host.to_string()),
        ..Default::default()
    };
    let mut connection = Connection::connect(&connect_params)?;

    // create query
    let query = format!("MATCH (n)-[r:Contains|Port_Of|Wire*1..5]->(m) WHERE id(n) = {}
        \nwith DISTINCT m\nmatch (m:Opi) where not m.name = 'un-named'\nreturn collect(m.name)", module_id);


    // Run Query.
    connection.execute(&query, None)?;

    // Check that the first value of the first record is a list
    let mut port_names = Vec::<String>::new();
    if let Value::List(xs) = &connection.fetchall()?[0].values[0] {
        port_names = xs.iter().filter_map(|x| match x {
            Value::String(x) => Some(x.clone()),
            _ => None
        }).collect();
    }
    connection.commit()?;
    
    Ok(port_names)
}

pub fn named_opo_query(module_id: i64, host: &str) -> Result<Vec<String>,MgError> {
    // construct the query that will delete the module with a given unique identifier

    // Connect to Memgraph.
    let connect_params = ConnectParams {
        host: Some(host.to_string()),
        ..Default::default()
    };
    let mut connection = Connection::connect(&connect_params)?;

    // create query
    let query = format!("MATCH (n)-[r:Contains|Port_Of|Wire*1..5]->(m) WHERE id(n) = {}
        \nwith DISTINCT m\nmatch (m:Opo) where not m.name = 'un-named'\nreturn collect(m.name)", module_id);


    // Run Query.
    connection.execute(&query, None)?;

    // Check that the first value of the first record is a list
    let mut port_names = Vec::<String>::new();
    if let Value::List(xs) = &connection.fetchall()?[0].values[0] {
        port_names = xs.iter().filter_map(|x| match x {
            Value::String(x) => Some(x.clone()),
            _ => None
        }).collect();
    }
    connection.commit()?;
    
    Ok(port_names)
}

pub fn module_query(host: &str) -> Result<Vec<i64>, MgError> {
    // Connect to Memgraph.
    let connect_params = ConnectParams {
        host: Some(host.to_string()),
        ..Default::default()
    };
    let mut connection = Connection::connect(&connect_params)?;

    // Run Query.
    connection.execute("MATCH (n:Module) RETURN collect(id(n));", None)?;

    // Check that the first value of the first record is a list
    let mut ids = Vec::<i64>::new();
    if let Value::List(xs) = &connection.fetchall()?[0].values[0] {
        ids = xs.iter().filter_map(|x| match x {
            Value::Int(x) => Some(x.clone()),
            _ => None
        }).collect();
    }
    connection.commit()?;

    Ok(ids)
}

/// This retrieves the model IDs.
#[utoipa::path(
    responses(
        (status = 200, description = "Successfully retrieved model IDs")
    )
)]
#[get("/models")]
pub async fn get_model_ids(config: web::Data<Config>) -> HttpResponse {
    let response = module_query(&config.db_host).unwrap();
    HttpResponse::Ok().json(web::Json(response))
}

/// Pushes a gromet JSON to the Memgraph database
#[utoipa::path(
    request_body = Gromet,
    responses(
        (status = 200, description = "Model successfully pushed")
    )
)]
#[post("/models")]
pub async fn post_model(payload: web::Json<Gromet>, config: web::Data<Config>) -> HttpResponse {
    let model_id = push_model_to_db(payload.into_inner(), &config.db_host).unwrap();
    HttpResponse::Ok().json(web::Json(model_id))
}

/// Deletes a model from the database based on its id.
#[utoipa::path(
    responses(
        (status = 200, description = "Model deleted")
    )
)]
#[delete("/models/{id}")]
pub async fn delete_model(path: web::Path<i64>, config: web::Data<Config>) -> HttpResponse {
    let id = path.into_inner();
    delete_module(id, &config.db_host);
    HttpResponse::Ok().body("Model deleted")
}


/// This retrieves named Opos based on model id.
#[utoipa::path(
    responses(
        (status = 200, description = "Successfully retrieved named outports")
    )
)]
#[get("/models/{id}/named_opos")]
pub async fn get_named_opos(path: web::Path<i64>, config: web::Data<Config>) -> HttpResponse {
    let response = named_opo_query(path.into_inner(), &config.db_host).unwrap();
    HttpResponse::Ok().json(web::Json(response))
}

/// This retrieves named Opis based on model id.
#[utoipa::path(
    responses(
        (status = 200, description = "Successfully retrieved named outports")
    )
)]
#[get("/models/{id}/named_opis")]
pub async fn get_named_opis(path: web::Path<i64>, config: web::Data<Config>) -> HttpResponse {
    let response = named_opi_query(path.into_inner(), &config.db_host).unwrap();
    HttpResponse::Ok().json(web::Json(response))
}