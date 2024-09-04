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

I forgot to mark the time when the data was created. Let's do that.
I'll create a common crate to hold these functions that it will be common between the modules

To implement this function I need chrono.
Now it is really finished. That's it.

## Step 05
To retrieve the data. It's necessary to read the file.
So to accomplish that. In this step, I will read the file and give a response of a vector 
of Report Response. The diagram It's like that.

The server is capable to answer the operations already made. That's it.

## Step 06
The get data from file can be very complicate when there are a lot of data to retrieve.
To make things easier We can switch to a database engine like MySQL or Postgres.
The last one will be our choice here. For that, We need a database. To provide it. I'll use docker for accomplish that. I faced some problems to put and retrieve data from postgres. I solved that problem looking into crate's page.

I'll create the table to follow the conversion rules.
The docker compose file I'll get from C project in SAT library.

Now the application is able to get data from postgres database. That's it. Bye.

## Step 07

Now we have a database. But imagine yourself one thing. We are getting all data from the database. We need to set a limit for that, and on more thing. We'll make the server capable to retrieve data through a period.

The query are optional. If they exist we need to consider in our sql query.
Going to report route we need to create a DTO to receive the query to fill our structure

Now we need a function to verify if the string was received is in the right format.

Now I'll clear the database.

Our filter is working...
Now we can get the data using query. On thing here is: We get only 20 records. We need to give an information how many pages there are in the database. That's it. Bye.

## Step 08
We already have a server working. Well, this is a good point to test it. Let's benchmark the server to see how many requests we get without errors.

For do that. I'll use Apache HTTP server benchmarking tool.

I'll clear the database to see how many registers will be there after the test.

The result was 135s to complete the 1000 requests. No errors.
An average of 1 request in 7 seconds
Transfer rate 1.4 Kbytes/sec
All 1000 registers into database

I'll increment to 100 clients

For this new setup
We got 85 requests errors
We got 915 valid requests
Requests per second:    29.46 [#/sec] (mean)

There are some ways to improve that, but it is good to have a performance idea about how our server is performing.

That's it. Bye
