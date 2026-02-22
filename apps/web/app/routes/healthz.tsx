import type { Route } from "./+types/healthz";

export function loader() {
  return Response.json({ status: "ok" });
}

export default function Healthz({ loaderData }: Route.ComponentProps) {
  return <pre>{JSON.stringify(loaderData)}</pre>;
}
