

### Task Overview

#### Task 1: "Everything is OK"
- **Objective**: Deploy a minimal web application.
- **Requirements**: 
  - The root endpoint `/` should respond to GET requests with a 200 OK status code.
  - The Hello world exemple will be enough

#### Task 2: "Fake error" (Bonus)
- **Objective**: Add an additional endpoint to the web app.
- **Requirements**: 
  - This new endpoint, `/-1/error`, should respond to GET requests with a 500 Internal Server Error status code.
  - The content of the response is not important.

### Solution Approach

#### For Task 1
- **Tools Used**: Axum (a web framework in Rust).
- **Implementation**: 
  - Set up a basic web server using Axum.
  - Define a route for the root endpoint `/`.
  - The route handler for this endpoint sends back a response "Hello, world!" with a 200 OK status.

#### For Task 2
- **Additional Setup**: 
  - Extend the Axum server configuration to include an additional route for `/-1/error`.
  - Create a route handler for this new endpoint that deliberately sends a 500 Internal Server Error response.


