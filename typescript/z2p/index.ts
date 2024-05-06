import express from "express";
import dotenv from "dotenv";
import { trace } from "@opentelemetry/api";

dotenv.config();

const app = express();
const port = process.env.PORT || 3000;

const tracer = trace.getTracer("z2p", "0.0.1");

app.use((request, _, next) => {
  tracer.startActiveSpan(`${request.method} ${request.url}`, (span) => {
    try {
      next();
    } finally {
      // Making sure that this span will end
      span.end();
    }
  });
});

app.get("/", (_, response) => {
  response.json({
    message: "Hello World",
  });
});

app.listen(port, () => {
  console.log(`Express server running at http://localhost:${port}`);
});
