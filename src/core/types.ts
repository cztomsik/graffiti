import * as yoga from 'yoga-layout'
import { BucketId, BridgeRect } from './ResourceManager'

export interface Container<T> {
  appendChild(child: T): void
  insertBefore(child: T, before: T): void
  removeChild(child: T): void
}

export interface HasLayout {
  yogaNode: yoga.YogaNode
}

export type DrawBrushFunction = (buckets: BucketId[], rect: BridgeRect) => void
