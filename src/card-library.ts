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
  plantCard(11, 'potato'),
  plantCard(12, 'cactus'),
  contractCard(13, 'carrot', 3, 9),
  contractCard(14, 'celery', 5, 14),
  contractCard(15, 'melon', 4, 12),
  contractCard(16, 'tomato', 6, 18),
  contractCard(17, 'sunflower', 6, 18),
  contractCard(18, 'potato', 6, 18),
  contractCard(19, 'cactus', 6, 18),
] as const
