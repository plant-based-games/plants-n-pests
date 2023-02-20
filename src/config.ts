import { z } from 'zod'
import type { Infer } from './zod-helper'

export const Config = z.object({
  cookieSecret: z.string().min(40),
  playerCount: z.preprocess((v) => Number(v), z.union([z.literal(3), z.literal(4)])),
  seed: z.preprocess(
    (v) => (v === undefined ? v : Number(v)),
    z.number().int().positive().min(9).optional(),
  ),
})

type Config = Infer<typeof Config>
type Raw<T> = Record<keyof T, unknown>
type Env = Record<string, string | undefined>

export const loadConfig = (env: Env): Config => {
  const getEnv = getEnvFactory(env)
  const raw: Raw<Config> = {
    cookieSecret: getEnv('cookie_secret'),
    playerCount: getEnv('player_count'),
    seed: env['seed'],
  }
  return Config.parse(raw)
}

const getEnvFactory =
  (env: Env) =>
  (name: string): string => {
    const value = env[name]
    if (value === undefined) {
      throw new Error(`${name} env var is undefined :(`)
    }
    return value
  }
