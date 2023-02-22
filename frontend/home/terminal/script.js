

const formSubmitButton = document.getElementById("form-submit-button");

formSubmitButton.addEventListener('click', function (event) {
    console.log("form submitted")
    event.preventDefault();
    const request = new XMLHttpRequest();
    const form = document.getElementById('choose-terminal-form');
    const formData = new FormData(form);
    request.open('POST', '/choose_terminal');
    request.send(formData);
    updateIframe(form)
});

function updateIframe(form) {
    const elements = form.elements;
    const port = elements["port"];
    document.getElementById("terminal-iframe").src = "http://localhost:" + port
}