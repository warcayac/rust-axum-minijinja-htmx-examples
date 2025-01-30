use std::time::Duration;

use axum::{extract::State, http::{HeaderMap, HeaderValue}, response::IntoResponse};
use chrono::{DateTime, Utc};
use minijinja::context;
use tokio::time::sleep;

use crate::app::{presentation::helpers::*, TAppStateState, TSharedAppState, APP_ROUTES};


pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/progress_bar/index.html");
    let context = context! {
        target => APP_ROUTES["progressbar"].full_pathname(1)
    };

    HtmlTemplate(template, context).into_response()
}

async fn task_hdlr(state: TSharedAppState, value: u8) -> impl IntoResponse {
    let template = state.env.get_template("pages/progress_bar/partials/job.html");
    let context = context! {
        target_start => APP_ROUTES["progressbar"].full_pathname(1),
        target_progress => APP_ROUTES["progressbar"].full_pathname_variant(0,0),
        target_job => APP_ROUTES["progressbar"].full_pathname(0),
        value => value,
        start_time => Utc::now().timestamp(),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn start_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    task_hdlr(state, 0).await
}

pub async fn done_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    task_hdlr(state, 122).await
}

pub async fn progress_hdlr(State(state): TAppStateState, headers: HeaderMap) -> impl IntoResponse {
    let start_time_ts = headers.get("start_time").unwrap().to_str().unwrap().parse::<i64>().unwrap();
    let _ = sleep(Duration::from_secs(1)).await;
    let now = Utc::now();
    let start_time = DateTime::from_timestamp(start_time_ts, 0).unwrap();
    let time_delta = now - start_time;
    let diff_secs = time_delta.num_milliseconds();
    let new_percentage = (diff_secs as f64 / 100.0).round();

    let template = state.env.get_template("pages/progress_bar/partials/progress.html");
    let context = context! {
        value => new_percentage,
    };

    let mut response = HtmlTemplate(template, context).into_response();

    if new_percentage >= 100.0 {
        response
            .headers_mut()
            .insert("hx-trigger", HeaderValue::from_static("done"))
        ;
    }

    response
}
