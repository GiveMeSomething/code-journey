import express, { Request, Response } from "express";
import dotenv from "dotenv";
import { Span, trace } from "@opentelemetry/api";
import { NodeSDK } from "@opentelemetry/sdk-node";
import CustomLogger from "./logger";

// Extend default Request to hold request span
declare module "express-serve-static-core" {
  interface Request {
    span?: Span;
    logger?: CustomLogger;
  }
}

// Initialize dotenv
dotenv.config();

// Initialize opentelemetry
const sdk = new NodeSDK();
sdk.start();

// Initialize Express server
const app = express();
const port = process.env.PORT || 3000;

const tracer = trace.getTracer("z2p", "0.0.1");

app.use((request, _, next) => {
  tracer.startActiveSpan(`${request.method} ${request.url}`, (span) => {
    try {
      request.span = span;
      next();
    } finally {
      // Making sure that this span will end
      span.end();
    }
  });
});

app.use((request, _, next) => {
  // Inject logger into request if a span is available
  if (request.span) {
    request.logger = new CustomLogger(request, request.span);
  }

  next();
});

app.get("/", (request: Request, response: Response) => {
  console.log(request.span?.spanContext());
  response.json({
    message: "Hello World",
  });
});

app.get("/:name", (request: Request, response: Response) => {
  console.log(request.span?.spanContext());
  const name = request.params["name"];
  if (!name) {
    response.status(400).json({ message: "Invalid name" });
    return;
  }

  response.json({
    message: `Hello ${name}`,
  });
});

app.listen(port, () => {
  console.log(`Express server running at http://localhost:${port}`);
});
