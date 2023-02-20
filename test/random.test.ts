import { newChance, generateDraftDeck } from '../src/random.js'

describe('generateDraftDeck', () => {
  it('generates the same draft deck given the same seed', () => {
    const makeTestDeck = (seed: number | undefined) => generateDraftDeck(newChance(seed))
    const draftDeck1 = makeTestDeck(7777)
    expect(draftDeck1.slice(0, 8)).toStrictEqual([12, 3, 10, 7, 6, 12, 16, 19])
    const draftDeck2 = generateDraftDeck(newChance(5555))
    expect(draftDeck2.slice(0, 8)).toStrictEqual([8, 3, 8, 7, 8, 13, 5, 1])
    const draftDeck3 = generateDraftDeck(newChance(undefined))
    expect(draftDeck3).not.toStrictEqual(draftDeck1)
    expect([draftDeck1, draftDeck2, draftDeck3].map((list) => list.length)).toStrictEqual(
      Array(3).fill(110),
    )
  })
})
