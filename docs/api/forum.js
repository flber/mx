const http = require("http")
const address = "157.131.93.70"

async function getInputValue(){
	let input = document.getElementById("comment").value
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
	req.write(input)
	req.end()
	
	let data = response.json()
	console.log(data)
}

