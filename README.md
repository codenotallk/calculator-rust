# Calculator

To learn Rust I'm trying to rewrite most my personal projects made in C to Rust.

Now isn't different. I'll convert the calculator project first implemented in C.
So far, I didn't finished yet, but I'll start to rewrite in Rust.

Let's see the project first

I have a couple steps to reach the last diagram. We'll start with a single server
and it starts to change along the steps.

The first step is create a server and set up a route with a name health.

For this project I'll use axum and tokio as base to implement the webserver.

The server is running, and it is providing us a health endpoint in JSON enconding.
That's it. Bye