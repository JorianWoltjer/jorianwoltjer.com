import Link from 'next/link';

export default function Breadcrumbs({ slug, title }) {
    return <div className='breadcrumbs'>
        <Link href='/blog'>~</Link>
        {slug.split("/").slice(0, -1).map((slug, i) => {
            const path = slug.split("/").slice(0, i + 1).join("/")

            return <Link key={i} href={`/blog/f/${path}`}>{slug}</Link>
        })}
        <h1>{title}</h1>
    </div>
}