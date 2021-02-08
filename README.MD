# Getting going with Rust Lambda's, API Gateway and Dynamodb in 2021

We want to deploy code easily when we're ready to ship, but we also want to quickly iterate locally.

If we want to include infrastructure that is elsewhere on the web then we could Terraform our stack instead using a combination of https://spin.atomicobject.com/2020/02/03/localstack-terraform-circleci/ and this https://medium.com/faun/how-to-use-aws-dynamodb-locally-ad3bb6bd0163 

For the purposes of this exercise however we will use the AWS CDK, moreover the example provided by Codetalkio that utilizes the Netlify Rust runtime.

## Running the template

First we can clone the repository.

``` bash
git clone https://github.com/codetalkio/patterns-serverless-rust-minimal.git
```

Then start the project to download the required Docker containers.

``` bash
npm run cdklocal:start
```

We can install the musl dependency by running

``` bash
npm run build
```

Then we can watch our code for changes by running 

``` bash
cargo watch -s 'npm run build:debug'
```

Once that's downloaded we can run the tests to check that they work.

## What are we building?

A REST API that has four entities that enable product management and engineers to objectively decide on the health of the system.

These entites are based on SRE Principles and include:
- The objectives, generally decided by the Product managers and dictated by what is possible within an SLA.
- The indicators, what aspects of the application's behaviour do we need to observe to understand if we can achieve our SLO.
- The error budgets, in a specific time window how close are we to breaking the SLO threshold and upsetting the customers.
- The users, the product manager or engineer who wants to know how much error budget there is available to spend in a given time period, to help make better decisions about what features the teams can ship in a given sprint and where focus needs to be directed towards eliminating toil.

## What about the implementation?

Using Localstack we can choose what we have enabled by removing it from the docker-compose file.

``` diff
    environment:
-      - SERVICES=${SERVICES-serverless,cloudfront,cloudformation,iam,sts,sqs,ssm,s3,route53,acm,cloudwatch,cloudwatch-logs,lambda,dynamodb,apigateway}
+      - SERVICES=${SERVICES-serverless,iam,cloudwatch,cloudwatch-logs,lambda,dynamodb,apigateway}
```

Next we can rerun the container with `npm run cdklocal:start`.

We probably want understand how to test our components, we can do so with an invocation of our REST API endpoints using a Rust Test Client.

TODO CHOOSE TEST CLIENT