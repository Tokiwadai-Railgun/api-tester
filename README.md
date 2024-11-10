# A simple cli to test http apis using. 
For now, only testing the response status and only available for get requests

# Usage
First you'll need to write all your cases in a file with the following syntax 
```
[testName]
GET url
ExpectedStatus
```
then simply execute ``api-tester fileName``

## Example
requests.http
```
[Try to get cats]
GET localhost:3000/cats
200
```

``api-tester requests.http``



# Notes :
As this is a first project in rust it might not be optimal, expect bugs and not full functionalities
