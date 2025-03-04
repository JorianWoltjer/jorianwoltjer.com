import Head from 'next/head';
import { useRouter } from 'next/router';

export default function Metadata({ title, description, img }) {
  const router = useRouter();

  title += " | Jorian Woltjer";

  return <>
    <Head>
      <title>{title}</title>
      <meta name="theme-color" content="#3498db" />
      <link rel="icon" type="image/png" href="/favicon-96x96.png" sizes="96x96" />
      <link rel="icon" type="image/svg+xml" href="favicon.svg" />
      <link rel="shortcut icon" href="/favicon.ico" />
      <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />
      <meta name="apple-mobile-web-app-title" content="Jorian" />
      <link rel="manifest" href="/site.webmanifest" />
      {title && <>
        <meta property="og:title" content={title} />
        <meta name="twitter:title" content={title} />
      </>}
      {description && <>
        <meta name="description" content={description} />
        <meta property="og:description" content={description} />
        <meta name="twitter:description" content={description} />
      </>}
      {img ? <>
        <meta property="og:image" content={`https://jorianwoltjer.com${img}`} />
        <meta name="twitter:image" content={`https://jorianwoltjer.com${img}`} />
        <meta name="twitter:card" content="summary_large_image" />
      </> : <>
        <meta property="og:image" content="https://jorianwoltjer.com/img/logo.png" />
        <meta name="twitter:image" content="https://jorianwoltjer.com/img/logo.png" />
        <meta name="twitter:card" content="summary" />
      </>}
      <meta property="og:type" content="website" />
      <meta property="og:url" content={`https://jorianwoltjer.com${router.asPath}`} />
      <meta name="twitter:url" content={`https://jorianwoltjer.com${router.asPath}`} />
      <meta property="og:site_name" content="jorianwoltjer.com" />
      <meta name="twitter:domain" content="jorianwoltjer.com" />
    </Head>
  </>
}
