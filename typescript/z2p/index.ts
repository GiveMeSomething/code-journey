import express from "express";
import dotenv from "dotenv";

dotenv.config();

const app = express();
const port = process.env.PORT || 3000;

app.get("/", (_, response) => {
  response.json({
    message: "Hello World",
  });
});

app.listen(port, () => {
  console.log(`Express server running at http://localhost:${port}`);
});
