export class Game {
  private state: GameState = 'no players'
  private playerCount: 3 | 4 = 4

  login(): number | StatusCode {
    switch (this.state) {
      case 'no players':
        return 1
      case '1 player':
        return 2
      case '2 players':
        if (this.playerCount === 3) {
          this.state = 'ready'
        }
        return 3
      case '3 players':
        if (this.playerCount === 3) {
          return statusCode(400)
        }
        this.state = 'ready'
        return 4
      default:
        return statusCode(400)
    }
  }

  getState(): GameState {
    return this.state
  }

  playContract(playerId: number, cardId?: string): GameState | StatusCode {
    console.log(`process player ${playerId} played contract card ${cardId}`)
    return this.state
  }

  play(
    playerId: number,
    cardId?: string,
    xLocation?: string,
    yLocation?: string,
  ): GameState | StatusCode {
    console.log(
      `Processed player ${playerId} played card ${cardId} at location (${xLocation}, ${yLocation})`,
    )
    return this.state
  }

  draft(playerId: number, cardId?: string): GameState | StatusCode {
    console.log(`Processed player ${playerId} drafted card ${cardId}`)
    return this.state
  }
}

type PlayersJoining = 'no players' | '1 player' | '2 players' | '3 players'
export type GameState = PlayersJoining | 'ready'

export type StatusCode = { kind: 'status'; code: number }

const statusCode = (code: number): StatusCode => ({ kind: 'status', code })
