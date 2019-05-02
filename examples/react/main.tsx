import * as React from 'react'
import { getApp } from '../../src/core'
import { render } from '../../src/react'
import { App } from './App';

const window = getApp().createWindow()

let Root = App

const renderRoot = () =>
  render(<Root />, window)

if ('hot' in module) {
  (module as any).hot.accept('./App', (file) => {
    Root = require(file).App
    renderRoot()
  })
}

renderRoot()
