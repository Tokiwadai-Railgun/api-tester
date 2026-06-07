# A simple cli to test http apis using. 
This program is meant to test apis automatically by providing a file with urls and expected status, it will so query all urls and compare what is expected for every case returning it in the console

> [!WARNING]
> Only work with JSON for now, and response comparaison not supported yet

## Planned
* [ ] Add option to export test cases to excel or other files 
* [ ] Add headers support

# Usage
This tools accept a ``.http`` file following the [Guidelines](https://http-files.org/spec/basics/) along with two custom anotations.

Test Names are in the separator, follow this [link](https://http-files.org/spec/separators/) for more info

> [!NOTE]
> I'm placing here references to the http-files documentation, please keep in mind that not all syntax documented there will be applicable here. 

Comments are only considered to start with "#" as of now

## Anotations
There are two anotations used for the test result comparaison : 
- ``@expect-status <status>`` -> check for the response status **NEEDED IN EACH TEST CASE**
- ``@expect-result <JsonResponse>`` -> check for the response body, Optional
- ``@save-cookies`` -> Save the response cookies
- ``@use-cookies`` -> Use the saved response cookies

## Example file
```http
### Test 1
GET https://google.com
# @expect-status 200

### Test Post request
POST https://dummyjson.com/posts/add
Content-Type: application/json

{
  "title": "I am in love with someone.", 
  "userId": 5, 
  "body": "Your post content here"
}

# @expect-status 201
# @expect-response {
# "userId":5, 
# "id":252,
# "title":"I am in love with someone.",
# "body":"Your post content here"
# }

### Test login

POST http://sample/api/login
Content-Type: application/json

{
  "username": "test",
  "password": "test"
}

# @expect-status 201
# @save-cookies

### Get all users

GET http://sample/api/users
# @expect-status 200
# @use-cookies
```


# Notes :
This implementation does not aim to fully cover the cases or the syntax, simply provide a lightweight tester for my own api automated testing.
Feel free to fork or open pull requests if you need more functionalities
