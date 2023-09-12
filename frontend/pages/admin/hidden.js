import { Loading, Metadata, PostItem } from "@/components"
import { BACKEND_API } from "@/config"
import useSWR from 'swr'

const fetcher = url => fetch(url).then(res => res.json());

export default function Hidden() {
    const { data: content } = useSWR(BACKEND_API + "/blog/hidden", fetcher)

    return <>
        <Metadata title="Hidden Posts" />
        <h1>Hidden Posts</h1>
        {content ? content.map(post => (
            <PostItem key={post.id} {...post} />
        )) : <Loading />}
    </>
}
