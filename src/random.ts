import { Chance } from 'chance'
import { cardLibrary } from './card-library'

export const newChance = (seed: number | undefined): Chance.Chance =>
  seed === undefined ? Chance() : Chance(seed)

export const generateDraftDeck = (chance: Chance.Chance): ReadonlyArray<number> => {
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
