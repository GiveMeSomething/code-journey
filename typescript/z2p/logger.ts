import { Span } from "@opentelemetry/api";
import { Request } from "express";

export default class CustomLogger {
  private span: Span;
  private route: string;

  constructor(request: Request, span: Span) {
    this.span = span;
    this.route = `${request.method} ${request.url}`;
  }

  getPrefix(): string {
    const { traceId, spanId } = this.span.spanContext();
    const payload = [`Trace ID ${traceId}`, `Span ID ${spanId}`, this.route];
    return payload.join("\t");
  }

  log(message?: any, ...optionalParams: any[]): void {
    console.log(this.getPrefix(), "\t\t", message, ...optionalParams);
  }

  error(message?: any, ...optionalParams: any[]): void {
    console.error(this.getPrefix(), "\t\t", message, ...optionalParams);
  }
}
