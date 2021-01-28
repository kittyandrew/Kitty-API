# Documentation for "Kitty-API"
## TODO: WARNING - work in progress, documentation might not be relevant nor complete

### Global options
&lt;delay&gt; - value in seconds (from 1 to 10), prepended to any request, will make server wait for that amount before processing the request.  
&lt;page&gt; - if endpoint states that it supports pagination, you can prepend to get certain amount of items (paginated).

### Users Endpoint
*Supports pagination*
##### /api/users
* POST
* GET
* DELETE
##### /api/users/&lt;id&gt;
* POST
* PUT
* GET
* DELETE

### Accounts Endpoint
##### /api/accounts/register
* POST
##### /api/accounts/login
* POST
