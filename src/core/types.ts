import * as yoga from 'yoga-layout'

export interface Container<T> {
  appendChild(child: T): void
  insertBefore(child: T, before: T): void
  removeChild(child: T): void
}

export interface HasLayout {
  yogaNode: yoga.YogaNode
}
