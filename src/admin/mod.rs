use std::{fmt::Display, sync::Arc};

use axum::{extract::State, response::Html};
use dioxus::{prelude::*, ssr::render_lazy};

use crate::app_state::AppState;

pub async fn load_admin(State(state): State<Arc<AppState>>) -> Html<String> {
    let users = state.users.lock();
    let cosmetics = state.cosmetics.lock();
    let users_len = users.len();
    let cosmetics_len = cosmetics.len();
    Html(render_lazy(rsx! {
        head {
                title {
                    "DWS - {users_len} users, {cosmetics_len} cosmetics"
                }
                meta {
                    charset: "utf-8"
                }
                meta {
                    name: "viewport",
                    content: "width=device-width, initial-scale=1"
                }
                meta {
                    name: "description",
                    content: "Admin panel"
                }
                meta {
                    name: "author",
                    content: "Tricked-dev"
                }
                meta {
                    name: "theme-color",
                    content: "black"
                }
                style {
                    vec![
                        r#"
                            :root {
                                color-scheme: dark;
                            }
                            th {
                                min-width: 100px;
                                text-align: left;
                            }

                        "#
                    ]
                }

            }
        body {
            div {
                h1 { "Admin" }
                h2 { "Users" }
                table {
                    tr {
                        th { "Username" }
                        th { "Cosmetic" }
                        th { "Connected" }
                        th { "Discord Id" }
                        th { "Irc Blacklisted" }
                    }
                    users.iter().map(|(uuid,data)| rsx!{
                        tr {
                            td { pre { "{uuid}" } }
                            td { pre { "{data.enabled_prefix:?}" } }
                            td { pre { "{data.connected}" } }
                            td { pre { "{data.linked_discord:?}" } }
                            td { pre { "{data.irc_blacklisted}" } }
                        }
                    })
                    form {
                        action: "/admin/force_update",
                        method: "POST",
                        input {
                            r#type: "hidden",
                            name: "broadcast_secret",
                            value: ""
                        }
                        button {
                            r#type: "submit",
                            "Force update"
                        }
                    }
                }
                h2 { "Cosmetics" }
                table {
                    tr {
                        th { "Name" }
                        th { "Id" }
                        th { "Preview" }
                        th { "Flags" }
                    }
                    cosmetics.iter().map(|cosmetic| rsx!{
                        tr {
                            td { pre { "{cosmetic.name}" } }
                            td { pre { "{cosmetic.id}" } }
                            td { pre { "{cosmetic.data}" } }
                            td { pre { "{cosmetic.required_flags:?}" } }
                        }
                    })
                }
            }
          }
    }))
}
