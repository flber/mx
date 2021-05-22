const http = require("http")
const fs = require("fs");
const address = fs.readFileSync("/home/benh/ip/ip.txt");

// module.exports = function(request, response){
	// let options = {
		// hostname: address,
		// port: 5000,
		// path: "/",
		// method: "GET",
	// }
// 
	// let innerRequest = http.request(options, innerResponse => {
		// innerResponse.on("data", data => {
			// response.send(data)
		// })
	// })
// 
	// innerRequest.on("error", error => response.status(500).send("Something went wrong! That's all we know. :("))
	// innerRequest.end()
// }

function sayhi(){
	let options = {
		hostname: address,
		port: 5000,
		path: "/",
		method: "GET",
	}

	let data = null

	let request = http.request(options, response => {
		response.on("data", tmpData => {
			data = tmpData
		})
	})

	let countElement = document.getElementById("count") 
	document.getElementById("hi-button").disabled = true; 
	countElement.innerHTML = `Visitors: ${data.count} so far! Welcome on in ^_^` 

	request.on("error", error => response.status(500).send("Something went wrong! That's all we know. :("))
	request.end()
}
