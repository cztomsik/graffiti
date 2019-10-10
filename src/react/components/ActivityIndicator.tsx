import * as React from 'react'
import { useState, useEffect } from 'react'
import { View, StyleSheet } from '..';

export function ActivityIndicator() {
  const [alpha, setAlpha] = useState(0)

  useEffect(() => {
    let a = alpha
    let running = true

    requestAnimationFrame(function loop(t) {
      if (running) {
        setAlpha((t / 100) % 9)
        requestAnimationFrame(loop)
      }
    })

    return () => running = false
  }, [])

  return <View style={[styles.indicator, { backgroundColor: `#ccf${Math.floor(alpha + 1)}` }]}></View>
}

const styles = StyleSheet.create({
  indicator: {
    width: 20,
    height: 20,
    borderRadius: 20
  }
})
