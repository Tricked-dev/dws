use std::sync::Arc;

use axum::{extract::State, response::Html};
use dioxus::{prelude::*, ssr::render_lazy};

use crate::app_state::AppState;

pub mod cosmetics;
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
                            th:not(:first-child) {
                                min-width: 100px;
                                text-align: left;
                            }
                            tr{background-color: #111111;}
                            tr:nth-child(even){background-color: #262626}
                            #admin {
                                margin: 0 auto;
                                width: 100%;
                                max-width: 100rem;
                            }
                        "#
                    ]
                }


            }
        body {
            id: "admin",
            div {
                h1 { "Admin" }
                h2 { "Users" }
                table {
                    tr {
                        th { "" }
                        th { "Username" }
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
                    }})
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
                    const elements = document.getElementsByClassName("delete");
                    for(let i = 0; i < elements.length; i++) {
                        elements[i].addEventListener("click", async (e) => {
                            let res = await fetch(`/users?uuid=${e.target.value}`, {
                                method: "DELETE"
                            })
                            if(res.status == 200) {
                                alert("User deleted");
                                window.location.reload();
                            } else {
                                alert(`Error deleting user: ${await res.text()}`);
                            }
                        })
                    }
                    const cform = document.getElementById("add-cosmetic");
                    cform.addEventListener('submit',async (e) => {
                        e.preventDefault();
                        e.stopPropagation();
                        const d = Object.fromEntries([...new FormData(cform).entries()].filter(x=>x[1] != ""));
                        d.id = parseInt(d.id);
                        d.required_flags = parseInt(d.required_flags);
                        d.type = parseInt(d.type);
                        const data = JSON.stringify(d);
                        let res = await fetch("/cosmetics", {
                            method: "POST",
                            body: data,
                            headers: {
                                "Content-Type": "application/json"
                            }
                        })
                        if(res.status == 200) {
                            alert("Cosmetic added");
                            window.location.reload();
                        } else {
                            alert(`Error adding cosmetic: ${await res.text()}`);
                        }
                    });

                    const celements = document.getElementsByClassName("cdelete");
                    for(let i = 0; i < celements.length; i++) {
                        celements[i].addEventListener("click", async (e) => {
                            let res = await fetch(`/cosmetics?id=${e.target.value}`, {
                                method: "DELETE"
                            })
                            if(res.status == 200) {
                                alert("Cosmetic deleted");
                                window.location.reload();
                            } else {
                                alert(`Error deleting cosmetic: ${await res.text()}`);
                            }
                        })
                    }
                "##]
                }
          }
    }))
}
