import express, { Express, Request, Response } from 'express'
import cookieSession from 'cookie-session'
import { hello } from './hello.js'
import * as dotenv from 'dotenv'
import { z } from 'zod'

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

app.post('/login', (req: Request, res: Response) => {
  const cookie: Cookie = { player: 1 }
  req.session = cookie
  res.send(cookie)
})

app.post('/draft/:cardId', (req: Request, res: Response) => {
  console.log(req.params['cardId'])
  if (req.session && req.session['player']) {
    const player = req.session['player']
    res.send(`processed player ${player} card draft`)
  } else {
    res.status(401).end()
  }
})

app.post('/play/:cardId/:xLocation/:yLocation', (req: Request, res: Response) => {
  console.log(req.params['cardId'], req.params['xLocation'], req.params['yLocation'])
  if (req.session && req.session['player']) {
    const player = req.session['player']
    res.send(`processed player ${player} card play`)
  } else {
    res.status(401).end()
  }
})

app.listen(port, () => {
  console.log(`Plants-n-pests server up on ${port}`)
})
