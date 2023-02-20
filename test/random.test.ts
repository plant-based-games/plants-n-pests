import { makeChance, generateDraftDeck, generateMarket } from '../src/random.js'

describe('generateDraftDeck', () => {
  it('generates the same draft deck given the same seed', () => {
    const makeTestDeck = (seed: number | undefined) => generateDraftDeck(makeChance(seed))
    const draftDeck1 = makeTestDeck(7777)
    expect(draftDeck1.slice(0, 8)).toStrictEqual([12, 3, 10, 7, 6, 12, 16, 19])
    const draftDeck2 = generateDraftDeck(makeChance(5555))
    expect(draftDeck2.slice(0, 8)).toStrictEqual([8, 3, 8, 7, 8, 13, 5, 1])
    const draftDeck3 = generateDraftDeck(makeChance(undefined))
    expect(draftDeck3).not.toStrictEqual(draftDeck1)
    expect([draftDeck1, draftDeck2, draftDeck3].map((list) => list.length)).toStrictEqual(
      Array(3).fill(110),
    )
  })
})

describe('generateMarket', () => {
  it('generates the same market given the same seed', () => {
    const makeTestMarket = (seed: number | undefined) => generateMarket(makeChance(seed))
    const market1 = makeTestMarket(7777)
    const expectedMarket1 = {
      cactus: 8,
      carrot: 8,
      celery: 5,
      melon: 7,
      potato: 8,
      sunflower: 6,
      tomato: 7,
    }
    expect(market1).toStrictEqual(expectedMarket1)
    const market2 = makeTestMarket(5555)
    const expectedMarket2 = {
      cactus: 6,
      carrot: 7,
      celery: 5,
      melon: 7,
      potato: 8,
      sunflower: 7,
      tomato: 7,
    }
    expect(market2).toStrictEqual(expectedMarket2)
    const market3 = makeTestMarket(undefined)
    expect(market3).not.toStrictEqual(market1)
  })
})
