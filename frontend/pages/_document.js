import { Html, Head, Main, NextScript } from 'next/document'

export default function Document() {
  return (
    <Html lang="en">
      <Head />
      <body className='d-flex flex-column' data-bs-theme="dark">
        <Main />
        <NextScript />
      </body>
    </Html>
  )
}
