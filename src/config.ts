import { z } from 'zod'
import type { Infer } from './zod-helper'

type Config = Infer<typeof Config>

export const Config = z.object({
  cookieSecret: z.string().min(40),
  playerCount: z.preprocess((v) => Number(v), z.union([z.literal(3), z.literal(4)])),
})

type Raw<T> = Record<keyof T, unknown>
type Env = Record<string, string | undefined>

export const loadConfig = (env: Env): Config => {
  const getEnv = getEnvFactory(env)
  const raw: Raw<Config> = {
    cookieSecret: getEnv('cookie_secret'),
    playerCount: getEnv('player_count'),
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
