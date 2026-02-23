import { createFileRoute } from "@tanstack/react-router";
import { createServerFn } from "@tanstack/react-start";

const getAgents = createServerFn({ method: "GET" }).handler(async () => {
  const { db } = await import("@/db");
  const agents = await db.agent.findMany();
  return agents;
});

export const Route = createFileRoute("/agents")({
  component: Page,
  loader: async () => {
    try {
      const agents = await getAgents();
      return agents;
    } catch (error) {
      console.log("test test", error);
    }
  },
});

function Page() {
  const agents = Route.useLoaderData();

  return (
    <ul>
      {agents?.map((a) => (
        <li key={a.id}>{a.id}</li>
      ))}
    </ul>
  );
}
