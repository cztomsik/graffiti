import * as React from 'react'
import { useState } from 'react'
import {
  View,
  Button,
  Text,
  FlatList,
  StyleSheet,
  TextInput
} from '../../src/react'

const initialMessages = [
  { text: 'Hello world!' },
  { text: 'Lorem ipsum' },
  { text: 'Dolor sit amet' },
  { text: 'lorem' },
  { text: 'ipsum' },
  { text: 'dolor' },
  { text: 'sit amet' },
  { text: 'lorem ipsum' },
  { text: 'dolor' },
  { text: 'sit amet' },
  { text: 'lorem ipsum' }
]

export function Messages() {
  const [text, setText] = useState('')
  const [messages, setMessages] = useState(initialMessages)
  const send = () => {
    setMessages([...messages, { text }])
    setText('')
  }
  const sendOnEnter = (e) => {
    if (e.code === 'Enter') {
      send()
    }
  }

  return (
    <View style={{ flex: 1, width: 400 }}>
      <FlatList
        data={messages}
        renderItem={({ item, index }) => (
          <MessageItem key={index} item={item} me={index % 2 === 0} />
        )}
        contentContainerStyle={{ alignItems: 'flex-start' }}
      />

      <View style={{ flexDirection: 'row', marginTop: 10 }} onKeyDown={sendOnEnter}>
        <TextInput style={styles.input} value={text} onChangeText={setText} />
        <Button title="Send" onPress={send} />
      </View>
    </View>
  )
}

function MessageItem({ item, me }) {
  return (
    <View style={[styles.msg, me ? styles.msgMe : styles.msgYou]}>
      <Text style={[styles.text, me && styles.textMe]}>{item.text}</Text>
    </View>
  )
}

const styles = StyleSheet.create({
  msg: {
    borderRadius: 4,
    backgroundColor: '#ccc',
    padding: 5,
    paddingHorizontal: 12,
    marginVertical: 4
  },

  msgMe: {
    alignSelf: 'flex-end',
    backgroundColor: '#07f',
    borderTopLeftRadius: 14,
    borderBottomLeftRadius: 14
  },

  msgYou: {
    borderTopRightRadius: 14,
    borderBottomRightRadius: 14
  },

  text: {
    textAlign: 'right',
    lineHeight: 24
  },

  textMe: {
    color: '#fff',
    textAlign: 'left'
  },

  input: {
    flex: 1,
    marginRight: 10
  }
})
