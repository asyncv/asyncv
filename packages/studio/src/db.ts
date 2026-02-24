import { ZenStackClient } from "@zenstackhq/orm";
import { SqliteDialect } from "@zenstackhq/orm/dialects/sqlite";
import SQLite from "better-sqlite3";
import { schema } from "@/generated/schema";

if (process.env.DATABASE_URL == null) {
  throw new Error("DATABASE_URL should be set");
}

export const db = new ZenStackClient(schema, {
  dialect: new SqliteDialect({
    database: new SQLite(process.env.DATABASE_URL),
  }),
});

