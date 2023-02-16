const WebSocket = require('ws');

const server = new WebSocket.Server({ port: 8080})

server.on('connection', (socket) => {
    console.log("client connected");

    socket.on("message", (message) => {
        console.log(`received message: ${message}`);
        //message sent back to client
        socket.send(message);
    });

    socket.on("close", () => {
        console.log('Client disconnected');
    });
});

console.log("WebSocket server started");