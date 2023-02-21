import { Chance } from 'chance'
import { cardLibrary } from './card-library.js'
import type { Deck, Market } from './model.js'

export const makeChance = (seed: number | undefined): Chance.Chance =>
  seed === undefined ? Chance() : Chance(seed)

export const generateDraftDeck = (chance: Chance.Chance): Deck => {
  const deck: number[] = cardLibrary.flatMap((card): number[] => {
    switch (card.cardType) {
      case 'terrain':
        return Array(8).fill(card.id)
      case 'plant':
        return Array(8).fill(card.id)
      case 'contract':
        return Array(2).fill(card.id)
    }
  })
  return chance.shuffle(deck)
}

export const generateMarket = (chance: Chance.Chance): Market => {
  const price = () => 4 + chance.d4()
  return {
    carrot: price(),
    celery: price(),
    melon: price(),
    tomato: price(),
    sunflower: price(),
    potato: price(),
    cactus: price(),
  }
}
