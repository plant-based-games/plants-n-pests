import express, { Express, Request, Response } from 'express'
import cookieSession from 'cookie-session'
import { hello } from './hello.js'
import * as dotenv from 'dotenv'

dotenv.config()
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
  req.session = { player: 1 }
  res.send({ player: 1 })
})

app.post('/play/:cardId', (req: Request, res: Response) => {
  if (req.session && req.session['player']) {
    const player = req.session['player']
    res.send(`processed player ${player} turn`)
  } else {
    res.status(401).end()
  }
})

app.listen(port, () => {
  console.log(`Plants-n-pests server up on ${port}`)
})
