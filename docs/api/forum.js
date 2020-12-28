const http = require("http")
const address = "157.131.93.70"

module.exports = function(request, response){
	let data = JSON.stringify(request)
	let options = {
		hostname: address,
		port: 8080,
		path: "/",
		method: "POST"
    }

	let req = http.request(options, res => {
		res.on("data", data => {
	    	response.send(data)
		})
	})

	req.on("error", error => console.error(error))
	req.write(data)
	req.end()
}
