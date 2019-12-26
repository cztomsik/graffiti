/* @jsx m */
import * as m from 'mithril'
import axios from 'axios'
import * as open from 'open'

const Listing = ({ attrs: { key: category } }) => {
  let items = null
  let page = 1

  const fetch = async () => {
    items = null
    const { data } = await axios.get(`https://api.hnpwa.com/v0/${category}/${page}.json`)
    items = data
    m.redraw()
  }

  const next = () => (page++, fetch())
  const prev = () => (page--, fetch())

  return {
    oninit: fetch,

    view: () => (
      <Layout>
        <span>Showing page {page}</span>
        <div style={styles.row}>
          <button onclick={prev}>Previous page</button>
          <button onclick={next}>Next page</button>
        </div>

        <div>{items ? items.map(it => <Item {...it} />) : <span>Loading...</span>}</div>
      </Layout>
    )
  }
}

const Item: any = {
  view: ({ attrs: { id, title, url, points, time_ago, comments_count } }) => {
    const openItem = () => (url ? open(url) : m.route.set(`item/${id}`))

    return (
      <div style={styles.item}>
        <h3 style={styles.itemHeading} onclick={openItem}>
          {title}
        </h3>
        <div style={{ display: 'flex', justifyContent: 'space-between', maxWidth: 300 }}>
          <span>{points} points</span>
          <span>{time_ago}</span>
          <m.route.Link href={`/item/${id}`}>{comments_count} comments</m.route.Link>
        </div>
      </div>
    )
  }
}

const ItemDetail = ({ attrs: { key: id } }) => {
  let item = null

  const fetch = async () => {
    const { data } = await axios.get(`https://api.hnpwa.com/v0/item/${id}.json`)
    item = stripHtml(data)
    m.redraw()
  }

  return {
    oninit: fetch,

    view: () => (
      <Layout>
        {item ? (
          <div>
            <div style={{ marginBottom: 20 }}>
              <h2>{item.title}</h2>
              <a href={item.url}>{item.domain}</a>
            </div>

            <h5>Comments</h5>

            {item.comments.map(c => (
              <div style={{ marginTop: 20 }}>
                <h6>{c.user}</h6>
                <p>{c.content}</p>
              </div>
            ))}
          </div>
        ) : (
          <span>Loading...</span>
        )}
      </Layout>
    )
  }
}

const Layout: any = {
  view: ({ children }) => (
    <div>
      <div style={styles.header}>
        <m.route.Link href="/">News</m.route.Link>
        <m.route.Link href="/newest">Newest</m.route.Link>
        <m.route.Link href="/show">Show</m.route.Link>

        <span style={{ width: '65%' }} />

        <m.route.Link href="/ask">Ask</m.route.Link>
        <m.route.Link href="/jobs">Jobs</m.route.Link>
      </div>

      <div style={{ padding: 20 }}>{children}</div>
    </div>
  )
}

const styles = {
  row: {
    display: 'flex'
  },

  header: {
    padding: 20,
    display: 'flex',
    justifyContent: 'space-between',
    backgroundColor: '#f2e9c9'
  },

  item: {
    marginTop: 30
  },

  itemHeading: {
    fontSize: 18,
    lineHeight: 24,
    color: '#f00'
  }
}

m.route(document.body, '/news', {
  '/item/:key': ItemDetail,
  '/:key': Listing
})

const ENTITIES = {
  quot: '"',
  amp: '&',
  gt: '>',
  lt: '<'
}

const stripHtml = item => {
  for (const c of item.comments || []) {
    c.content = c.content
      .replace(/<\/?[^>]*>/g, '')
      .replace(/&#x([\dA-F]+);/g, (str, hex) => String.fromCharCode(parseInt(hex, 16)))
      .replace(/&(\w+);/g, (str, name) => ENTITIES[name])
  }

  return item
}
