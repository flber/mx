async function postData(data = {}){
	const response = await fetch("http://157.131.93.70:8080", {
	    method: 'POST',
	    headers: {
	      'Content-Type': 'application/json'
	    },
	    redirect: 'follow',
	    body: JSON.stringify(data)
  	});
	return response.json(); // parses JSON response into native JavaScript objects
}

