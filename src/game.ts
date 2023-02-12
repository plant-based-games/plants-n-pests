export class Game {
  private state: GameState = 'no players'
  private playerCount: 3 | 4 = 4

  login(): number | Status {
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
          return { code: 400 }
        }
        this.state = 'ready'
        return 4
      default:
        return { code: 400 }
    }
  }

  getState(): GameState {
    return this.state
  }

  playContract(playerId: number, cardId?: number): GameState | Status {
    console.log(`process player ${playerId} played contract card ${cardId}`)
    return this.state
  }

  play(playerId: number, cardId?: number, xLocation?: number, yLocation?: number) {
    console.log(
      `Processed player ${playerId} played card ${cardId} at location (${xLocation}, ${yLocation})`,
    )
  }

  draft(playerId: number, cardId?: number) {
    console.log(`Processed player ${playerId} drafted card ${cardId}`)
  }
}

type PlayersJoining = 'no players' | '1 player' | '2 players' | '3 players'
type GameState = PlayersJoining | 'ready'

type Status = { code: number }
