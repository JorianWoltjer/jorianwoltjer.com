import Link from 'next/link';

export default function Breadcrumbs({ slug, title }) {
    return <nav aria-label='breadcrumb'>
        <ol className='breadcrumb my-4'>
            <li className='breadcrumb-item'><Link href='/blog' style={{ padding: "10px", margin: "-10px" }}>~</Link></li>
            {slug.split("/").slice(0, -1).map((part, i) => {
                const path = slug.split("/").slice(0, i + 1).join("/")

                return <li key={i} className='breadcrumb-item'><Link href={`/blog/f/${path}`}>{part}</Link></li>
            })}
            <li className='breadcrumb-item active' aria-current='page'><h1 className='breadcrumb-title'>{title}</h1></li>
        </ol>
    </nav>
}
