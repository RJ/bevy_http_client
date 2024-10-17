use bevy::{prelude::*, time::common_conditions::on_timer};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use bevy_http_client::{prelude::*, HttpResponsePart};

#[derive(Serialize, Deserialize, Debug)]
pub enum SessionRequestFeedback {
    /// The service has begun processing the request.
    Acknowledged,
    /// The edgegap session was created, we are now awaiting readyness
    SessionRequestAccepted(String),
    /// Session readyness update
    ProgressReport(String),
    /// The session is ready to connect to
    SessionReady {
        token: String,
        ip: String,
        port: u16,
        cert_digest: String,
    },
    /// There was an error.
    Error(u16, String),
}

fn main() {
    App::new()
        .add_plugins((
            MinimalPlugins,
            bevy_log::LogPlugin::default(),
            HttpClientPlugin,
        ))
        .add_systems(
            Update,
            (
                handle_response,
                handle_response_part,
                handle_error,
                dump_ents.run_if(on_timer(Duration::from_millis(500))),
            ),
        )
        .add_systems(Startup, send_request)
        .register_request_type::<SessionRequestFeedback>()
        .run();
}

fn send_request(mut ev_request: EventWriter<TypedRequest<SessionRequestFeedback>>) {
    // let id = commands.spawn(ReqMarker).id();
    let request = HttpClient::new()
        .post("http://localhost:3000/matchmaker/wannaplay2")
        .with_type::<SessionRequestFeedback>()
        .with_streaming();
    info!("Making request: {request:?}");
    ev_request.send(request);
}

fn dump_ents(q: Query<&RequestTask>) {
    println!("ReqMarker entity exists: {}", !q.is_empty());
}

fn handle_response(mut ev_resp: EventReader<TypedResponse<SessionRequestFeedback>>) {
    for response in ev_resp.read() {
        println!("response: {:?}", response);
    }
}

fn handle_response_part(mut ev_resp: EventReader<TypedResponsePart<SessionRequestFeedback>>) {
    for part in ev_resp.read() {
        println!("part: {:?}", part);
    }
}

fn handle_error(mut ev_error: EventReader<TypedResponseError<SessionRequestFeedback>>) {
    for error in ev_error.read() {
        println!("Error retrieving IP: {}", error.err);
    }
}
