import { PostItem } from '@/components'
import { getWebsocketURL } from '@/config'
import { faCheck, faRotate, faXmark } from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { useEffect, useState } from 'react'

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
                return <a key={index} href={href} className='no-style' style={{ backgroundColor: "rgb(255 255 255 / 15%)" }}>{part}</a>;
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

    useEffect(() => {
        const ws = new WebSocket(getWebsocketURL('/blog/search'))

        ws.onopen = () => {
            console.log('Connected!')
            setSocket(ws)
            ws.send('')
        }

        ws.onmessage = (e) => {
            console.log(e)
            const results = JSON.parse(e.data).map((post) => {
                if (post.markdown.includes('{~')) {  // Replace description with highlighted content
                    post.description = '… ' + post.markdown.replace('...', '…') + ' …'
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
        }
        ws.onerror = (e) => {
            console.error("Socket error:", e)
            ws.close()
        }

        return () => ws.close()
    }, [])

    return <>
        <h1>Search</h1>
        <div className="input-group mb-3">
            <span className="input-group-text" style={{ width: "50px" }}>{loading}</span>
            <input className="form-control form-control-lg" type="text" placeholder="Search..." autoComplete="off" autoFocus
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
    </>
}
