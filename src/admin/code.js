//@ts-check

//@ts-ignore
window.download = function (url, name) {
  var anchor = document.createElement("a");
  anchor.href = url;
  anchor.target = "_blank";
  anchor.download = name;
  anchor.click();
};

/**
 *
 * @param {HTMLFormElement} form
 * @returns {Record<string, string>}
 */
function formToObject(form) {
  // @ts-ignore
  return Object.fromEntries(
    [...new FormData(form).entries()].filter((x) => x[1] != "")
  );
}
const addUserForm = document.getElementById("add-user");
addUserForm?.addEventListener("submit", async (e) => {
  e.preventDefault();
  e.stopPropagation();
  // @ts-ignore
  const data = JSON.stringify(formToObject(addUserForm));
  let res = await fetch("/users", {
    method: "POST",
    body: data,
    headers: {
      "Content-Type": "application/json",
    },
  });
  if (res.status == 200) {
    alert("User added");
    window.location.reload();
  } else {
    alert(`Error adding user: ${await res.text()}`);
  }
});

const elements = document.getElementsByClassName("delete");
for (let i = 0; i < elements.length; i++) {
  elements[i].addEventListener("click", async (e) => {
    // @ts-ignore
    let res = await fetch(`/users?uuid=${e?.target?.value}`, {
      method: "DELETE",
    });
    if (res.status == 200) {
      alert("User deleted");
      window.location.reload();
    } else {
      alert(`Error deleting user: ${await res.text()}`);
    }
  });
}

const addCosmeticsForm = document.getElementById("add-cosmetic");
// @ts-ignore
addCosmeticsForm.addEventListener("submit", async (e) => {
  e.preventDefault();
  e.stopPropagation();
  /* @type {Record<string,string>} */
  // @ts-ignore
  const d = formToObject(addCosmeticsForm);
  // @ts-ignore
  d.id = parseInt(d.id);
  // @ts-ignore
  d.required_flags = parseInt(d.required_flags);
  // @ts-ignore
  d.type = parseInt(d.type);
  const data = JSON.stringify(d);
  let res = await fetch("/cosmetics", {
    method: "POST",
    body: data,
    headers: {
      "Content-Type": "application/json",
    },
  });
  if (res.status == 200) {
    alert("Cosmetic added");
    window.location.reload();
  } else {
    alert(`Error adding cosmetic: ${await res.text()}`);
  }
});

const celements = document.getElementsByClassName("cdelete");
for (let i = 0; i < celements.length; i++) {
  celements[i].addEventListener("click", async (e) => {
    // @ts-ignore
    let res = await fetch(`/cosmetics?id=${e.target.value}`, {
      method: "DELETE",
    });
    if (res.status == 200) {
      alert("Cosmetic deleted");
      window.location.reload();
    } else {
      alert(`Error deleting cosmetic: ${await res.text()}`);
    }
  });
}

(() => {
  let form = document.getElementById("broadcast");
  // @ts-ignore
  form.addEventListener("submit", async (e) => {
    e.preventDefault();
    e.stopPropagation();
    const data = Object.fromEntries(
      // @ts-ignore
      [...new FormData(form).entries()].filter((x) => x[1] != "")
    );
    // @ts-ignore
    data["to"] = [];
    console.log(data);
    let res = await fetch("/broadcast", {
      method: "POST",
      body: JSON.stringify(data),
      headers: {
        "Content-Type": "application/json",
      },
    });
    if (res.status == 200) {
      alert("Broadcast sent");
    } else {
      alert(`Error sending broadcast: ${await res.text()}`);
    }
  });
  const fform = document.getElementById("uuids_to_username");
  fform?.addEventListener("submit", async (e) => {
    e.preventDefault();
    e.stopPropagation();
    // @ts-ignore
    const uuids = fform.elements["uuids"].value
      .split("\n")
      .map((x) => x.trim().split(" "))
      .flat()
      .map((x) => x.trim())
      .filter((x) => x != "");
    let res = await fetch("/uuids_to_usernames", {
      method: "POST",
      body: JSON.stringify(uuids),
      headers: {
        "Content-Type": "application/json",
      },
    });
    if (res.status == 200) {
      let data = await res.json();
      let textarea = document.getElementById("usernames");
      // @ts-ignore
      textarea.value = data.map((x) => x.name).join("\n");
      // @ts-ignore
      textarea.style.height = 0;
      // @ts-ignore
      textarea.style.height = textarea.scrollHeight + "px";
    } else {
      alert(`Error converting uuids: ${await res.text()}`);
    }
  });
})();

const txs = document.getElementsByTagName("textarea");
for (let tx of txs) {
  tx.setAttribute(
    "style",
    "height:" + tx.scrollHeight + "px;overflow-y:hidden;"
  );
  tx.addEventListener("input", OnInput, false);
}

function OnInput() {
  this.style.height = 0;
  this.style.height = this.scrollHeight + "px";
}
