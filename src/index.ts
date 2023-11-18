import eventsource from "eventsource";
// @ts-ignore
global.EventSource = eventsource;

import { Elysia, t } from "elysia";
import PocketBase from "pocketbase"
import { EnvConfig } from "./config"

const pocketbase = new PocketBase(EnvConfig.pocketbaseUrl);
pocketbase.admins.authWithPassword(EnvConfig.pocketbaseAdminEmail, EnvConfig.pocketbaseAdminPassword)


const app = new Elysia().get("/", () => "Hello Elysia").get("/user/:id", async ({ params: { id } }) => {
  const result = await pocketbase.collection("users").getOne(id)
  return result
}, {
  params: t.Object({
    id: t.String()
  })
}).listen(EnvConfig.PORT);

console.log(
  `🦊 Elysia is running at ${app.server?.hostname}:${app.server?.port}`
);
