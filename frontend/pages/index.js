import Link from "next/link";

export default function Home({ content }) {
  return (
    <>
      <h1>Hello, world!</h1>
      <Link href="/blog">Blog</Link>
    </>
  )
}
