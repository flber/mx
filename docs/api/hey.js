const http = require("http")
const fs = require("fs");
const address = fs.readFileSync("/home/benh/ip/ip.txt");

module.exports = function(request, response){ 
	let options = {
		hostname: address,
		port: 5000,
		path: "/",
		method: "GET",
	}

	let innerRequest = http.request(options, innerResponse => {
			innerResponse.on("data", data => {
			response.send(data)
		})
	})

	innerRequest.on("error", error => response.status(500).send("Something went wrong! That's all we know. :("))
	innerRequest.end()

	console.log("sent data")
}
