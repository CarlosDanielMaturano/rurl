The project only contain core functionalities:

- To **create** a new url: `curl -X POST http://{host}/new?url={url}`
  Where 'host' is where the project is running, and url is a valid url.
  A Invalid or empty url will return the following :

  ```json
  {
    "err": "400 Bad Request",
    "message": "The given url is not a valid url",
    "status": 400
  }
  ```

  If everthing went ok:

  ```json
  {
    "message": "Sucessfully created a new shorten url",
    "shorten_url": {
      "hash": {hash},
      "url": {url}
    },
    "status": 201
  }
  ```

  Where `url` is the url that was given in the parameter.
  And hash is a new random hash consisting of 18 numerical digits.

- To **view** information about a existing shorten_url: `curl -X GET http://{host}/view/{hash}`
  Again, "host" is where the application is running.
  The "hash" is the hash value returned when creating a new url.
  If everthing suceeds, it will return:

  ```json
  {
    "message": "Sucessfully found the url.",
    "shorten_url": {
      "hash": "11247712352324810010",
      "url": "https://github.com/"
    },
    "status": 302
  }
  ```

  Otherwhise:

  ```json
  {
    "err": "404 Not Found",
    "message": "Could not find a shorten url with the given hash",
    "status": 404
  }
  ```

- To **delete** a existing url: `curl -X DELETE http://{host}/delete/{hash}`
  If nothing went wrong, it will return:

  ```json
  {
    "message": "Sucessfully deleted the url.",
    "status": 200
  }
  ```

  Again, if the program could not find a shorten_url with the given hash, it will return:

  ```json
  {
    "err": "404 Not Found",
    "message": "Could not find a shorten url with the given hash",
    "status": 404
  }
  ```

- To get **redirected** to the original url: `curl -X GET http://{host}:`
  If it coul not find the shorten_url:

  ```json
  {
    "err": "404 Not Found",
    "message": "Could not find a shorten url with the given hash",
    "status": 404
  }
  ```

Defaults response:

- If a route does not exists, the program with return:

  ```json
  {
    "err": "404 Not Found",
    "message": "Sorry. The server was unable to found what are you looking for.",
    "status": 404
  }
  ```

- If for some reason a internal server occurs:

  ```json
  {
    "err": "500 Internal Server Error",
    "message": "Sorry. A unknow internal error occurred.",
    "status": 500
  }
  ```
