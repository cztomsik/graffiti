import * as React from 'react'
import { useState, useEffect } from 'react'
import {
  View,
  Button,
  Text,
  FlatList,
  StyleSheet,
  ActivityIndicator,
  Linking
} from '../../src/react'
import * as http from 'http'

export function HackerNews() {
  const [page, setPage] = useState(1)
  const [submissions, setSubmissions] = useState(null as [])
  const [selection, setSelection] = useState(null)

  useEffect(() => {
    setSubmissions(null)

    getJSON(`http://node-hnapi.herokuapp.com/news?page=${page}`).then(items => {
      setSubmissions(items)
      setSelection(items[0])
    })
  }, [page])

  return (
    <View style={{ flex: 1, flexDirection: 'row' }}>
      {submissions && selection ? (
        <>
          <View style={{ flex: 2 }}>
            <FlatList
              data={submissions}
              renderItem={({ item, index }) => (
                <SubmissionListItem
                  key={index}
                  submission={item}
                  active={selection === item}
                  onClick={() => setSelection(item)}
                />
              )}
            />

            <Button title="Next page" onPress={() => setPage(page + 1)} />
          </View>

          <View style={{ flex: 3, marginLeft: 10 }}>
            <SubmissionDetail submission={selection} />
          </View>
        </>
      ) : (
        <ActivityIndicator />
      )}
    </View>
  )
}

const SubmissionListItem = ({
  submission: { title, time_ago, comments_count },
  active,
  onClick
}) => (
  <View style={[styles.item, active && styles.itemActive]} onClick={onClick}>
    <Text style={styles.itemTitle}>{title}</Text>
    <View style={styles.rowBetween}>
      <Text style={styles.meta}>{comments_count} comments</Text>
      <Text style={styles.meta}>{time_ago}</Text>
    </View>
  </View>
)

const SubmissionDetail = ({ submission: { title, id, url } }) => {
  const [detail, setDetail] = useState(null)

  useEffect(() => {
    setDetail(null)
    getJSON(`http://node-hnapi.herokuapp.com/item/${id}`).then(stripHtml).then(setDetail)
  }, [id])

  return (
    <View style={{ flex: 1 }}>
      <Text style={styles.heading}>{title}</Text>
      <Text
        style={[styles.meta, styles.link]}
        onClick={() => Linking.openURL(url)}
      >
        {url}
      </Text>

      {/* TODO: this is far from being useful, actually */}
      {detail ? (
        <FlatList
          data={detail.comments}
          renderItem={({ item, index }: any) => (
            <View key={index} style={{ margin: 10 }}>
              <Text style={styles.meta}>{item.user}</Text>
              <Text style={{ lineHeight: 18 }}>{item.content}</Text>
            </View>
          )}
        />
      ) : (
        <ActivityIndicator />
      )}
    </View>
  )
}

const styles = StyleSheet.create({
  item: {
    padding: 10
  },

  itemActive: {
    backgroundColor: '#eee'
  },

  itemTitle: {
    lineHeight: 18,
    marginBottom: 5
  },

  rowBetween: {
    flexDirection: 'row',
    justifyContent: 'space-between'
  },

  meta: {
    fontSize: 14,
    lineHeight: 18,
    color: '#666'
  },

  link: {
    color: '#00f'
  },

  heading: {
    fontSize: 24
  }
})

const getJSON = url => {
  return new Promise(resolve => {
    http.get(url, res => {
      res.setEncoding('utf-8')

      let data = ''
      res.on('data', chunk => (data += chunk))
      res.on('end', () => resolve(JSON.parse(data)))
    })
  }) as Promise<any>
}

const ENTITIES = {
  quot: '"',
  amp: '&',
  gt: '>',
  lt: '<'
}

const stripHtml = item => {
  for (const c of item.comments || []) {
    c.content = c.content.replace(/<\/?[^>]*>/g, '').replace(/&#x([\dA-F]+);/g, (str, hex) => String.fromCharCode(parseInt(hex, 16))).replace(/&(\w+);/g, (str, name) => ENTITIES[name])
  }

  return item
}
