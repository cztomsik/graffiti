import * as React from 'react'
import { getApp } from '../../src'
import { render } from '../../src/react'
import { App } from './App';

const window = getApp().createWindow()

let Root = App

const renderRoot = () =>
  render(<Root />, window)

if (module['hot']) {
  module['hot'].onChange(() => {
    Root = require('./App').App
    renderRoot()
  })
}

renderRoot()
