/*
type Hand = number[]
type Deck = number[]
type DeployedTerrain = { terrainId: number, plantId?: number }
type Slot = { terrain?: DeployedTerrain }
type Garden = Slot[][]
type CardType = 'terrain' | 'plant' | 'contract'
*/
export type Market = Record<PlantType, number>

type TerrainType = 'prairie' | 'desert' | 'jungle' | 'mountain' | 'lake'
type PlantType =
  | 'carrot'
  | 'celery'
  | 'melon'
  | 'tomato'
  | 'sunflower'
  | 'potato'
  | 'cactus'

export type Card = TerrainCard | PlantCard | ContractCard

type TerrainCard = Readonly<{
  cardType: 'terrain'
  id: number
  terrainType: TerrainType
}>

export const terrainCard = (id: number, terrainType: TerrainType): TerrainCard =>
  ({
    cardType: 'terrain',
    id,
    terrainType,
  } as const)

type PlantCard = Readonly<{
  cardType: 'plant'
  id: number
  plantType: PlantType
}>

export const plantCard = (id: number, plantType: PlantType): PlantCard =>
  ({
    cardType: 'plant',
    id,
    plantType,
  } as const)

type ContractCard = Readonly<{
  cardType: 'contract'
  id: number
  plantType: PlantType
  quantity: number
  payment: number
}>

export const contractCard = (
  id: number,
  plantType: PlantType,
  quantity: number,
  payment: number,
): ContractCard =>
  ({
    cardType: 'contract',
    id,
    plantType,
    quantity,
    payment,
  } as const)
