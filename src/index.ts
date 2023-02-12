import express, { Express, Request, Response } from 'express'
import cookieSession from 'cookie-session'
import { hello } from './hello.js'
import * as dotenv from 'dotenv'
import { z } from 'zod'
import { Game, GameState } from './game.js'

dotenv.config()

const Player = z.number().int().min(1).max(4)
const Cookie = z.object({ player: Player })
type Player = z.infer<typeof Player>
type Cookie = z.infer<typeof Cookie>

const cookieSecret = process.env['cookie_secret']
if (cookieSecret === undefined) {
  throw new Error('cookie_secret env var is undefined :(')
}

console.log(hello())

const app: Express = express()
app.disable('x-powered-by')
app.use(express.static('public'))
app.use(express.json())
app.use(
  cookieSession({
    name: 'session',
    keys: [cookieSecret],
    httpOnly: true,
    maxAge: 24 * 60 * 60 * 1000, // 24 hours
  }),
)
const port = 8000

const game = new Game()

const doIfValidPlayer = (
  req: Request,
  res: Response,
  f: (playerId: number) => GameState,
) => {
  const result = Cookie.safeParse(req.session)
  if (result.success === true) {
    const gameState = f(result.data.player)
    res.send(gameState)
  } else {
    res.status(401).end
  }
}

app.post('/login', (req: Request, res: Response) => {
  const result = game.login()
  const cookie: Cookie = { player: result }
  req.session = cookie
  res.send(cookie)
})

app.post('/play-contract/:cardId', (req: Request, res: Response) => {
  doIfValidPlayer(req, res, (playerId) =>
    game.playContract(playerId, req.params['cardId']),
  )
})

app.post('/play/:cardId/:xLocation/:yLocation', (req: Request, res: Response) => {
  doIfValidPlayer(req, res, (playerId) =>
    game.play(
      playerId,
      req.params['cardId'],
      req.params['xLocation'],
      req.params['yLocation'],
    ),
  )
})

app.post('/draft/:cardId', (req: Request, res: Response) => {
  doIfValidPlayer(req, res, (playerId) => game.draft(playerId, req.params['cardId']))
})

app.listen(port, () => {
  console.log(`Plants-n-pests server up on ${port}`)
})
