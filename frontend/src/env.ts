import { z } from "zod";

const envSchema = z.object({
  NODE_ENV: z.enum(["DEV", "PROD"]).default("DEV"),
  SERVER_URL: z.string().url().min(1),
});

export type ENV = z.infer<typeof envSchema>;
export const config: ENV = envSchema.parse(process.env);
