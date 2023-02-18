import { Card, contractCard, plantCard, terrainCard } from './model.js'

export const cardLibrary: ReadonlyArray<Card> = [
  terrainCard(1, 'desert'),
  terrainCard(2, 'jungle'),
  terrainCard(3, 'mountain'),
  terrainCard(4, 'lake'),
  plantCard(5, 'carrot'),
  plantCard(6, 'celery'),
  plantCard(7, 'melon'),
  plantCard(8, 'tomato'),
  contractCard(9, 'carrot', 3, 9),
  contractCard(10, 'celery', 5, 14),
  contractCard(11, 'melon', 4, 12),
  contractCard(12, 'tomato', 6, 18),
] as const
