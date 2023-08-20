import { BACKEND } from "@/config";

export default function Home({ content }) {
  return (
    <>
      <h1>Hello, world!</h1>
      <pre><code>{content}</code></pre>
    </>
  )
}

export async function getStaticProps() {
  let content = await fetch(BACKEND + "/query").then(res => res.text());

  return {
    props: {
      content
    }
  }
}
