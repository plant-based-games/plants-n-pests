export class Game {
  private state: State = new PlayersJoining(0)

  constructor(private readonly playerCount: 3 | 4) {}

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
        this.state = this.playerCount === 3 ? new DraftPhase() : this.state.increment()
        return 3
      case 3:
        if (this.playerCount === 3) {
          throw new UserError(400)
        }
        this.state = new DraftPhase()
        return 4
    }
  }

  getState(): State {
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

/*
class GameState {}
*/

class DraftPhase {
  public readonly stateName: 'draftPhase' = 'draftPhase' as const
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
