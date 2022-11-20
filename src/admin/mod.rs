#![allow(unused_braces)]

use std::sync::Arc;

use axum::{
    extract::{Query, State},
    response::Html,
};
use dioxus::{prelude::*, ssr::render_lazy};
use serde::Deserialize;
use uuid::Uuid;

use crate::app_state::{AppState, User};

pub mod broadcast;
pub mod cosmetics;
pub mod metrics;
pub mod users;

#[derive(Deserialize)]
pub struct AdminQuery {
    page: Option<u32>,
    limit: Option<u32>,
}

pub async fn load_admin(State(state): State<Arc<AppState>>, Query(query): Query<AdminQuery>) -> Html<String> {
    let users = state.users.lock();
    let cosmetics = state.cosmetics.lock();
    let users_len = users.len();
    let cosmetics_len = cosmetics.len();
    let mut tmp = users.iter().collect::<Vec<_>>();
    tmp.sort_by_key(|x| x.0);

    let (page, limit) = (query.page.unwrap_or(1) as usize, query.limit.unwrap_or(50) as usize);

    let users = tmp.iter().skip((page - 1) * limit).take(limit).collect::<Vec<_>>();

    Html(render_lazy(rsx! {
        head { {meta(users_len, cosmetics_len)} }
        body {
            margin: "0 auto",
            max_width: "100rem",
            div {
                h1 { "Admin" }
                div {
                h2 { "Users" }
                form {
                    id: "add-user",
                    input {
                        name: "uuid",
                        placeholder: "uuid",
                        required: "true"
                    }
                        input {
                            name: "linked_discord",
                            placeholder: "linked_discord"
                        }
                        input {
                            name: "enabled_prefix",
                            placeholder: "enabled_prefix"
                        }
                        input {
                            name: "irc_blacklisted",
                            placeholder: "irc_blacklisted"
                        }
                        input {
                            name: "flags",
                            placeholder: "flags"
                        }

                        button {
                            r#type: "submit",
                            "Add user"
                        }
                    }

                {users_table(users)}
                {paginate_buttons(page, limit, users_len)}
                a {
                        href: "javascript:download('/users', 'users.json')",
                        "Download"
                }
                div {
                    {uuid_to_username()}
                }
                }
                div {
                h2 { "Cosmetics" }
                form {
                    id: "add-cosmetic",
                    input {
                        name: "id",
                        placeholder: "id",
                        required: "true"
                    }
                    input {
                        name: "name",
                        placeholder: "name",
                        required: "true"
                    }
                    input {
                        name: "description",
                        placeholder: "description",
                        required: "true"
                    }
                    input {
                        name: "data",
                        placeholder: "data",
                        required: "true"
                    }
                    input {
                        name: "type",
                        placeholder: "type",
                        required: "true"
                    }
                    input {
                        name: "required_flags",
                        placeholder: "required_flags",
                        required: "true"
                    }
                    button {
                        r#type: "submit",
                        "Add cosmetic"
                    }
                }
                table {
                    tr {
                        th { "" }
                        th { "Name" }
                        th { "Id" }
                        th { "Preview" }
                        th { "Flags" }
                    }
                    cosmetics.iter().map(|cosmetic| rsx!{
                        tr {
                            id: "cos-{cosmetic.id}",
                            td {
                                button { class: "cdelete", value: "{cosmetic.id}", "X" }
                            }
                            td { pre { "{cosmetic.name}" } }
                            td { pre { "{cosmetic.id}" } }
                            td { pre { "{cosmetic.data}" } }
                            td { pre { "{cosmetic.required_flags:?}" } }
                        }
                    })

                }
                a {
                    href: "javascript:download('/cosmetics', 'cosmetics.json')",
                    "Download"
                }
                }
            }
            div {
                h2 { "Broadcast" }
                {broadcast_field()}
            }


            script {
                r#type: "module",
                vec![
                    include_str!("./code.js")
                ]
            }
          }
    }))
}

fn paginate_buttons<'a, 'b>(page: usize, limit: usize, count: usize) -> LazyNodes<'a, 'b> {
    let pages = (count / limit) + 1;
    let next_page = page + 1;
    let prev_page = page - 1;
    rsx!(
        p {
            "Page: {page} of {pages}"
        }
        p {
        {(page != 1).then(||
            rsx!(
                a {
                    href: "/?page={prev_page}&limit={limit}",
                    "Previous page"
                }
                span {
                    " "
                })
            )}

        {(page != pages).then(||
            rsx!(
                a {
                    href: "/?page={next_page}&limit={limit}",
                    "Next page"
                })
            )}
    })
}

fn broadcast_field<'a, 'b>() -> LazyNodes<'a, 'b> {
    rsx!(
        form {
            id: "broadcast",

            div {
              textarea {
                  name: "message",
                  placeholder: "message",
                  required: "true"
              }
            }
            button {
                r#type: "submit",
                "Broadcast"
            }
        }

    )
}

fn users_table<'a, 'b>(users: Vec<&'b (&Uuid, &User)>) -> LazyNodes<'a, 'b> {
    rsx!(
        table {
            tr {
                th { "" }
                th { "Uuid" }
                th { "Cosmetic" }
                th { "Connected" }
                th { "Discord Id" }
                th { "Irc Blacklisted" }
                th { "Flags" }
            }
            users.iter().map(|(uuid,data)| {
                let prefix = serde_json::to_string(&data.enabled_prefix).unwrap();
                let linked_discord = serde_json::to_string(&data.linked_discord).unwrap();
                let flags = serde_json::to_string(&data.flags).unwrap();
                rsx!{
                    tr {
                        td {
                            button { class: "delete", value: "{uuid}", "X" }
                        }
                        td { pre { "{uuid}" } }
                        td { pre { a { href: "#cos-{prefix}", "{prefix}" } } }
                        td { pre { "{data.connected}" } }
                        td { pre { "{linked_discord}" } }
                        td { pre { "{data.irc_blacklisted}" } }
                        td { pre { "{flags}" } }
                    }
                }
            })
        }
    )
}
fn meta<'a, 'b>(users: usize, cosmetics: usize) -> LazyNodes<'a, 'b> {
    rsx!(
          title {
            "DWS - {users} users, {cosmetics} cosmetics"
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
                    --font-mono: Ubuntu Mono, Fira Code, Roboto Mono, Sans Mono, monospace;
                }
                th:not(:first-child) {
                    min-width: 100px;
                    text-align: left;
                }
                tr { 
                    background-color: #111111; 
                }
                tr:nth-child(even) {
                    background-color: #262626
                }
                body {
                    font-family: Roboto, Sans, Arial;
                }
                pre {
                    font-family: var(--font-mono);
                }
                #uuids {
                    width: 36em;
                    font-family: var(--font-mono);
                }
                "#
                ]
          }
    )
}
fn uuid_to_username<'a, 'b>() -> LazyNodes<'a, 'b> {
    rsx!(
            h2 { "Uuid to username" }
            form {
                id: "uuids_to_username",
                textarea {
                    name: "uuids",
                    id: "uuids",
                    placeholder: "uuids split by new lines",
                    required: "true",
                }
                textarea {
                    name: "usernames",
                    id: "usernames",
                    placeholder: "Usernames will come here when converted",
                    readonly: "true",
                }
                button {
                    r#type: "submit",
                    "Convert"
                }
            }

    )
}
