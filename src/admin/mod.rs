use std::sync::Arc;

use axum::{extract::State, response::Html};
use dioxus::{prelude::*, ssr::render_lazy};

use crate::app_state::AppState;

pub mod users;

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
            script {
                    defer: "true",
                   vec![ r##"
                    const form = document.getElementById("add-user");
                   form.addEventListener('submit',async (e) => {
                        e.preventDefault();
                        e.stopPropagation();
                        const data = JSON.stringify(Object.fromEntries([...new FormData(form).entries()].filter(x=>x[1] != "")));
                        let res = await fetch("/users", {
                            method: "POST",
                            body: data,
                            headers: {
                                "Content-Type": "application/json"
                            }
                        })
                        if(res.status == 200) {
                            alert("User added");
                            window.location.reload();
                        } else {
                            alert(`Error adding user: ${await res.text()}`);
                        }
                    });
                "##]
                }
          }
    }))
}
