import { Card, contractCard, plantCard, terrainCard } from './model.js'

export const cardLibrary: ReadonlyArray<Card> = [
  terrainCard(1, 'prairie'),
  terrainCard(2, 'desert'),
  terrainCard(3, 'jungle'),
  terrainCard(4, 'mountain'),
  terrainCard(5, 'lake'),
  plantCard(6, 'carrot'),
  plantCard(7, 'celery'),
  plantCard(8, 'melon'),
  plantCard(9, 'tomato'),
  plantCard(10, 'sunflower'),
  contractCard(11, 'carrot', 3, 9),
  contractCard(12, 'celery', 5, 14),
  contractCard(13, 'melon', 4, 12),
  contractCard(14, 'tomato', 6, 18),
] as const
