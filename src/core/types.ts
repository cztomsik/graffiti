import * as yoga from 'yoga-layout'
import { BridgeBrush, BridgeClip, BridgeRect } from './ResourceManager'

export interface Container<T> {
  appendChild(child: T): void
  insertBefore(child: T, before: T): void
  removeChild(child: T): void
}

export interface HasLayout {
  yogaNode: yoga.YogaNode
}

export type DrawBrushFunction = (
  item: BridgeBrush | BridgeClip,
  rect: BridgeRect
) => void
