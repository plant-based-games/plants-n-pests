import express, { Express, Request, Response } from 'express'
import { hello } from './hello.js'

console.log(hello())

const app: Express = express()
const port = 8000

app.get('/', (_req: Request, res: Response) => {
  res.send('This is plants-n-pests server...what do you want?')
})

app.listen(port, () => {
  console.log(`Plants-n-pests server up on ${port}`)
})
