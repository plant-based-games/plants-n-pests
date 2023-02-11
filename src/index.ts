import express, { Express, Request, Response } from 'express'

export const hello = () => 'Hello world!\n'

console.log(hello())

const app: Express = express()
const port = 8000

app.get('/', (_req: Request, res: Response) => {
  res.send('This is plants-n-pests server...what do you want?')
})

app.listen(port, () => {
  console.log(`Plants-n-pests server up on ${port}`)
})
