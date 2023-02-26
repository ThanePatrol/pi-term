const form = document.querySelector("#choose-terminal-form");


form.addEventListener('submit', event => {
    console.log("form submitted");
    event.preventDefault();
    const formData = new FormData(form);
    console.log("form data: ")
    console.log(formData);

    const urlSearchParams = new URLSearchParams();
    for (const pair of formData.entries()) {
        urlSearchParams.append(pair[0], pair[1]);
    }

    fetch('/add_terminal', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded'
        },
        body: urlSearchParams.toString()
    })
        .then(response => {
            response.json().then(r => console.log(r));
        })
        .catch(error => {
            console.error(error);
        });

    updateIframe(form);
});

function updateIframe(form) {
    const elements = form.elements;
    const port = elements["port"].value;
    document.getElementById("terminal-iframe").src = "http://localhost:" + port;
}

