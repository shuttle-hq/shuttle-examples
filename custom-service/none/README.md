This template shows how to create your own custom Shuttle service that can run anything HTTP-based.

## Example usage

Set up whatever you want to do in the `bind()` function, then visit <http://localhost:8000> (or your relevant routes) to try it out.

Variables from resource annotations can be added to `MyService` struct to use them in the `bind()` function.
