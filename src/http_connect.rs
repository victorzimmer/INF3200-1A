use minreq::Response;
use rocket::serde;

#[derive(Debug)]
pub struct NodeConnectionError {
    pub connection_established: bool,
    pub http_response: Option<Response>,
}

pub enum WriteOperations {
    Post,
    Put,
    Delete,
}

pub fn check_if_node_is_connected() {}

pub fn get_from_node(
    hostname: &str,
    port: u16,
    path: &str,
) -> Result<Response, NodeConnectionError> {
    let request_uri = format!("http://{}:{}/{}", hostname, port, path);

    let received_response = match minreq::get(request_uri).send() {
        Err(_err) => {
            return Err(NodeConnectionError {
                connection_established: false,
                http_response: None,
            });
        }
        Ok(response) => response,
    };

    if received_response.status_code != 200 {
        return Err(NodeConnectionError {
            connection_established: true,
            http_response: Some(received_response),
        });
    }

    return Ok(received_response);
}

pub fn write_body_to_node<T>(
    operation: WriteOperations,
    hostname: &str,
    port: u16,
    path: &str,
    content_type: &str,
    body: T,
) -> Result<Response, NodeConnectionError>
where
    T: Into<Vec<u8>>,
{
    let request_uri = format!("http://{}:{}/{}", hostname, port, path);

    let func = match operation {
        WriteOperations::Post => minreq::post,
        WriteOperations::Put => minreq::put,
        WriteOperations::Delete => minreq::delete,
    };

    let received_response = match func(request_uri)
        .with_body(body)
        .with_header("Content-Type", content_type)
        .send()
    {
        Err(_err) => {
            return Err(NodeConnectionError {
                connection_established: false,
                http_response: None,
            });
        }
        Ok(response) => response,
    };

    if received_response.status_code != 200 {
        return Err(NodeConnectionError {
            connection_established: true,
            http_response: Some(received_response),
        });
    }

    return Ok(received_response);
}

pub fn write_json_to_node<T>(
    operation: WriteOperations,
    hostname: &str,
    port: u16,
    path: &str,
    content: T,
) -> Result<Response, NodeConnectionError>
where
    T: serde::ser::Serialize,
{
    let request_uri = format!("http://{}:{}/{}", hostname, port, path);

    let func = match operation {
        WriteOperations::Post => minreq::post,
        WriteOperations::Put => minreq::put,
        WriteOperations::Delete => minreq::delete,
    };

    let received_response = match func(request_uri)
        .with_json(&content)
        .expect("Could not serialize content.")
        .send()
    {
        Err(_err) => {
            return Err(NodeConnectionError {
                connection_established: false,
                http_response: None,
            });
        }
        Ok(response) => response,
    };

    if received_response.status_code != 200 {
        return Err(NodeConnectionError {
            connection_established: true,
            http_response: Some(received_response),
        });
    }

    return Ok(received_response);
}
