# Calculator

## Step 01
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

## Step 02

Server's running. Now We can add a new route, the calculate route. 
The serve's is calculating a plus operation. That's it.

## Step 03

For this step We can add more operations. the Diagram remains the same.
To calculate. Let's create a domain.

In another moment We'll handle the errors properly. Not now.
Now the server can handle the basic operations. That's it

## Step 04

Now we can calculate some operations. In this step I will create a persistence file.
Here we intent to store to get a report after.

In the save we need to clone the operation.

To store the data. Here We use a CSV crate.

Alright! All the operations were saved into file. That's it.