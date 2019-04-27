import * as React from 'react'
import { View, Text, StyleSheet } from '.'

class ErrorBoundary extends React.Component {
  state = { error: null }

  static getDerivedStateFromError(error) {
    return { error }
  }

  componentDidCatch(error, info) {
    console.log('ERR', error, info);
  }

  render() {
    return this.state.error ?this.renderError() :this.props.children
  }

  renderError() {
    const e = this.state.error

    return (
      <View style={styles.container}>
        <Text style={styles.headerText}>Error</Text>

        <Text style={styles.messageText}>{e.message}</Text>

        <Text style={styles.messageText}>{e.stack}</Text>
      </View>
    )
  }

  static wrap(vnode) {
    return <this>{vnode}</this>
  }
}

const styles = StyleSheet.create({
  container: {
    padding: 10
  },

  headerText: {
    color: '#ff0000',
    fontSize: 20
  },

  messageText: {
    fontSize: 12,
    lineHeight: 14
  }
})

export default ErrorBoundary
