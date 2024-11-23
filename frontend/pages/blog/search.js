import { Metadata, PostItem, TransitionAnimator } from '@/components'
import { getWebsocketURL } from '@/config'
import { faCheck, faRotate, faXmark } from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { useRouter } from 'next/router'
import { useEffect, useState } from 'react'
import Head from 'next/head'
import Link from 'next/link'

const States = {
  Error: <FontAwesomeIcon className='w-100' icon={faXmark} style={{ color: "var(--red)" }} title='Error' />,
  Loading: <FontAwesomeIcon className='w-100 spinner' icon={faRotate} style={{ color: "var(--blue)" }} title='Loading...' />,
  Done: <FontAwesomeIcon className='w-100' icon={faCheck} style={{ color: "var(--green)" }} title='Done!' />,
}

export function replaceHighlights(input, slug) {
  return input.split(/{~(.*?)~}/g).map((part, index) => {
    if (index % 2 === 1) {
      if (slug) {
        const href = `/blog/p/${slug}#:~:text=${encodeURIComponent(part)}`
        return <Link key={index} href={href} className='no-style' style={{ backgroundColor: "rgb(255 255 255 / 15%)" }}>{part}</Link>;
      } else {
        return <span key={index} style={{ backgroundColor: "rgb(255 255 255 / 15%)" }}>{part}</span>;
      }
    } else {
      return part;
    }
  });
}

export default function Search() {
  const [socket, setSocket] = useState(null)
  const [results, setResults] = useState([])
  const [loading, setLoading] = useState(States.Loading)
  const { q } = useRouter().query;

  useEffect(() => {
    const createSocket = () => {
      setLoading(States.Loading)
      const ws = new WebSocket(getWebsocketURL('/blog/search'))

      ws.onopen = () => {
        setSocket(ws)
        ws.send(q || '')
      }

      ws.onmessage = (e) => {
        const results = JSON.parse(e.data).map((post) => {
          // Replace description with highlighted content
          if (post.markdown.includes('{~')) {
            post.description = '… ' + post.markdown.replaceAll('...', '…') + ' …'
          }
          post.title = replaceHighlights(post.title)
          post.description = replaceHighlights(post.description, post.slug)
          return post
        })
        setResults(results)
        setLoading(States.Done)
      }

      ws.onclose = (e) => {
        console.error("Socket closed unexpectedly:", e.reason)
        setSocket(null)
        setLoading(States.Error)
        setTimeout(createSocket, 2000)
      }
      ws.onerror = (e) => {
        console.error("Socket error:", e)
        ws.close()
      }

      return () => ws.close()
    }

    createSocket()
  }, [q])

  return <>
    <Metadata title="Blog - Search" description="Search through all posts on my blog about cybersecurity. Quickly find what you're looking for by typing in the search bar." />
    <Head>
      <link rel="alternate" type="application/rss+xml" href="https://jorianwoltjer.com/blog/rss.xml" title="Blog | Jorian Woltjer" />
    </Head>
    <h1>Search</h1>
    <TransitionAnimator>
      <div className="input-group mb-3">
        <span className="input-group-text" style={{ width: "50px" }}>{loading}</span>
        <input className="form-control form-control-lg" type="text" placeholder="Search..." defaultValue={q} autoComplete="off" autoFocus
          onBlur={(e) => {
            const queryString = e.target.value ? `?q=${encodeURIComponent(e.target.value)}` : '';
            history.replaceState({}, '', '/blog/search' + queryString)
          }}
          onInput={(e) => {
            if (socket) {
              socket.send(e.target.value)
              setLoading(States.Loading)
            }
          }} />
      </div>
      {results.length > 0 ? results.map((post) =>
        <PostItem key={post.id} {...post} />
      ) : loading === States.Done && <p className="lead text-muted">No results found.</p>}
    </TransitionAnimator>
  </>
}
