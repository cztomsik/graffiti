import * as React from 'react'
import { useState, useMemo } from 'react'
import { View, Text, Button, StyleSheet } from '../../src/react'

const Bench = () => {
  const LIMIT = 200
  const bench = useBench(LIMIT)

  return (
    <View style={styles.ct}>
      <Text>
        This will do {LIMIT} updates, measure how long
        it will take and show the average number of updates per second.
        We use this to check if we are improving or not
      </Text>
      <Text>
        Run with VSYNC=0 to avoid capping to 60fps
      </Text>
      <Text>
        Last run took: {bench.time / 1000}s{'\n'}
        Updates per second: {LIMIT / (bench.time / 1000)}
      </Text>

      <Button title="Run" onPress={bench.run} />

      <Panel>
        <View style={{ backgroundColor: '#ff0000', height: 20, width: bench.num }} />
      </Panel>
    </View>
  )
}

function useBench(limit) {
  const [_, forceUpdate] = useState(10)

  const bench = useMemo(() => ({
    num: limit,
    start: 0,
    time: 0,
    running: false,
    run: () => {
      if (bench.running) {
        return
      }

      const tick = () => forceUpdate(bench.num += 1)
      const loop = () => {
        if (bench.num === limit) {
          bench.time = Date.now() - bench.start
          forceUpdate(0)
          bench.running = false
          return
        }

        tick()
        requestAnimationFrame(loop)
      }

      bench.running = true
      bench.num = 0
      bench.start = Date.now()

      requestAnimationFrame(loop)
    }
  }), [limit])

  return bench
}

const Panel = ({ children }) => <View style={styles.panel}>{children}</View>

const styles = StyleSheet.create({
  ct: { flex: 1, justifyContent: 'space-between' },

  panel: { padding: 20, borderWidth: 1, borderColor: '#eeeeee' }
})

export { Bench }
