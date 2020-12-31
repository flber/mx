async function postData(data = {}){
	console.log("data received by postData is ", JSON.stringify(data))
	const response = await fetch("", {
	    method: 'POST',
	    headers: {
	      'Content-Type': 'application/json'
	    },
	    redirect: 'follow',
	    body: JSON.stringify(data)
  	});
	return response.json();
}

