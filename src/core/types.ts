import * as yoga from 'yoga-layout'
import { BridgeBrush, BridgeClip } from './ResourceManager'

export interface Container<T> {
  appendChild(child: T): void
  insertBefore(child: T, before: T): void
  removeChild(child: T): void
}
