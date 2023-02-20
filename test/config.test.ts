import { loadConfig } from '../src/config.js'

describe('loadConfig', () => {
  const makeSecret = () => 'x'.repeat(40)

  it('loads config values from env vars and parses them according to schemas', () => {
    const env = {
      cookie_secret: makeSecret(),
      player_count: '3',
    }
    expect(loadConfig(env)).toStrictEqual({
      cookieSecret: makeSecret(),
      playerCount: 3,
      seed: undefined,
    })
  })

  it('also loads optional seed env var, if present', () => {
    const env = {
      cookie_secret: makeSecret(),
      player_count: '3',
      seed: '7777',
    }
    expect(loadConfig(env)).toStrictEqual({
      cookieSecret: makeSecret(),
      playerCount: 3,
      seed: 7777,
    })
  })

  it('fails if cookie_secret env var is missing', () => {
    const env = {
      player_count: '3',
    }
    expect(() => loadConfig(env)).toThrow('cookie_secret env var is undefined :(')
  })

  it('fails if player_count env var is missing', () => {
    const env = {
      cookie_secret: makeSecret(),
    }
    expect(() => loadConfig(env)).toThrow('player_count env var is undefined :(')
  })

  it('fails if cookie_secret is less than 40 chars', () => {
    const env = {
      cookie_secret: 'x'.repeat(39),
      player_count: '4',
    }
    expect(() => loadConfig(env)).toThrow(/String must contain at least 40 character/)
  })

  it('fails if player_count is not 3 or 4', () => {
    const env = {
      cookie_secret: makeSecret(),
      player_count: '2',
    }
    expect(() => loadConfig(env)).toThrow(/Invalid literal/)
  })

  it('fails if seed is present but not a number', () => {
    const env = {
      cookie_secret: makeSecret(),
      player_count: '3',
      seed: 'not-a-number',
    }
    expect(() => loadConfig(env)).toThrow(/Expected number, received nan/)
  })

  it('fails if seed is present but less than 9', () => {
    const env = {
      cookie_secret: makeSecret(),
      player_count: '3',
      seed: '8',
    }
    expect(() => loadConfig(env)).toThrow(/Number must be greater than or equal to 9/)
  })
})
