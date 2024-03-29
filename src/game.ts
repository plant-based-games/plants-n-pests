import type { Config } from './config.js'
import type { Deck, Garden, Hand, Market } from './model.js'
import { generateDraftDeck, generateMarket, makeChance } from './random.js'

export const makeGame = (config: Config): Game => {
  const chance = makeChance(config.seed)
  const market = generateMarket(chance)
  const draftDeck = generateDraftDeck(chance)
  return new Game(config.playerCount, market, draftDeck)
}

class Game {
  private state: State = new PlayersJoining(0)

  constructor(
    private readonly playerCount: 3 | 4,
    private readonly market: Market,
    private readonly draftDeck: Deck,
  ) {
    console.log(this.market)
    console.log(this.draftDeck)
  }

  login(): number {
    if (this.state.stateName !== 'players-joining') {
      throw new UserError(400)
    }
    switch (this.state.count) {
      case 0:
      case 1:
        this.state = this.state.increment()
        return this.state.count
      case 2:
        this.state =
          this.playerCount === 3 ? this.initDraftPhase() : this.state.increment()
        return 3
      case 3:
        if (this.playerCount === 3) {
          throw new UserError(400)
        }
        this.state = this.initDraftPhase()
        return 4
    }
  }

  getState(): State {
    console.log(`current state is: ${this.state.stateName}`)
    return this.state
  }

  playContract(playerId: number, cardId?: string): State {
    console.log(`process player ${playerId} played contract card ${cardId}`)
    return this.state
  }

  play(playerId: number, cardId?: string, xLocation?: string, yLocation?: string): State {
    console.log(
      `Processed player ${playerId} played card ${cardId} at location (${xLocation}, ${yLocation})`,
    )
    return this.state
  }

  draft(playerId: number, cardId?: string): State {
    console.log(`Processed player ${playerId} drafted card ${cardId}`)
    return this.state
  }

  private initDraftPhase(): DraftPhase {
    return new DraftPhase(this.market, [], [])
  }
}

class PlayersJoining {
  public readonly stateName: 'players-joining' = 'players-joining' as const
  constructor(public readonly count: 0 | 1 | 2 | 3) {}

  increment(): PlayersJoining {
    if (this.count === 3) {
      return this
    }
    return new PlayersJoining((this.count + 1) as 3)
  }
}

class DraftPhase {
  public readonly stateName: 'draftPhase' = 'draftPhase' as const

  constructor(market: Market, playerHands: Hand[], garden: Garden) {
    console.log(market, playerHands, garden)
  }
}

class RoundPhase {
  public readonly stateName: 'roundPhase' = 'roundPhase' as const
  constructor(public readonly roundNumber: 1 | 2 | 3) {}
}

export type State = PlayersJoining | DraftPhase | RoundPhase

class UserError extends Error {
  constructor(public readonly statusCode: number) {
    super(`http request user error ${statusCode}`)
  }
}
